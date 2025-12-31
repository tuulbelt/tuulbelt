# Demo Recording Plan

**Purpose:** Standardize demo recordings across all Tuulbelt tools with consistent structure, comprehensive examples, and proper link management.

**Status:** ✅ APPROVED - WORKFLOWS RUNNING

**Progress:**
- ✅ Phase 1 Complete: All demos reset (placeholders created, links reset)
- ✅ Phase 2 Complete: All 10 demo scripts updated with 6-step comprehensive structure
- ✅ Phase 3 Complete: All 10 workflows manually triggered
- ✅ Phase 4A Complete: Fixed README links for 3 completed demos (flaky, prog, odiff)
- ✅ Phase 4B Complete: Added README link update step to all 10 workflows
- ✅ Phase 4C Complete: Fixed YAML syntax errors in all 10 workflows
- ✅ Phase 4D Complete: Re-triggered all 7 remaining workflows
- 🔄 Phase 5 In Progress: Workflows generating demos with correct links
- ⏳ Final: Verify all demos and asciinema URLs work

**What Was Fixed:**
- **README Links:** 3 tools (flaky, prog, odiff) manually updated with asciinema URLs
- **Workflow Enhancement:** All 10 workflows now auto-update README with asciinema URL after recording
- **YAML Syntax:** Fixed `tr -d '\n')` newline escaping issue in all 10 workflows
- **Re-triggered:** 7 tools with placeholder demos now regenerating with new scripts

**Workflow Monitoring:**
All workflows re-triggered with fixes. Monitor progress at:
- https://github.com/tuulbelt/test-flakiness-detector/actions
- https://github.com/tuulbelt/cli-progress-reporting/actions
- https://github.com/tuulbelt/cross-platform-path-normalizer/actions
- https://github.com/tuulbelt/config-file-merger/actions
- https://github.com/tuulbelt/structured-error-handler/actions
- https://github.com/tuulbelt/file-based-semaphore/actions
- https://github.com/tuulbelt/output-diffing-utility/actions
- https://github.com/tuulbelt/snapshot-comparison/actions
- https://github.com/tuulbelt/file-based-semaphore-ts/actions
- https://github.com/tuulbelt/port-resolver/actions

---

## 🎯 Goals

1. **Replace all existing demos** - Delete current demo.gif and demo-url.txt to trigger regeneration
2. **Standardize demo structure** - Use consistent template for all tools
3. **Show complete workflows** - From installation to real-world usage
4. **Fix link issues** - Ensure asciinema URLs work correctly
5. **Use short names only** - Always use `flaky`, `prog`, `odiff`, etc. (never long names)

---

## 📋 Demo Recording Template

### Standard Structure (All Tools)

Every demo recording should follow this structure:

```bash
# ═══════════════════════════════════════════
# Tool Name / shortname - Tuulbelt
# ═══════════════════════════════════════════

# 1. INSTALLATION (5-10 seconds)
echo "# Step 1: Install the tool globally"
npm link                    # TypeScript tools
# OR
cargo install --path .      # Rust tools

# 2. HELP / OVERVIEW (5 seconds)
echo ""
echo "# Step 2: View available commands"
shortname --help

# 3. BASIC USAGE (10-15 seconds)
echo ""
echo "# Step 3: Basic usage example"
shortname [basic-command] [simple-args]

# 4. INTERMEDIATE USAGE (15-20 seconds)
echo ""
echo "# Step 4: Common workflow"
shortname [workflow-commands-with-realistic-data]

# 5. ADVANCED FEATURES (10-15 seconds)
echo ""
echo "# Step 5: Advanced features"
shortname [advanced-options-or-composition]

# 6. ERROR HANDLING (5-10 seconds)
echo ""
echo "# Step 6: Error handling"
shortname [invalid-input-or-edge-case]

# Total duration: ~60-75 seconds
```

### GIF Parameters

**Consistent across all tools:**
```bash
GIF_COLS=100        # Wide enough for long command lines
GIF_ROWS=30         # Tall enough for full help output
GIF_SPEED=1.0       # Normal speed (no acceleration)
GIF_FONT_SIZE=14    # Readable on GitHub
GIF_THEME=monokai   # Consistent dark theme
```

---

## 🔧 Tool-Specific Demo Scripts

