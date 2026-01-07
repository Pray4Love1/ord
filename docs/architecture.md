# BRC-20 v2 Layered Architecture

The BRC-20 v2 protocol is best understood as a layered stack, where each layer
builds on the guarantees of the layer below it.

```
┌───────────────────────────────────────────────────────────────┐
│ 8. Application Layer                                           │
│    Wallets, marketplaces, analytics, exchanges, end-user UX    │
├───────────────────────────────────────────────────────────────┤
│ 7. Interoperability & Incentive Layer                          │
│    Cross-chain bridges, fee markets, relayers, liquidity       │
├───────────────────────────────────────────────────────────────┤
│ 6. Governance & Compliance Layer                               │
│    Upgrade policies, DAOs, compliance rules, emergency stops   │
├───────────────────────────────────────────────────────────────┤
│ 5. Identity & Attestation Layer                                │
│    DID, KYC attestations, reputation, proof-of-personhood      │
├───────────────────────────────────────────────────────────────┤
│ 4. ZK & Privacy Layer                                          │
│    State commitments, ZK proofs, selective disclosure          │
├───────────────────────────────────────────────────────────────┤
│ 3. BRC-20 v2 Protocol Layer                                    │
│    Token rules, state transitions, deterministic hashing       │
├───────────────────────────────────────────────────────────────┤
│ 2. Ordinals / Inscriptions Layer                               │
│    Inscribed operations, indexing, canonical serialization     │
├───────────────────────────────────────────────────────────────┤
│ 1. Bitcoin Base Layer                                          │
│    UTXOs, consensus, finality, censorship resistance           │
└───────────────────────────────────────────────────────────────┘
```

**Guiding principles**

- **Separation of concerns**: application UX should not redefine protocol rules.
- **Determinism**: protocol state must be fully reproducible from inscriptions.
- **Composability**: identity, governance, and privacy layers can be swapped or
  augmented without changing the base protocol.
- **Auditability**: each layer is independently reviewable and testable.
