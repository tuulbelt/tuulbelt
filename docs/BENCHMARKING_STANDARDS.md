# Tuulbelt Benchmarking Standards

**Last Updated:** 2026-01-04

This document defines benchmarking standards for Tuulbelt tools to ensure consistent, reliable, and reproducible performance measurements.

---

## Overview

Every Tuulbelt tool should have benchmarks to:
1. **Validate competitive performance** against alternatives
2. **Detect performance regressions** during development
3. **Guide optimization efforts** with data-driven decisions
4. **Document performance characteristics** for users

---

## Benchmark Frameworks

### TypeScript: tatami-ng

**Why tatami-ng over tinybench:**
- Statistical significance testing (p-values, confidence intervals)
- Automatic outlier detection and removal
- Variance, standard deviation, error margin built-in
- Target variance: <5% (vs tinybench's ±10-20%)
- 20x longer benchmarks (2s vs 100ms) for stable averages

**Installation:**
```bash
cd benchmarks
npm install tatami-ng tsx --save-dev
```

**Template:** See `templates/tool-repo-template/benchmarks/`

### Rust: Criterion

**Why Criterion:**
- Statistical analysis with confidence intervals
- Automatic warmup and outlier detection
- HTML reports with graphs
- Comparison with previous runs
- Zero runtime dependencies (dev-dependency only)

**Installation:**
```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "benchmark"
harness = false
```

**Template:** See `templates/rust-tool-template/benches/`

---

## Directory Structure

### TypeScript Tools

```
tool-name/
├── benchmarks/
│   ├── package.json          # Benchmark-specific dependencies
│   ├── index.bench.ts        # Main benchmarks
│   ├── README.md             # Results and documentation
│   ├── .gitignore            # Ignore node_modules
│   ├── competitors/          # Optional: competitor benchmarks
│   │   ├── competitor1.bench.ts
│   │   └── competitor2.bench.ts
│   └── fixtures/             # Optional: test data
│       ├── small.json
│       └── large.json
└── ...
```

### Rust Tools

```
tool-name/
├── benches/
│   ├── benchmark.rs          # Main benchmarks
│   └── README.md             # Results and documentation
├── Cargo.toml                # criterion in [dev-dependencies]
└── ...
```

---

## Configuration Standards

### TypeScript (tatami-ng)

```typescript
await run({
  units: false,        // Don't show unit reference
  silent: false,       // Show progress
  json: false,         // Human-readable output
  samples: 256,        // More samples = more stable results
  time: 2_000_000_000, // 2 seconds per benchmark (nanoseconds)
  warmup: true,        // Enable warm-up iterations for JIT
  latency: true,       // Show time per iteration
  throughput: true,    // Show operations per second
});
```

**Rationale:**
- **256 samples:** Provides statistical significance
- **2 seconds:** Long enough to smooth out variance
- **warmup:** Ensures JIT optimization before measurement

### Rust (Criterion)

```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn configure() -> Criterion {
    Criterion::default()
        .sample_size(100)           // Default: 100 samples
        .measurement_time(Duration::from_secs(5))
        .warm_up_time(Duration::from_secs(3))
}

criterion_group! {
    name = benches;
    config = configure();
    targets = benchmark_function
}
criterion_main!(benches);
```

---

## Writing Benchmarks

### Prevent Dead Code Elimination

**TypeScript:**
```typescript
import { bench, baseline, group, run } from 'tatami-ng';

// Store result to prevent elimination
let result: any;

bench('operation', () => {
  result = yourFunction(input);
});
```

**Rust:**
```rust
use criterion::black_box;

c.bench_function("operation", |b| {
    b.iter(|| {
        black_box(your_function(black_box(input)))
    });
});
```

### Group Related Benchmarks

**TypeScript:**
```typescript
group('String Operations', () => {
  baseline('parse: simple', () => { ... });
  bench('parse: complex', () => { ... });
  bench('parse: edge case', () => { ... });
});
```

**Rust:**
```rust
fn string_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("String Operations");
    group.bench_function("parse: simple", |b| { ... });
    group.bench_function("parse: complex", |b| { ... });
    group.finish();
}
```

### Parameterized Benchmarks

**TypeScript:**
```typescript
for (const size of [10, 100, 1000, 10000]) {
  bench(`process ${size} items`, () => {
    result = processItems(generateItems(size));
  });
}
```

**Rust:**
```rust
use criterion::BenchmarkId;

for size in [10, 100, 1000, 10000] {
    group.bench_with_input(
        BenchmarkId::from_parameter(size),
        &size,
        |b, &size| {
            b.iter(|| process_items(black_box(size)));
        },
    );
}
```

---

## Competitor Comparisons

### When to Compare

- Tool solves a common problem with existing alternatives
- Claims performance advantages over alternatives
- Validates that zero-dependency approach doesn't sacrifice speed

### How to Compare Fairly

1. **Same input data** - Use identical test cases
2. **Same output verification** - Ensure correct results
3. **Same API type** - Compare equivalent operations (e.g., safeParse vs safeParse)
4. **Document trade-offs** - Note what competitors provide that you don't

### Example Structure

```
benchmarks/
├── index.bench.ts           # Your tool's benchmarks
└── competitors/
    ├── zod.bench.ts         # Competitor 1
    ├── yup.bench.ts         # Competitor 2
    └── valibot.bench.ts     # Competitor 3
```

### Competitor Dependencies

Keep competitor dependencies in `benchmarks/package.json` only:

```json
{
  "devDependencies": {
    "tatami-ng": "^0.8.18",
    "tsx": "^4.19.2",
    "zod": "^3.24.1",
    "yup": "^1.6.0"
  }
}
```

This keeps the main tool zero-dependency while allowing fair comparisons.

---

## Baseline Tracking

### Purpose

Track performance over time to:
- Detect regressions early
- Measure optimization impact
- Document performance history

### BASELINE.md Format

```markdown
# Performance Baseline

**Version:** v0.1.0
**Date:** 2026-01-04
**Environment:** Node.js v22.x / Rust 1.75

## Results

| Operation | ops/sec | time/iter | Variance |
|-----------|---------|-----------|----------|
| parse simple | 1,234,567 | 810ns | ±1.2% |
| parse complex | 456,789 | 2.19μs | ±1.8% |

## Competitor Comparison

| Tool | Simple Parse | Complex Parse | Notes |
|------|--------------|---------------|-------|
| this-tool | 1.2M ops/sec | 456K ops/sec | Winner |
| zod | 800K ops/sec | 300K ops/sec | |
| yup | 600K ops/sec | 200K ops/sec | |
```

### When to Update

- After significant optimizations
- Before major releases
- After adding new operations

---

## CI Integration

### Optional Benchmark CI

Benchmarks are primarily for development, not CI gating. However, you can optionally run them:

**TypeScript:**
```yaml
- name: Run benchmarks
  run: |
    cd benchmarks
    npm install
    npm run bench
```

**Rust:**
```yaml
- name: Run benchmarks
  run: cargo bench --no-fail-fast
```

### Regression Detection (Advanced)

For critical performance paths, consider:
- Storing baseline results in CI artifacts
- Comparing against previous runs
- Alerting on significant regressions (>10%)

---

## Variance Guidelines

### Target Variance

| Quality | Variance | Interpretation |
|---------|----------|----------------|
| Excellent | <2% | Very stable measurement |
| Good | <5% | Reliable for comparisons |
| Acceptable | <10% | Usable with caution |
| Poor | >10% | Results unreliable |

### Reducing Variance

1. **Longer measurement time** - Use 2+ seconds per benchmark
2. **More samples** - Use 100+ samples
3. **Warmup iterations** - Enable JIT optimization
4. **Consistent environment** - Close other applications
5. **Multiple runs** - Average across several benchmark runs

---

## Reporting Results

### README.md Format

Include in `benchmarks/README.md`:

1. **Quick Start** - How to run benchmarks
2. **Environment** - Node/Rust version, OS, hardware notes
3. **Results table** - Latest baseline measurements
4. **Competitor comparison** - If applicable
5. **Methodology notes** - Any important caveats

### Example

```markdown
## Results (v0.1.0)

**Environment:** Node.js v22.x, Linux x64, tatami-ng v0.8.18

| Operation | ops/sec | time/iter | Variance |
|-----------|---------|-----------|----------|
| validate simple | 1.2M | 833ns | ±1.5% |
| validate complex | 450K | 2.2μs | ±2.1% |

### vs Competitors

| Library | Simple | Complex | Notes |
|---------|--------|---------|-------|
| **this-tool** | 1.2M | 450K | ✅ Winner |
| zod | 800K | 300K | |
| yup | 600K | 200K | |
```

---

## Quick Reference

### TypeScript Benchmark Template

```typescript
import { bench, baseline, group, run } from 'tatami-ng';
import { yourFunction } from '../src/index.ts';

let result: any;

group('Core Operations', () => {
  baseline('operation: basic', () => {
    result = yourFunction('simple input');
  });

  bench('operation: complex', () => {
    result = yourFunction('complex input with more data');
  });
});

await run({
  samples: 256,
  time: 2_000_000_000,
  warmup: true,
  latency: true,
  throughput: true,
});
```

### Rust Benchmark Template

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use your_crate::your_function;

fn benchmark_basic(c: &mut Criterion) {
    c.bench_function("basic operation", |b| {
        b.iter(|| {
            black_box(your_function(black_box("simple input")))
        });
    });
}

fn benchmark_complex(c: &mut Criterion) {
    c.bench_function("complex operation", |b| {
        b.iter(|| {
            black_box(your_function(black_box("complex input")))
        });
    });
}

criterion_group!(benches, benchmark_basic, benchmark_complex);
criterion_main!(benches);
```

---

## References

- [tatami-ng documentation](https://github.com/aspect-build/aspect)
- [Criterion.rs documentation](https://bheisler.github.io/criterion.rs/book/)
- [property-validator BENCHMARKING_MIGRATION.md](../tools/property-validator/docs/BENCHMARKING_MIGRATION.md)

---

**Last Review:** 2026-01-04
**Next Review:** After next tool release
