super::*,
  crate::{
    runes::MintError,
    subcommand::{find::FindRangeOutput, server::query},
    templates::StatusHtml,
  },
  bitcoin::block::Header,
  bitcoincore_rpc::{
    Client,
    json::{
      GetBlockHeaderResult, GetBlockStatsResult, GetRawTransactionResult,
      GetRawTransactionResultVout, GetRawTransactionResultVoutScriptPubKey, GetTxOutResult,
    },
  },
  chrono::SubsecRound,
  indicatif::{ProgressBar, ProgressStyle},
  log::log_enabled,
  redb::{
    Database, DatabaseError, MultimapTable, MultimapTableDefinition, MultimapTableHandle,
    ReadOnlyTable, ReadableDatabase, ReadableMultimapTable, ReadableTable, ReadableTableMetadata,
    RepairSession, StorageError, Table, TableDefinition, TableHandle, TableStats, WriteTransaction,
  },
  std::{
    collections::HashMap,
    io::{BufWriter, Write},
    sync::Once,
  },
};

pub use self::entry::RuneEntry;

pub(crate) mod entry;
pub mod event;
mod fetcher;
mod lot;
mod reorg;
mod rtx;
mod updater;
mod utxo_entry;

#[cfg(test)]
pub(crate) mod testing;

const SCHEMA_VERSION: u64 = 31;

define_multimap_table! { SAT_TO_SEQUENCE_NUMBER, u64, u32 }
define_multimap_table! { SCRIPT_PUBKEY_TO_OUTPOINT, &[u8], OutPointValue }
define_multimap_table! { SEQUENCE_NUMBER_TO_CHILDREN, u32, u32 }
define_table! { HEIGHT_TO_BLOCK_HEADER, u32, &HeaderValue }
define_table! { HEIGHT_TO_LAST_SEQUENCE_NUMBER, u32, u32 }
define_table! { HOME_INSCRIPTIONS, u32, InscriptionIdValue }
define_table! { INSCRIPTION_ID_TO_SEQUENCE_NUMBER, InscriptionIdValue, u32 }
define_table! { INSCRIPTION_NUMBER_TO_SEQUENCE_NUMBER, i32, u32 }
define_table! { NUMBER_TO_OFFER, u64, &[u8] }
define_table! { OUTPOINT_TO_RUNE_BALANCES, &OutPointValue, &[u8] }
define_table! { OUTPOINT_TO_UTXO_ENTRY, &OutPointValue, &UtxoEntry }
define_table! { RUNE_ID_TO_RUNE_ENTRY, RuneIdValue, RuneEntryValue }
define_table! { RUNE_TO_RUNE_ID, u128, RuneIdValue }
define_table! { SAT_TO_SATPOINT, u64, &SatPointValue }
define_table! { SEQUENCE_NUMBER_TO_INSCRIPTION_ENTRY, u32, InscriptionEntryValue }
define_table! { SEQUENCE_NUMBER_TO_RUNE_ID, u32, RuneIdValue }
define_table! { SEQUENCE_NUMBER_TO_SATPOINT, u32, &SatPointValue }
define_table! { STATISTIC_TO_COUNT, u64, u64 }
define_table! { TRANSACTION_ID_TO_RUNE, &TxidValue, u128 }
define_table! { TRANSACTION_ID_TO_TRANSACTION, &TxidValue, &[u8] }
define_table! { WRITE_TRANSACTION_STARTING_BLOCK_COUNT_TO_TIMESTAMP, u32, u128 }

#[derive(Copy, Clone)]
pub(crate) enum Statistic {
  Schema = 0,
  BlessedInscriptions = 1,
  Commits = 2,
  CursedInscriptions = 3,
  IndexAddresses = 4,
  IndexInscriptions = 5,
  IndexRunes = 6,
  IndexSats = 7,
  IndexTransactions = 8,
  InitialSyncTime = 9,
  LostSats = 10,
  OutputsTraversed = 11,
  ReservedRunes = 12,
  Runes = 13,
  SatRanges = 14,
  UnboundInscriptions = 16,
  LastSavepointHeight = 17,
}
