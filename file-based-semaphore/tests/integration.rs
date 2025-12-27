//! Integration tests for file-based semaphore
//!
//! These tests verify concurrent access patterns and real-world usage scenarios.

use file_based_semaphore::{LockInfo, Semaphore, SemaphoreConfig, SemaphoreError};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

fn temp_lock_path(name: &str) -> PathBuf {
    let mut path = env::temp_dir();
    path.push(format!(
        "file-semaphore-integration-{}-{}.lock",
        name,
        process::id()
    ));
    path
}

fn cleanup(path: &PathBuf) {
    let _ = fs::remove_file(path);
}

#[test]
fn test_concurrent_access_same_process() {
    let path = temp_lock_path("concurrent-same");
    cleanup(&path);

    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];

    // Spawn 3 threads trying to acquire the lock
    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        let path = path.clone();

        handles.push(thread::spawn(move || {
            barrier.wait();

            // All threads try to acquire at roughly the same time
            let sem_instance = Semaphore::with_defaults(&path).unwrap();
            let result = sem_instance.try_acquire();

            match result {
                Ok(_guard) => {
                    // Simulate some work
                    thread::sleep(Duration::from_millis(50));
                    true
                }
                Err(SemaphoreError::AlreadyLocked { .. }) => false,
                Err(e) => panic!("Unexpected error in thread {}: {:?}", i, e),
            }
        }));
    }

    let results: Vec<bool> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // Exactly one thread should have acquired the lock
    let acquired_count = results.iter().filter(|&&x| x).count();
    assert_eq!(
        acquired_count, 1,
        "Expected exactly 1 thread to acquire lock, got {}",
        acquired_count
    );

    cleanup(&path);
}

#[test]
fn test_sequential_acquire_release() {
    let path = temp_lock_path("sequential");
    cleanup(&path);

    let sem = Semaphore::with_defaults(&path).unwrap();

    // Acquire and release multiple times
    for i in 0..5 {
        {
            let guard = sem
                .try_acquire()
                .expect(&format!("Failed to acquire on iteration {}", i));
            assert!(sem.is_locked());
            // Guard drops here, releasing the lock
            drop(guard);
        }
        assert!(
            !sem.is_locked(),
            "Lock should be released after guard drops"
        );
    }

    cleanup(&path);
}

#[test]
fn test_acquire_with_timeout_success() {
    let path = temp_lock_path("timeout-success");
    cleanup(&path);

    // First, acquire and release in a thread with short hold time
    let path_clone = path.clone();
    let handle = thread::spawn(move || {
        let sem = Semaphore::with_defaults(&path_clone).unwrap();
        let _guard = sem.try_acquire().unwrap();
        thread::sleep(Duration::from_millis(100));
        // Guard drops here
    });

    // Wait a bit for thread to acquire
    thread::sleep(Duration::from_millis(50));

    // Try to acquire with timeout longer than hold time
    let config = SemaphoreConfig {
        acquire_timeout: Some(Duration::from_millis(500)),
        retry_interval: Duration::from_millis(10),
        ..Default::default()
    };
    let sem2 = Semaphore::new(&path, config).unwrap();

    let result = sem2.acquire();
    assert!(result.is_ok(), "Should have acquired lock after waiting");

    handle.join().unwrap();
    cleanup(&path);
}

#[test]
fn test_stale_lock_recovery() {
    let path = temp_lock_path("stale-recovery");
    cleanup(&path);

    // Create a stale lock file manually
    let old_info = LockInfo {
        pid: 99999,   // Non-existent PID
        timestamp: 0, // Very old timestamp
        tag: Some("stale-test".to_string()),
    };
    fs::write(&path, old_info.serialize()).unwrap();

    // Create semaphore with short stale timeout
    let config = SemaphoreConfig {
        stale_timeout: Some(Duration::from_secs(1)),
        ..Default::default()
    };
    let sem = Semaphore::new(&path, config).unwrap();

    // Should be able to acquire because lock is stale
    let result = sem.try_acquire();
    assert!(result.is_ok(), "Should acquire stale lock");

    cleanup(&path);
}

#[test]
fn test_lock_info_persistence() {
    let path = temp_lock_path("info-persistence");
    cleanup(&path);

    let sem = Semaphore::with_defaults(&path).unwrap();
    let tag = "integration-test-tag";

    {
        let info = LockInfo::with_tag(tag);
        let _guard = sem.try_acquire_with_info(info).unwrap();

        // Read back lock info
        let read_info = sem.lock_info().unwrap();
        assert_eq!(read_info.pid, process::id());
        assert_eq!(read_info.tag, Some(tag.to_string()));
    }

    cleanup(&path);
}

