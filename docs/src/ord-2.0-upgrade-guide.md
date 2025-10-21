# Ord 2.0 Upgrade Guide

Ord 2.0 ships with a new indexer, refreshed wallet tooling, and expanded API
coverage. This guide highlights the key steps to upgrade from the 0.x releases
and adopt the latest functionality safely.

## Before You Begin

- Ensure your environment has **Rust 1.85.0 or later** and **Bitcoin Core v29**.
- Capture the current state of your node by recording `ord --version`,
  `ord info`, and the output of `bitcoin-cli getblockcount`.
- Back up your `.ord` data directory and any custom configuration files.
- Export wallet metadata with `ord wallet inscriptions --name <wallet>` so you
  have a pre-upgrade inventory of tracked artifacts.
- Review the [2.0.0 changelog entry](../../CHANGELOG.md#200---2025-10-15) for an
  overview of the major changes and any deprecated features your automation may
  rely on.

### Environment Readiness Checklist

- Confirm the Bitcoin Core node you pair with `ord` is fully synchronized and
  running with `txindex=1` if you serve historical data.
- Verify you have at least 30% free disk space relative to your current `.ord`
  directory so the migration journal can be written without pressure.
- If you operate in a managed environment, schedule a maintenance window that
  exceeds your longest historical reorg plus an additional buffer for validation.
- Notify downstream consumers (marketplace integrations, dashboards, bots) of
  the upgrade window so they can expect brief API interruptions.
- Back up your `.ord` data directory and any custom configuration files.
- Review the [2.0.0 changelog entry](../../CHANGELOG.md#200---2025-10-15) for an
  overview of the major changes.

## Database Migration

Ord 2.0 introduces a compacted database layout. When you start the new binary it
will automatically perform an in-place migration.

### Pre-Migration Checks

1. Stop any HTTP frontends or background workers that depend on the ord indexer
   to avoid concurrent access while files are being rewritten.
2. Run `ord info` and confirm the reported `height` matches your Bitcoin Core
   tip. Resolve discrepancies before proceeding.
3. Take a final snapshot of the `.ord` directory and verify it is stored on
   durable media:
   ```sh
   cp -a ~/.ord ~/.ord.pre-2-0-$(date +%Y%m%dT%H%M%S)
   ```

### Migration Steps

1. Stop any running `ord` processes.
2. Make a copy of your data directory (if you have not already done so):
   ```sh
   cp -a ~/.ord ~/.ord.backup-$(date +%Y%m%d)
1. Stop any running `ord` processes.
2. Make a copy of your data directory:
   ```sh
   cp -r ~/.ord ~/.ord.backup-$(date +%Y%m%d)
   ```
3. Start `ord` 2.0 with the `--reindex` flag if you run archival infrastructure
   or operate close to the chain tip:
   ```sh
   ord index --reindex
   ```
   The new indexer resumes automatically after interruptions and can tolerate
   deeper chain reorganizations without manual intervention.
4. Operators that only need the compacted database can omit `--reindex` and run
   a standard start:
   ```sh
   ord index
   ```
5. Follow the structured migration log entries. A successful migration will emit
   a final `indexer.migration_complete` record with the target height.
6. After the first sync finishes, prune stale cache entries by restarting once
   without `--reindex`; the follow-up boot validates that the new compaction
   boundaries have been persisted.

### Post-Migration Verification

- Confirm your node answers API calls end-to-end:
  ```sh
  curl -H 'Accept: application/json' localhost:3000/inscriptions | jq '.ids | length'
  ```
- Run `ord info` and verify `database` reports the new `v2` format and the
  tracked chain tip.
- Inspect disk utilization; healthy nodes should observe ~30% less storage usage
  relative to the legacy layout once pruning finishes.
- Query the built-in status endpoint and ensure `queue.depth.index` trends back
  toward zero after the initial sync:
  ```sh
  curl -s localhost:3000/status | jq '{height: .tip.height, index_depth: .queue.depth.index}'
  ```

## Wallet Cutover Plan

Ord 2.0 refines wallet ergonomics while staying anchored to Bitcoin Core for key
management. Use the following steps to transition wallets safely.

### Inventory and Segregation

- Run `ord wallet inscriptions --name <wallet>` and `ord wallet balance --name <wallet>`
  to inventory collectibles and cardinal change before upgrading.
- Keep ordinal and cardinal funds separated. If you previously mixed them,
  create a dedicated Ord wallet name and migrate collectibles by sending to the
  new account after the upgrade completes.
- Document any watch-only descriptors or xpubs connected to the wallet so they
  can be re-imported if a restore is required.

### Signing Flows and Device Pairing

- Connect hardware wallets with `ord wallet connect --device <name>` and confirm
  the fingerprint Ord displays matches the device screen before approving any
  PSBTs.
- For hot wallets, rotate RPC credentials shared with automation to align with
  the principle of least privilege. The richer wallet telemetry in 2.0 can expose
  operational details to overly permissive clients.
- Capture the structured JSON logs Ord emits for each wallet command. They
  provide a deterministic history that can be replayed to validate custody flows.

### PSBT Handling and Review

- PSBT-based offers can now be submitted from the command line:
  ```sh
  ord wallet offers submit --psbt offer.psbt
  ```
- Before signing, inspect every PSBT on the hardware device or a trusted
  inspector. Confirm that inscription-bearing inputs remain intact and that
  change outputs align with your segregation policy.
- If you automate offer submissions, require a human-in-the-loop review for the
  first transactions after the upgrade to validate parsing logic against the new
  wallet events.

### Post-Cutover Validation

- After the first sweep or send, rerun `ord wallet inscriptions` to confirm each
  artifact resides on the intended output.
- Use `bitcoin-cli getwalletinfo` to verify the backing Core wallet remains in a
  healthy state and that descriptors stayed synchronized.
- Establish alerts on wallet log streams for unexpected intents or destinations.

### Wallet Hardening Checklist

1. Confirm new PSBT workflows with a small value test trade before moving
   high-value inscriptions.
2. Back up your `wallet.json` metadata after the first successful sweep so the
   new change-tracking heuristics can be restored quickly if needed.
3. Rotate any API tokens, webhook secrets, or RPC credentials shared with tooling
   that depended on legacy wallet layouts.
4. Update internal runbooks to reference the new structured log fields and
   include step-by-step recovery procedures for accidental change reuse.

## Explorer and API Updates

## Wallet Enhancements

Ord 2.0 refines wallet ergonomics while staying anchored to Bitcoin Core for key
management.

- Use `ord wallet sweep` to consolidate dust and recover stranded inscriptions.
- Continue to manage hardware wallets through Bitcoin Core or external tools;
  Ord 2.0 does not add a dedicated `wallet connect` command.
- Create PSBT offers with `ord wallet offer create --inscription <id> --amount
  <sats> --fee-rate <rate>` and optionally `--submit <url>` to post them to a
  marketplace.
- Accept incoming offers with `ord wallet offer accept --psbt <base64>
  --inscription <id> --amount <sats>` to sign and broadcast the finalized
  transaction.

Continue to keep ordinal and cardinal funds separated. Bitcoin Core still does
not track inscription provenance and ordinary Bitcoin RPC calls can spend your
collectibles inadvertently.

## Explorer & API Updates

- Explorer endpoints now default to JSON output. Pass `format=html` when you
  need the legacy template rendering.
- Deprecated `/v1/` endpoints remain functional for now but plan to migrate to
  the versioned `/api/` routes to access pagination metadata and improved error
  handling.
- Long-polling websocket feeds are more reliable in 2.0. If you run custom bots,
  verify they reconnect using the new exponential backoff behavior.
- HTTP responses now echo pagination hints and rate-limit metadata in headers.
  Capture them in client logs to understand your remaining burst capacity.
- Expanded query parameters let you embed related inscriptions and rune
  metadata, reducing the number of round trips marketplaces must perform.

### Client Migration Playbook

1. Update integration tests to hit the `/api/` routes and assert against the new
   error codes.
2. Persist pagination cursors exactly as returned; they are opaque identifiers
   rather than integers.
3. Record the `schemaVersion` field from JSON responses and alarm if it changes
   unexpectedly in production so breaking API shifts can be triaged quickly.
4. For websocket consumers, cache the last delivered sequence number and
  re-submit it when reconnecting to avoid duplicate processing.

When migrating bots or dashboards, request the JSON schema once and cache it
locally; 2.0 ships with a stable `schemaVersion` field to allow validation.

## Operations Runbook

## Operational Tips

- Monitor disk usage during the first start; pruning stale rows can briefly
  spike IO.
- Update any automation that depended on the old indexer logs. Structured logs
  now use JSON with `event` and `target` fields.
- Revisit your backup cadence once the migration completes. The compacted data
  may allow you to increase snapshot frequency without additional storage.
- Schedule follow-up health checks 24 hours after the upgrade to confirm journal
  growth has stabilized and that alert thresholds remain appropriate.

### Advanced Operations

- **Indexer concurrency**: Monitor CPU pressure during initial syncs and adjust
  your process manager's limits if `ord` competes with other workloads.
- **Compaction windows**: Schedule maintenance windows shortly after daily
  prunes to snapshot a fully compacted database for backups.
- **Archive strategy**: Operators who previously pinned the full UTXO set should
  retain historical snapshot exports alongside the new compact format for
  compliance reviews.

### Observability and Alerting

- Ingest the new structured logs into your logging stack and alert on
  `"level":"error"` events from the indexer and wallet components.
- Track sync progress by comparing the reported chain tip against a trusted
  Bitcoin RPC source; page the on-call team if the delta grows beyond your SLA.
- Record compaction runtimes and disk usage in your monitoring system so you can
  spot regressions when future releases modify index heuristics.
- Expose dashboard panels for `queue.depth.index`, `tip.lag_blocks`, and wallet
  intent counts to identify emerging bottlenecks before they impact users.

## Staged Rollout and Contingency Planning

1. Upgrade a canary node first and leave it running for a full reorg window
   (typically 144 blocks) while monitoring API, wallet, and explorer behavior.
2. Once the canary is stable, roll the upgrade to additional nodes in waves to
   keep spare capacity online for rollback if needed.
3. If you encounter blocking issues, stop the 2.0 binary, restore the `.ord`
   backup taken earlier, and restart the previous release. Confirm clients fall
   back cleanly before retrying the migration.

## Where to Learn More

- Consult the [Ord 2.0 operations manual](ord-2.0-operations-manual.md) for
  detailed diagrams and architecture discussions.
- Join the [Ordinals Discord](https://discord.gg/ordinals) or open a discussion
  on [GitHub](https://github.com/ordinals/ord/discussions) if you run into
  issues while upgrading.

## Need Help?

Join the [Ordinals Discord](https://discord.gg/ordinals) or open a discussion on
[GitHub](https://github.com/ordinals/ord/discussions) if you run into issues
while upgrading.
