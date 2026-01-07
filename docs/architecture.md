# BRC-20 v2 Architecture (PoC)

This document is the human-readable, reviewer-friendly mental model for the BRC-20 v2 proof-of-concept.

```
Layer 8 ─ Applications & UX
Layer 7 ─ Indexers & Verification
Layer 6 ─ Execution & State Transitions
Layer 5 ─ Token Logic & Policy
Layer 4 ─ Identity & Rules (Soulbound/Vesting)
Layer 3 ─ Commitments & Proofs (Merkle/ZK)
Layer 2 ─ Inscription Payloads (CBOR/JSON)
Layer 1 ─ Bitcoin Base Layer (TXs + Inscriptions)
```

Each layer is designed to be independently reviewable, while still composing into a full system that can be built and validated today.
