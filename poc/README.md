# BRC-20 v2 Reference PoC

This Python reference implementation acts as an executable specification for
BRC-20 v2 state transitions. It is **not** production code; it is designed to be
predictable and easy to audit.

## Quick start

```bash
python3 brc20_poc.py init \
  --symbol ORD2 \
  --max-supply 21000000 \
  --decimals 0 \
  --state state.json
```

### Mint

```bash
python3 brc20_poc.py mint \
  --state state.json \
  --to bc1qexampleaddress \
  --amount 1000
```

### Transfer

```bash
python3 brc20_poc.py transfer \
  --state state.json \
  --sender bc1qexampleaddress \
  --recipient bc1qrecipient \
  --amount 250
```

### Export

```bash
python3 brc20_poc.py export --state state.json --out export.json
```

### Verify

```bash
python3 brc20_poc.py verify --state state.json
```

## Notes

- `state_hash` provides deterministic state hash chaining via `prev_state_hash`.
- `merkle_root` commits to balance state for ZK-friendly proofs.
- Transfer rules (`soulbound`, `max_per_tx`) are enforced during transitions.
