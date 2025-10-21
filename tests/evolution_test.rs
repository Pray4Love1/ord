use ord::evolution::{evolve, EvolutionTrigger};
use ord::fusion::fuse_with_traits;
use ord::living_inscription::sample_inscription;

#[test]
fn test_block_evolution() {
    let a = sample_inscription("A");
    let b = sample_inscription("B");
    let child = fuse_with_traits(&a, &b);
    println!("Initial traits: {}", child.core.metadata);
    let evolved = evolve(&child, EvolutionTrigger::BlockHeight(child.state.block_height + 100));
    println!("Evolved traits: {}", evolved.core.metadata);
}
