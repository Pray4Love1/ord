# Ord 2.0 Upgrade Guide

Ord 2.0 ships with a new indexer, refreshed wallet tooling, and expanded API
coverage. This guide highlights the key steps to upgrade from the 0.x releases
and adopt the latest functionality safely.

## Before You Begin

- Ensure your environment has **Rust 1.85.0 or later** and **Bitcoin Core v29**.
- Back up your `.ord` data directory and any custom configuration files.
- Review the [2.0.0 changelog entry](../../CHANGELOG.md#200---2025-10-15) for an
  overview of the major changes.

## Database Migration

Ord 2.0 introduces a compacted database layout. When you start the new binary it
will automatically perform an in-place migration.

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

## Operational Tips

- Monitor disk usage during the first start; pruning stale rows can briefly
  spike IO.
- Update any automation that depended on the old indexer logs. Structured logs
  now use JSON with `event` and `target` fields.
- Revisit your backup cadence once the migration completes. The compacted data
  may allow you to increase snapshot frequency without additional storage.

## Need Help?

Join the [Ordinals Discord](https://discord.gg/ordinals) or open a discussion on
[GitHub](https://github.com/ordinals/ord/discussions) if you run into issues
while upgrading.
