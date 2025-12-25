//! Stale lock recovery example
//!
//! Demonstrates how file-based semaphore handles crashed processes
//! by detecting and recovering from stale locks.

use file_based_semaphore::{LockInfo, Semaphore, SemaphoreConfig};
use std::env;
use std::fs;
use std::time::Duration;

fn main() {
    let mut lock_path = env::temp_dir();
    lock_path.push("file-semaphore-stale-example.lock");

    println!("Stale Lock Recovery Example");
    println!("============================\n");
    println!("Lock file: {}\n", lock_path.display());

    // Clean up any existing lock
    let _ = fs::remove_file(&lock_path);

    // Step 1: Simulate a crashed process by creating an old lock file
    println!("Step 1: Simulating a crashed process...");
    let old_lock = LockInfo {
        pid: 99999,   // Non-existent PID
        timestamp: 0, // Very old timestamp (1970)
        tag: Some("crashed-process".to_string()),
    };
    fs::write(&lock_path, old_lock.serialize()).expect("Failed to create stale lock");
    println!("  Created stale lock file (PID 99999, timestamp 0)");
    println!("  Lock file contents:");
    println!("  {}", fs::read_to_string(&lock_path).unwrap().trim());
    println!();

    // Step 2: Try to acquire with stale detection disabled
    println!("Step 2: Trying to acquire with stale detection DISABLED...");
    let config_no_stale = SemaphoreConfig {
        stale_timeout: None, // Disable stale detection
        ..Default::default()
    };
    let sem_no_stale = Semaphore::new(&lock_path, config_no_stale).unwrap();
    match sem_no_stale.try_acquire() {
        Ok(_) => println!("  Unexpected: Lock acquired!"),
        Err(e) => println!("  Expected failure: {}", e),
    }
    println!();

    // Step 3: Now try with stale detection enabled
    println!("Step 3: Trying to acquire with stale detection ENABLED (1 second timeout)...");
    let config_with_stale = SemaphoreConfig {
        stale_timeout: Some(Duration::from_secs(1)), // 1 second stale timeout
        acquire_timeout: Some(Duration::from_secs(5)),
        retry_interval: Duration::from_millis(100),
    };
    let sem_with_stale = Semaphore::new(&lock_path, config_with_stale).unwrap();

    match sem_with_stale.try_acquire() {
        Ok(guard) => {
            println!("  SUCCESS: Stale lock was detected and cleaned up!");
            println!("  New lock acquired by this process.");

            // Show the new lock info
            if let Some(info) = sem_with_stale.lock_info() {
                println!("\n  New lock info:");
                println!("    PID: {}", info.pid);
                println!("    Timestamp: {}", info.timestamp);
            }

            drop(guard);
            println!("\n  Lock released.");
        }
        Err(e) => println!("  Unexpected failure: {}", e),
    }
    println!();

    // Step 4: Demonstrate process liveness check (Unix only)
    #[cfg(unix)]
    {
        println!("Step 4: Process liveness check (Unix only)...");
        println!(
            "  Is PID 1 running? {}",
            if Semaphore::is_process_running(1) {
                "yes (init/systemd)"
            } else {
                "no"
            }
        );
        println!(
            "  Is PID 99999 running? {}",
            if Semaphore::is_process_running(99999) {
                "yes"
            } else {
                "no (as expected)"
            }
        );
        println!(
            "  Is current process running? {}",
            if Semaphore::is_process_running(std::process::id()) {
                "yes"
            } else {
                "no"
            }
        );
    }

    // Clean up
    let _ = fs::remove_file(&lock_path);
    println!("\nExample complete!");
}
