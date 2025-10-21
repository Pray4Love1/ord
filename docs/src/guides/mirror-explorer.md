# Mirror Explorer Quickstart

This guide walks through spinning up the full Mirror explorer loop on a local
machine. You will deploy the `MirrorVerifier` contract, run the watcher daemon
that mirrors simulated inscriptions, expose the data through the viewer API, and
finally visualize the results with the React viewer.

## 1. Prerequisites

Ensure the following tools are installed and available in your `PATH`:

- `cargo`
- `anvil` (or another Ethereum JSON-RPC test node such as Ganache or Hardhat)
- `forge`
- `npm`

Verify each command by running `<tool> --version` before continuing.

## 2. Start a Local EVM Chain and Deploy the Verifier

1. Launch an Anvil instance and configure environment variables:

   ```bash
   anvil --silent &
   export RPC_URL=http://127.0.0.1:8545
   export PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
   ```

2. Deploy `MirrorVerifier.sol` and capture the resulting address:

   ```bash
   forge create src/MirrorVerifier.sol:MirrorVerifier \
     --rpc-url "$RPC_URL" \
     --private-key "$PRIVATE_KEY"

   export VERIFIER_ADDR=0xDeployedVerifierAddress
   ```

   Replace `0xDeployedVerifierAddress` with the address printed by `forge`.

## 3. Run the Watcher Daemon

In **Terminal A**:

```bash
cd ord2
cargo run --bin watcher_daemon
```

The watcher simulates Bitcoin blocks and emits a new inscription every 100
blocks. When a proof is broadcast you will see log entries such as:

```
⛓️  Checking Bitcoin block 839800
📜 New simulated inscription detected at block 839800
🚀 Broadcasting mirror proof...
Mirror response: OK
```

Each proof results in a `post_proof()` call to the local verifier contract.

## 4. Start the Viewer API

In **Terminal B**:

```bash
# Reuse the exact values exported in step 2
export RPC_URL=http://127.0.0.1:8545
export VERIFIER_ADDR=0xDeployedVerifierAddress
cargo run --bin viewer_api
```

Replace `0xDeployedVerifierAddress` with the verifier address printed during the
deployment step above.

The API listens on `http://127.0.0.1:8787`. Fetch a proof with:

```bash
curl http://127.0.0.1:8787/mirror/0x<commitment_from_watcher>
```

You should receive JSON containing the inscription creator, the originating
block height, and a millisecond timestamp.

## 5. Launch the React Viewer

In **Terminal C**:

```bash
cd ord2-viewer
npm install
npm run dev
```

Open `http://127.0.0.1:5173` in a browser, paste the commitment hash from the
watcher output, and click **View** to display the on-chain record, a query
history, and a timeline of block heights.

## 6. Verify the End-to-End Loop

Use the following checks to confirm the system is healthy:

| Component | Purpose | Verification |
| --- | --- | --- |
| Watcher Daemon | Generates inscriptions and posts proofs | Logs include `🚀 Broadcasting mirror proof...` |
| MirrorVerifier.sol | Anchors proofs on-chain | `cast call $VERIFIER_ADDR "mirrors(bytes32)" 0x<commitment>` |
| Viewer API | Exposes chain data | `curl http://127.0.0.1:8787/mirror/0x<commitment>` |
| React Viewer | Visualizes proofs | Browser displays the mirrored record |

## 7. Next Steps

With the local loop running you can:

- Replace the simulated inscription source with a real Bitcoin RPC or Ordinals
  indexer feed.
- Post proofs to alternative chains such as Sei, Base, or Arbitrum.
- Add WebSocket support so the viewer updates whenever `MirrorPosted` fires.
- Mint a "Living Inscription" and watch its timeline update in real time.

Leave the watcher running to keep producing inscriptions—the explorer becomes a
self-verifying, cross-chain, lineage-aware Ordinal engine.
