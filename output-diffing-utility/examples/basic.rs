//! Basic example of using output-diffing-utility as a library
//!
//! Run with: cargo run --example basic

use output_diffing_utility::{diff_text, DiffConfig};

fn main() {
    // Create diff configuration
    let config = DiffConfig::default();

    // Compare two text strings
    let old_text = "Hello World\nThis is a test\nLine 3";
    let new_text = "Hello World\nThis is modified\nLine 3";

    println!("=== Text Diff Example ===\n");

    let result = diff_text(old_text, new_text, &config);

    if result.has_changes() {
        println!("Files differ!");
        println!("Additions: {}", result.additions());
        println!("Deletions: {}", result.deletions());
        println!("\nChanges:");

        for change in &result.changes {
            match change {
                output_diffing_utility::LineChange::Added { new_line_num, line } => {
                    println!("  + Line {}: {}", new_line_num, line);
                }
                output_diffing_utility::LineChange::Removed { old_line_num, line } => {
                    println!("  - Line {}: {}", old_line_num, line);
                }
                output_diffing_utility::LineChange::Unchanged { .. } => {
                    // Context lines
                }
            }
        }
    } else {
        println!("Files are identical");
    }
}
