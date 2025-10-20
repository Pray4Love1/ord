use blake3;
use chrono::Utc;
use ord::fusion::fuse;
use ord::living_inscription::{InscriptionCore, InscriptionState, LivingInscription};
use serde_json::json;

#[test]
fn test_fusion() {
  let a = sample_inscription("A");
  let b = sample_inscription("B");
  let child = fuse(&a, &b);

  println!("\u{1f9ec} Fused commitment: {}", child.commitment());
  println!("Parents:\n  A {}\n  B {}", a.commitment(), b.commitment());

  let expected_lineage_input = format!("{}{}", a.commitment(), b.commitment());
  let expected_lineage = blake3::hash(expected_lineage_input.as_bytes()).to_hex().to_string();

  assert_eq!(child.core.parent_hash.as_ref(), Some(&expected_lineage));
  assert_eq!(child.state.block_height, a.state.block_height.max(b.state.block_height) + 1);
  assert_eq!(child.core.metadata["name"], json!("B"));
  assert_eq!(child.signature, "0xfusion");
}

fn sample_inscription(name: &str) -> LivingInscription {
  let core = InscriptionCore {
    version: 1,
    parent_hash: None,
    creator: name.into(),
    timestamp: Utc::now(),
    content_uri: format!("ipfs://{}", name),
    metadata: json!({ "name": name }),
  };
  let state = InscriptionState {
    block_height: 100,
    external_entropy: None,
    mood: Some("ready".into()),
    mirror_hash: None,
  };
  LivingInscription {
    core,
    state,
    signature: "0xmock".into(),
  }
}
