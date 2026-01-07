# BRC-20 v2 — Proof-Native, Identity-Aware State Machines on Bitcoin

BRC-20 was deliberately minimal.
That minimalism is not a weakness — it is the opening.

**BRC-20 v2 does not attempt to recreate smart contracts on Bitcoin.**
Instead, it treats Bitcoin for what it actually is:

> A globally ordered, time-anchored, immutable state log.

This repository implements a **next-generation BRC-20 architecture** that upgrades fungible inscriptions into **verifiable state machines**, secured by **zero-knowledge proofs**, **identity commitments**, and **cross-chain proof relays** — without modifying Bitcoin.

---

## Design Philosophy

BRC-20 v2 follows five core principles:

1. **State over execution**
2. **Proofs over contracts**
3. **Time over gas**
4. **Identity as a first-class constraint**
5. **Bitcoin as the root of truth**

Everything in this repo exists to serve those principles.

---

## Architecture Overview (End-to-End)

BRC-20 v2 is structured as **explicit layers**, each composed of **microlayers**.
Each layer can be verified independently, but composes deterministically with the others.

```
Bitcoin
 └─ Inscription State
     └─ ZK Proofs
         └─ Identity / SoulSync
             └─ Protocol Rules
                 └─ Relay / Settlement
                     └─ External Verification (Ethereum, L2s)
```

---

## Layer 0 — Bitcoin Base Layer

**Purpose:** Global ordering, time, and immutability.

**What Bitcoin provides (and nothing more):**

* Total transaction ordering
* Block height as time
* Immutable data availability
* Fee market & miner incentives

BRC-20 v2 never attempts to bypass or abstract this layer.

---

## Layer 1 — Inscription & State Layer

**Purpose:** Deterministic state machines, not ad-hoc JSON blobs.

Each BRC-20 v2 operation represents a **state transition**, not just an event.

### Canonical State Model

Every inscription commits to:

```
prev_state_hash → new_state_hash
```

The inscription payload (see `inscription.rs`) contains:

* `protocol`: fixed identifier (`brc20v2`)
* `token`: token symbol
* `action`: mint / transfer / burn / govern
* `state_hash`: resulting state commitment
* `merkle_root`: optional balance tree root
* `proof`: optional ZK proof reference

**Result:**

* Stateless clients
* Indexer-independent replay
* Deterministic verification

---

## Layer 2 — ZK Proof Layer

**Purpose:** Programmability without smart contracts.

Bitcoin does not execute logic — **users prove that logic was executed correctly**.

### What ZK Proofs Enforce

ZK proofs can attest to:

* Balance correctness
* Supply caps
* Transfer constraints
* Vesting schedules
* Time locks
* Governance rules
* Identity requirements

Only the **proof commitment** is inscribed.
Verification happens off-chain, deterministically.

### Implementation

* `zk.rs` / `zk_proof.rs`
* Pluggable circuits
* Proof hashes committed into inscription state

---

## Layer 3 — Identity & SoulSync Layer

**Purpose:** Make identity a constraint, not an afterthought.

Not all tokens should be freely transferable.

### Identity Commitments

The `identity.rs` microlayer provides:

* Deterministic identity commitments
* Hash-based verification
* No on-chain PII
* No central registry

Identity proofs can gate:

* Who can mint
* Who can receive
* Who can vote
* Who can relay state

### Soulbound Semantics

Tokens may be:

* Non-transferable
* Revocable
* Expirable
* Role-bound

This enables:

* Reputation systems
* DAO credentials
* Access control
* Proof-of-personhood

---

## Layer 4 — Protocol Rules & Errors

**Purpose:** Deterministic enforcement of token law.

All rule failures are explicit and enumerable (`errors.rs`):

* Identity failure
* Insufficient balance
* Soulbound restriction
* Vesting lock
* Transfer caps
* Invalid proofs
* Relay failures

There are **no silent failures**.

This layer ensures:

* Clear replay semantics
* Auditable failure reasons
* Identical behavior across indexers

---

## Layer 5 — Relay & Settlement Layer

**Purpose:** Extend Bitcoin state to other execution environments without wrapping assets.

### Proof-Carrying Relays

Instead of custodial bridges:

* Bitcoin inscriptions commit state
* ZK proofs attest correctness
* External chains verify proofs

The `relay/ethereum.rs` microlayer demonstrates:

* Stateless Ethereum verification
* Proof submission as calldata
* No asset custody
* No mint/burn mirrors

Bitcoin remains the **settlement oracle**.

---

## Layer 6 — Indexer Independence

**Purpose:** Eliminate indexer centralization risk.

BRC-20 v2 enforces:

* Canonical parsing rules
* Deterministic serialization
* Replayable state transitions
* Optional Merkle snapshots

Anyone can:

* Recompute full state from genesis
* Verify balances independently
* Reject invalid histories

There are no “soft rules”.

---

## Layer 7 — Application Layer

**Purpose:** Enable real systems, not demos.

Built on top of BRC-20 v2:

* Programmable wallets
* Streaming payments
* Vesting payroll
* Reputation-based DAOs
* Identity-gated economies
* Cross-chain liquidity proofs

All without changing Bitcoin.

---

## Economic Alignment

BRC-20 v2 is designed to **pay Bitcoin**, not extract from it.

Possible mechanisms:

* Mandatory inscription fees
* Proof verification costs
* Miner-aligned incentives
* Activity-driven blockspace demand

This ensures BRC-20 v2 strengthens Bitcoin’s security model post-halving.

---

## What This Repository Is

This repo is:

* A **protocol implementation**
* A **reference architecture**
* A **proof-native design**
* A **migration path beyond primitive BRC-20**

It is **not**:

* An ERC-20 clone
* A smart contract framework
* A custodial bridge
* A speculative wrapper

---

## Migration Path

1. Existing BRC-20 tokens derive initial state hash
2. State machine rules are declared
3. Proof-enabled transfers begin
4. Identity & soulbound features opt-in
5. Cross-chain verification added incrementally

Backward compatibility is preserved where possible.

---

## Summary

BRC-20 v2 reframes Bitcoin tokens as:

* **State machines**
* **Proof-verified**
* **Time-aware**
* **Identity-constrained**
* **Cross-chain verifiable**

Bitcoin becomes not just money — but the **root of global digital truth**.

---

If you want, next we can:

* Formalize this into a **BRC-20 v2 spec**
* Add **canonical JSON / CBOR schemas**
* Produce **protocol diagrams**
* Or wire the full **E2E transfer flow** across all layers

Just say which file is next.
