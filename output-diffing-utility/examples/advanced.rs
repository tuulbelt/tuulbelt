//! Advanced example demonstrating different output formats
//!
//! Run with: cargo run --example advanced

use output_diffing_utility::{
    diff_json, diff_text, format_as_json_report, format_compact, format_json_diff,
    format_side_by_side, format_unified_diff, DiffConfig, DiffResult, FileType, OutputFormat,
};

fn main() {
    println!("=== Output Format Demonstrations ===\n");

    // Text diff example
    text_diff_examples();

    // JSON diff example
    json_diff_examples();
}

fn text_diff_examples() {
    println!("--- Text Diff Examples ---\n");

    let old_text = "line 1\nline 2\nline 3";
    let new_text = "line 1\nline 2 modified\nline 3";

    let config = DiffConfig::default();
    let result = diff_text(old_text, new_text, &config);

    // Unified diff (default)
    println!("1. UNIFIED FORMAT:");
    println!("{}", format_unified_diff(&result, "old.txt", "new.txt"));

    // Compact format
    println!("\n2. COMPACT FORMAT:");
    let diff_result = DiffResult::Text(result.clone());
    println!("{}", format_compact(&diff_result));

    // JSON report
    println!("3. JSON REPORT:");
    println!(
        "{}",
        format_as_json_report(&diff_result, "old.txt", "new.txt", FileType::Text)
    );

    // Side-by-side
    println!("4. SIDE-BY-SIDE:");
    println!("{}", format_side_by_side(&diff_result, "old.txt", "new.txt", 120));
}

fn json_diff_examples() {
    println!("\n--- JSON Diff Examples ---\n");

    let old_json = r#"{"name": "Alice", "age": 30, "city": "Boston"}"#;
    let new_json = r#"{"name": "Bob", "age": 30, "hobbies": ["reading"]}"#;

    let config = DiffConfig {
        context_lines: 3,
        format: OutputFormat::Unified,
        color: false,
        verbose: false,
    };

    match diff_json(old_json, new_json, &config) {
        Ok(result) => {
            println!("1. UNIFIED FORMAT (JSON):");
            println!("{}", format_json_diff(&result));

            let diff_result = DiffResult::Json(result.clone());

            println!("\n2. COMPACT FORMAT (JSON):");
            println!("{}", format_compact(&diff_result));

            println!("3. JSON REPORT (JSON):");
            println!(
                "{}",
                format_as_json_report(&diff_result, "old.json", "new.json", FileType::Json)
            );
        }
        Err(e) => {
            eprintln!("JSON parsing error: {}", e);
        }
    }
}
