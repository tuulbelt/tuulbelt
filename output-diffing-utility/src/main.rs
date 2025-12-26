//! CLI entry point for output-diff

use output_diffing_utility::{
    detect_file_type, diff_binary, diff_json, diff_text, format_as_json_report,
    format_binary_diff, format_compact, format_json_diff, format_side_by_side,
    format_unified_diff_with_color, DiffConfig, DiffResult, FileType, OutputFormat,
};
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

fn print_help() {
    println!(
        r#"output-diff {}
Semantic diff tool for JSON, text, and binary files

USAGE:
    output-diff [OPTIONS] <FILE1> <FILE2>

ARGS:
    <FILE1>    First file to compare
    <FILE2>    Second file to compare

OPTIONS:
    -f, --format <FORMAT>       Output format [default: unified] [possible: unified, json, side-by-side, compact]
    -t, --type <TYPE>           Force file type [default: auto] [possible: text, json, binary, auto]
    -c, --context <LINES>       Context lines around diffs [default: 3]
        --color <WHEN>          Colored output [default: auto] [possible: auto, always, never]
    -q, --quiet                 Suppress output, only exit code
    -o, --output <FILE>         Write output to file
    -v, --verbose               Verbose output
    -h, --help                  Print help
    -V, --version               Print version

EXIT CODES:
    0    Files are identical
    1    Files differ
    2    Error occurred

EXAMPLES:
    # Basic diff
    output-diff file1.json file2.json

    # JSON report
    output-diff --format json data1.json data2.json

    # Force text diff on JSON files
    output-diff --type text config.json config2.json

    # Compact output
    output-diff --format compact file1.txt file2.txt

For more information, see: https://github.com/tuulbelt/output-diffing-utility
"#,
        env!("CARGO_PKG_VERSION")
    );
}

