#!/usr/bin/env python3
"""BRC-20 v2 proof-of-concept reference implementation."""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
from copy import deepcopy
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Any, Dict, List


@dataclass
class VestingSchedule:
    total: int
    start_height: int
    cliff_height: int
    duration: int
    spent: int = 0

    def unlocked(self, height: int) -> int:
        if height < self.cliff_height:
            return 0
        if height >= self.start_height + self.duration:
            return self.total
        if height < self.start_height:
            return 0
        elapsed = height - self.start_height
        unlocked = int(self.total * (elapsed / self.duration))
        return min(self.total, max(0, unlocked))

    def available(self, height: int) -> int:
        return max(0, self.unlocked(height) - self.spent)


DEFAULT_VERSION = "brc20v2-poc-0.1"


def sha256_hex(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical_json(obj: Any) -> bytes:
    return json.dumps(obj, sort_keys=True, separators=(",", ":")).encode("utf-8")


def compute_merkle_root(balances: Dict[str, int]) -> str:
    if not balances:
        return sha256_hex(b"")
    leaves = [sha256_hex(f"{addr}:{amount}".encode("utf-8")) for addr, amount in sorted(balances.items())]
    while len(leaves) > 1:
        if len(leaves) % 2 == 1:
            leaves.append(leaves[-1])
        leaves = [sha256_hex((leaves[i] + leaves[i + 1]).encode("utf-8")) for i in range(0, len(leaves), 2)]
    return leaves[0]


def compute_state_hash(state: Dict[str, Any]) -> str:
    payload = deepcopy(state)
    payload.pop("state_hash", None)
    return sha256_hex(canonical_json(payload))


def load_state(path: Path) -> Dict[str, Any]:
    if not path.exists():
        return {}
    return json.loads(path.read_text())


def save_state(path: Path, state: Dict[str, Any]) -> None:
    path.write_text(json.dumps(state, indent=2, sort_keys=True))


def ensure_state(state: Dict[str, Any], args: argparse.Namespace) -> Dict[str, Any]:
    if state:
        return state
    if not args.token_id or not args.name or not args.symbol:
        raise ValueError("token metadata required for initial mint")
    return {
        "version": DEFAULT_VERSION,
        "token_id": args.token_id,
        "name": args.name,
        "symbol": args.symbol,
        "decimals": args.decimals,
        "max_supply": args.max_supply,
        "minted_supply": 0,
        "rules": {
            "soulbound": args.soulbound,
            "transferable": not args.soulbound,
            "vesting": {"enabled": False},
        },
        "prev_state_hash": "",
        "state_hash": "",
        "merkle_root": "",
        "balances": {},
        "vesting": {},
        "ledger": [],
    }


def apply_vesting(state: Dict[str, Any], address: str, schedule: VestingSchedule) -> None:
    state.setdefault("vesting", {})[address] = {
        "total": schedule.total,
        "start_height": schedule.start_height,
        "cliff_height": schedule.cliff_height,
        "duration": schedule.duration,
        "spent": schedule.spent,
    }
    state["rules"]["vesting"]["enabled"] = True


def get_vesting(state: Dict[str, Any], address: str) -> VestingSchedule | None:
    raw = state.get("vesting", {}).get(address)
    if not raw:
        return None
    return VestingSchedule(**raw)


def bump_hashes(state: Dict[str, Any]) -> None:
    state["merkle_root"] = compute_merkle_root(state.get("balances", {}))
    state["state_hash"] = compute_state_hash(state)


def mint(args: argparse.Namespace) -> Dict[str, Any]:
    state = ensure_state(load_state(args.state), args)
    amount = args.amount
    if amount <= 0:
        raise ValueError("mint amount must be positive")
    if state["minted_supply"] + amount > state["max_supply"]:
        raise ValueError("mint exceeds max supply")

    state["prev_state_hash"] = state.get("state_hash", "")
    state["minted_supply"] += amount
    state["balances"][args.to] = state["balances"].get(args.to, 0) + amount

    if args.vesting_start is not None:
        schedule = VestingSchedule(
            total=amount,
            start_height=args.vesting_start,
            cliff_height=args.vesting_cliff or args.vesting_start,
            duration=args.vesting_duration or 1,
            spent=0,
        )
        apply_vesting(state, args.to, schedule)

    state["ledger"].append(
        {
            "op": "mint",
            "timestamp": datetime.utcnow().isoformat() + "Z",
            "tx": {
                "to": args.to,
                "amount": amount,
                "issuer": args.issuer,
            },
        }
    )
    bump_hashes(state)
    return state


def enforce_transfer_rules(state: Dict[str, Any], sender: str, amount: int, height: int) -> None:
    if state["rules"].get("soulbound"):
        raise ValueError("token is soulbound; transfers are disabled")
    if amount <= 0:
        raise ValueError("transfer amount must be positive")
    if state["balances"].get(sender, 0) < amount:
        raise ValueError("insufficient balance")

    vesting = get_vesting(state, sender)
    if vesting:
        available = vesting.available(height)
        if amount > available:
            raise ValueError("transfer exceeds vested balance")


def apply_vesting_spend(state: Dict[str, Any], sender: str, amount: int) -> None:
    schedule = get_vesting(state, sender)
    if not schedule:
        return
    schedule.spent += amount
    apply_vesting(state, sender, schedule)


def transfer(args: argparse.Namespace) -> Dict[str, Any]:
    state = load_state(args.state)
    if not state:
        raise ValueError("state file not found; mint first")

    amount = args.amount
    height = args.height
    enforce_transfer_rules(state, args.from_addr, amount, height)

    state["prev_state_hash"] = state.get("state_hash", "")
    state["balances"][args.from_addr] -= amount
    state["balances"][args.to] = state["balances"].get(args.to, 0) + amount
    apply_vesting_spend(state, args.from_addr, amount)

    state["ledger"].append(
        {
            "op": "transfer",
            "timestamp": datetime.utcnow().isoformat() + "Z",
            "tx": {
                "from": args.from_addr,
                "to": args.to,
                "amount": amount,
                "height": height,
            },
        }
    )
    bump_hashes(state)
    return state


def export_state(args: argparse.Namespace) -> None:
    state = load_state(args.state)
    if not state:
        raise ValueError("state file not found")
    output = json.dumps(state, indent=2, sort_keys=True)
    if args.output:
        Path(args.output).write_text(output)
    else:
        print(output)


def verify_state(args: argparse.Namespace) -> None:
    state = load_state(args.state)
    if not state:
        raise ValueError("state file not found")
    expected_merkle = compute_merkle_root(state.get("balances", {}))
    expected_hash = compute_state_hash(state)

    merkle_ok = expected_merkle == state.get("merkle_root")
    hash_ok = expected_hash == state.get("state_hash")
    if not merkle_ok or not hash_ok:
        raise ValueError(
            "verification failed: "
            + ", ".join(
                part
                for part, ok in ("merkle_root", merkle_ok), ("state_hash", hash_ok)
                if not ok
            )
        )
    print("verification ok")


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="BRC-20 v2 proof-of-concept CLI")
    parser.add_argument("--state", type=Path, default=Path("poc_state.json"))

    subparsers = parser.add_subparsers(dest="command", required=True)

    mint_parser = subparsers.add_parser("mint", help="mint new tokens")
    mint_parser.add_argument("--token-id")
    mint_parser.add_argument("--name")
    mint_parser.add_argument("--symbol")
    mint_parser.add_argument("--decimals", type=int, default=0)
    mint_parser.add_argument("--max-supply", type=int, default=21_000_000)
    mint_parser.add_argument("--issuer", required=True)
    mint_parser.add_argument("--to", required=True)
    mint_parser.add_argument("--amount", type=int, required=True)
    mint_parser.add_argument("--soulbound", action="store_true")
    mint_parser.add_argument("--vesting-start", type=int)
    mint_parser.add_argument("--vesting-cliff", type=int)
    mint_parser.add_argument("--vesting-duration", type=int)
    mint_parser.set_defaults(func=mint)

    transfer_parser = subparsers.add_parser("transfer", help="transfer tokens")
    transfer_parser.add_argument("--from", dest="from_addr", required=True)
    transfer_parser.add_argument("--to", required=True)
    transfer_parser.add_argument("--amount", type=int, required=True)
    transfer_parser.add_argument("--height", type=int, default=0)
    transfer_parser.set_defaults(func=transfer)

    export_parser = subparsers.add_parser("export", help="export state")
    export_parser.add_argument("--output")
    export_parser.set_defaults(func=export_state)

    verify_parser = subparsers.add_parser("verify", help="verify state integrity")
    verify_parser.set_defaults(func=verify_state)

    return parser


def main() -> None:
    parser = build_parser()
    args = parser.parse_args()
    try:
        result = args.func(args)
    except Exception as exc:
        print(f"error: {exc}", file=sys.stderr)
        sys.exit(1)

    if isinstance(result, dict):
        save_state(args.state, result)
        print(f"state updated: {args.state}")


if __name__ == "__main__":
    main()
