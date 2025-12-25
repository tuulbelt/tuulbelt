//! File-Based Semaphore CLI
//!
//! Cross-platform file-based semaphore for process coordination.

use file_based_semaphore::{LockInfo, Semaphore, SemaphoreConfig, SemaphoreError};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::process;
use std::time::Duration;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_usage() {
    eprintln!(
        r#"file-semaphore {}

Cross-platform file-based semaphore for process coordination.

USAGE:
    file-semaphore <COMMAND> <LOCK_PATH> [OPTIONS]

COMMANDS:
    acquire     Acquire a lock (blocks until available or timeout)
    try         Try to acquire a lock without blocking
    release     Release a lock (force remove lock file)
    status      Show lock status and holder information
    wait        Wait for a lock to be released

OPTIONS:
    --timeout <SECONDS>     Timeout for acquire/wait operations (default: no timeout)
    --stale <SECONDS>       Stale lock timeout (default: 3600)
    --tag <STRING>          Tag to include in lock info
    --json                  Output status in JSON format
    --quiet, -q             Suppress non-essential output
    --help, -h              Show this help message
    --version, -V           Show version

EXAMPLES:
    # Try to acquire a lock (non-blocking)
    file-semaphore try /tmp/my.lock

    # Acquire a lock with 10 second timeout
    file-semaphore acquire /tmp/my.lock --timeout 10

    # Check lock status
    file-semaphore status /tmp/my.lock

    # Release/force-remove a lock
    file-semaphore release /tmp/my.lock

    # Wait for a lock to be released
    file-semaphore wait /tmp/my.lock --timeout 30

EXIT CODES:
    0   Success (lock acquired/released, or lock is free)
    1   Lock already held (for try/acquire with timeout)
    2   Invalid arguments or usage error
    3   IO or system error
"#,
        VERSION
    );
}

fn print_version() {
    println!("file-semaphore {}", VERSION);
}

#[derive(Debug)]
struct Args {
    command: String,
    lock_path: String,
    timeout: Option<Duration>,
    stale_timeout: Option<Duration>,
    tag: Option<String>,
    json: bool,
    quiet: bool,
}

fn parse_args() -> Result<Args, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("missing command".into());
    }

    // Check for help/version first
    for arg in &args[1..] {
        match arg.as_str() {
            "--help" | "-h" => {
                print_usage();
                process::exit(0);
            }
            "--version" | "-V" => {
                print_version();
                process::exit(0);
            }
            _ => {}
        }
    }

    if args.len() < 3 {
        return Err("missing lock path".into());
    }

    let command = args[1].clone();
    let lock_path = args[2].clone();

    // Validate command
    if !["acquire", "try", "release", "status", "wait"].contains(&command.as_str()) {
        return Err(format!("unknown command: {}", command));
    }

    let mut timeout = None;
    let mut stale_timeout = Some(Duration::from_secs(3600));
    let mut tag = None;
    let mut json = false;
    let mut quiet = false;

    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--timeout" => {
                i += 1;
                if i >= args.len() {
                    return Err("--timeout requires a value".into());
                }
                let secs: u64 = args[i]
                    .parse()
                    .map_err(|_| "invalid timeout value".to_string())?;
                timeout = Some(Duration::from_secs(secs));
            }
            "--stale" => {
                i += 1;
                if i >= args.len() {
                    return Err("--stale requires a value".into());
                }
                let secs: u64 = args[i]
                    .parse()
                    .map_err(|_| "invalid stale timeout value".to_string())?;
                if secs == 0 {
                    stale_timeout = None;
                } else {
                    stale_timeout = Some(Duration::from_secs(secs));
                }
            }
            "--tag" => {
                i += 1;
                if i >= args.len() {
                    return Err("--tag requires a value".into());
                }
                tag = Some(args[i].clone());
            }
            "--json" => {
                json = true;
            }
            "--quiet" | "-q" => {
                quiet = true;
            }
            arg => {
                return Err(format!("unknown option: {}", arg));
            }
        }
        i += 1;
    }

    Ok(Args {
        command,
        lock_path,
        timeout,
        stale_timeout,
        tag,
        json,
        quiet,
    })
}

/// Try to create a lock file directly (for CLI use)
fn try_create_lock_file(
    path: &Path,
    info: &LockInfo,
    stale_timeout: Option<Duration>,
) -> Result<(), SemaphoreError> {
    // Check for stale lock first
    if path.exists() {
        if let Some(timeout) = stale_timeout {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(existing_info) = LockInfo::parse(&content) {
                    if existing_info.is_stale(timeout) {
                        let _ = fs::remove_file(path);
                    }
                }
            }
        }
    }

    // Try to create lock file exclusively
    match OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(mut file) => {
            file.write_all(info.serialize().as_bytes())?;
            file.sync_all()?;
            Ok(())
        }
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            // Read existing lock info
            let lock_info = fs::read_to_string(path)
                .ok()
                .and_then(|c| LockInfo::parse(&c).ok());
            Err(SemaphoreError::AlreadyLocked {
                holder_pid: lock_info.as_ref().map(|i| i.pid),
                locked_since: lock_info.as_ref().map(|i| i.timestamp),
            })
        }
        Err(e) => Err(SemaphoreError::IoError(e)),
    }
}

