//! CLI entry point for output-diff

use output_diffing_utility::{
    detect_file_type, diff_binary, diff_text, format_binary_diff, format_unified_diff,
    DiffConfig, FileType,
};
use std::env;
use std::fs;
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

    // Read both files as bytes
    let content1 = match fs::read(file1) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading {}: {}", file1.display(), e);
            return ExitCode::from(2);
        }
    };

    let content2 = match fs::read(file2) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading {}: {}", file2.display(), e);
            return ExitCode::from(2);
        }
    };

    // Detect file types
    let ext1 = file1.extension().and_then(|e| e.to_str());
    let ext2 = file2.extension().and_then(|e| e.to_str());
    let type1 = detect_file_type(&content1, ext1);
    let type2 = detect_file_type(&content2, ext2);

    if verbose {
        eprintln!("[DEBUG] File 1 type: {:?}", type1);
        eprintln!("[DEBUG] File 2 type: {:?}", type2);
    }

    // Use the more specific type (prefer Text/Json over Binary)
    let file_type = match (type1, type2) {
        (FileType::Json, _) | (_, FileType::Json) => FileType::Json,
        (FileType::Text, _) | (_, FileType::Text) => FileType::Text,
        _ => FileType::Binary,
    };

    let config = DiffConfig {
        verbose,
        ..Default::default()
    };

    // Perform diff based on file type
    match file_type {
        FileType::Text => {
            // Convert to UTF-8 strings
            let text1 = match std::str::from_utf8(&content1) {
                Ok(t) => t,
                Err(_) => {
                    eprintln!("Error: File 1 is not valid UTF-8");
                    return ExitCode::from(2);
                }
            };
            let text2 = match std::str::from_utf8(&content2) {
                Ok(t) => t,
                Err(_) => {
                    eprintln!("Error: File 2 is not valid UTF-8");
                    return ExitCode::from(2);
                }
            };

            let result = diff_text(text1, text2, &config);
            if result.has_changes() {
                let formatted = format_unified_diff(
                    &result,
                    file1.to_str().unwrap_or("file1"),
                    file2.to_str().unwrap_or("file2"),
                );
                print!("{}", formatted);
                ExitCode::from(1)
            } else {
                if verbose {
                    println!("Files are identical");
                }
                ExitCode::SUCCESS
            }
        }
        FileType::Binary => {
            let result = diff_binary(&content1, &content2, &config);
            if result.is_identical() {
                if verbose {
                    println!("Files are identical");
                }
                ExitCode::SUCCESS
            } else {
                let formatted = format_binary_diff(&result);
                print!("{}", formatted);
                ExitCode::from(1)
            }
        }
        FileType::Json => {
            eprintln!("JSON diff not yet implemented");
            ExitCode::from(2)
        }
    }
}
