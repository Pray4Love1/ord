use clap::{Parser, Subcommand};
use ord2::{mirror_bridge::default_db_path, MirrorBridge};

#[derive(Parser, Debug)]
#[command(about = "Inspect mirrored living inscriptions", version)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
  /// Fetch a mirror record by commitment hash
  Get { commitment: String },
  /// List stored mirror records
  List,
  /// Remove a commitment from the local mirror store
  Delete { commitment: String },
}

fn main() -> anyhow::Result<()> {
  let cli = Cli::parse();
  let bridge = MirrorBridge::new(default_db_path())?;

  match cli.command {
    Commands::Get { commitment } => match bridge.get(&commitment)? {
      Some(record) => println!("{}", serde_json::to_string_pretty(&record)?),
      None => println!("No mirror found for {commitment}"),
    },
    Commands::List => {
      let records = bridge.list()?;
      for record in records {
        println!("{} @ block {}", record.commitment, record.block_height);
      }
    }
    Commands::Delete { commitment } => {
      if bridge.delete(&commitment)? {
        println!("Removed {commitment}");
      } else {
        println!("No mirror found for {commitment}");
      }
    }
  }

  Ok(())
}
