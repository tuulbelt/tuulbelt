# Research

Experimental projects pushing boundaries and exploring novel approaches.

## What Belongs Here

Research projects are **exploratory**:
- Test hypotheses and novel ideas
- May fail (that's okay and expected)
- Relaxed principles for experimentation
- Clear "experimental" labeling

## Principles

| Principle | Requirement |
|-----------|-------------|
| External Dependencies | Relaxed (pragmatism over purity) |
| Tuulbelt Dependencies | Allowed |
| Interface | Whatever works |
| Scope | Exploratory |
| Documentation | HYPOTHESIS.md required |

## Governance

Research requires **low governance**:
- Exploration is the goal
- Failure is acceptable
- Document findings (positive and negative)
- May graduate to other categories when proven

## Examples

- `zero-copy-parsing` — Zero-copy parser experiments
- `wasm-runtime` — WebAssembly runtime experiments
- `effect-system` — Effect system for TypeScript
- `incremental-compute` — Incremental computation framework
- `formal-verification` — Proof-carrying code experiments

## Creating a New Research Project

```bash
/new-research <name>
```

## Structure

```
research-name/
├── HYPOTHESIS.md          # What we're exploring (REQUIRED)
├── FINDINGS.md            # What we've learned
├── STATUS.md              # Current state (active/paused/concluded)
├── src/
├── experiments/           # Experiment scripts
└── README.md
```

## Status Labels

| Status | Meaning |
|--------|---------|
| `active` | Currently being explored |
| `paused` | Temporarily on hold |
| `concluded` | Exploration complete (success or failure) |
| `graduated` | Moved to another category |

## Graduation Path

When a research project proves successful:

```
research/effect-system → libraries/effect-system
(hypothesis validated, implementation stable)
```

**Graduation requirements:**
1. Hypothesis validated with evidence
2. Implementation stable and tested
3. Documentation complete
4. Meets target category's principles

## HYPOTHESIS.md Template

```markdown
# Hypothesis: [Project Name]

## Question
What are we trying to learn or prove?

## Hypothesis
Our prediction about the outcome.

## Approach
How we plan to test this.

## Success Criteria
What would prove/disprove the hypothesis?

## Related Work
Prior art and references.
```
