//! CLI entry point for tool-name

use std::env;
use std::process::ExitCode;
use tool_name::{process, Config};

fn print_help() {
    println!(
        r#"Usage: tool-name [OPTIONS] <INPUT>

One sentence description of what this tool does.

Arguments:
  <INPUT>  The input string to process

Options:
  -v, --verbose  Enable verbose output
  -h, --help     Show this help message
  -V, --version  Show version information

Examples:
  tool-name "hello world"
  tool-name --verbose "hello"
"#
    );
}

fn print_version() {
    println!("tool-name {}", env!("CARGO_PKG_VERSION"));
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut config = Config::default();
    let mut input: Option<String> = None;

    for arg in &args {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return ExitCode::SUCCESS;
            }
            "-V" | "--version" => {
                print_version();
                return ExitCode::SUCCESS;
            }
            "-v" | "--verbose" => {
                config.verbose = true;
            }
            s if s.starts_with('-') => {
                eprintln!("Error: Unknown option '{}'", s);
                eprintln!("Use --help for usage information");
                return ExitCode::FAILURE;
            }
            s => {
                input = Some(s.to_string());
            }
        }
    }

    let input = match input {
        Some(i) => i,
        None => {
            eprintln!("Error: No input provided");
            eprintln!("Use --help for usage information");
            return ExitCode::FAILURE;
        }
    };

    match process(&input, &config) {
        Ok(result) => {
            println!("{}", result);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}
