# /add-tool-dependency Command

Add a Tuulbelt tool as a dependency of another tool.

## Usage

```
/add-tool-dependency <tool-name> <dependency-tool> [required|optional]
```

## Parameters

| Parameter | Required | Description | Example |
|-----------|----------|-------------|---------|
| tool-name | Yes | Tool that needs the dependency | `test-port-resolver` |
| dependency-tool | Yes | Tool to depend on | `file-based-semaphore-ts` |
| type | No | `required` (default) or `optional` | `required` |

## Examples

```bash
# Required dependency
/add-tool-dependency test-port-resolver file-based-semaphore-ts required

# Optional dependency
/add-tool-dependency test-flakiness-detector cli-progress-reporting optional

# Rust to Rust
/add-tool-dependency snapshot-comparison output-diffing-utility required
```

## What This Command Does

### For TypeScript Tools

#### 1. Add Git Dependency to package.json

```json
{
  "dependencies": {
    "@tuulbelt/file-based-semaphore-ts": "git+https://github.com/tuulbelt/file-based-semaphore-ts.git"
  }
}
```

For optional dependencies, users do dynamic imports in code:
```typescript
// Optional dependency pattern
async function loadOptionalDependency() {
  try {
    const dep = await import('@tuulbelt/dependency-tool');
    return dep;
  } catch {
    return null; // Tool works without it
  }
}
```

#### 2. Install Dependency
```bash
cd tools/{tool-name}
npm install
```

#### 3. Update README.md
Add "Library Composition" section:
```markdown
## Library Composition

This tool uses [File-Based Semaphore](/tools/file-based-semaphore-ts/) as a library dependency for process-level locking.

See [PRINCIPLES.md Exception 2](../../PRINCIPLES.md#2-zero-external-dependencies-standard-library-only) for details on Tuulbelt-to-Tuulbelt composition.
```

#### 4. Update VitePress docs/index.md
Add tip callout:
```markdown
::: tip <img src="/icons/package.svg" class="inline-icon" alt=""> Uses File-Based Semaphore Library
This tool uses [file-based-semaphore-ts](/tools/file-based-semaphore-ts/) as a library dependency for atomic port allocation.
:::
```

#### 5. Update DOGFOODING_STRATEGY.md
```markdown
## Compositions

### With file-based-semaphore-ts (Required)
- **Pattern:** Library integration via git dependency
- **Purpose:** Atomic port registry operations
- **Test:** npm install verifies dependency resolution
- **Standalone:** Works when installed via npm (auto-fetches from GitHub)
```

### For Rust Tools

#### 1. Add Git Dependency to Cargo.toml

```toml
[dependencies]
output_diffing_utility = { git = "https://github.com/tuulbelt/output-diffing-utility" }
```

#### 2. Build to Verify
```bash
cd tools/{tool-name}
cargo build
```

#### 3. Update README.md
Same as TypeScript (adjust paths if needed)

#### 4. Update VitePress docs
Same as TypeScript

#### 5. Update DOGFOODING_STRATEGY.md
Same as TypeScript

### Both Languages

#### 6. Update PRINCIPLES.md Reference (if first composition)

If this is the tool's first library dependency, add note:
```markdown
> **Note:** This tool uses [dependency-tool] as a library dependency (PRINCIPLES.md Exception 2).
> Since all Tuulbelt tools have zero external dependencies, this preserves the zero-dep guarantee.
```

#### 7. Add docs/index.md Card Badge

Add library badge to tool card:
```markdown
<div class="tool-card">
  <h3>Tool Name / <code>short</code></h3>
  <span class="library-badge">
    <img src="/icons/package.svg" class="inline-icon" alt="Library Composition">
    Uses <a href="/tools/dependency-tool/">dependency-tool</a>
  </span>
  ...
</div>
```

#### 8. Verify Build and Tests
```bash
# TypeScript
cd tools/{tool-name}
npm install
npm test
npm run build

# Rust
cd tools/{tool-name}
cargo test
cargo build
```

## Dependency Types

### Required Dependencies

**When to use:**
- Tool cannot function without dependency
- Core functionality depends on it
- No graceful fallback possible

