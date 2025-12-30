# Dogfooding Strategy: {{tool-name}}

This document outlines how this Rust tool uses other Tuulbelt tools for validation.

## How Dogfooding Works for Rust Tools

Rust tools can't directly add npm devDependencies, so we use a minimal `package.json` for dogfooding:

```json
{
  "private": true,
  "scripts": {
    "dogfood": "flaky --test 'cargo test' --runs 20"
  },
  "devDependencies": {
    "@tuulbelt/test-flakiness-detector": "git+https://github.com/tuulbelt/test-flakiness-detector.git"
  }
}
```

This runs in CI automatically after installing Node.js dependencies.

---

## Step 1: Answer These Questions

Before customizing dogfooding, answer these questions about your tool:

### Q1: Does your tool have tests? (Almost always YES)
- **YES** → Use test-flakiness-detector (validates test determinism)
- **NO** → Add tests first!

### Q2: Does your tool produce deterministic output?
- **YES** (same input → same output every time) → Consider output-diffing-utility
- **NO** (output varies by design, e.g., port allocation, timestamps) → Skip diff validation

### Q3: Does your tool use file-based locking or semaphores?
- **YES** (uses `file-based-semaphore`) → Test concurrent scenarios
- **NO** → Skip semaphore validation

### Q4: Is this tool used as a library by other Tuulbelt tools?
- **YES** → Document the library composition relationship
- **NO** → Skip library documentation

---

## Step 2: Default Setup (All Rust Tools)

**Every Rust tool includes test-flakiness-detector via package.json:**

The template includes a `package.json` with:
- test-flakiness-detector as devDependency
- `npm run dogfood` command that runs `cargo test` 20 times

This validates that all tests pass consistently, catching:
- Race conditions in concurrent tests
- Shared state between tests
- Non-deterministic logic
- Timing-dependent failures

**CI workflow includes:**

```yaml
- name: Setup Node.js (for dogfooding)
  uses: actions/setup-node@v4

- name: Install dogfood dependencies
  run: npm install

- name: Dogfood (flakiness detection)
  run: npm run dogfood
```

---

## Step 3: Additional Compositions (Based on Answers)

### If Q2 = YES: Output Consistency Validation

For Rust tools with deterministic output, you can add a test that:
1. Runs the tool with known inputs
2. Compares output against expected baseline
3. Uses output-diffing-utility for rich diff output

Example integration test:

```rust
#[test]
fn test_output_consistency() {
    let output = run_tool_with_input("test.json");
    let expected = include_str!("fixtures/expected_output.txt");
    assert_eq!(output, expected);
}
```

### If Q3 = YES: Concurrency Validation

For tools using file-based-semaphore, add concurrent stress tests:

```rust
#[test]
fn test_concurrent_access() {
    let handles: Vec<_> = (0..10)
        .map(|_| std::thread::spawn(|| acquire_lock_and_work()))
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
```

### If Q4 = YES: Document Library Usage

If your Rust tool is used by other tools, document in README:

```markdown
## Library Usage

Other Tuulbelt tools can use this as a dependency:

\`\`\`toml
[dependencies]
{{tool-name}} = { git = "https://github.com/tuulbelt/{{tool-name}}" }
\`\`\`
```

---

## CI Integration

The `test.yml` workflow automatically:
1. Runs cargo test
2. Installs Node.js
3. Runs `npm install` (fetches test-flakiness-detector)
4. Runs `npm run dogfood` (validates test determinism)

---

## Available Tuulbelt Tools for Composition

| Tool | Language | Use For |
|------|----------|---------|
| test-flakiness-detector | TypeScript | Validating test determinism (via npm) |
| output-diffing-utility | Rust | Semantic diffs (library dep) |
| file-based-semaphore | Rust | Cross-process locking (library dep) |
| snapshot-comparison | Rust | Snapshot testing (library dep) |

---

## Anti-Patterns (What NOT to Do)

1. **Don't validate non-deterministic output** (will always fail)
2. **Don't dogfood just to dogfood** - each integration must provide real value
3. **Don't use a tool to validate itself** (circular validation)

---

## Checklist

- [ ] Answer all 4 questions above
- [ ] Verify `npm run dogfood` passes locally (requires Node.js)
- [ ] CI runs dogfood automatically
- [ ] Update README Dogfooding section

---

**Status:** Template - answer questions and customize for your tool
