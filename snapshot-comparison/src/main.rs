//! CLI entry point for snapshot-comparison / snapcmp

use snapshot_comparison::{CompareResult, FileType, SnapshotConfig, SnapshotError, SnapshotStore};
use std::env;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::ExitCode;

fn print_help() {
    println!(
        r#"Usage: snapcmp <COMMAND> [OPTIONS]

Snapshot testing utility for regression detection with integrated diff output.

Commands:
  create <name>     Create a snapshot from stdin
  check <name>      Check stdin against stored snapshot
  update <name>     Update snapshot with new content from stdin
  list              List all snapshots
  delete <name>     Delete a snapshot
  clean             Clean orphaned snapshots (requires --keep)

Options:
  -d, --dir <PATH>  Snapshot directory (default: ./snapshots)
  -t, --type <TYPE> File type: text, json, binary (default: auto-detect)
  -c, --color       Enable colored diff output
  -u, --update      Update mode: update snapshot on mismatch instead of failing
  --context <N>     Number of context lines in diff (default: 3)
  --keep <names>    Comma-separated list of snapshot names to keep (for clean)
  --dry-run         For clean: show what would be deleted without deleting
  -h, --help        Show this help message
  -V, --version     Show version information

Examples:
  # Create a snapshot from command output
  my-program --generate | snapcmp create my-test

  # Check current output against snapshot
  my-program --generate | snapcmp check my-test

  # Update snapshot with new expected output
  my-program --generate | snapcmp update my-test

  # List all snapshots
  snapcmp list

  # Clean orphaned snapshots
  snapcmp clean --keep test-1,test-2
"#
    );
}

fn print_version() {
    println!("snapcmp {}", env!("CARGO_PKG_VERSION"));
}

#[derive(Debug)]
enum Command {
    Create(String),
    Check(String),
    Update(String),
    List,
    Delete(String),
    Clean,
}

#[derive(Debug)]
struct Args {
    command: Command,
    dir: PathBuf,
    file_type: Option<FileType>,
    color: bool,
    update_mode: bool,
    context_lines: usize,
    keep: Vec<String>,
    dry_run: bool,
}

fn parse_args() -> Result<Args, String> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No command provided. Use --help for usage information.".to_string());
    }

    let mut command: Option<Command> = None;
    let mut dir = PathBuf::from("snapshots");
    let mut file_type: Option<FileType> = None;
    let mut color = false;
    let mut update_mode = false;
    let mut context_lines = 3;
    let mut keep: Vec<String> = Vec::new();
    let mut dry_run = false;

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            "-V" | "--version" => {
                print_version();
                std::process::exit(0);
            }
            "-d" | "--dir" => {
                i += 1;
                if i >= args.len() {
                    return Err("--dir requires a path argument".to_string());
                }
                dir = PathBuf::from(&args[i]);
            }
            "-t" | "--type" => {
                i += 1;
                if i >= args.len() {
                    return Err("--type requires an argument: text, json, or binary".to_string());
                }
                file_type = Some(match args[i].as_str() {
                    "text" => FileType::Text,
                    "json" => FileType::Json,
                    "binary" => FileType::Binary,
                    other => {
                        return Err(format!(
                            "Unknown file type: {}. Use text, json, or binary.",
                            other
                        ))
                    }
                });
            }
            "-c" | "--color" => {
                color = true;
            }
            "-u" | "--update" => {
                update_mode = true;
            }
            "--context" => {
                i += 1;
                if i >= args.len() {
                    return Err("--context requires a number".to_string());
                }
                context_lines = args[i]
                    .parse()
                    .map_err(|_| format!("Invalid context lines: {}", args[i]))?;
            }
            "--keep" => {
                i += 1;
                if i >= args.len() {
                    return Err("--keep requires a comma-separated list of names".to_string());
                }
                keep = args[i].split(',').map(|s| s.trim().to_string()).collect();
            }
            "--dry-run" => {
                dry_run = true;
            }
            "create" => {
                i += 1;
                if i >= args.len() {
                    return Err("create requires a snapshot name".to_string());
                }
                command = Some(Command::Create(args[i].clone()));
            }
            "check" => {
                i += 1;
                if i >= args.len() {
                    return Err("check requires a snapshot name".to_string());
                }
                command = Some(Command::Check(args[i].clone()));
            }
            "update" => {
                i += 1;
                if i >= args.len() {
                    return Err("update requires a snapshot name".to_string());
                }
                command = Some(Command::Update(args[i].clone()));
            }
            "list" => {
                command = Some(Command::List);
            }
            "delete" => {
                i += 1;
                if i >= args.len() {
                    return Err("delete requires a snapshot name".to_string());
                }
                command = Some(Command::Delete(args[i].clone()));
            }
            "clean" => {
                command = Some(Command::Clean);
            }
            s if s.starts_with('-') => {
                return Err(format!("Unknown option: {}", s));
            }
            _ => {
                // If no command yet, treat as command
                if command.is_none() {
                    return Err(format!("Unknown command: {}. Use --help for usage.", arg));
                }
            }
        }
        i += 1;
    }

    let command =
        command.ok_or_else(|| "No command provided. Use --help for usage.".to_string())?;

    Ok(Args {
        command,
        dir,
        file_type,
        color,
        update_mode,
        context_lines,
        keep,
        dry_run,
    })
}

