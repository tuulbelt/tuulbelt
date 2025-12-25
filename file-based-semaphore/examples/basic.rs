//! Basic usage example for file-based semaphore

use file_based_semaphore::{Semaphore, SemaphoreConfig};
use std::env;
use std::time::Duration;

fn main() {
    // Create lock file path in temp directory
    let mut lock_path = env::temp_dir();
    lock_path.push("file-semaphore-example.lock");

    println!("Lock file: {}", lock_path.display());

    // Create semaphore with default configuration
    let config = SemaphoreConfig {
        stale_timeout: Some(Duration::from_secs(60)),
        acquire_timeout: Some(Duration::from_secs(5)),
        retry_interval: Duration::from_millis(100),
    };

    let semaphore = Semaphore::new(&lock_path, config).expect("Failed to create semaphore");

    // Check if already locked
    println!("Is locked: {}", semaphore.is_locked());

    // Try to acquire without blocking
    println!("\nTrying non-blocking acquire...");
    match semaphore.try_acquire() {
        Ok(guard) => {
            println!("Lock acquired!");
            println!("Lock info: {:?}", semaphore.lock_info());

            // Simulate some work
            println!("\nDoing work while holding lock...");
            std::thread::sleep(Duration::from_millis(500));

            // Guard automatically releases when dropped
            drop(guard);
            println!("Lock released!");
        }
        Err(e) => {
            println!("Could not acquire lock: {}", e);
        }
    }

    // Verify lock is released
    println!(
        "\nFinal lock status: {}",
        if semaphore.is_locked() {
            "LOCKED"
        } else {
            "FREE"
        }
    );
}
