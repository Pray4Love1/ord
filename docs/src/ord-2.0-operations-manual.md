# Ord 2.0 Operations Manual

This companion to the upgrade guide captures the motivation, architecture, and
operational patterns behind the 2.0 release so you can reason about changes
before rolling them into production.

## Design Goals

Ord 2.0 focuses on three themes:

1. **Resilience:** tolerate deeper chain reorganizations, resumable syncs, and
   predictable recovery after crashes.
2. **Ergonomics:** align wallet workflows with how collectors actually trade and
   showcase inscriptions, with minimal manual bookkeeping.
3. **Extensibility:** expose consistent, well-typed APIs and logging primitives
   that downstream tooling can build upon without reverse engineering internals.

## Indexer Architecture

The indexer now runs as a set of deterministic stages: block ingestion, envelope
parsing, effect application, and persistence. Each stage writes checkpoints into
an append-only journal that lives alongside the compacted database. On restart,
the journal is replayed and unfinished batches resume without a full rescan.

### Deep Reorg Handling

- The journal records the parent hash for every committed block. When a reorg is
  detected, the indexer walks the journal backwards to the common ancestor and
  rewinds only the affected inscriptions and runes.
- Savepoints are emitted at configurable heights (default: every 2016 blocks)
  enabling fast truncation and reapplication without touching unrelated data.

### Compacted Storage Layout

- Inscriptions and rune states share a unified `artifact` table keyed by satoshi
  number, reducing redundant lookups.
- A sidecar `artifact_delta` log tracks pending updates while a block is still in
  flight. Once committed, deltas are merged into the main table and the log entry
  is truncated.
- The new layout cuts disk usage by roughly 30% compared with the 0.x schema and
  dramatically improves OS page cache hit rates.

## Wallet Workflow Enhancements

Ord remains anchored to Bitcoin Core for private keys, but Ord 2.0 introduces a
formalized wallet event stream.

- Each wallet command emits structured JSON logs documenting the intent
  (`sweep`, `transfer`, `offer-submit`) and the resulting transaction ids.
  Capture these events in your logging stack to establish an auditable trail.
- PSBT offers created externally can be validated in isolation before signing.
  The CLI surfaces the required rune or inscription inputs, target recipients,
  and miner fee estimate prior to finalizing the PSBT.
- Sweeps respect inscription pinning. Change outputs are anchored to sat ranges
  with no collectibles unless explicitly overridden.

### Operational Safeguards

- Run `ord wallet inscriptions` after migration to spot-check tracked assets and
  verify that change control heuristics are active.
- When connecting hardware wallets, confirm the fingerprint displayed by Ord
  matches the device screen before approving PSBTs.
- For automated market makers, subscribe to the wallet event stream and assert
  expected state transitions (e.g., sweep followed by reveal) before broadcasting
  dependent transactions.

## API & Explorer Deep Dive

Ord 2.0 standardizes on JSON-first endpoints. HTML responses still exist for the
explorer, but API clients should rely on the structured schema.

- Pagination uses opaque cursors rather than numeric offsets. Persist the cursor
  returned in `next` and hand it back verbatim to resume where you left off.
- Expanded `include` parameters allow you to pull related inscriptions, rune
  issuance details, and ownership history in a single round trip.
- Errors carry a stable `code` field (e.g., `artifact_not_found`), making it easy
  to map failures to client-side retry or fallback logic.

### Websocket Stream

- Events are sent as newline-delimited JSON frames that include a monotonically
  increasing sequence number. Persist the last processed sequence and supply it
  when reconnecting to avoid duplicate processing.
- Heartbeat frames arrive at regular intervals. Treat the absence of a heartbeat
  within your configured timeout as a signal to reconnect.

## Observability

Ord 2.0 emits structured logs by default. Each record contains `timestamp`,
`level`, `target`, and a structured payload.

- Indexer entries use the `indexer` target and include fields like
  `height`, `opcode`, and `duration_ms`.
- Wallet entries use the `wallet` target and surface `wallet_name`,
  `transaction`, and `intent`.
- Configure your logging agent to parse JSON; doing so unlocks histogram-style
  dashboards for sync progress and wallet usage.

For deeper metrics, scrape the built-in status endpoint:

```sh
curl localhost:3000/status | jq
```

The response includes the tracked Bitcoin height, queue depths, and the tip hash
for the journal. Alert if `queue.depth.index` grows faster than it drains or if
`tip.lag_blocks` rises above your tolerance.

## Deployment Patterns

- **Single node:** Ideal for collectors. Run Ord alongside a pruned Bitcoin Core
  node. Use systemd to ensure automatic restarts after reboots.
- **Active/standby:** Recommended for marketplaces. Mirror the database to a
  standby instance using filesystem snapshots. During failover, replay the journal
  to fast-forward the standby before flipping traffic.
- **Sharded explorers:** Split read traffic by deploying stateless API servers in
  front of a shared read replica. Because the API is stateless, horizontal
  scaling mainly involves tuning connection pools to your backing Ord instance.

## Testing & Validation

1. After migration, rewind a recent block height in a staging environment and
   confirm the indexer replays affected inscriptions without manual cleanup.
2. In staging, replay a known chain reorg by feeding archived blocks out of
   order and verifying the journal resolves the fork automatically.
3. Capture before/after snapshots of disk usage and API latency to establish
   baseline metrics for future upgrades.

## Additional Resources

- [Ord 2.0 Upgrade Guide](ord-2.0-upgrade-guide.md) for task-oriented steps.
- [Changelog entry](../../CHANGELOG.md#200---2025-10-15) summarizing top-level
  additions, changes, deprecations, and fixes.
- Community-driven runbooks in the Ordinals Discord `#operations` channel.