fn read_stdin() -> Result<Vec<u8>, String> {
    let mut buffer = Vec::new();
    io::stdin()
        .read_to_end(&mut buffer)
        .map_err(|e| format!("Failed to read stdin: {}", e))?;
    Ok(buffer)
}

fn run(args: Args) -> Result<(), String> {
    let store = SnapshotStore::new(args.dir);
    let config = SnapshotConfig {
        file_type: args.file_type,
        color: args.color,
        update_mode: args.update_mode,
        context_lines: args.context_lines,
    };

    match args.command {
        Command::Create(name) => {
            let content = read_stdin()?;
            let snapshot = store
                .create(&name, &content, &config)
                .map_err(format_error)?;
            println!(
                "Created snapshot '{}' ({} bytes, type: {:?})",
                snapshot.metadata.name, snapshot.metadata.size, snapshot.metadata.file_type
            );
        }

        Command::Check(name) => {
            let content = read_stdin()?;
            let result = store
                .check(&name, &content, &config)
                .map_err(format_error)?;

            match result {
                CompareResult::Match => {
                    println!("✓ Snapshot '{}' matches", name);
                }
                CompareResult::Mismatch { diff, .. } => {
                    eprintln!("✗ Snapshot '{}' does not match", name);
                    eprintln!();
                    eprintln!("{}", diff.format(&name, args.color));
                    return Err("Snapshot mismatch".to_string());
                }
                CompareResult::Updated { .. } => {
                    println!("↻ Snapshot '{}' updated", name);
                }
            }
        }

        Command::Update(name) => {
            let content = read_stdin()?;
            let snapshot = store
                .update(&name, &content, &config)
                .map_err(format_error)?;
            println!(
                "Updated snapshot '{}' ({} bytes)",
                snapshot.metadata.name, snapshot.metadata.size
            );
        }

        Command::List => {
            let snapshots = store.list().map_err(format_error)?;

            if snapshots.is_empty() {
                println!("No snapshots found in '{}'", store.base_dir().display());
            } else {
                println!("Snapshots in '{}':", store.base_dir().display());
                println!();
                for snapshot in &snapshots {
                    println!(
                        "  {} ({} bytes, {:?})",
                        snapshot.name, snapshot.size, snapshot.file_type
                    );
                }
                println!();
                println!("Total: {} snapshot(s)", snapshots.len());
            }
        }

        Command::Delete(name) => {
            store.delete(&name).map_err(format_error)?;
            println!("Deleted snapshot '{}'", name);
        }

        Command::Clean => {
            if args.keep.is_empty() {
                return Err("clean requires --keep to specify which snapshots to keep".to_string());
            }

            let keep_refs: Vec<&str> = args.keep.iter().map(|s| s.as_str()).collect();
            let deleted = store
                .clean(&keep_refs, args.dry_run)
                .map_err(format_error)?;

            if deleted.is_empty() {
                println!("No orphaned snapshots to clean");
            } else if args.dry_run {
                println!("Would delete {} snapshot(s):", deleted.len());
                for name in &deleted {
                    println!("  {}", name);
                }
            } else {
                println!("Deleted {} snapshot(s):", deleted.len());
                for name in &deleted {
                    println!("  {}", name);
                }
            }
        }
    }

    Ok(())
}

fn format_error(e: SnapshotError) -> String {
    match e {
        SnapshotError::NotFound(name) => format!("Snapshot not found: {}", name),
        SnapshotError::IoError(msg) => format!("IO error: {}", msg),
        SnapshotError::InvalidName(name) => format!("Invalid snapshot name: {}", name),
        SnapshotError::CorruptedSnapshot(msg) => format!("Corrupted snapshot: {}", msg),
        SnapshotError::InvalidUtf8(msg) => format!("Invalid UTF-8: {}", msg),
    }
}

fn main() -> ExitCode {
    match parse_args() {
        Ok(args) => match run(args) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("Error: {}", e);
                ExitCode::FAILURE
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}
