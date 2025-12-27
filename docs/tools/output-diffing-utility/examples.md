# Examples

Real-world usage examples for Output Diffing Utility.

## Testing & Validation

### Example 1: Test Assertion with Better Error Messages

```rust
#[cfg(test)]
mod tests {
    use output_diffing_utility::{diff_text, format_compact, DiffConfig, DiffResult};

    #[test]
    fn test_api_response() {
        let expected = r#"{
  "status": "success",
  "data": {
    "id": 123,
    "name": "Alice"
  }
}"#;

        let actual = call_api();  // Your function

        let config = DiffConfig::default();
        let result = diff_text(expected, actual, &config);

        assert!(
            !result.has_changes(),
            "API response differs:\n{}",
            format_compact(&DiffResult::Text(result))
        );
    }

    fn call_api() -> &'static str {
        r#"{
  "status": "success",
  "data": {
    "id": 123,
    "name": "Alice"
  }
}"#
    }
}
```

### Example 2: Snapshot Testing

```bash
#!/bin/bash
# snapshot-test.sh

# Generate output
./my-tool --mode production > /tmp/actual-output.json

# Compare with snapshot
if odiff --quiet snapshots/expected-output.json /tmp/actual-output.json; then
    echo "✅ Snapshot test passed"
else
    echo "❌ Snapshot test failed:"
    odiff --format compact snapshots/expected-output.json /tmp/actual-output.json

    # Update snapshot if UPDATE=1
    if [ "$UPDATE" = "1" ]; then
        echo "Updating snapshot..."
        cp /tmp/actual-output.json snapshots/expected-output.json
    else
        exit 1
    fi
fi
```

### Example 3: CI/CD Integration

```yaml
# .github/workflows/validate.yml
name: Validate Output
on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Build Output Diff Tool
        run: |
          cd output-diffing-utility
          cargo build --release

      - name: Generate Baseline
        run: ./scripts/generate-baseline.sh > baseline.json

      - name: Compare with Expected
        run: |
          if ! ./output-diffing-utility/target/release/odiff \
                 --format json \
                 --output diff-report.json \
                 expected-baseline.json baseline.json; then
            echo "Baseline validation failed"
            cat diff-report.json | jq '.summary'
            exit 1
          fi
```

## Configuration Management

### Example 4: Compare Config Files

```bash
# compare-configs.sh
#!/bin/bash

echo "Comparing staging vs production config..."

odiff \
    --format side-by-side \
    --color always \
    config/staging.json \
    config/production.json | less -R

# Generate JSON report for auditing
odiff \
    --format json \
    --output config-diff-report.json \
    config/staging.json \
    config/production.json

# Check for critical differences
if jq '.differences[] | select(.path | contains("database"))' config-diff-report.json | grep -q .; then
    echo "⚠️  WARNING: Database configuration differs!"
fi
```

### Example 5: Multi-Environment Validation

```rust
use output_diffing_utility::{diff_json, DiffConfig};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let environments = vec!["dev", "staging", "production"];
    let baseline = fs::read_to_string("config/dev.json")?;

    for env in &environments[1..] {
        let config_path = format!("config/{}.json", env);
        let config = fs::read_to_string(&config_path)?;

        let result = diff_json(&baseline, &config, &DiffConfig::default())?;

        if result.has_changes() {
            println!("❌ {} differs from dev:", env);
            for change in &result.changes {
                println!("  {:?}", change);
            }
        } else {
            println!("✅ {} matches dev", env);
        }
    }

    Ok(())
}
```

## Data Validation

### Example 6: Database Export Comparison

```bash
#!/bin/bash
# validate-export.sh

# Export databases
pg_dump prod_db > /tmp/prod-export.sql
pg_dump staging_db > /tmp/staging-export.sql

# Compare schemas
odiff \
    --format compact \
    --context 5 \
    <(grep "CREATE TABLE" /tmp/prod-export.sql | sort) \
    <(grep "CREATE TABLE" /tmp/staging-export.sql | sort)

if [ $? -eq 1 ]; then
    echo "Schema differences detected!"
    exit 1
fi
```

### Example 7: JSON Data Validation

```rust
use output_diffing_utility::{diff_json, format_json_diff, DiffConfig};

fn validate_api_migration(
    old_api_response: &str,
    new_api_response: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let config = DiffConfig::default();
    let result = diff_json(old_api_response, new_api_response, &config)?;

    if result.has_changes() {
        println!("API changes detected:");
        println!("{}", format_json_diff(&result));

        // Check for breaking changes
        for change in &result.changes {
            match change {
                output_diffing_utility::JsonChange::Removed { path, .. } => {
                    println!("⚠️  BREAKING: Field removed: {}", path);
                }
                _ => {}
            }
        }
    }

    Ok(())
}
```

## Build & Code Generation

### Example 8: Validate Generated Code

```bash
#!/bin/bash
# check-generated-code.sh

# Generate code
./codegen/generate.sh > /tmp/generated.rs

# Compare with committed version
if ! odiff --quiet src/generated.rs /tmp/generated.rs; then
    echo "❌ Generated code differs from committed version"
    echo "Run: ./codegen/generate.sh > src/generated.rs"
    odiff --format compact src/generated.rs /tmp/generated.rs
    exit 1
fi

echo "✅ Generated code is up to date"
```

