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
