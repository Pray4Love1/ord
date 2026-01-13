# BRC-20 v2 End-to-End Architecture

This document describes a complete, layered flow for a Bitcoin-native BRC-20 v2 token
transfer that is proof-ready, identity-aware, cross-chain capable, auditable, and safe
for mainnet usage.

## Layer Map

| Layer | Responsibility | Primary Artifacts |
| --- | --- | --- |
| L0 | Bitcoin base ordering, time, and fees | Bitcoin blocks, mempool inclusion |
| L1 | Canonical state + inscription serialization | TokenState, serialization format |
| L2 | Merkle commitments | `merkle_root` field, leaf hashing rules |
| L3 | ZK proof envelope | `ZkProofEnvelope`, domain-separated proof hash |
| L4 | Identity & policy enforcement | `identity_verified`, chain policy rules |
| L5 | Cross-chain relay | Ethereum relay transaction | 
| L6 | Indexer & verifier compatibility | Deterministic hash, canonical schema |
| L7 | CLI / UX orchestration | Orchestrated transfer flow |

## Canonical State Machine (L1–L2)

A token instance is represented by a deterministic `TokenState` object:

- `token_id`: immutable identifier for the token instance.
- `balances`: canonical map of owner identifier to balance (u64).
- `prev_state_hash`: hash of the previous canonical state.
- `merkle_root`: Merkle root over leaf entries of the form `owner || balance`.

State transition is defined as:

1. Validate sender balance.
2. Apply debit/credit updates.
3. Compute new canonical hash (SHA-256 of canonical JSON).
4. Update `prev_state_hash` for lineage tracking.

## Merkle Commitments (L2)

The Merkle root is computed over leaves with this encoding:

```
leaf = sha256("brc20v2.leaf" || owner || ":" || balance)
```

Leaves are sorted lexicographically by `owner` and then merkleized using a
Bitcoin-style double SHA-256 tree. The output is stored in `TokenState.merkle_root`.

## ZK Proof Envelope (L3–L4)

The transfer proof is encapsulated as a domain-separated object:

- `domain = "brc20v2.zk.transfer"`
- `proof_hash = sha256(domain || from || to || amount || prev_state_hash || nonce || chain_id)`

The envelope binds the transfer to its previous state and chain, enabling
auditable replay protection through `nonce` and `chain_id`.

Identity and policy requirements are represented by:

- `identity_verified: true` is required prior to proof creation.
- Policies can enforce per-token or per-chain constraints and are audited
  through deterministic serialization.

## Cross-Chain Relay (L5)

The proof JSON is submitted to a relay contract on Ethereum. The relayer
transaction includes:

- The serialized proof envelope as call data.
- Fixed gas bounds for deterministic relay cost accounting.

Relays are designed to be replay-safe by checking `nonce` and `proof_hash` on
chain in the verifier contract.

## Indexer & Verifier Compatibility (L6)

Indexers and verifiers share these deterministic rules:

- JSON serialization uses stable field ordering.
- Hashing is SHA-256 over the full JSON blob.
- Proofs are rejected if the `prev_state_hash` does not match the canonical
  chain state.

## CLI / UX Orchestration (L7)

The CLI flow is:

1. Load token state.
2. Execute transfer mutation.
3. Generate proof envelope.
4. Serialize proof JSON.
5. Relay or persist the proof as required.

This sequence allows local auditability before broadcast or relay.
Layer 8 ─ Application & UX
         ▸ Wallets, marketplaces, DEXs, analytics, mobile/web clients
         ▸ End-user flows: mint, transfer, vest, verify

Layer 7 ─ Interoperability & Verifiers
         ▸ Indexers, bridges, relayers
         ▸ On/off-chain proof verification (ZK/identity/state)
         ▸ Fee markets and liquidity routing

Layer 6 ─ Execution & State Transitions
         ▸ Stateless → stateful updates
         ▸ Rules for valid mints/transfers/burns
         ▸ Replay protection, nonces, rate limits

Layer 5 ─ Token Logic & Policy
         ▸ Token definitions: max_supply, mintable, royalties, cap
         ▸ Per-token policies (e.g., soulbound, whitelist-only)
         ▸ Upgrade hooks, programmable token behavior

Layer 4 ─ Identity & Attestation
         ▸ DID, KYC, Sybil resistance
         ▸ Proof-of-personhood (e.g., World ID, Gitcoin Passport)
         ▸ ZK-backed attestations, vesting schedules

Layer 3 ─ Commitment & ZK Proof Layer
         ▸ Merkle roots, state hashes, proof generation
         ▸ BRC20v2::ZK::TRANSFER domain separation
         ▸ Selective disclosure, shielded logic (optional)

Layer 2 ─ Inscription Payloads
         ▸ CBOR or JSON-based operation definitions
         ▸ Canonical format for indexing and relay
         ▸ Fully auditable, no execution ambiguity

Layer 1 ─ Bitcoin Base Layer
         ▸ Finality, timestamping, chain-of-record
         ▸ UTXO anchoring + inscriptions
         ▸ Censorship resistance and protocol neutrality