**Examples:**
- `snapshot-comparison` REQUIRES `output-diffing-utility` for semantic diffs
- `test-port-resolver` REQUIRES `file-based-semaphore-ts` for atomic operations

**Implementation:**
```typescript
// TypeScript - direct import
import { Semaphore } from '@tuulbelt/file-based-semaphore-ts';
const sem = new Semaphore('/tmp/lock');
```

```rust
// Rust - direct use
use output_diffing_utility::{diff_text, diff_json};
```

### Optional Dependencies

**When to use:**
- Tool has enhanced functionality with dependency
- Core functionality works without it
- Graceful fallback exists

**Examples:**
- `test-flakiness-detector` optionally uses `cli-progress-reporting` for better UX
- Fallback: Simple console.log progress

**Implementation:**
```typescript
// TypeScript - dynamic import with fallback
async function getProgressReporter() {
  try {
    const prog = await import('@tuulbelt/cli-progress-reporting');
    return prog.createProgress();
  } catch {
    // Fallback: console logging
    return {
      update: (msg: string) => console.log(msg),
      complete: () => console.log('Done')
    };
  }
}
```

**Rust:** Optional dependencies use `cfg` feature flags or runtime checks (less common in Tuulbelt).

## Quality Gates

Before adding dependency, verify:

- [ ] Dependency tool exists in `tools/` or GitHub
- [ ] Dependency tool has passing tests
- [ ] Dependency tool has zero runtime dependencies
- [ ] Circular dependency check (A→B, B→A not allowed)
- [ ] Build succeeds after adding dependency
- [ ] Tests pass after adding dependency

If any check fails, dependency addition is aborted.

## Post-Addition Tasks

After dependency is added:

1. **Update Tool Implementation**
   ```typescript
   // Import and use the dependency
   import { SomeThing } from '@tuulbelt/dependency-tool';
   ```

2. **Add Tests**
   ```typescript
   test('uses dependency correctly', () => {
     // Test integration
   });
   ```

3. **Update Examples**
   Add example showing library integration

4. **Verify Standalone Works**
   ```bash
   # Clone tool independently
   git clone https://github.com/tuulbelt/{tool-name}.git
   cd {tool-name}
   npm install  # Should auto-fetch dependency from GitHub
   npm test
   ```

## Troubleshooting

### "Dependency not found"
```
ERROR: Tool 'unknown-tool' not found in tools/ or GitHub.

Fix:
- Check tool name spelling
- Verify tool exists: ls tools/
- Or create tool first: /new-tool unknown-tool typescript
```

### "Circular dependency detected"
```
ERROR: Adding this dependency creates a circular dependency:
  tool-a → tool-b → tool-a

Circular dependencies are not allowed.

Fix:
- Refactor to break the cycle
- Extract common logic to a third tool
- Reconsider architecture
```

### "Build fails after adding dependency"
```
ERROR: Build failed after adding dependency.

Common causes:
- Dependency not installed: npm install
- Import path wrong: check @tuulbelt/ prefix
- Type errors: verify dependency exports match usage

Fix:
1. Check error messages
2. Verify dependency API in its README
3. Fix imports and types
4. Retry build
```

### "Tests fail after adding dependency"
```
ERROR: Tests fail with dependency added.

Fix:
1. Run tests: npm test (or cargo test)
2. Check if dependency behavior differs from expected
3. Update test mocks if needed
4. Verify dependency version is stable (v0.1.0+)
```

## Dependency Graph Visualization

After adding dependencies, you can visualize the graph:

```bash
# Future: /visualize-dependencies command
# For now, manual check:
grep -r "dependencies" tools/*/package.json
grep -r "dependencies" tools/*/Cargo.toml
```

Current dependency graph:
```
test-port-resolver → file-based-semaphore-ts (required)
snapshot-comparison → output-diffing-utility (required)
test-flakiness-detector → cli-progress-reporting (optional)
```

## Related Commands

- `/new-tool` - Create new tool
- `/release-tool` - Release new version
- `/update-all-counts` - Update dependency references

## References

- PRINCIPLES.md Exception 2 (Tool Composition)
- Git Dependencies: https://docs.npmjs.com/cli/v10/configuring-npm/package-json#git-urls-as-dependencies
- Cargo Git Dependencies: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories
