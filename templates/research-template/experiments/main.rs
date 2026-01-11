//! Experiment Runner
//!
//! Run experiments to validate the hypothesis.
//!
//! Usage: cargo run --bin experiment

use research_name::experiment;
use std::time::Instant;

fn main() {
    println!("Research Name - Experiment Runner\n");
    println!("See HYPOTHESIS.md for what we're testing.\n");

    // Experiment 1: Basic functionality
    println!("=== Experiment 1: Basic Functionality ===");
    let start = Instant::now();

    let result = experiment("test input");
    println!("Result: {}", result);

    let elapsed = start.elapsed();
    println!("Time: {:?}\n", elapsed);

    // Experiment 2: Performance
    println!("=== Experiment 2: Performance ===");
    let iterations = 1000;
    let start = Instant::now();

    for _ in 0..iterations {
        let _ = experiment("performance test");
    }

    let elapsed = start.elapsed();
    let per_op = elapsed / iterations;
    println!("Total: {:?}", elapsed);
    println!("Per operation: {:?}", per_op);
    println!("Ops/sec: {:.0}\n", 1.0 / per_op.as_secs_f64());

    // Summary
    println!("=== Summary ===");
    println!("Update FINDINGS.md with these results.");
}