fn cmd_try_acquire(args: &Args) -> i32 {
    let path = Path::new(&args.lock_path);

    let info = if let Some(ref tag) = args.tag {
        LockInfo::with_tag(tag)
    } else {
        LockInfo::new()
    };

    match try_create_lock_file(path, &info, args.stale_timeout) {
        Ok(()) => {
            if !args.quiet {
                println!("Lock acquired: {}", args.lock_path);
            }
            0
        }
        Err(SemaphoreError::AlreadyLocked {
            holder_pid,
            locked_since,
        }) => {
            if args.json {
                println!(
                    r#"{{"locked":true,"pid":{},"since":{}}}"#,
                    holder_pid.unwrap_or(0),
                    locked_since.unwrap_or(0)
                );
            } else if !args.quiet {
                eprintln!("Lock already held");
                if let Some(pid) = holder_pid {
                    eprintln!("  Holder PID: {}", pid);
                }
            }
            1
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            3
        }
    }
}

fn cmd_acquire(args: &Args) -> i32 {
    let path = Path::new(&args.lock_path);
    let start = std::time::Instant::now();

    let info = if let Some(ref tag) = args.tag {
        LockInfo::with_tag(tag)
    } else {
        LockInfo::new()
    };

    loop {
        match try_create_lock_file(path, &info, args.stale_timeout) {
            Ok(()) => {
                if !args.quiet {
                    println!("Lock acquired: {}", args.lock_path);
                }
                return 0;
            }
            Err(SemaphoreError::AlreadyLocked { .. }) => {
                // Check timeout
                if let Some(timeout) = args.timeout {
                    if start.elapsed() >= timeout {
                        if !args.quiet {
                            eprintln!("Timeout waiting for lock");
                        }
                        return 1;
                    }
                }
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                return 3;
            }
        }
    }
}

fn cmd_release(args: &Args) -> i32 {
    let sem = match Semaphore::with_defaults(&args.lock_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {}", e);
            return 3;
        }
    };

    match sem.force_release() {
        Ok(()) => {
            if !args.quiet {
                println!("Lock released: {}", args.lock_path);
            }
            0
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            3
        }
    }
}

fn cmd_status(args: &Args) -> i32 {
    let sem = match Semaphore::with_defaults(&args.lock_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {}", e);
            return 3;
        }
    };

    let locked = sem.is_locked();
    let info = sem.lock_info();

    if args.json {
        if let Some(info) = info {
            println!(
                r#"{{"locked":true,"pid":{},"timestamp":{},"tag":{}}}"#,
                info.pid,
                info.timestamp,
                info.tag
                    .map(|t| format!(r#""{}""#, t))
                    .unwrap_or_else(|| "null".to_string())
            );
        } else {
            println!(r#"{{"locked":false}}"#);
        }
    } else if locked {
        println!("Lock status: LOCKED");
        println!("  Path: {}", args.lock_path);
        if let Some(info) = info {
            println!("  PID: {}", info.pid);
            println!("  Timestamp: {}", info.timestamp);
            if let Some(tag) = info.tag {
                println!("  Tag: {}", tag);
            }

            // Check if process is still running (Unix only)
            #[cfg(unix)]
            {
                let running = Semaphore::is_process_running(info.pid);
                println!("  Process running: {}", if running { "yes" } else { "no" });
            }
        }
    } else {
        println!("Lock status: FREE");
        println!("  Path: {}", args.lock_path);
    }

    if locked {
        1
    } else {
        0
    }
}

fn cmd_wait(args: &Args) -> i32 {
    let config = SemaphoreConfig {
        stale_timeout: args.stale_timeout,
        retry_interval: Duration::from_millis(100),
        acquire_timeout: None,
    };

    let sem = match Semaphore::new(&args.lock_path, config) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {}", e);
            return 3;
        }
    };

    let start = std::time::Instant::now();

    loop {
        if !sem.is_locked() {
            if !args.quiet {
                println!("Lock is free: {}", args.lock_path);
            }
            return 0;
        }

        if let Some(timeout) = args.timeout {
            if start.elapsed() >= timeout {
                if !args.quiet {
                    eprintln!("Timeout waiting for lock to be released");
                }
                return 1;
            }
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}

fn main() {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!();
            print_usage();
            process::exit(2);
        }
    };

    let exit_code = match args.command.as_str() {
        "try" => cmd_try_acquire(&args),
        "acquire" => cmd_acquire(&args),
        "release" => cmd_release(&args),
        "status" => cmd_status(&args),
        "wait" => cmd_wait(&args),
        _ => {
            eprintln!("Unknown command: {}", args.command);
            2
        }
    };

    // Flush stdout before exit
    let _ = io::stdout().flush();
    process::exit(exit_code);
}
