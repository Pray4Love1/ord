# BRC-20 v2

### A Proof-Oriented, Identity-Constrained State Transition Protocol Anchored to Bitcoin

## Abstract

BRC-20 v2 specifies a deterministic, replayable, proof-verified token protocol anchored to Bitcoin’s transaction ordering and inscription mechanism.
It abandons the notion of token events in favor of state transitions, and replaces execution with cryptographic attestation.

The protocol introduces:

- Canonical state machines
- Zero-knowledge validity proofs
- Identity-constrained transfer semantics
- Time-bounded state evolution
- Cross-domain proof relays

All without altering Bitcoin consensus.

## 0. Terminology

| Term | Definition |
| --- | --- |
| **State** | A complete, deterministic description of token balances and parameters |
| **Transition** | A mapping from a prior state to a new state |
| **Commitment** | A cryptographic hash representing state or proof validity |
| **Inscription** | An immutable Bitcoin data carrier anchoring protocol messages |
| **Proof** | A zero-knowledge attestation of transition correctness |
| **Subject** | An identity-bearing participant |
| **Epoch** | A block-height-bounded time interval |

## 1. System Model

The system consists of:

1. Bitcoin as an immutable, totally ordered log
2. Stateless verifiers reconstructing token state
3. Participants generating validity proofs
4. Optional external verifiers consuming Bitcoin-anchored commitments

There exists no global mutable storage beyond Bitcoin itself.

## 2. Layered Architecture

The protocol is composed of seven layers, each decomposed into microlayers.
Each layer is individually verifiable and strictly compositional.

## 3. Layer 0 — Bitcoin Substrate

### Purpose

Provide ordering, immutability, time, and economic finality.

### Microlayers

#### 3.0.1 Transaction Ordering

Bitcoin block height and intra-block ordinal ordering define a total order over all protocol messages.

#### 3.0.2 Temporal Anchoring

Block height serves as the sole time oracle.
`nLockTime` and `OP_CHECKSEQUENCEVERIFY` MAY constrain state transitions.

#### 3.0.3 Economic Coupling

Protocol usage consumes blockspace and fees, aligning miner incentives with protocol activity.

## 4. Layer 1 — State & Inscription Layer

### Purpose

Define token logic as deterministic state machines.

### Microlayers

#### 4.1.1 Canonical State Definition

A token state `S` is defined as:

```
S := {
  token_id,
  balances,
  supply,
  parameters,
  epoch
}
```

#### 4.1.2 State Commitment

Each state is represented by:

```
H(S) = hash(serialize(S))
```

#### 4.1.3 Transition Function

A transition `T` is valid iff:

```
H(S_n) = T(H(S_{n-1}), proof, context)
```

#### 4.1.4 Inscription Payload

Each Bitcoin inscription contains:

- `prev_state_hash`
- `new_state_hash`
- `action`
- `metadata_hash`
- `proof_commitment (optional)`

## 5. Layer 2 — Zero-Knowledge Validity Layer

### Purpose

Replace execution with provable correctness.

### Microlayers

#### 5.2.1 Proof Semantics

A proof attests:

- Balance preservation
- Supply invariants
- Transfer constraints
- Temporal validity
- Identity compliance

#### 5.2.2 Proof Commitments

Only `hash(proof)` is inscribed.
Proof material is verified off-chain.

#### 5.2.3 Aggregation

Multiple transitions MAY be validated within a single proof.

#### 5.2.4 Circuit Modularity

Proof circuits are versioned, parameterized, and non-self-modifying.

## 6. Layer 3 — Identity & SoulSync Layer

### Purpose

Introduce identity as a first-class constraint.

### Microlayers

#### 6.3.1 Identity Commitments

An identity commitment is defined as:

```
I := hash(subject || domain || epoch)
```

No personal data is revealed.

#### 6.3.2 Verification

Identity validity is asserted via:

- Signature
- ZK proof
- Pre-committed hash

#### 6.3.3 Soulbound Semantics

Tokens MAY be marked non-transferable or conditionally transferable.

#### 6.3.4 Revocation & Expiry

Identity-bound privileges MAY expire or be revoked via state transition.

## 7. Layer 4 — Protocol Rule & Error Layer

### Purpose

Ensure explicit, deterministic failure.

### Microlayers

#### 7.4.1 Rule Enumeration

All failure conditions are explicitly enumerated:

- Identity failure
- Balance violation
- Vesting lock
- Soulbound restriction
- Proof invalidity
- Relay failure

#### 7.4.2 Replay Determinism

Given the same ordered inscriptions, all honest verifiers MUST derive identical failure or success outcomes.

## 8. Layer 5 — Relay & External Verification Layer

### Purpose

Export Bitcoin-anchored truth to external domains.

### Microlayers

#### 8.5.1 Proof-Carrying Messages

Messages contain:

- Bitcoin state root
- Proof commitment
- Domain-specific metadata

#### 8.5.2 Stateless Verification

External chains verify proofs without custody or mint/burn mirroring.

#### 8.5.3 No Wrapped Assets

No asset custody, federation, or synthetic representation is required.

## 9. Layer 6 — Indexer Independence Layer

### Purpose

Eliminate centralized interpretation.

### Microlayers

#### 9.6.1 Canonical Parsing

Serialization formats are canonical and collision-resistant.

#### 9.6.2 Replayable State

Any verifier MAY reconstruct full state from genesis.

#### 9.6.3 Merkle Snapshots

Optional Merkle roots accelerate light-client verification.

## 10. Layer 7 — Application Layer

### Purpose

Enable systems, not scripts.

### Microlayers

- Streaming payments
- Vesting schedules
- Reputation systems
- DAO governance
- Identity-gated access
- Cross-chain liquidity proofs

## 11. Security Model

Security derives from:

- Bitcoin finality
- Cryptographic soundness of proofs
- Deterministic state replay
- Absence of mutable global state

No trusted third parties exist.

## 12. Economic Considerations

Protocol usage:

- Consumes blockspace
- Pays miners
- Strengthens Bitcoin post-subsidy

No parasitic incentives are introduced.

## 13. Conclusion

BRC-20 v2 reframes fungible tokens as:

- Deterministic state machines
- Verified by proofs
- Governed by time
- Constrained by identity
- Anchored to Bitcoin

This is not “DeFi on Bitcoin”.
It is Bitcoin as the canonical state oracle.
