# Examples

Real-world usage patterns for Snapshot Comparison.

## API Testing

### REST API Response Validation

```bash
#!/bin/bash
# test-api.sh - Validate API responses against snapshots

set -e

API_BASE="https://api.example.com"
SNAPSHOT_DIR="./snapshots/api"

# Test users endpoint
echo "Testing /users..."
curl -s "$API_BASE/users" | snapcmp check api-users -d "$SNAPSHOT_DIR"

# Test products endpoint
echo "Testing /products..."
curl -s "$API_BASE/products" | snapcmp check api-products -d "$SNAPSHOT_DIR"

# Test with authentication
echo "Testing /me (authenticated)..."
curl -s -H "Authorization: Bearer $TOKEN" "$API_BASE/me" | \
    snapcmp check api-me -d "$SNAPSHOT_DIR"

echo "All API tests passed!"
```

### JSON Response with Semantic Diff

```bash
# Create JSON snapshot
curl -s https://api.example.com/config | snapcmp create config -t json

# Later, check for regressions
curl -s https://api.example.com/config | snapcmp check config -t json

# If different, you'll see semantic diff:
# + "new_field": "value"
# - "old_field": "removed"
# ~ "changed_field": "old" → "new"
```

## Build Verification

### Deterministic Build Output

```bash
#!/bin/bash
# verify-build.sh - Ensure builds are reproducible

set -e

# Build and capture output
echo "Building project..."
./build.sh > /tmp/build-output.txt 2>&1

# Check against snapshot
if ! cat /tmp/build-output.txt | snapcmp check build-output; then
    echo "Build output changed! Review diff above."
    exit 1
fi

echo "Build output matches expected"
```

### Binary Artifact Verification

```bash
# Store hash of compiled binary (for reproducible builds)
sha256sum target/release/myapp | snapcmp create binary-hash -t text

# Verify on next build
sha256sum target/release/myapp | snapcmp check binary-hash
```

## CI Integration

### GitHub Actions

```yaml
# .github/workflows/snapshot-tests.yml
name: Snapshot Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install snapcmp
        run: |
          cargo install --path snapshot-comparison

      - name: Run snapshot tests
        run: |
          ./run-tests.sh | snapcmp check test-output

      - name: Check API responses
        run: |
          for endpoint in users products orders; do
            curl -s "$API_URL/$endpoint" | snapcmp check "api-$endpoint"
          done
```

### Jenkins Pipeline

```groovy
pipeline {
    agent any
    stages {
        stage('Snapshot Tests') {
            steps {
                sh '''
                    # Check all snapshots
                    for snapshot in $(snapcmp list | grep -oP "\\w+"); do
                        ./generate-output.sh $snapshot | snapcmp check $snapshot
                    done
                '''
            }
        }
    }
    post {
        failure {
            archiveArtifacts artifacts: 'snapshots/**'
        }
    }
}
```

## Development Workflow

### Update Mode During Development

```bash
# Development: auto-update snapshots
./run-tests.sh | snapcmp check test-output --update

# Review changes
git diff snapshots/

# Commit if looks good
git add snapshots/
git commit -m "Update test snapshots"
```

### Interactive Approval

```bash
#!/bin/bash
# approve-snapshots.sh - Review and approve snapshot changes

for snapshot in $(snapcmp list | grep -oP '\w+'); do
    echo "=== Checking $snapshot ==="

    if ! ./generate.sh $snapshot | snapcmp check $snapshot 2>&1; then
        echo ""
        read -p "Update this snapshot? [y/N] " answer
        if [[ "$answer" == "y" ]]; then
            ./generate.sh $snapshot | snapcmp update $snapshot
            echo "Updated!"
        fi
    fi
    echo ""
done
```

## Rust Library Examples

### Test Helper Macro

```rust
use snapshot_comparison::{SnapshotStore, SnapshotConfig, CompareResult};
use std::path::PathBuf;
use std::sync::OnceLock;

static STORE: OnceLock<SnapshotStore> = OnceLock::new();

fn get_store() -> &'static SnapshotStore {
    STORE.get_or_init(|| {
        SnapshotStore::new(PathBuf::from("tests/snapshots"))
    })
}

macro_rules! assert_snapshot {
    ($name:expr, $value:expr) => {{
        let store = get_store();
        let config = SnapshotConfig::default();
        let content = format!("{:?}", $value);

        match store.check($name, content.as_bytes(), &config) {
            Ok(CompareResult::Match) => {}
            Ok(CompareResult::Mismatch { diff, .. }) => {
                panic!("Snapshot mismatch for '{}':\n{}",
                    $name, diff.format($name, true));
            }
            Err(e) => panic!("Snapshot error: {}", e),
            _ => unreachable!(),
        }
    }};
}

#[test]
fn test_user_serialization() {
    let user = User { name: "Alice", age: 30 };
    assert_snapshot!("user-serialization", user);
}
```

