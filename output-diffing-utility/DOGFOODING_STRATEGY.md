# Dogfooding Strategy: Output Diffing Utility

This document outlines how the Output Diffing Utility will leverage other Tuulbelt tools to demonstrate composability.

## Multi-Tool Composition Goals

**Objective:** Demonstrate that Tuulbelt tools compose naturally through CLI interfaces, even across different programming languages (Rust + TypeScript).

## Identified Dogfooding Opportunities

### 1. CLI Progress Reporting (TypeScript → Rust)

**Use Case:** Show progress when diffing large files (>1MB)

**Integration Points:**
- When comparing large JSON files (multi-MB)
- When processing binary files byte-by-byte
- When generating large diff outputs

**Implementation:**
```rust
// Rust code calls TypeScript CLI tool via process execution
use std::process::Command;

fn report_progress(current: usize, total: usize, message: &str) {
    Command::new("npx")
        .args(&["tsx", "../cli-progress-reporting/src/index.ts", "set"])
        .args(&["--current", &current.to_string()])
        .args(&["--total", &total.to_string()])
        .args(&["--message", message])
        .args(&["--id", "diff-progress"])
        .output()
        .ok(); // Graceful fallback if tool not available
}
```

**Example Output:**
```
$ odiff large-file1.json large-file2.json
[45%] 4.5MB/10MB - Comparing JSON structures (12s)
```

---

### 2. Cross-Platform Path Normalizer (TypeScript → Rust)

**Use Case:** Accept file paths in any format (Windows/Unix), normalize before processing

**Integration Points:**
- CLI file path arguments
- Relative vs absolute path handling
- Mixed separator cleanup (`C:/Users\Documents/file.txt`)

**Implementation:**
```rust
// Rust code calls path normalizer via CLI
fn normalize_path(input_path: &str) -> Result<String, String> {
    let output = Command::new("npx")
        .args(&["tsx", "../cross-platform-path-normalizer/src/index.ts"])
        .args(&["--absolute", input_path])
        .output()
        .ok();

    if let Some(out) = output {
        if out.status.success() {
            let json: serde_json::Value = serde_json::from_slice(&out.stdout).ok()?;
            return Ok(json["path"].as_str()?.to_string());
        }
    }

    // Graceful fallback: use input as-is
    Ok(input_path.to_string())
}
```

**Example:**
```
$ odiff "C:\data\file1.json" "/home/user/file2.json"
# Internally normalizes both paths to consistent format
```

---

### 3. File-Based Semaphore (Rust library)

**Use Case:** Protect concurrent access to diff cache

**Integration Points:**
- Diff result caching (avoid re-computing identical diffs)
- Concurrent processes diffing same files
- Atomic cache read/write operations

**Implementation:**
```rust
use file_based_semaphore::{Semaphore, SemaphoreConfig};

fn get_cached_diff(file1: &str, file2: &str) -> Option<DiffResult> {
    let cache_lock = Semaphore::new(
        "/tmp/diff-cache.lock",
        SemaphoreConfig::default()
    ).ok()?;

    let _guard = cache_lock.try_acquire().ok()?;
    // Critical section: read from cache
    read_from_cache(file1, file2)
}
```

**Example:**
```
$ odiff file1.json file2.json &
$ odiff file1.json file2.json &
# Second process waits for cache lock before reading
```

---

### 4. Test Flakiness Detector (TypeScript validates Rust)

**Use Case:** Validate test suite reliability

**Integration Points:**
- Run `cargo test` multiple times via flakiness detector
- Detect non-deterministic test behavior
- Ensure diff algorithm consistency

**Implementation:**
```bash
#!/bin/bash
# scripts/dogfood-flaky.sh

RUNS="${1:-10}"
cd ../test-flakiness-detector
flaky \
    --test "cd ../output-diffing-utility && cargo test" \
    --runs "$RUNS" \
    --verbose
```

**Example:**
```
$ cd output-diffing-utility
$ bash scripts/dogfood-flaky.sh 20
✅ NO FLAKINESS DETECTED (85 tests × 20 runs = 1,700 executions)
```

---

## Composability Demonstrations

### Multi-Tool Pipeline Example

```bash
# 1. Normalize paths (cross-platform-path-normalizer)
FILE1=$(npx tsx ../cross-platform-path-normalizer/src/index.ts --absolute "C:\data\v1.json" | jq -r .path)
FILE2=$(npx tsx ../cross-platform-path-normalizer/src/index.ts --absolute "/home/user/v2.json" | jq -r .path)

# 2. Initialize progress tracking (cli-progress-reporting)
npx tsx ../cli-progress-reporting/src/index.ts init --total 100 --id diff-task --message "Comparing files"

# 3. Run diff with progress updates (output-diffing-utility)
odiff "$FILE1" "$FILE2" --progress-id diff-task

# 4. Mark complete (cli-progress-reporting)
npx tsx ../cli-progress-reporting/src/index.ts finish --id diff-task --message "Diff complete"
```

### Cross-Language Composition

**Rust tool (output-diffing-utility) → TypeScript tools → Back to Rust**

```
User invokes Rust CLI
    ↓
Rust calls cross-platform-path-normalizer (TypeScript via npx)
    ↓
Normalized paths returned to Rust
    ↓
Rust performs diff, calling cli-progress-reporting (TypeScript) for updates
    ↓
Rust uses file-based-semaphore (Rust library) for cache locking
    ↓
Final diff output returned to user
```

---

## Implementation Checklist

- [x] ~~Integrate CLI Progress Reporting for large files (>1MB)~~ → `scripts/dogfood-progress.sh`
- [x] ~~Integrate Cross-Platform Path Normalizer for input paths~~ → `scripts/dogfood-paths.sh`
- [x] ~~Integrate File-Based Semaphore for cache locking~~ → `scripts/dogfood-semaphore.sh`
- [x] ~~Add `scripts/dogfood-flaky.sh` for Test Flakiness Detector validation~~ → `scripts/dogfood-flaky.sh`
- [x] ~~Document all integrations in README~~ → See "Composability with Tuulbelt Tools" section
- [x] ~~Add examples showing multi-tool composition~~ → `scripts/dogfood-pipeline.sh`
- [x] ~~Test graceful fallback when tools not available (standalone mode)~~ → All scripts check for tool availability

**Status:** ✅ COMPLETE - All dogfooding composition scripts implemented and documented.

---

## Expected Outcomes

1. **Demonstrates Composability Principle**
   - Tools work together via simple CLI interfaces
   - No tight coupling or shared dependencies
   - Works across programming languages

2. **Real-World Value**
   - Each integration solves an actual problem
   - Not forced or artificial composition
   - Users benefit from combined functionality

3. **Validates Tuulbelt Philosophy**
   - Zero dependencies (each tool standalone)
   - Portable interfaces (CLI, files, env vars)
   - Single problem per tool (no framework bloat)
   - Independently cloneable (graceful fallback)

---

**Status:** ✅ IMPLEMENTATION COMPLETE (2025-12-26)
**Composition Scripts:** 4 individual + 1 pipeline = 5 total scripts
**Documentation:** README, GitHub Pages, and DOGFOODING_STRATEGY.md all updated
**Next:** Use these patterns for future Tuulbelt tools