#[test]
fn test_multiple_semaphores_different_files() {
    let path1 = temp_lock_path("multi-1");
    let path2 = temp_lock_path("multi-2");
    cleanup(&path1);
    cleanup(&path2);

    let sem1 = Semaphore::with_defaults(&path1).unwrap();
    let sem2 = Semaphore::with_defaults(&path2).unwrap();

    // Both should be acquirable since they're different files
    let guard1 = sem1.try_acquire().unwrap();
    let guard2 = sem2.try_acquire().unwrap();

    assert!(sem1.is_locked());
    assert!(sem2.is_locked());

    drop(guard1);
    drop(guard2);

    cleanup(&path1);
    cleanup(&path2);
}

#[test]
fn test_wait_for_lock_release() {
    let path = temp_lock_path("wait-release");
    cleanup(&path);

    let path_clone = path.clone();

    // Thread 1: Hold lock for 200ms
    let holder = thread::spawn(move || {
        let sem = Semaphore::with_defaults(&path_clone).unwrap();
        let _guard = sem.try_acquire().unwrap();
        thread::sleep(Duration::from_millis(200));
        // Guard drops, releasing lock
    });

    // Wait for holder to acquire
    thread::sleep(Duration::from_millis(50));

    // Thread 2: Wait for lock to be released (not acquire, just wait)
    let sem = Semaphore::with_defaults(&path).unwrap();
    let start = std::time::Instant::now();

    while sem.is_locked() {
        if start.elapsed() > Duration::from_secs(1) {
            panic!("Timeout waiting for lock to be released");
        }
        thread::sleep(Duration::from_millis(10));
    }

    holder.join().unwrap();

    // Lock should be free now
    assert!(!sem.is_locked());

    cleanup(&path);
}

#[test]
fn test_force_release_while_locked() {
    let path = temp_lock_path("force-release");
    cleanup(&path);

    let sem1 = Semaphore::with_defaults(&path).unwrap();
    let _guard = sem1.try_acquire().unwrap();

    // Another instance force-releases
    let sem2 = Semaphore::with_defaults(&path).unwrap();
    assert!(sem2.force_release().is_ok());

    // Lock should be gone
    assert!(!sem1.is_locked());

    cleanup(&path);
}

#[test]
fn test_acquire_after_release() {
    let path = temp_lock_path("acquire-after-release");
    cleanup(&path);

    let sem = Semaphore::with_defaults(&path).unwrap();

    // Acquire, release, acquire again
    {
        let _guard = sem.try_acquire().unwrap();
    }

    // Should be able to acquire again
    let result = sem.try_acquire();
    assert!(result.is_ok());

    cleanup(&path);
}

#[test]
fn test_config_no_stale_detection() {
    let path = temp_lock_path("no-stale");
    cleanup(&path);

    // Create old lock
    let old_info = LockInfo {
        pid: 99999,
        timestamp: 0,
        tag: None,
    };
    fs::write(&path, old_info.serialize()).unwrap();

    // Create semaphore with stale detection disabled
    let config = SemaphoreConfig {
        stale_timeout: None,
        ..Default::default()
    };
    let sem = Semaphore::new(&path, config).unwrap();

    // Should NOT be able to acquire because stale detection is off
    let result = sem.try_acquire();
    assert!(result.is_err());

    cleanup(&path);
}

#[test]
fn test_error_types() {
    let path = temp_lock_path("error-types");
    cleanup(&path);

    let sem = Semaphore::with_defaults(&path).unwrap();
    let _guard = sem.try_acquire().unwrap();

    let sem2 = Semaphore::with_defaults(&path).unwrap();
    let err = sem2.try_acquire().unwrap_err();

    // Verify error info
    match err {
        SemaphoreError::AlreadyLocked {
            holder_pid,
            locked_since,
        } => {
            assert_eq!(holder_pid, Some(process::id()));
            assert!(locked_since.is_some());
        }
        _ => panic!("Expected AlreadyLocked error"),
    }

    cleanup(&path);
}

// =============================================================================
// Security Tests
// =============================================================================