### 1. Test Flakiness Detector (`flaky`)

**Purpose:** Detect non-deterministic tests

**Demo Content:**
```bash
# ═══════════════════════════════════════════
# Test Flakiness Detector / flaky - Tuulbelt
# ═══════════════════════════════════════════

# Step 1: Install globally
npm link

# Step 2: View help
flaky --help

# Step 3: Basic flakiness detection (small runs)
echo "# Detect flaky tests (5 runs)"
flaky --test "npm test" --runs 5

# Step 4: Detailed analysis (with progress)
echo "# Run with progress tracking (10 runs)"
flaky --test "npm test" --runs 10 --verbose

# Step 5: JSON output for CI integration
echo "# JSON output for automation"
flaky --test "npm test" --runs 5 --format json

# Step 6: Error handling
echo "# Handle invalid test command"
flaky --test "invalid-command" --runs 3
```

**Key Points:**
- Show progress tracking feature (cli-progress-reporting integration)
- Demonstrate JSON output for CI
- Show what happens with invalid commands

---

### 2. CLI Progress Reporting (`prog`)

**Purpose:** Concurrent-safe progress updates

**Demo Content:**
```bash
# ═══════════════════════════════════════════
# CLI Progress Reporting / prog - Tuulbelt
# ═══════════════════════════════════════════

# Step 1: Install globally
npm link

# Step 2: View help
prog --help

# Step 3: Initialize progress tracker
prog init --total 100 --message "Processing files"

# Step 4: Increment progress
prog increment --amount 10
prog increment --amount 20
prog increment --amount 30

# Step 5: Check current state
prog get

# Step 6: Complete and finish
prog finish --message "All files processed!"

# Step 7: Multiple trackers with IDs
prog init --total 50 --id build --message "Building project"
prog increment --id build --amount 25
prog get --id build
```

**Key Points:**
- Show init → increment → get → finish workflow
- Demonstrate multiple progress trackers with IDs
- Show real-time state queries

---

### 3. Cross-Platform Path Normalizer (`normpath`)

**Purpose:** Windows/Unix path consistency

**Demo Content:**
```bash
# ═══════════════════════════════════════════
# Cross-Platform Path Normalizer / normpath - Tuulbelt
# ═══════════════════════════════════════════

# Step 1: Install globally
npm link

# Step 2: View help
normpath --help

# Step 3: Auto-detect format
normpath "C:\Users\Documents\file.txt"
normpath "/home/user/documents/file.txt"

# Step 4: Force Unix format
normpath --format unix "C:\Program Files\App\config.ini"

# Step 5: Force Windows format
normpath --format windows "/usr/local/bin/app"

# Step 6: Absolute path resolution
normpath --absolute "./relative/path.txt"

# Step 7: Edge cases
normpath ""
normpath "///multiple///slashes///path"
```

**Key Points:**
- Show auto-detection for both Windows and Unix paths
- Demonstrate format forcing
- Show absolute path resolution
- Handle edge cases (empty, multiple slashes)

---

### 4. Configuration File Merger (`cfgmerge`)

**Purpose:** Merge ENV + config + CLI args

**Demo Content:**
```bash
# ═══════════════════════════════════════════
# Configuration File Merger / cfgmerge - Tuulbelt
# ═══════════════════════════════════════════

# Step 1: Install globally
npm link

# Step 2: View help
cfgmerge --help

# Step 3: Create sample files
echo '{"port": 3000, "host": "localhost"}' > config.json
echo '{"debug": false, "timeout": 30}' > defaults.json

# Step 4: Merge config file with defaults
cfgmerge --defaults defaults.json --file config.json

# Step 5: Override with environment variables
export APP_PORT=8080
export APP_DEBUG=true
cfgmerge --env --prefix APP_ --file config.json

# Step 6: CLI args take highest precedence
cfgmerge --file config.json --args "port=9000,debug=true"

# Step 7: Track sources
cfgmerge --file config.json --args "port=9000" --track-sources

# Cleanup
rm config.json defaults.json
unset APP_PORT APP_DEBUG
```

**Key Points:**
- Show precedence order (CLI > env > file > defaults)
- Demonstrate source tracking
- Show environment variable prefix filtering

---