fn print_version() {
    println!("output-diff {}", env!("CARGO_PKG_VERSION"));
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    // Parse command-line options
    let mut format = OutputFormat::Unified;
    let mut force_type: Option<FileType> = None;
    let mut context_lines = usize::MAX; // Show all context by default
    let mut color_when = "auto";
    let mut quiet = false;
    let mut output_file: Option<String> = None;
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
            "-q" | "--quiet" => {
                quiet = true;
                i += 1;
            }
            "-f" | "--format" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --format requires a value");
                    return ExitCode::from(2);
                }
                format = match args[i + 1].as_str() {
                    "unified" => OutputFormat::Unified,
                    "json" => OutputFormat::Json,
                    "side-by-side" => OutputFormat::SideBySide,
                    "compact" => OutputFormat::Compact,
                    other => {
                        eprintln!(
                            "Error: Invalid format '{}'. Use: unified, json, side-by-side, or compact",
                            other
                        );
                        return ExitCode::from(2);
                    }
                };
                i += 2;
            }
            "-t" | "--type" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --type requires a value");
                    return ExitCode::from(2);
                }
                force_type = Some(match args[i + 1].as_str() {
                    "text" => FileType::Text,
                    "json" => FileType::Json,
                    "binary" => FileType::Binary,
                    "auto" => {
                        force_type = None;
                        i += 2;
                        continue;
                    }
                    other => {
                        eprintln!(
                            "Error: Invalid type '{}'. Use: text, json, binary, or auto",
                            other
                        );
                        return ExitCode::from(2);
                    }
                });
                i += 2;
            }
            "-c" | "--context" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --context requires a value");
                    return ExitCode::from(2);
                }
                match args[i + 1].parse::<usize>() {
                    Ok(n) => context_lines = n,
                    Err(_) => {
                        eprintln!("Error: --context must be a number");
                        return ExitCode::from(2);
                    }
                }
                i += 2;
            }
            "--color" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --color requires a value");
                    return ExitCode::from(2);
                }
                color_when = match args[i + 1].as_str() {
                    "auto" | "always" | "never" => &args[i + 1],
                    other => {
                        eprintln!(
                            "Error: Invalid color value '{}'. Use: auto, always, or never",
                            other
                        );
                        return ExitCode::from(2);
                    }
                };
                i += 2;
            }
            "-o" | "--output" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --output requires a value");
                    return ExitCode::from(2);
                }
                output_file = Some(args[i + 1].clone());
                i += 2;
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

    // Read files
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

    // Detect or use forced file type
    let ext1 = file1.extension().and_then(|e| e.to_str());
    let ext2 = file2.extension().and_then(|e| e.to_str());

    let file_type = if let Some(forced) = force_type {
        if verbose {
            eprintln!("[DEBUG] Using forced file type: {:?}", forced);
        }
        forced
    } else {
        let type1 = detect_file_type(&content1, ext1);
        let type2 = detect_file_type(&content2, ext2);

        if verbose {
            eprintln!("[DEBUG] File 1 type: {:?}", type1);
            eprintln!("[DEBUG] File 2 type: {:?}", type2);
        }

        // Use the more specific type (prefer Text/Json over Binary)
        match (type1, type2) {
            (FileType::Json, _) | (_, FileType::Json) => FileType::Json,
            (FileType::Text, _) | (_, FileType::Text) => FileType::Text,
            _ => FileType::Binary,
        }
    };

    // Determine if color should be enabled
    let use_color = match color_when {
        "always" => true,
        "never" => false,
        "auto" => {
            // Check if stdout is a terminal
            std::io::IsTerminal::is_terminal(&std::io::stdout())
        }
        _ => false,
    };

    // Create config
    let config = DiffConfig {
        context_lines,
        format,
        color: use_color,
        verbose,
    };

    // Perform diff and format output
    let (output, exit_code) = match file_type {
        FileType::Text => {
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
            let diff_result = DiffResult::Text(result.clone());

            let formatted = match format {
                OutputFormat::Unified => {
                    format_unified_diff_with_color(&result, &files[0], &files[1], config.color)
                }
                OutputFormat::Json => format_as_json_report(
                    &diff_result,
                    &files[0],
                    &files[1],
                    FileType::Text,
                ),
                OutputFormat::SideBySide => {
                    format_side_by_side(&diff_result, &files[0], &files[1], 120)
                }
                OutputFormat::Compact => format_compact(&diff_result),
            };

            let code = if result.has_changes() { 1 } else { 0 };
            (formatted, code)
        }
        FileType::Binary => {
            let result = diff_binary(&content1, &content2, &config);
            let diff_result = DiffResult::Binary(result.clone());

            let formatted = match format {
                OutputFormat::Unified => format_binary_diff(&result),
                OutputFormat::Json => format_as_json_report(
                    &diff_result,
                    &files[0],
                    &files[1],
                    FileType::Binary,
                ),
                OutputFormat::SideBySide => {
                    format_side_by_side(&diff_result, &files[0], &files[1], 120)
                }
                OutputFormat::Compact => format_compact(&diff_result),
            };

            let code = if result.is_identical() { 0 } else { 1 };
            (formatted, code)
        }
        FileType::Json => {
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

            match diff_json(text1, text2, &config) {
                Ok(result) => {
                    let diff_result = DiffResult::Json(result.clone());

                    let formatted = match format {
                        OutputFormat::Unified => format_json_diff(&result),
                        OutputFormat::Json => format_as_json_report(
                            &diff_result,
                            &files[0],
                            &files[1],
                            FileType::Json,
                        ),
                        OutputFormat::SideBySide => {
                            format_side_by_side(&diff_result, &files[0], &files[1], 120)
                        }
                        OutputFormat::Compact => format_compact(&diff_result),
                    };

                    let code = if result.has_changes() { 1 } else { 0 };
                    (formatted, code)
                }
                Err(e) => {
                    eprintln!("Error parsing JSON: {}", e);
                    return ExitCode::from(2);
                }
            }
        }
    };

    // Output results
    if !quiet {
        if exit_code == 0 && verbose {
            println!("Files are identical");
        }

        if exit_code == 1 || verbose {
            if let Some(file_path) = output_file {
                match fs::File::create(&file_path) {
                    Ok(mut file) => {
                        if let Err(e) = file.write_all(output.as_bytes()) {
                            eprintln!("Error writing to {}: {}", file_path, e);
                            return ExitCode::from(2);
                        }
                        if verbose {
                            eprintln!("[DEBUG] Output written to {}", file_path);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error creating {}: {}", file_path, e);
                        return ExitCode::from(2);
                    }
                }
            } else {
                print!("{}", output);
            }
        }
    }

    ExitCode::from(exit_code as u8)
}
