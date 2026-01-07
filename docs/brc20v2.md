# **BRC-20 v2 — A Deterministic, Proof-Oriented Asset Protocol for Bitcoin**

---

## Abstract

BRC-20 v2 defines a **state-transition protocol** for fungible and semi-fungible assets on Bitcoin, constructed entirely from **ordinal inscriptions, cryptographic commitments, and verifiable off-chain proofs**.

The protocol does not introduce execution, contracts, or consensus modifications.
Instead, it formalizes **state**, **validity**, and **time** as first-class primitives, allowing Bitcoin to function as a **root of truth for programmable assets** without violating its minimal design philosophy.

---

## Design Principles

1. **Determinism over execution**
2. **Proofs over trust**
3. **State over events**
4. **Time over gas**
5. **Identity as constraint, not metadata**
6. **Bitcoin as settlement oracle**

---

# **Layered Architecture**

The protocol is decomposed into **macro-layers**, each composed of **strictly defined micro-layers**.
No layer assumes implicit behavior from another.

---

## **Layer 0 — Bitcoin Substrate Layer**

### Purpose

Provide immutable ordering, time, and economic finality.

### Micro-Layers

**0.1 Transaction Ordering**

* `(block_height, tx_index, ordinal_offset)`
* Defines total ordering for state transitions

**0.2 Time Semantics**

* Block height as canonical time
* `nLockTime`, `OP_CSV` usable as external constraints

**0.3 Economic Anchoring**

* All protocol activity consumes blockspace
* Fees are explicit and unavoidable

**0.4 Immutability Guarantee**

* Once inscribed, protocol history is irreducible
* No rollbacks, only forward state evolution

---

## **Layer 1 — Inscription Encoding Layer**

### Purpose

Encode protocol data into ordinal inscriptions.

### Micro-Layers

**1.1 Protocol Envelope**

```json
{
  "protocol": "brc20v2",
  "version": 2,
  "payload": { ... }
}
```

**1.2 Canonical Serialization**

* UTF-8
* Sorted keys
* No whitespace
* Deterministic encoding

**1.3 Payload Hash Commitment**

* All inscriptions commit to payload hash
* Enables light-client verification

**1.4 Backward Compatibility Flag**

* Legacy BRC-20 detectable but isolated

---

## **Layer 2 — State Machine Layer**

### Purpose

Transform inscriptions from *events* into *state transitions*.

### Micro-Layers

**2.1 Explicit State Object**

```text
Stateₙ = {
  balances,
  metadata,
  supply,
  constraints,
  epoch
}
```

**2.2 Transition Function**

```
Stateₙ₊₁ = Apply(Stateₙ, Action, Proof)
```

**2.3 Previous State Hash Binding**

* Each transition references `state_hashₙ`
* Prevents reordering or replay

**2.4 Deterministic Failure States**

* Invalid transitions do not mutate state
* Failure is observable and replayable

---

## **Layer 3 — Merkle Commitment Layer**

### Purpose

Enable partial verification and light clients.

### Micro-Layers

**3.1 Balance Merkle Tree**

* Leaves: `(address, balance)`
* Root committed per transition

**3.2 Metadata Merkle Commit**

* Token parameters hashed independently

**3.3 Inclusion Proofs**

* Wallets verify balances without full state

---

## **Layer 4 — Zero-Knowledge Proof Layer**

### Purpose

Enable programmable constraints without on-chain execution.

### Micro-Layers

**4.1 Proof Scope Definition**

Proof attests to:

* Balance sufficiency
* Supply invariants
* Constraint satisfaction

**4.2 Constraint Encoding**

* Max transfer
* Vesting
* Epoch rules
* Identity requirements

**4.3 Proof Hash Inscription**

* Only proof commitment is inscribed
* Full proof verified off-chain

**4.4 Batch & Aggregate Proofs**

* Multiple transitions validated atomically

---

## **Layer 5 — Identity & SoulSync Layer**

### Purpose

Introduce identity as a cryptographic constraint.

### Micro-Layers

**5.1 Identity Commitment**

* Subject → commitment hash
* No PII on-chain

**5.2 Verification Function**

* Deterministic signature / commitment match

**5.3 Soulbound Constraint**

* `transfer = false`
* Enforced by proof layer

**5.4 Revocation & Expiry**

* Identity validity bounded by epoch / block height

---

## **Layer 6 — Temporal Logic Layer**

### Purpose

Exploit Bitcoin’s strongest primitive: time.

### Micro-Layers

**6.1 Block-Height Locks**

* Vesting schedules
* Delayed minting

**6.2 Epoch Windows**

* Governance rounds
* Emission periods

**6.3 Time-Bound Proof Validity**

* Proofs expire automatically

---

## **Layer 7 — Governance Layer**

### Purpose

Enable protocol evolution without contracts.

### Micro-Layers

**7.1 Proposal Inscriptions**

* Canonical proposal hash

**7.2 Voting as State Transition**

* Votes modify governance state

**7.3 Threshold Rules**

* Quorum & majority encoded in constraints

**7.4 Finality Epochs**

* Governance results apply only after epoch close

---

## **Layer 8 — Interoperability Layer**

### Purpose

Allow Bitcoin-anchored state to be consumed elsewhere.

### Micro-Layers

**8.1 Proof-Carrying Messages**

* `(state_root, proof)` exported

**8.2 External Verification**

* Ethereum, Solana, Cosmos verify proof validity

**8.3 No Asset Wrapping**

* No custody
* No lock-and-mint

---

## **Layer 9 — Economic Alignment Layer**

### Purpose

Ensure protocol strengthens Bitcoin.

### Micro-Layers

**9.1 Mandatory Fee Consumption**

* Every transition pays miners

**9.2 Fee-Weighted Activity**

* High-value operations cost more

**9.3 Post-Subsidy Security Contribution**

* Protocol remains viable after halvings

---

## **Layer 10 — Application Layer**

### Purpose

Expose protocol to users.

### Micro-Layers

**10.1 Stateless Wallets**

* Verify via proofs
* No trusted indexer required

**10.2 Programmable Transfers**

* Vesting
* Streaming
* Conditional delivery

**10.3 Reputation & Access Tokens**

* Soulbound credentials
* DAO participation

---

## **Problem → Solution Mapping**

| Problem            | Micro-Layer Solution                   |
| ------------------ | -------------------------------------- |
| Indexer trust      | Canonical serialization + state hashes |
| No rules           | ZK constraint proofs                   |
| No time            | Block-height locks                     |
| No identity        | Commitment-based identity layer        |
| Fragile bridges    | Proof-carrying messages                |
| Miner misalignment | Mandatory fee layer                    |

---

## Final Assertion

BRC-20 v2 does not attempt to make Bitcoin expressive.

It makes **correctness provable**.

Bitcoin becomes:

> A globally verifiable state court
> where proofs, not execution, determine truth.

---

If you want, the next artifacts to complete the stack would be:

* **Formal state transition pseudocode**
* **ZK circuit definitions**
* **Canonical CBOR schemas**
* **Security and adversarial analysis**
* **Reference Rust + ord implementation**

State which one you want next, and we continue.
