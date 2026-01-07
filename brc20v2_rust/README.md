# BRC-20 v2 Rust E2E Example

This directory contains a standalone Rust example that models a full BRC-20 v2 workflow end-to-end, including minting, transfers, vesting checks, soulbound enforcement, inscription JSON serialization, a placeholder zk-proof, Bitcoin broadcast via `ord`, and an Ethereum relay.

## Layout

```
brc20v2_rust/
├─ Cargo.toml
├─ config.json
├─ README.md
└─ src/
   ├─ brc20v2.rs
   ├─ config.rs
   ├─ cross_chain.rs
   ├─ main.rs
   └─ zk_proof.rs
```

## Build

```bash
cargo build --release
```

## Run

```bash
cargo run --release
```

The example writes `mint_inscription.json` and `transfer_inscription.json` in the current directory and demonstrates broadcasting via `ord` and relaying to Ethereum.
