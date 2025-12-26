//! CLI entry point for output-diff

use output_diffing_utility::{diff_files, format_unified_diff, DiffConfig};
use std::env;
use std::path::Path;
use std::process::ExitCode;

fn print_help() {
    println!(
        r#"Usage: output-diff [OPTIONS] <FILE1> <FILE2>

Semantic diff tool for JSON, text, and binary files

Arguments:
  <FILE1>  First file to compare
  <FILE2>  Second file to compare

Options:
  -h, --help     Show this help message
  -V, --version  Show version information
  -v, --verbose  Enable verbose output

Examples:
  output-diff file1.txt file2.txt
  output-diff old.json new.json
"#
    );
}

fn print_version() {
    println!("output-diff {}", env!("CARGO_PKG_VERSION"));
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let mut verbose = false;
    let mut files = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_help();
                return ExitCode::SUCCESS;
            }
            "-V" | "--version" => {
                print_version();
                return ExitCode::SUCCESS;
            }
            "-v" | "--verbose" => {
                verbose = true;
                i += 1;
            }
            arg if !arg.starts_with('-') => {
                files.push(arg.to_string());
                i += 1;
            }
            unknown => {
                eprintln!("Error: Unknown option '{}'", unknown);
                eprintln!("Use --help for usage information");
                return ExitCode::from(2);
            }
        }
    }

    if files.len() != 2 {
        eprintln!(
            "Error: Expected exactly 2 file arguments, got {}",
            files.len()
        );
        eprintln!("Use --help for usage information");
        return ExitCode::from(2);
    }

    let file1 = Path::new(&files[0]);
    let file2 = Path::new(&files[1]);

    let config = DiffConfig {
        verbose,
        ..Default::default()
    };

    match diff_files(file1, file2, &config) {
        Ok(result) => {
            if result.has_changes() {
                let formatted = format_unified_diff(
                    &result,
                    file1.to_str().unwrap_or("file1"),
                    file2.to_str().unwrap_or("file2"),
                );
                print!("{}", formatted);
                ExitCode::from(1) // Files differ
            } else {
                if verbose {
                    println!("Files are identical");
                }
                ExitCode::SUCCESS // Files are identical
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::from(2) // Error occurred
        }
    }
}