### 5. Structured Error Handler (`serr`)

**Purpose:** Error format with context preservation

**Demo Content:**
```bash
# ═══════════════════════════════════════════
# Structured Error Handler / serr - Tuulbelt
# ═══════════════════════════════════════════

# Step 1: Install globally
npm link

# Step 2: View help
serr --help

# Step 3: View demo
serr demo --format text

# Step 4: Parse JSON error
serr parse '{"message":"File not found","context":[{"key":"path","value":"/tmp/missing.txt"}]}'

# Step 5: Parse with stack traces
serr parse '{"message":"Division by zero","context":[{"key":"line","value":"42"}]}' --stack

# Step 6: Validate error format
serr validate '{"message":"Valid error"}'
serr validate '{"invalid":"structure"}'

# Step 7: JSON output format
serr demo --format json
```

**Key Points:**
- Show demo mode for quick understanding
- Demonstrate parsing and validation
- Show both text and JSON output formats

---

### 6. File-Based Semaphore (Rust) (`sema`)

**Purpose:** Cross-platform process locking

**Demo Content:**
```bash
# ═══════════════════════════════════════════
# File-Based Semaphore / sema - Tuulbelt
# ═══════════════════════════════════════════

# Step 1: Install globally
cargo install --path .

# Step 2: View help
sema --help

# Step 3: Try to acquire lock (non-blocking)
sema try /tmp/demo.lock --tag "demo process"
echo "Lock acquired!"

# Step 4: Check lock status
sema status /tmp/demo.lock

# Step 5: Try to acquire again (should fail)
sema try /tmp/demo.lock --tag "second process"

# Step 6: Release lock
sema release /tmp/demo.lock

# Step 7: Acquire with timeout
sema acquire /tmp/demo.lock --timeout 5 --tag "timed lock"
sema status /tmp/demo.lock --json
sema release /tmp/demo.lock

# Step 8: Wait for lock
sema acquire /tmp/demo.lock &
sleep 2
sema wait /tmp/demo.lock --timeout 10
```

**Key Points:**
- Show try (non-blocking) vs acquire (blocking) vs wait
- Demonstrate status checking (both text and JSON)
- Show concurrent access scenarios
- Display tag information in status

---

### 7. Output Diffing Utility (Rust) (`odiff`)

**Purpose:** Semantic diff for JSON, text, binary

**Demo Content:**
```bash
# ═══════════════════════════════════════════
# Output Diffing Utility / odiff - Tuulbelt
# ═══════════════════════════════════════════

# Step 1: Install globally
cargo install --path .

# Step 2: View help
odiff --help

# Step 3: Create sample files
echo "line 1\nline 2\nline 3" > file1.txt
echo "line 1\nline X\nline 3" > file2.txt

# Step 4: Basic text diff
odiff file1.txt file2.txt

# Step 5: JSON semantic diff
echo '{"name":"Alice","age":30}' > data1.json
echo '{"name":"Alice","age":31,"city":"NYC"}' > data2.json
odiff data1.json data2.json

# Step 6: Different output formats
odiff --format json data1.json data2.json
odiff --format compact file1.txt file2.txt
odiff --format side-by-side file1.txt file2.txt

# Step 7: Force type and context lines
odiff --type text --context 5 data1.json data2.json

# Step 8: Quiet mode (exit code only)
odiff --quiet file1.txt file2.txt
echo "Exit code: $?"

# Cleanup
rm file1.txt file2.txt data1.json data2.json
```

**Key Points:**
- Show text, JSON, and different output formats
- Demonstrate semantic understanding (JSON field paths)
- Show exit codes (0=identical, 1=differ)
- Display all format options (unified, json, side-by-side, compact)

**NOTE:** Current demo is only 2 seconds - this will fix it with comprehensive examples.

---

### 8. Snapshot Comparison (Rust) (`snapcmp`)

**Purpose:** Snapshot testing with diffs

