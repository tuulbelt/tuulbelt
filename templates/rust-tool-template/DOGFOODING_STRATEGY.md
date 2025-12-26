# Dogfooding Strategy

**Status:** Planning
**Last Updated:** {{DATE}}

This document outlines how this tool can use other Tuulbelt tools and be validated by them.

## Purpose

Dogfooding creates:
- Real-world validation of tools
- Integration examples
- Quality assurance network
- Practical use cases

## Tools This Tool Can Use

### Tool 1: [Tuulbelt Tool Name]

**How to integrate:**
```rust
// Monorepo context only (graceful fallback)
async fn load_optional_tool() -> Option<Module> {
    let tool_path = join(cwd(), "..", "tool-name", "src", "index.ts");
    if !exists(tool_path) {
        return None; // Not in monorepo
    }

    match import(&tool_path).await {
        Ok(module) => Some(module),
        Err(_) => None // Graceful fallback
    }
}
```

**Use case:**
- When: During X operation
- Why: Provides Y benefit
- Fallback: Works without it

**Example:**
```bash
# In monorepo
cargo run  # Uses tool-name automatically

# Standalone
cargo run  # Works fine without tool-name
```

### Tool 2: [Tuulbelt Tool Name]

**How to integrate:**
```rust
// CLI-based integration (cross-language)
fn use_tool() -> Option<String> {
    let binary = "../tool-name/target/release/tool-name";
    if !exists(binary) {
        return None;
    }

    Command::new(binary)
        .args(&["--option", "value"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
}
```

**Use case:**
- When: During validation
- Why: Cross-checks results
- Fallback: Skip validation

## Tools That Can Validate This Tool

### Test Flakiness Detector

**Validation Type:** Test suite reliability

**How to use:**
```bash
# In monorepo
cd ../test-flakiness-detector
npx tsx src/index.ts \
    --test "cd {{TOOL_DIR}} && cargo test 2>&1" \
    --runs 20

# Standalone
# (Tool works independently; flakiness check optional)
```

**Add script:**
```bash
# scripts/dogfood.sh
#!/bin/bash
DETECTOR_DIR="../test-flakiness-detector"

if [ ! -d "$DETECTOR_DIR" ]; then
    echo "Not in monorepo, skipping dogfooding"
    exit 0
fi

cd "$DETECTOR_DIR"
npx tsx src/index.ts \
    --test "cd '{{TOOL_DIR}}' && cargo test 2>&1" \
    --runs 10
```

**Expected result:**
```
✅ NO FLAKINESS DETECTED
85 tests × 10 runs = 850 executions
0 failures detected
```

### CLI Progress Reporting

**Validation Type:** Progress tracking (if applicable)

**How to use:**
```rust
// Only in monorepo context
if let Some(progress) = load_cli_progress_reporting().await {
    progress.init("Processing files", total_files);

    for file in files {
        process_file(file)?;
        progress.update();
    }

    progress.finish("Complete");
}
```

**Fallback:** Works without progress tracking

### Output Diffing Utility

**Validation Type:** Output comparison (if applicable)

**How to use:**
```bash
# Validate expected output
{{TOOL_NAME}} input.txt > actual.txt

# Compare with expected (if output-diffing-utility available)
if [ -x ../output-diffing-utility/target/release/output-diff ]; then
    ../output-diffing-utility/target/release/output-diff \
        expected.txt actual.txt
fi
```

## Dogfooding Implementation Checklist

- [ ] Identify which tools can enhance this tool
- [ ] Implement graceful fallback (works standalone)
- [ ] Add monorepo detection logic
- [ ] Create `scripts/dogfood.sh` validation script
- [ ] Document in README that tool uses others in monorepo
- [ ] Add test that validates standalone mode works
- [ ] Add test that validates monorepo mode works (if applicable)

## Standalone vs Monorepo Behavior

### Standalone Mode

```bash
git clone https://github.com/tuulbelt/{{TOOL_NAME}}.git
cd {{TOOL_NAME}}
cargo build --release
./target/release/{{TOOL_NAME}}
```

**Behavior:** All core functionality works
**Missing:** Optional enhancements from other tools
**Result:** ✅ Fully functional

### Monorepo Mode

```bash
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt/{{TOOL_NAME}}
cargo build --release
./target/release/{{TOOL_NAME}}
```

**Behavior:** Core + optional enhancements
**Enhanced:** Progress tracking, validation, etc.
**Result:** ✅ Fully functional + extra features

## Testing Dogfooding

### Test 1: Standalone Works

```rust
#[test]
fn test_standalone_mode() {
    // Simulate not being in monorepo
    let result = run_tool_without_optional_deps();
    assert!(result.is_ok());
}
```

### Test 2: Monorepo Integration Works

```rust
#[test]
fn test_monorepo_mode() {
    // Only runs if optional tools available
    if let Some(tool) = load_optional_tool() {
        let result = run_tool_with_optional_deps(tool);
        assert!(result.is_ok());
    }
}
```

### Test 3: Graceful Fallback

```rust
#[test]
fn test_graceful_fallback() {
    // Verify tool works even if import fails
    let result = run_tool();
    assert!(result.is_ok());
    // Should not panic even if optional deps missing
}
```

## Documentation Requirements

**README.md must include:**

```markdown
## Dogfooding

This tool integrates with other Tuulbelt tools when in monorepo context:

- **Uses:** tool-name for X functionality (optional)
- **Validated by:** test-flakiness-detector for reliability testing

### Monorepo Integration

When cloned as part of the full Tuulbelt monorepo, this tool automatically:
- Uses cli-progress-reporting for progress tracking
- Can be validated by test-flakiness-detector

### Standalone Mode

Works independently without any optional integrations.

```bash
git clone https://github.com/tuulbelt/{{TOOL_NAME}}.git
# All features work standalone
```

## Benefits Achieved

- ✅ Real-world testing of integration patterns
- ✅ Proof that tools compose well
- ✅ Practical examples for users
- ✅ Quality assurance network
- ✅ No breaking standalone mode

---

*Delete this template before first release. Keep actual dogfooding documentation.*
