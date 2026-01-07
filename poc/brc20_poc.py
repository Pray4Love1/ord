#!/usr/bin/env python3
"""Reference BRC-20 v2 Python implementation (executable spec)."""

from __future__ import annotations

import argparse
import hashlib
import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Dict, List, Optional

ZERO_HASH = "0" * 64


def _canonical_json(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def _sha256_hex(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _balance_leaf(address: str, amount: str) -> str:
    payload = f"{address}:{amount}".encode()
    return _sha256_hex(payload)


def compute_merkle_root(balances: Dict[str, str]) -> str:
    if not balances:
        return _sha256_hex(b"")

    level = sorted(_balance_leaf(addr, amt) for addr, amt in balances.items())
    while len(level) > 1:
        if len(level) % 2 == 1:
            level.append(level[-1])
        next_level = []
        for left, right in zip(level[0::2], level[1::2]):
            combined = bytes.fromhex(left) + bytes.fromhex(right)
            next_level.append(_sha256_hex(combined))
        level = next_level
    return level[0]


@dataclass
class TokenRules:
    soulbound: bool = False
    max_per_tx: Optional[str] = None

    def to_dict(self) -> Dict[str, Any]:
        return {"soulbound": self.soulbound, "max_per_tx": self.max_per_tx}


@dataclass
class TokenMetadata:
    name: Optional[str] = None
    description: Optional[str] = None

    def to_dict(self) -> Dict[str, Any]:
        data: Dict[str, Any] = {}
        if self.name is not None:
            data["name"] = self.name
        if self.description is not None:
            data["description"] = self.description
        return data


@dataclass
class TokenState:
    symbol: str
    max_supply: str
    decimals: int
    balances: Dict[str, str] = field(default_factory=dict)
    metadata: TokenMetadata = field(default_factory=TokenMetadata)
    rules: TokenRules = field(default_factory=TokenRules)
    vesting: List[Dict[str, Any]] = field(default_factory=list)
    prev_state_hash: str = ZERO_HASH

    def to_canonical_dict(self) -> Dict[str, Any]:
        return {
            "version": "2",
            "token": {
                "symbol": self.symbol,
                "max_supply": self.max_supply,
                "decimals": self.decimals,
            },
            "balances": dict(sorted(self.balances.items())),
            "metadata": self.metadata.to_dict(),
            "rules": self.rules.to_dict(),
            "vesting": list(self.vesting),
            "prev_state_hash": self.prev_state_hash,
            "merkle_root": compute_merkle_root(self.balances),
        }

    def compute_state_hash(self) -> str:
        payload = _canonical_json(self.to_canonical_dict()).encode()
        return _sha256_hex(payload)

    def to_export_dict(self) -> Dict[str, Any]:
        data = self.to_canonical_dict()
        data["state_hash"] = self.compute_state_hash()
        return data

    def mint(self, address: str, amount: int) -> None:
        self._apply_amount(address, amount)

    def transfer(self, sender: str, recipient: str, amount: int) -> None:
        if self.rules.soulbound:
            raise ValueError("Transfers disabled (soulbound token)")
        if self.rules.max_per_tx is not None and amount > int(self.rules.max_per_tx):
            raise ValueError("Transfer exceeds max_per_tx")
        self._apply_amount(sender, -amount)
        self._apply_amount(recipient, amount)

    def _apply_amount(self, address: str, delta: int) -> None:
        current = int(self.balances.get(address, "0"))
        next_value = current + delta
        if next_value < 0:
            raise ValueError("Insufficient balance")
        self.balances[address] = str(next_value)


def load_state(path: Path) -> TokenState:
    data = json.loads(path.read_text())
    token = data["token"]
    metadata = TokenMetadata(**data.get("metadata", {}))
    rules = TokenRules(**data.get("rules", {}))
    state = TokenState(
        symbol=token["symbol"],
        max_supply=token["max_supply"],
        decimals=token["decimals"],
        balances=data.get("balances", {}),
        metadata=metadata,
        rules=rules,
        vesting=data.get("vesting", []),
        prev_state_hash=data.get("prev_state_hash", ZERO_HASH),
    )
    return state


def save_state(path: Path, state: TokenState) -> None:
    path.write_text(_canonical_json(state.to_export_dict()) + "\n")


def validate_state(state: TokenState, payload: Dict[str, Any]) -> List[str]:
    errors = []
    expected_merkle = compute_merkle_root(state.balances)
    if payload.get("merkle_root") != expected_merkle:
        errors.append("merkle_root mismatch")
    expected_hash = state.compute_state_hash()
    if payload.get("state_hash") != expected_hash:
        errors.append("state_hash mismatch")
    return errors


def command_init(args: argparse.Namespace) -> None:
    state = TokenState(
        symbol=args.symbol,
        max_supply=str(args.max_supply),
        decimals=args.decimals,
        rules=TokenRules(soulbound=args.soulbound, max_per_tx=args.max_per_tx),
        metadata=TokenMetadata(name=args.name, description=args.description),
    )
    save_state(args.state, state)


def command_mint(args: argparse.Namespace) -> None:
    state = load_state(args.state)
    state.prev_state_hash = json.loads(args.state.read_text()).get("state_hash", ZERO_HASH)
    state.mint(args.to, args.amount)
    save_state(args.state, state)


def command_transfer(args: argparse.Namespace) -> None:
    state = load_state(args.state)
    state.prev_state_hash = json.loads(args.state.read_text()).get("state_hash", ZERO_HASH)
    state.transfer(args.sender, args.recipient, args.amount)
    save_state(args.state, state)


def command_export(args: argparse.Namespace) -> None:
    state = load_state(args.state)
    output = state.to_export_dict()
    args.out.write_text(_canonical_json(output) + "\n")


def command_verify(args: argparse.Namespace) -> None:
    payload = json.loads(args.state.read_text())
    state = load_state(args.state)
    errors = validate_state(state, payload)
    if errors:
        raise SystemExit("\n".join(errors))
    print("state verified")


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="BRC-20 v2 reference PoC")
    subparsers = parser.add_subparsers(dest="command", required=True)

    init_parser = subparsers.add_parser("init", help="Create a new token state file")
    init_parser.add_argument("--symbol", required=True)
    init_parser.add_argument("--max-supply", required=True)
    init_parser.add_argument("--decimals", type=int, default=0)
    init_parser.add_argument("--state", type=Path, required=True)
    init_parser.add_argument("--name")
    init_parser.add_argument("--description")
    init_parser.add_argument("--soulbound", action="store_true")
    init_parser.add_argument("--max-per-tx")
    init_parser.set_defaults(func=command_init)

    mint_parser = subparsers.add_parser("mint", help="Mint tokens to an address")
    mint_parser.add_argument("--state", type=Path, required=True)
    mint_parser.add_argument("--to", required=True)
    mint_parser.add_argument("--amount", type=int, required=True)
    mint_parser.set_defaults(func=command_mint)

    transfer_parser = subparsers.add_parser("transfer", help="Transfer tokens")
    transfer_parser.add_argument("--state", type=Path, required=True)
    transfer_parser.add_argument("--sender", required=True)
    transfer_parser.add_argument("--recipient", required=True)
    transfer_parser.add_argument("--amount", type=int, required=True)
    transfer_parser.set_defaults(func=command_transfer)

    export_parser = subparsers.add_parser("export", help="Export canonical state")
    export_parser.add_argument("--state", type=Path, required=True)
    export_parser.add_argument("--out", type=Path, required=True)
    export_parser.set_defaults(func=command_export)

    verify_parser = subparsers.add_parser("verify", help="Verify state integrity")
    verify_parser.add_argument("--state", type=Path, required=True)
    verify_parser.set_defaults(func=command_verify)

    return parser


def main() -> None:
    parser = build_parser()
    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