**Demo Content:**
```bash
# ═══════════════════════════════════════════
# Snapshot Comparison / snapcmp - Tuulbelt
# ═══════════════════════════════════════════

# Step 1: Install globally
cargo install --path .

# Step 2: View help
snapcmp --help

# Step 3: Create a snapshot
echo "Expected output v1.0" | snapcmp create test-output

# Step 4: Check against snapshot (should match)
echo "Expected output v1.0" | snapcmp check test-output
echo "Result: $?"

# Step 5: Check with different output (mismatch)
echo "Expected output v2.0" | snapcmp check test-output || true

# Step 6: Update snapshot
echo "Expected output v2.0" | snapcmp update test-output

# Step 7: List all snapshots
snapcmp list

# Step 8: Create JSON snapshot
echo '{"version":"1.0","status":"ok"}' | snapcmp create api-response --type json

# Step 9: Check JSON snapshot
echo '{"version":"1.0","status":"ok"}' | snapcmp check api-response

# Step 10: Clean orphaned snapshots
snapcmp clean --keep test-output,api-response --dry-run

# Cleanup
snapcmp delete test-output
snapcmp delete api-response
```

**Key Points:**
- Show create → check → update workflow
- Demonstrate diff output on mismatch (uses odiff)
- Show both text and JSON snapshots
- Display snapshot management (list, clean, delete)

---

### 9. File-Based Semaphore (TypeScript) (`semats`)

**Purpose:** Cross-platform process locking (TS version)

**Demo Content:**
```bash
# ═══════════════════════════════════════════
# File-Based Semaphore (TS) / semats - Tuulbelt
# ═══════════════════════════════════════════

# Step 1: Install globally
npm link

# Step 2: View help
semats --help

# Step 3: Acquire a lock
semats acquire /tmp/demo.lock --tag "demo process"

# Step 4: Check status
semats status /tmp/demo.lock

# Step 5: Try to acquire (non-blocking, should fail)
semats try-acquire /tmp/demo.lock --tag "second process"

# Step 6: Release lock
semats release /tmp/demo.lock

# Step 7: Status with JSON output
semats acquire /tmp/demo.lock --tag "json demo"
semats status /tmp/demo.lock --json
semats release /tmp/demo.lock

# Step 8: Clean stale locks
semats clean /tmp/demo.lock
```

**Key Points:**
- Show acquire → status → release workflow
- Demonstrate try-acquire (non-blocking)
- Show JSON output format
- Display stale lock cleaning

---

### 10. Test Port Resolver (`portres`)

**Purpose:** Concurrent test port allocation

**Demo Content:**
```bash
# ═══════════════════════════════════════════
# Test Port Resolver / portres - Tuulbelt
# ═══════════════════════════════════════════

# Step 1: Install globally
npm link

# Step 2: View help
portres --help

# Step 3: Get a random available port
portres get

# Step 4: Get port with specific tag
portres get --tag "api-server"

# Step 5: Get port within range
portres get --min 8000 --max 9000 --tag "web-server"

# Step 6: Reserve specific port
portres reserve --port 3000 --tag "main-app"

# Step 7: Check port status
portres status --port 3000

# Step 8: Release port
portres release --port 3000

# Step 9: List all allocated ports
portres list

# Step 10: Cleanup all ports
portres cleanup
```

**Key Points:**
- Show dynamic port allocation
- Demonstrate port ranges and tagging
- Show status checking and listing
- Display reservation and cleanup

---

## 🔗 Link Placement Strategy

### Where Links Appear

**In README.md:**
```markdown
## Demo

See the tool in action:

![Tool Name Demo](/path/to/demo.gif)

**[▶ View interactive recording on asciinema.org](https://asciinema.org/a/XXXXXX)**

<div style="margin: 20px 0;">
  <span style="display: inline-block; vertical-align: middle; margin-right: 8px;">
    <strong>Try it online:</strong>
  </span>
  <a href="https://stackblitz.com/github/tuulbelt/tool-name" style="display: inline-block; vertical-align: middle;">
    <img src="https://developer.stackblitz.com/img/open_in_stackblitz.svg" alt="Open in StackBlitz" style="vertical-align: middle;">
  </a>
</div>
```

**In VitePress (docs/tools/tool-name/index.md):**
```markdown
## Demo

See the tool in action:

![Tool Name Demo](/tool-name/demo.gif)

**[▶ View interactive recording on asciinema.org](https://asciinema.org/a/XXXXXX)**

<div style="margin: 20px 0;">
  <span style="display: inline-block; vertical-align: middle; margin-right: 8px;">
    <strong>Try it online:</strong>
  </span>
  <a href="https://stackblitz.com/github/tuulbelt/tuulbelt/tree/main/tool-name" style="display: inline-block; vertical-align: middle;">
    <img src="https://developer.stackblitz.com/img/open_in_stackblitz.svg" alt="Open in StackBlitz" style="vertical-align: middle;">
  </a>
</div>
```