#[test]
fn test_security_path_traversal_attempt() {
    // Path with traversal attempt should still work (paths are not sanitized,
    // but the OS handles them safely)
    let traversal_path = temp_lock_path("../../../tmp/traversal-test");

    // This will create the lock in the temp directory with a normalized path
    // The OS handles path canonicalization
    let result = Semaphore::with_defaults(&traversal_path);

    // May succeed or fail depending on OS path handling, but should not crash
    match result {
        Ok(sem) => {
            // Clean up if it succeeded
            let _ = sem.force_release();
        }
        Err(_) => {
            // Failure is acceptable - no crash is the important part
        }
    }
}

#[test]
fn test_security_null_bytes_in_lock_file() {
    let path = temp_lock_path("null-byte-lock");
    cleanup(&path);

    // Write lock info with null bytes manually
    fs::write(&path, "pid=123\0timestamp=0\0tag=test").unwrap();

    let sem = Semaphore::with_defaults(&path).unwrap();

    // Should handle gracefully - either treat as corrupted or locked
    let result = sem.try_acquire();

    // Should not panic - either error or success is acceptable
    match result {
        Ok(_guard) => {
            // Managed to recover from corrupted file
        }
        Err(_) => {
            // Treated as locked - acceptable
        }
    }

    cleanup(&path);
}

#[test]
fn test_security_very_long_path() {
    // Test with path that exceeds typical OS limits
    let long_name = "a".repeat(500);
    let path = temp_lock_path(&long_name);

    let result = Semaphore::with_defaults(&path);

    // Should fail gracefully, not crash or hang
    match result {
        Ok(sem) => {
            // If it somehow succeeded, clean up
            let _ = sem.force_release();
        }
        Err(_) => {
            // Expected - long paths typically fail
        }
    }
}

#[test]
fn test_security_very_long_lock_content() {
    let path = temp_lock_path("long-content-test");
    cleanup(&path);

    // Write very large lock file content
    let large_content = "x".repeat(1_000_000);
    fs::write(&path, &large_content).unwrap();

    let sem = Semaphore::with_defaults(&path).unwrap();

    // Should handle gracefully - file is invalid JSON so should handle as corrupted
    let result = sem.try_acquire();

    // Should not panic - either error or success is acceptable
    match result {
        Ok(_guard) => {
            // Managed to recover from corrupted file
        }
        Err(_) => {
            // Treated as locked - acceptable
        }
    }

    cleanup(&path);
}

#[test]
fn test_security_unicode_in_path() {
    // Unicode paths are valid on modern systems
    let path = temp_lock_path("unicode-测试-тест-🔒");
    cleanup(&path);

    let result = Semaphore::with_defaults(&path);

    // Should work on most modern systems
    match result {
        Ok(sem) => {
            let guard = sem.try_acquire();
            assert!(guard.is_ok());
            drop(guard);
            cleanup(&path);
        }
        Err(_) => {
            // Some filesystems may not support unicode
        }
    }
}

#[test]
fn test_security_concurrent_force_release_safety() {
    // Verify concurrent force_release doesn't cause issues
    let path = temp_lock_path("concurrent-force");
    cleanup(&path);

    let sem = Semaphore::with_defaults(&path).unwrap();
    let _guard = sem.try_acquire().unwrap();

    let path_clone = path.clone();
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let p = path_clone.clone();
            thread::spawn(move || {
                let sem = Semaphore::with_defaults(&p).unwrap();
                // Multiple force_release calls should be safe
                let _ = sem.force_release();
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    cleanup(&path);
}

#[test]
fn test_security_lock_info_tampering() {
    let path = temp_lock_path("tampered-lock");
    cleanup(&path);

    // Write invalid JSON to lock file
    fs::write(&path, "not valid json at all!").unwrap();

    let sem = Semaphore::with_defaults(&path).unwrap();

    // Should handle corrupted lock file gracefully
    // Either reject (already locked) or clean up and acquire
    let result = sem.try_acquire();

    // Should not panic - either error or success is acceptable
    match result {
        Ok(_guard) => {
            // Managed to recover
        }
        Err(_) => {
            // Couldn't recover - that's fine too
        }
    }

    cleanup(&path);
}

#[test]
fn test_security_stale_timeout_overflow() {
    let path = temp_lock_path("timeout-overflow");
    cleanup(&path);

    // Use maximum possible timeout to check for overflow issues
    let config = SemaphoreConfig {
        stale_timeout: Some(Duration::from_secs(u64::MAX)),
        ..Default::default()
    };

    let result = Semaphore::new(&path, config);

    // Should handle extreme values without crashing
    match result {
        Ok(sem) => {
            let _ = sem.try_acquire();
        }
        Err(_) => {
            // Rejection is acceptable
        }
    }

    cleanup(&path);
}
