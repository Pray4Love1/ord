# BRC-20 v2 Proof-of-Concept

This reference implementation demonstrates the end-to-end flow for a BRC-20 v2
transfer, including canonical state hashing and proof envelope construction.

## Usage

```bash
python3 brc20_poc.py
```

The script prints a JSON document containing:

- The updated canonical `TokenState`.
- The `ZkProofEnvelope` with a deterministic `proof_hash`.

## Behavior

1. Initialize a token with a single balance.
2. Apply a transfer from `alice` to `bob`.
3. Generate a domain-separated proof bound to the previous state hash.
# BRC-20 v2 Proof-of-Concept CLI

This PoC provides a minimal, executable specification for minting and transferring tokens while enforcing soulbound and vesting rules. It also produces deterministic hashes and merkle commitments suitable for ZK-ready verification pipelines.

## Quickstart

```bash
python3 poc/brc20_poc.py mint \
  --token-id brc20v2-demo \
  --name "BRC-20 v2 Demo" \
  --symbol BRC2 \
  --issuer bc1qissuer \
  --to bc1qalice \
  --amount 1000 \
  --max-supply 21000000
```

```bash
python3 poc/brc20_poc.py transfer \
  --from bc1qalice \
  --to bc1qbob \
  --amount 250 \
  --height 100
```

```bash
python3 poc/brc20_poc.py export --output poc_state.json
```

```bash
python3 poc/brc20_poc.py verify
```

To create a soulbound token, mint with `--soulbound`. Transfers will be rejected.

```bash
python3 poc/brc20_poc.py mint \
  --token-id soulbound-demo \
  --name "Soulbound Demo" \
  --symbol SBD \
  --issuer bc1qissuer \
  --to bc1qalice \
  --amount 1 \
  --soulbound
```

Attach a vesting schedule during mint. Heights are arbitrary numbers that can map to block heights in an indexer.

```bash
python3 poc/brc20_poc.py mint \
  --token-id vesting-demo \
  --name "Vesting Demo" \
  --symbol VEST \
  --issuer bc1qissuer \
  --to bc1qalice \
  --amount 1000 \
  --vesting-start 100 \
  --vesting-cliff 150 \
  --vesting-duration 300
```

Transfers from a vested address require enough unlocked balance at the provided `--height`.

Extension points:

* ZK batching: swap out the hash functions and merkle builder for circuit-friendly equivalents.
* Identity hooks: enrich `ledger` entries with DID or attestation references.
* Indexer verification: replay `ledger` entries to reconstruct deterministic state.

```bash
python3 brc20_poc.py init \
  --symbol ORD2 \
  --max-supply 21000000 \
  --decimals 0 \
  --state state.json
```

```bash
python3 brc20_poc.py mint \
  --state state.json \
  --to bc1qexampleaddress \
  --amount 1000
```

```bash
python3 brc20_poc.py transfer \
  --state state.json \
  --sender bc1qexampleaddress \
  --recipient bc1qrecipient \
  --amount 250
```

```bash
python3 brc20_poc.py export --state state.json --out export.json
```

```bash
python3 brc20_poc.py verify --state state.json
```

* `state_hash` provides deterministic state hash chaining via `prev_state_hash`.
* `merkle_root` commits to balance state