### How Links Are Updated

**Workflow logic:**
1. **Record demo** → creates `demo.cast`
2. **Upload to asciinema.org** → saves URL to `demo-url.txt`
3. **Update README.md** → replaces `href="#"` with actual URL from `demo-url.txt`
4. **Convert to GIF** → creates `docs/demo.gif`
5. **Commit changes** → demo.cast, docs/demo.gif, demo-url.txt, README.md

**sed command to update README:**
```bash
ASCIINEMA_URL=$(cat demo-url.txt)
sed -i '' "s|href=\"#\"|href=\"$ASCIINEMA_URL\"|g" README.md
```

### Placeholder Detection Logic

**Current workflow checks:**
1. **Missing demo.gif** → `[ ! -f "docs/demo.gif" ]`
2. **Placeholder demo.gif** → File size ≤ 100 bytes
3. **Missing demo-url.txt** → `[ ! -f "demo-url.txt" ] || [ ! -s "demo-url.txt" ]`
4. **Placeholder link in README** → `grep -q 'href="#"' README.md`

---

## 🚀 Implementation Plan

### Phase 1: Reset All Demos (Clear Placeholders)

**For each tool repository:**
```bash
# Delete existing demo files
rm -f demo.cast demo-url.txt docs/demo.gif

# Create placeholder demo.gif (43 bytes, 1x1 transparent GIF)
mkdir -p docs
echo "R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7" | base64 -d > docs/demo.gif

# Reset README link to placeholder
sed -i '' 's|href="https://asciinema.org/a/[^"]*"|href="#"|g' README.md

# Commit
git add demo.cast demo-url.txt docs/demo.gif README.md
git commit -m "chore: reset demo for regeneration"
git push
```

### Phase 2: Update Demo Scripts

**For each tool:**
1. Read current `scripts/record-demo.sh`
2. Replace `demo_commands()` function with new comprehensive script (from above)
3. Set appropriate `GIF_COLS`, `GIF_ROWS`, `GIF_SPEED` parameters
4. Test locally: `bash scripts/record-demo.sh`
5. Commit and push

### Phase 3: Trigger Demo Generation

**After scripts are updated:**
- Workflows auto-trigger on push (detect placeholder)
- OR manually trigger: `bash scripts/trigger-all-demos.sh`

### Phase 4: Verify Results

**Check each repository:**
- [ ] demo.gif exists and is > 1KB (not placeholder)
- [ ] demo-url.txt contains valid asciinema.org URL
- [ ] README.md has real URL (not `href="#"`)
- [ ] asciinema.org link opens and plays recording
- [ ] Recording is 60-75 seconds long (not 2 seconds)
- [ ] Recording shows complete workflow (install → help → examples)

---

## ✅ Success Criteria

- [ ] All 10 tools have comprehensive demo recordings (60-75 seconds each)
- [ ] All demos follow the same structure (install → help → basic → intermediate → advanced → errors)
- [ ] All demos use short names only (`flaky`, not `test-flakiness-detector`)
- [ ] All demos show `npm link` or `cargo install --path .` as Step 1
- [ ] All asciinema.org links work and open the correct recording
- [ ] All demo.gif files are properly generated (not placeholders)
- [ ] Consistent GIF parameters across all tools
- [ ] No "2 second" recordings (comprehensive examples)

---

## 📝 Next Steps

**AWAITING APPROVAL:**

1. Review this plan
2. Verify demo content for each tool makes sense
3. Approve template structure
4. Approve link placement strategy
5. Approve implementation phases

**Once approved, I will:**

1. Create script to reset all demos (Phase 1)
2. Update all 10 demo recording scripts (Phase 2)
3. Trigger regeneration (Phase 3)
4. Verify all results (Phase 4)
5. Update templates for future tools

---

**Created:** 2025-12-30
**Status:** AWAITING APPROVAL
**Estimated Time:** 2-3 hours for full implementation
