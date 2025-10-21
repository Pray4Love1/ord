use chrono::{TimeZone, Utc};
use ord::genetics::fuse_with_traits;
use ord::living_inscription::{InscriptionCore, InscriptionState, LivingInscription};
use serde_json::json;

#[test]
fn test_fusion_with_traits() {
  let mut a = sample_inscription("A");
  a.core.metadata = json!({"traits": {"color": "red", "energy": 1.0}});
  let mut b = sample_inscription("B");
  b.core.metadata = json!({"traits": {"color": "blue", "energy": 0.6}});

  let child = fuse_with_traits(&a, &b);
  println!("\u{1f9ec} Fused child metadata: {}", child.core.metadata);

  let mutation_seed = child
    .core
    .metadata
    .get("mutation_seed")
    .and_then(|v| v.as_str())
    .expect("mutation seed present");

  let expected_seed = blake3::hash(format!("{}{}", a.commitment(), b.commitment()).as_bytes())
    .to_hex()
    .to_string();
  assert_eq!(mutation_seed, expected_seed);

  let traits = child
    .core
    .metadata
    .get("traits")
    .and_then(|v| v.as_object())
    .expect("traits present");

  let energy = traits
    .get("energy")
    .and_then(|v| v.as_f64())
    .expect("energy present");
  let avg_energy = (1.0 + 0.6) / 2.0;
  assert!(energy >= avg_energy * 0.95 && energy <= avg_energy * 1.05);

  let color = traits
    .get("color")
    .and_then(|v| v.as_str())
    .expect("color present");
  assert!(color.starts_with("red") || color.starts_with("blue"));

  let parents = child
    .core
    .metadata
    .get("parents")
    .and_then(|v| v.as_array())
    .expect("parents recorded");
  assert_eq!(parents.len(), 2);
  assert_eq!(parents[0], json!(a.commitment()));
  assert_eq!(parents[1], json!(b.commitment()));
}

fn sample_inscription(label: &str) -> LivingInscription {
  LivingInscription {
    core: InscriptionCore {
      version: 1,
      parent_hash: None,
      creator: format!("creator-{}", label),
      timestamp: Utc.timestamp_opt(1_650_000_000, 0).unwrap(),
      content_uri: format!("ipfs://{}", label),
      metadata: json!({}),
    },
    state: InscriptionState {
      block_height: 100,
      external_entropy: None,
      mood: Some("curious".into()),
      mirror_hash: None,
    },
    signature: format!("signature-{}", label),
  }
}