### JSON API Testing

```rust
use snapshot_comparison::{SnapshotStore, SnapshotConfig, CompareResult, FileType};
use std::path::PathBuf;

fn test_json_endpoint(endpoint: &str, expected_name: &str) {
    let store = SnapshotStore::new(PathBuf::from("tests/api-snapshots"));
    let config = SnapshotConfig {
        file_type: Some(FileType::Json),
        color: true,
        ..Default::default()
    };

    // Simulate API call
    let response = make_api_call(endpoint);

    match store.check(expected_name, response.as_bytes(), &config) {
        Ok(CompareResult::Match) => {
            println!("✓ {} matches", expected_name);
        }
        Ok(CompareResult::Mismatch { diff, .. }) => {
            eprintln!("✗ {} differs:", expected_name);
            eprintln!("{}", diff.format(expected_name, true));
            panic!("API response mismatch");
        }
        Err(e) => panic!("Error: {}", e),
        _ => {}
    }
}

#[test]
fn test_all_endpoints() {
    test_json_endpoint("/api/users", "api-users");
    test_json_endpoint("/api/products", "api-products");
    test_json_endpoint("/api/config", "api-config");
}
```

### Batch Operations

```rust
use snapshot_comparison::{SnapshotStore, SnapshotConfig, CompareResult};
use std::path::PathBuf;

fn verify_all_fixtures(fixtures_dir: &str, snapshot_prefix: &str) {
    let store = SnapshotStore::new(PathBuf::from("snapshots"));
    let config = SnapshotConfig::default();

    let mut passed = 0;
    let mut failed = 0;

    for entry in std::fs::read_dir(fixtures_dir).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_stem().unwrap().to_str().unwrap();
        let snapshot_name = format!("{}-{}", snapshot_prefix, name);

        let content = std::fs::read(&path).unwrap();

        match store.check(&snapshot_name, &content, &config) {
            Ok(CompareResult::Match) => {
                println!("✓ {}", snapshot_name);
                passed += 1;
            }
            Ok(CompareResult::Mismatch { .. }) => {
                println!("✗ {}", snapshot_name);
                failed += 1;
            }
            Err(e) => {
                println!("? {} ({})", snapshot_name, e);
                failed += 1;
            }
            _ => {}
        }
    }

    println!("\nResults: {} passed, {} failed", passed, failed);
    assert_eq!(failed, 0);
}
```

## Configuration Files

### Validate Config Changes

```bash
# Store known-good config
cat production.config | snapcmp create prod-config -t json

# Before deploy, verify config
cat production.config | snapcmp check prod-config -t json || {
    echo "Config changed! Diff shown above."
    echo "If intentional, run: cat production.config | snapcmp update prod-config -t json"
    exit 1
}
```

### Environment Comparison

```bash
# Store staging config
ssh staging "cat /etc/myapp/config.json" | snapcmp create staging-config -t json

# Store production config
ssh production "cat /etc/myapp/config.json" | snapcmp create prod-config -t json

# Compare them (using output-diffing-utility)
odiff json snapshots/staging-config.snap snapshots/prod-config.snap
```

## Cleanup Patterns

### CI Snapshot Cleanup

```bash
#!/bin/bash
# cleanup-snapshots.sh - Remove orphaned snapshots

# List of active tests
ACTIVE_TESTS=(
    "api-users"
    "api-products"
    "api-orders"
    "build-output"
)

# Convert to comma-separated
KEEP=$(IFS=,; echo "${ACTIVE_TESTS[*]}")

# Dry run first
echo "Would delete:"
snapcmp clean --keep "$KEEP" --dry-run

# Ask for confirmation
read -p "Proceed? [y/N] " answer
if [[ "$answer" == "y" ]]; then
    snapcmp clean --keep "$KEEP"
fi
```

### Automated Orphan Detection

```bash
# Find snapshots not referenced in test files
for snapshot in $(snapcmp list | grep -oP '\w+'); do
    if ! grep -r "$snapshot" tests/ > /dev/null; then
        echo "Orphaned: $snapshot"
    fi
done
```
