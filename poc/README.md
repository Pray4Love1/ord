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
