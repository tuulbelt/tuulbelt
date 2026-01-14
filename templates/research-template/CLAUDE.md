# Research Name - Development Context

## Overview

This is a Tuulbelt **research** project — an experimental exploration with relaxed principles.

**Key principle:** Document everything. Failures are valuable. Hypothesis validation is the goal.

## Quick Reference

```bash
# TypeScript
npm run experiment        # Run experiments
npm test                  # Run tests (if any)

# Rust
cargo run --bin experiment  # Run experiments
cargo test                  # Run tests
```

## Architecture

```
├── HYPOTHESIS.md       # Research hypothesis (REQUIRED)
├── FINDINGS.md         # Experiment results and discoveries
├── STATUS.md           # Current status and progress
├── src/                # Implementation (TypeScript or Rust)
├── experiments/        # Experiment scripts
│   ├── main.ts         # TypeScript experiment runner
│   └── main.rs         # Rust experiment runner
└── README.md
```

## Research Workflow

1. **Define Hypothesis** (HYPOTHESIS.md)
   - Clear, testable claims
   - Success criteria
   - Validation plan

2. **Run Experiments** (experiments/)
   - Test each claim
   - Document results in FINDINGS.md
   - Update STATUS.md

3. **Document Everything**
   - Successes AND failures
   - Unexpected discoveries
   - Open questions

4. **Evaluate for Graduation**
   - Does it meet a production category's principles?
   - Is the implementation stable?
   - Is documentation complete?

## Research Principles (Relaxed)

1. **Dependencies Allowed** — Pragmatism over purity for exploration
2. **Failure Is Acceptable** — Document what didn't work
3. **API May Change** — Stability not required
4. **HYPOTHESIS.md Required** — Clear research direction
5. **Document Findings** — Knowledge is the product

## Files to Update

When running experiments:
1. **FINDINGS.md** — Document results
2. **STATUS.md** — Update progress
3. **HYPOTHESIS.md** — Refine if needed

## Quality Checklist (Minimal)

Before committing:

- [ ] HYPOTHESIS.md exists and is clear
- [ ] FINDINGS.md updated with new results
- [ ] STATUS.md reflects current state
- [ ] Code runs without crashing
- [ ] (Tests passing — nice to have)

## Graduation Checklist

Before moving to a production category:

- [ ] Hypothesis validated
- [ ] Implementation stable
- [ ] Zero/minimal dependencies (per target category)
- [ ] Tests passing (80%+ coverage)
- [ ] Documentation complete
- [ ] Meets target category principles

## Part of Tuulbelt

See [Tuulbelt PRINCIPLES.md](https://github.com/tuulbelt/tuulbelt/blob/main/PRINCIPLES.md) for ecosystem principles.

Research projects have intentionally relaxed principles to enable exploration.
