# Dogfooding Strategy: [Tool Name]

This document outlines how this tool leverages other Tuulbelt tools to demonstrate composability.

## High-Value Compositions

### 1. Test Flakiness Detector - Validate test reliability

**Why:** [Explain why test reliability matters for this tool]

**Script:** `scripts/dogfood-flaky.sh`

```bash
./scripts/dogfood-flaky.sh [runs]
# Validates all tests are deterministic
```

### 2. Output Diffing Utility - Prove deterministic outputs

**Why:** [Explain why output consistency matters]

**Script:** `scripts/dogfood-diff.sh`

```bash
./scripts/dogfood-diff.sh
# Compares outputs between runs
```

### 3. [Other Relevant Tool] - [Purpose]

**Why:** [Explain the value this composition provides]

**Script:** `scripts/dogfood-[name].sh`

```bash
./scripts/dogfood-[name].sh
# Description
```

## Implementation Checklist

- [ ] Identify high-value compositions (focus on REAL utility, not checkboxes)
- [ ] Create composition scripts (`scripts/dogfood-*.sh`)
- [ ] Update README with dogfooding section
- [ ] Update GH Pages docs (if applicable)
- [ ] Test graceful fallback when tools not available
- [ ] Document in this file

## Expected Outcomes

1. **Proves Reliability:** Tests and outputs are deterministic
2. **Demonstrates Composability:** Shows tools working together via CLI
3. **Real Value:** Each composition solves an actual problem

---

**Guidelines:**
- Only implement compositions that provide REAL value
- Don't dogfood just for the sake of dogfooding
- Focus on 2-4 high-impact compositions
- Keep tools standalone (graceful fallback)
- Prioritize: Test validation > Output consistency > Domain-specific needs

**Status:** Template - customize for your tool
