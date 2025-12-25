# Examples

Real-world usage patterns for File-Based Semaphore.

## Shell Script Examples

### Deployment Guard

Prevent concurrent deployments:

```bash
#!/bin/bash
set -e

LOCK_FILE="/var/lock/deploy.lock"

# Try to acquire lock
if ! file-semaphore try "$LOCK_FILE" --tag "deploy-$(hostname)-$$"; then
    echo "Another deployment is already running"
    file-semaphore status "$LOCK_FILE"
    exit 1
fi

# Ensure lock is released on exit
trap 'file-semaphore release "$LOCK_FILE"' EXIT

echo "Starting deployment..."
./deploy.sh

echo "Deployment complete"
```

### Cron Job Singleton

Ensure only one instance of a cron job runs:

```bash
#!/bin/bash
# /etc/cron.daily/backup-job

LOCK="/var/lock/backup.lock"

# Non-blocking try
if ! file-semaphore try "$LOCK" --stale 86400 -q; then
    # Another backup running or stale lock
    exit 0
fi

trap 'file-semaphore release "$LOCK" -q' EXIT

/usr/local/bin/run-backup.sh
```

### CI/CD Resource Guard

Coordinate shared resources in CI:

```bash
#!/bin/bash
# Wait up to 5 minutes for database lock

DB_LOCK="/tmp/ci-database.lock"

echo "Waiting for database access..."
if file-semaphore acquire "$DB_LOCK" --timeout 300 --tag "ci-job-${CI_JOB_ID}"; then
    trap 'file-semaphore release "$DB_LOCK"' EXIT

    echo "Running database migrations..."
    ./migrate.sh

    echo "Running integration tests..."
    ./run-tests.sh
else
    echo "Timeout: Could not get database lock"
    exit 1
fi
```

## Rust Examples

### Worker Pool Coordination

```rust
use file_based_semaphore::{Semaphore, SemaphoreConfig, LockInfo};
use std::time::Duration;
use std::thread;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SemaphoreConfig {
        stale_timeout: Some(Duration::from_secs(300)),
        acquire_timeout: Some(Duration::from_secs(60)),
        retry_interval: Duration::from_millis(50),
    };

    let sem = Arc::new(Semaphore::new("/tmp/worker.lock", config)?);
    let mut handles = vec![];

    for worker_id in 0..5 {
        let sem = Arc::clone(&sem);

        handles.push(thread::spawn(move || {
            let info = LockInfo::with_tag(format!("worker-{}", worker_id));

            match sem.acquire_with_info(info) {
                Ok(guard) => {
                    println!("Worker {} processing...", worker_id);
                    thread::sleep(Duration::from_millis(200));
                    println!("Worker {} done", worker_id);
                    drop(guard);
                }
                Err(e) => {
                    println!("Worker {} failed: {}", worker_id, e);
                }
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    Ok(())
}
```

### Resource Pool with Multiple Locks

```rust
use file_based_semaphore::{Semaphore, SemaphoreConfig};
use std::time::Duration;

struct ResourcePool {
    locks: Vec<Semaphore>,
}

impl ResourcePool {
    fn new(count: usize, base_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config = SemaphoreConfig {
            stale_timeout: Some(Duration::from_secs(60)),
            ..Default::default()
        };

        let locks: Result<Vec<_>, _> = (0..count)
            .map(|i| Semaphore::new(format!("{}.{}.lock", base_path, i), config.clone()))
            .collect();

        Ok(Self { locks: locks? })
    }

    fn acquire_any(&self) -> Option<usize> {
        for (i, lock) in self.locks.iter().enumerate() {
            if lock.try_acquire().is_ok() {
                return Some(i);
            }
        }
        None
    }

    fn release(&self, index: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.locks[index].force_release()?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = ResourcePool::new(3, "/tmp/resource")?;

    if let Some(idx) = pool.acquire_any() {
        println!("Got resource {}", idx);
        // Use resource...
        pool.release(idx)?;
    } else {
        println!("No resources available");
    }

    Ok(())
}
```

### Graceful Degradation

```rust
use file_based_semaphore::{Semaphore, SemaphoreError};
use std::time::Duration;

fn process_with_lock(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let sem = Semaphore::with_defaults("/tmp/process.lock")?;

    match sem.acquire_timeout(Duration::from_secs(5)) {
        Ok(_guard) => {
            // Exclusive processing
            println!("Processing exclusively: {}", data);
            expensive_operation(data)?;
        }
        Err(SemaphoreError::Timeout) => {
            // Fallback to non-exclusive processing
            println!("Processing without lock (degraded mode): {}", data);
            cheaper_operation(data)?;
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}
```

### Health Check with Lock

```rust
use file_based_semaphore::Semaphore;

fn check_health() -> Result<bool, Box<dyn std::error::Error>> {
    let sem = Semaphore::with_defaults("/var/run/app.lock")?;

    // Check if lock is held (app is running)
    if !sem.is_locked() {
        return Ok(false); // App not running
    }

    // Check if holder process is alive
    if let Some(info) = sem.lock_info() {
        let running = Semaphore::is_process_running(info.pid);
        if !running {
            // Stale lock - app crashed
            sem.force_release()?;
            return Ok(false);
        }
    }

    Ok(true) // App is healthy
}
```

## Integration Examples

### With systemd Service

```bash
# /usr/lib/systemd/system/myapp.service
[Unit]
Description=My Application
After=network.target

[Service]
Type=simple
ExecStartPre=/usr/bin/file-semaphore try /var/run/myapp.lock --tag "service"
ExecStart=/usr/bin/myapp
ExecStopPost=/usr/bin/file-semaphore release /var/run/myapp.lock
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

### With Docker Compose

```yaml
# docker-compose.yml
services:
  worker:
    image: myapp
    volumes:
      - locks:/var/locks
    command: |
      sh -c '
        file-semaphore try /var/locks/worker.lock --tag "worker-$$HOSTNAME" &&
        trap "file-semaphore release /var/locks/worker.lock" EXIT &&
        ./start-worker.sh
      '

volumes:
  locks:
```

### With GitHub Actions

```yaml
# .github/workflows/deploy.yml
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Acquire deploy lock
        run: |
          file-semaphore acquire /tmp/deploy.lock \
            --timeout 300 \
            --tag "github-run-${{ github.run_id }}"

      - name: Deploy
        run: ./deploy.sh

      - name: Release lock
        if: always()
        run: file-semaphore release /tmp/deploy.lock
```
