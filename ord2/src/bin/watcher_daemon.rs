use ord2::{mirror_bridge::default_db_path, LivingInscription, MirrorBridge};
use std::{thread, time::Duration};

fn main() -> anyhow::Result<()> {
  let db_path = default_db_path();
  let bridge = MirrorBridge::new(db_path)?;

  println!("🔭 Starting Living Inscription Watcher...");

  let mut block_height = 839_800u64;
  loop {
    println!("⛓️  Checking Bitcoin block {block_height}");

    if block_height % 100 == 0 {
      let inscription = LivingInscription::simulated(block_height);
      println!(
        "📜 New simulated inscription detected at block {}",
        inscription.block_height
      );

      println!("🚀 Broadcasting mirror proof...");
      let record = bridge.store(&inscription)?;
      println!("Mirror response: OK ({})", record.commitment);
    }

    block_height += 1;
    thread::sleep(Duration::from_millis(500));
  }
}
