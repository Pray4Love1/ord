# Mirror Harness

This standalone crate exercises the `MirrorVerifier` Solidity contract end-to-end.
It creates a sample "Living Inscription" payload, derives the bridge proof, and
submits it to a fresh Anvil chain over JSON-RPC. The harness verifies that the
on-chain record matches the off-chain proof fields.

## Prerequisites

* [Foundry](https://book.getfoundry.sh/) `anvil` binary available on `PATH`.
* Network access so `ethers-solc` can download Solidity `0.8.24` via `svm` on the first run.

## Running

```bash
cargo run --manifest-path examples/mirror-harness/Cargo.toml
```

The harness prints the generated inscription JSON, commitment hash, signature,
and the stored on-chain record before exiting.
