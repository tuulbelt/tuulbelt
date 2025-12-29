---
name: New Tool Proposal
about: Propose a new tool for Tuulbelt
title: '[PROPOSAL] '
labels: proposal
assignees: ''
---

## Problem Statement

What single problem does this tool solve? (1-2 sentences)

## Why It's Needed

Why do existing solutions not work? What gap does this fill?

## Proposed Interface

How would users interact with this tool?

### CLI Interface

```bash
# Example commands
tool-name [options] <input>

# Flags
--option value   Description
--flag           Description
```

### Library API (if applicable)

```typescript
import { function } from 'tool-name';

const result = function(input, config);
```

### Input/Output Format

- **Input:** (describe format, examples)
- **Output:** (describe format, examples)

## Implementation Approach

Is there a proven way to implement this? Reference existing implementations or algorithms.

## Use Cases

List 2-3 concrete use cases:

1. Use case one
2. Use case two
3. Use case three

## Complexity Estimate

- [ ] Quick (days) — Simple logic, well-defined scope
- [ ] Medium (1-2 weeks) — Some complexity, multiple features
- [ ] Complex (weeks+) — Significant implementation effort

## Checklist

Before submitting, confirm:

- [ ] This solves a single, focused problem
- [ ] It can be implemented without external dependencies
- [ ] It uses portable interfaces (CLI, files, sockets)
- [ ] I've read [PRINCIPLES.md](https://github.com/tuulbelt/tuulbelt/blob/main/PRINCIPLES.md)
- [ ] Existing tools don't already solve this problem

## Additional Context

Any other information, references, or examples.