### Example 9: Build Output Validation

```rust
use output_diffing_utility::{diff_binary, format_binary_diff, DiffConfig};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compare build artifacts
    let build1 = fs::read("target/release/my-app")?;
    let build2 = fs::read("expected-build/my-app")?;

    let config = DiffConfig::default();
    let result = diff_binary(&build1, &build2, &config);

    if !result.is_identical() {
        println!("Build artifacts differ!");
        println!("{}", format_binary_diff(&result));

        if build1.len() != build2.len() {
            println!("Size changed: {} → {} bytes", result.size1, result.size2);
        }
    }

    Ok(())
}
```

## Shell Script Integration

### Example 10: Pre-Commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

# Check if lockfiles changed
if git diff --cached --name-only | grep -q "package-lock.json"; then
    echo "Validating lockfile changes..."

    git show HEAD:package-lock.json > /tmp/old-lock.json 2>/dev/null || echo "{}" > /tmp/old-lock.json
    cp package-lock.json /tmp/new-lock.json

    odiff \
        --format compact \
        /tmp/old-lock.json \
        /tmp/new-lock.json

    read -p "Lockfile changed. Proceed with commit? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi
```

### Example 11: Release Validation

```bash
#!/bin/bash
# validate-release.sh

VERSION=$1
PREVIOUS_VERSION=$2

# Compare release artifacts
for artifact in binary.tar.gz checksums.txt; do
    echo "Comparing $artifact..."

    curl -sL "https://releases.example.com/v${PREVIOUS_VERSION}/${artifact}" -o /tmp/prev-${artifact}
    cp "dist/${artifact}" /tmp/new-${artifact}

    if odiff --type binary /tmp/prev-${artifact} /tmp/new-${artifact}; then
        echo "⚠️  WARNING: $artifact unchanged from previous release"
    else
        echo "✅ $artifact updated"
    fi
done
```

## Advanced Patterns

### Example 12: Diff with Preprocessing

```bash
#!/bin/bash
# diff-normalized.sh

# Normalize JSON before diffing
jq -S '.' file1.json > /tmp/file1-sorted.json
jq -S '.' file2.json > /tmp/file2-sorted.json

odiff /tmp/file1-sorted.json /tmp/file2-sorted.json
```

### Example 13: Progressive Diff Review

```bash
#!/bin/bash
# review-changes.sh

FILES=$(find . -name "*.json" -type f)

for file in $FILES; do
    echo "Reviewing: $file"

    if git show HEAD:"./$file" > /tmp/old-version 2>/dev/null; then
        if ! odiff --quiet /tmp/old-version "$file"; then
            echo "Changed: $file"
            odiff --format side-by-side --color always /tmp/old-version "$file" | less -R

            read -p "Continue to next file? (y/n) " -n 1 -r
            echo
            [[ ! $REPLY =~ ^[Yy]$ ]] && break
        fi
    fi
done
```

### Example 14: Bulk Validation

```rust
use output_diffing_utility::{diff_text, DiffConfig};
use std::fs;
use std::path::Path;

fn validate_directory(
    baseline_dir: &Path,
    current_dir: &Path
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut failures = Vec::new();
    let config = DiffConfig::default();

    for entry in fs::read_dir(baseline_dir)? {
        let entry = entry?;
        let filename = entry.file_name();
        let baseline_path = entry.path();
        let current_path = current_dir.join(&filename);

        if current_path.exists() {
            let baseline = fs::read_to_string(&baseline_path)?;
            let current = fs::read_to_string(&current_path)?;

            let result = diff_text(&baseline, &current, &config);

            if result.has_changes() {
                failures.push(format!("{}: {} changes",
                    filename.to_string_lossy(),
                    result.additions() + result.deletions()
                ));
            }
        } else {
            failures.push(format!("{}: MISSING", filename.to_string_lossy()));
        }
    }

    Ok(failures)
}
```

## Library Integration Examples

### Example 15: Custom Test Framework

```rust
pub struct DiffAsserter;

impl DiffAsserter {
    pub fn assert_json_eq(expected: &str, actual: &str) {
        use output_diffing_utility::{diff_json, format_json_diff, DiffConfig};

        let result = diff_json(expected, actual, &DiffConfig::default())
            .expect("Invalid JSON");

        if result.has_changes() {
            panic!("JSON assertion failed:\n{}", format_json_diff(&result));
        }
    }

    pub fn assert_text_eq(expected: &str, actual: &str) {
        use output_diffing_utility::{diff_text, format_compact, DiffConfig, DiffResult};

        let result = diff_text(expected, actual, &DiffConfig::default());

        if result.has_changes() {
            panic!("Text assertion failed:\n{}",
                format_compact(&DiffResult::Text(result)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_diff_asserter() {
        let expected = r#"{"status": "ok"}"#;
        let actual = get_api_response();

        DiffAsserter::assert_json_eq(expected, actual);
    }

    fn get_api_response() -> &'static str {
        r#"{"status": "ok"}"#
    }
}
```

## See Also

- [CLI Usage](/tools/output-diffing-utility/cli-usage) - Command-line reference
- [Library Usage](/tools/output-diffing-utility/library-usage) - Integration guide
- [Getting Started](/tools/output-diffing-utility/getting-started) - Quick start
