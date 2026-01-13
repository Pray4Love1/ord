#!/usr/bin/env python3
import hashlib
import json
import time
from dataclasses import dataclass, asdict, field
from typing import Dict

DOMAIN = "brc20v2.zk.transfer"


def sha256_hex(data: str) -> str:
    return hashlib.sha256(data.encode("utf-8")).hexdigest()


@dataclass
class TokenState:
    token_id: str
    balances: Dict[str, int]
    prev_state_hash: str
    merkle_root: str

    def hash(self) -> str:
        payload = json.dumps(asdict(self), sort_keys=True, separators=(",", ":"))
        return sha256_hex(payload)

    def transfer(self, sender: str, recipient: str, amount: int) -> None:
        if self.balances.get(sender, 0) < amount:
            raise ValueError("insufficient balance")
        self.balances[sender] -= amount
        self.balances[recipient] = self.balances.get(recipient, 0) + amount
        self.prev_state_hash = self.hash()


@dataclass
class ZkProofEnvelope:
    domain: str
    from_address: str
    to_address: str
    amount: int
    prev_state_hash: str
    nonce: int
    timestamp: int
    chain_id: int
    identity_verified: bool
    proof_hash: str = field(init=False)

    def __post_init__(self) -> None:
        if not self.identity_verified:
            raise ValueError("identity verification failed")
        payload = "|".join(
            [
                self.domain,
                self.from_address,
                self.to_address,
                str(self.amount),
                self.prev_state_hash,
                str(self.nonce),
                str(self.chain_id),
            ]
        )
        self.proof_hash = sha256_hex(payload)

    def to_json(self) -> str:
        body = {
            "domain": self.domain,
            "from": self.from_address,
            "to": self.to_address,
            "amount": self.amount,
            "prev_state_hash": self.prev_state_hash,
            "nonce": self.nonce,
            "timestamp": self.timestamp,
            "chain_id": self.chain_id,
            "identity_verified": self.identity_verified,
            "proof_hash": self.proof_hash,
        }
        return json.dumps(body, sort_keys=True, separators=(",", ":"))


def main() -> None:
    state = TokenState(
        token_id="MYTOKEN",
        balances={"alice": 1000},
        prev_state_hash="0" * 64,
        merkle_root="",
    )

    state.transfer("alice", "bob", 100)

    proof = ZkProofEnvelope(
        domain=DOMAIN,
        from_address="alice",
        to_address="bob",
        amount=100,
        prev_state_hash=state.prev_state_hash,
        nonce=1,
        timestamp=int(time.time()),
        chain_id=1,
        identity_verified=True,
    )

    output = {
        "state": asdict(state),
        "proof": json.loads(proof.to_json()),
    }
    print(json.dumps(output, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
