# Getting Started

This guide will help you get started with File-Based Semaphore (TypeScript).

## Prerequisites

- Node.js 18.0.0 or later
- npm (for TypeScript compilation)

## Installation

### Clone the Repository

```bash
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt/file-based-semaphore-ts
```

### Install Development Dependencies

```bash
npm install
```

This installs TypeScript and type definitions. There are **zero runtime dependencies**.

### Global CLI Access (Recommended)

To use the `semats` command globally:

```bash
npm link
```

Now you can run `semats` from anywhere:

```bash
semats --help
```

### Local Development

For local development without global install:

```bash
npx tsx src/index.ts --help
```

## Quick Start

### 1. Acquire a Lock

```bash
# Try to acquire a lock (non-blocking)
semats try-acquire /tmp/my-lock.lock

# Acquire with a descriptive tag
semats try-acquire /tmp/my-lock.lock --tag "npm build"
```

### 2. Check Lock Status

```bash
# Human-readable output
semats status /tmp/my-lock.lock

# JSON output for scripting
semats status /tmp/my-lock.lock --json
```

### 3. Release the Lock

```bash
# Release lock (only works if you own it)
semats release /tmp/my-lock.lock

# Force release (use with caution)
semats release /tmp/my-lock.lock --force
```

### 4. Clean Stale Locks

```bash
# Remove stale locks from crashed processes
semats clean /tmp/my-lock.lock
```

## Using as a Library

```typescript
import { Semaphore } from '@tuulbelt/file-based-semaphore-ts';

const semaphore = new Semaphore('/tmp/my-lock.lock');

// Try to acquire
const result = semaphore.tryAcquire('my-process');
if (result.ok) {
  try {
    // Do protected work
    console.log('Lock acquired, doing work...');
  } finally {
    semaphore.release();
  }
} else {
  console.log('Lock held by:', result.error.holderPid);
}
```

## Verifying Installation

Run the test suite to verify everything works:

```bash
npm test
```

You should see:

```
# tests 160
# pass 160
# fail 0
```

## Next Steps

- [CLI Usage](/tools/file-based-semaphore-ts/cli-usage) - Learn all CLI commands
- [Library Usage](/tools/file-based-semaphore-ts/library-usage) - Use as a TypeScript library
- [Examples](/tools/file-based-semaphore-ts/examples) - Real-world usage patterns
