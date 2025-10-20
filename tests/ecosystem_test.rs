use ord::ecosystem::simulate_interaction;
use ord::evolution::{evolve, EvolutionTrigger};
use ord::genetics::fuse_with_traits;
use ord::living_inscription::LivingInscription;
use serde_json::json;

fn sample_inscription(name: &str) -> LivingInscription {
  let metadata = json!({
    "name": name,
    "traits": {
      "energy": 1.0,
      "resilience": 1.0,
    },
  });

  let base = LivingInscription::new(name, metadata);
  evolve(&base, EvolutionTrigger::MoodShift("curious".into()))
}

#[test]
fn simulate_small_ecosystem() {
  let a = sample_inscription("Alpha");
  let b = sample_inscription("Beta");
  let first_gen = fuse_with_traits(&a, &b);
  let second_gen = fuse_with_traits(&a, &first_gen);
  let results = simulate_interaction(&first_gen, &second_gen);
  for (i, child) in results.iter().enumerate() {
    println!("Entity {} -> commitment {}", i, child.commitment());
    println!("Mood: {:?}", child.state.mood);
    println!("Traits: {}", child.core.metadata);
  }
}
