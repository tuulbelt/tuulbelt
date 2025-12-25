//! Concurrent access example demonstrating multiple threads competing for a lock

use file_based_semaphore::{LockInfo, Semaphore, SemaphoreConfig, SemaphoreError};
use std::env;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

fn main() {
    let mut lock_path = env::temp_dir();
    lock_path.push("file-semaphore-concurrent.lock");

    println!("Lock file: {}", lock_path.display());
    println!("Starting 5 threads competing for the lock...\n");

    // Clean up any existing lock
    let _ = std::fs::remove_file(&lock_path);

    // Barrier to synchronize thread starts
    let barrier = Arc::new(Barrier::new(5));
    let mut handles = vec![];

    for i in 0..5 {
        let lock_path = lock_path.clone();
        let barrier = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            let config = SemaphoreConfig {
                stale_timeout: Some(Duration::from_secs(60)),
                acquire_timeout: Some(Duration::from_secs(10)),
                retry_interval: Duration::from_millis(50),
            };

            let sem = Semaphore::new(&lock_path, config).expect("Failed to create semaphore");

            // Wait for all threads to be ready
            barrier.wait();

            // Try to acquire with blocking
            let info = LockInfo::with_tag(format!("thread-{}", i));

            // Match result, ensuring temporary is dropped before sem
            match sem.acquire_with_info(info) {
                Ok(guard) => {
                    println!(
                        "Thread {} acquired the lock (PID: {})",
                        i,
                        std::process::id()
                    );

                    // Simulate work
                    thread::sleep(Duration::from_millis(200));

                    println!("Thread {} releasing the lock", i);
                    drop(guard);
                }
                Err(SemaphoreError::Timeout) => {
                    println!("Thread {} timed out waiting for lock", i);
                }
                Err(e) => {
                    println!("Thread {} error: {}", i, e);
                }
            };
            // Explicitly drop sem after match is done
            drop(sem);
        }));
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("\nAll threads completed!");

    // Clean up
    let _ = std::fs::remove_file(&lock_path);
}
