# Tuulbelt Benchmarking Standards

**Last Updated:** 2026-01-02
**Status:** Production Standard
**Applies To:** All Tuulbelt tools requiring performance validation

---

## Philosophy

Benchmarking in Tuulbelt serves three purposes:

1. **Validation** - Ensure performance is competitive with alternatives
2. **Regression Detection** - Catch performance regressions before release
3. **Optimization Guidance** - Identify bottlenecks worth optimizing

**We prioritize:**
- **Simplicity** over exhaustive coverage (benchmark common cases, not edge cases)
- **Reproducibility** over precision (consistent results matter more than nanosecond accuracy)
- **Developer Experience** over tooling sophistication (easy to create, run, and interpret)

---

## When to Benchmark

**✅ Benchmark when:**
- Tool performance is a primary selling point (e.g., validation, parsing, diffing)
- Comparing against established competitors (zod, yup, joi, serde, etc.)
- Tool processes large datasets or high-frequency operations

**❌ Skip benchmarking when:**
- Tool is primarily I/O bound (network, file system)
- Performance differences would be imperceptible to users
- No meaningful competitors exist for comparison

---

## Benchmarking Tools

### TypeScript / Node.js

**Primary:** [tatami-ng](https://github.com/poolifier/tatami-ng) (v0.8.18+)

**Why tatami-ng:**
- Criterion-equivalent statistical rigor for Node.js
- Automatic outlier detection and removal
- Significance testing (p-values, confidence intervals)
- Variance, standard deviation, error margin built-in
- Target variance: <5% (vs tinybench's ±10-20%)
- Modern (async/await, ESM support, TypeScript native)
- Multi-runtime support (Node.js, Bun, Deno)
- Zero dependencies (aligns with Tuulbelt principles)
- API backward compatible with mitata (Rust-based benchmarking)

**Migration from tinybench:**
Property-validator discovered tinybench had ±19.4% variance for unions and ±10.4% for arrays, making optimization work unreliable. tatami-ng provides statistical rigor needed for trustworthy performance work. See [property-validator's BENCHMARKING_MIGRATION.md](https://github.com/tuulbelt/property-validator/blob/main/docs/BENCHMARKING_MIGRATION.md) for full rationale.

**Alternative:** Node.js `perf_hooks` for simple cases (no external dep)

### Rust

**Primary:** [criterion.rs](https://github.com/bheisler/criterion.rs) (v0.5+)

**Why Criterion:**
- Gold standard in Rust ecosystem
- Statistical analysis built-in
- HTML report generation
- Baseline comparison support
- Warm-up and iteration handling

---

## Common Pitfalls & Prevention

### 1. Dead Code Elimination

**Problem:** Compiler removes unused computations, making benchmarks artificially fast.

**Prevention (TypeScript):**
```typescript
let result; // Declare outside loop
bench('operation', () => {
  result = expensiveOperation(data); // Assign to prevent DCE
});
```

**Prevention (Rust):**
```rust
use criterion::black_box;

c.bench_function("operation", |b| {
  b.iter(|| black_box(expensive_operation(black_box(&data))))
});
```

### 2. Insufficient Warm-up

**Problem:** JIT compilation skews initial measurements.

**Prevention:**
- tatami-ng: Automatic warm-up enabled with `warmup: true` in run() config
- Criterion: Automatic warm-up (default 3 seconds)
- Always use default warm-up unless profiling shows otherwise

### 3. Memory Allocation Noise

**Problem:** GC pauses during measurement distort results.

**Prevention:**
- Pre-allocate test data outside measurement loop
- Use same data across iterations (avoid per-iteration allocation)
- Run enough iterations to average out GC effects

### 4. Input Predictability

**Problem:** Constant inputs enable compiler optimizations not realistic in production.

**Prevention:**
```typescript
// Good: Vary inputs
const inputs = [small, medium, large];
bench('operation', () => {
  const data = inputs[Math.floor(Math.random() * inputs.length)];
  result = operation(data);
});

// Bad: Constant input
bench('operation', () => {
  result = operation(sameDataEveryTime); // May be optimized away
});
```

---

## Directory Structure

### TypeScript Tools

```
tool-name/
├── benchmarks/
│   ├── README.md                    # Results, interpretation, how to run
│   ├── fixtures/
│   │   ├── small-input.json         # 10 items
│   │   ├── medium-input.json        # 100 items
│   │   └── large-input.json         # 1,000 items
│   ├── competitors/
│   │   ├── zod.bench.ts             # Competitor A
│   │   ├── yup.bench.ts             # Competitor B
│   │   └── joi.bench.ts             # Competitor C
│   ├── index.bench.ts               # Main benchmark suite
│   └── package.json                 # Benchmark-specific deps
├── package.json                     # Tool package (no benchmark deps here)
└── ...
```

**Key principle:** Benchmark dependencies live in `benchmarks/package.json`, NOT in tool root.

### Rust Tools

```
tool-name/
├── benches/
│   ├── README.md                    # Results, interpretation
│   ├── fixtures/
│   │   ├── small.json
│   │   ├── medium.json
│   │   └── large.json
│   ├── main.rs                      # Criterion benchmarks
│   └── competitors.rs               # Optional: compare with other crates
├── Cargo.toml                       # Tool manifest
└── ...
```

**Criterion setup in Cargo.toml:**
```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "main"
harness = false
```

---

## Benchmark Scenarios

### Core Operations (Always Benchmark)

1. **Small Input** - 10 items/properties (latency-sensitive)
2. **Medium Input** - 100 items/properties (typical workload)
3. **Large Input** - 1,000 items/properties (stress test)

### Operation Types (Pick Relevant Ones)

- **Validation** - Pass/fail determination
- **Parsing** - String → structured data
- **Serialization** - Structured data → string
- **Transformation** - Data mutation
- **Comparison** - Equality/diffing

**Example for validation library:**
```typescript
// Scenarios to benchmark:
- Valid primitive (string, number, boolean)
- Valid simple object (5 properties)
- Valid nested object (3 levels deep)
- Invalid object (early rejection)
- Invalid object (deep rejection)
```

---

## Implementation Template

### TypeScript Benchmark (`benchmarks/index.bench.ts`)

```typescript
import { bench, baseline, group, run } from 'tatami-ng';
import { readFileSync } from 'node:fs';
import * as v from '../src/index.js';

// Load fixtures (do this ONCE, outside bench)
const fixtures = {
  small: JSON.parse(readFileSync('./fixtures/small-input.json', 'utf8')),
  medium: JSON.parse(readFileSync('./fixtures/medium-input.json', 'utf8')),
  large: JSON.parse(readFileSync('./fixtures/large-input.json', 'utf8')),
};

// Define schemas (do this ONCE, outside bench)
const schemas = {
  simple: v.object({ name: v.string(), age: v.number() }),
  complex: v.object({
    users: v.array(v.object({
      name: v.string(),
      email: v.string(),
      metadata: v.optional(v.object({ tags: v.array(v.string()) }))
    }))
  }),
};

// Prevent dead code elimination
let result: any;

// Group related benchmarks
group('Validation', () => {
  baseline('validate simple object (valid)', () => {
    result = v.validate(schemas.simple, { name: 'Alice', age: 30 });
  });

  bench('validate complex nested (valid)', () => {
    result = v.validate(schemas.complex, fixtures.medium);
  });

  bench('validate complex nested (invalid - early)', () => {
    result = v.validate(schemas.complex, { users: null });
  });
});

// Run benchmarks
await run({
  units: false,        // Don't show unit reference
  silent: false,       // Show progress
  json: false,         // Human-readable output
  samples: 256,        // More samples = more stable results
  time: 2_000_000_000, // 2 seconds per benchmark (20x longer than tinybench)
  warmup: true,        // Enable warm-up iterations for JIT
  latency: true,       // Show time per iteration
  throughput: true,    // Show operations per second
});
```

### Rust Benchmark (`benches/main.rs`)

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tool_name::{validate, Schema};

fn load_fixtures() -> Fixtures {
    Fixtures {
        small: serde_json::from_str(include_str!("fixtures/small.json")).unwrap(),
        medium: serde_json::from_str(include_str!("fixtures/medium.json")).unwrap(),
        large: serde_json::from_str(include_str!("fixtures/large.json")).unwrap(),
    }
}

fn benchmark_validation(c: &mut Criterion) {
    let fixtures = load_fixtures();
    let schema = Schema::object(/* ... */);

    c.bench_function("validate simple object", |b| {
        b.iter(|| {
            let result = validate(black_box(&schema), black_box(&fixtures.small));
            black_box(result);
        });
    });

    c.bench_function("validate complex nested", |b| {
        b.iter(|| {
            let result = validate(black_box(&schema), black_box(&fixtures.medium));
            black_box(result);
        });
    });
}

criterion_group!(benches, benchmark_validation);
criterion_main!(benches);
```

---

## Running Benchmarks

### TypeScript

```bash
# In benchmarks/ directory
npm install                           # Install benchmark deps (tatami-ng, competitors)
npm run bench                         # Run all benchmarks
npm run bench:compare                 # Run with competitor comparison
```

**package.json scripts:**
```json
{
  "scripts": {
    "bench": "node --import tsx index.bench.ts",
    "bench:compare": "node --import tsx index.bench.ts && node --import tsx competitors/zod.bench.ts && node --import tsx competitors/yup.bench.ts"
  }
}
```

### Rust

```bash
cargo bench                           # Run all benchmarks
cargo bench --bench main              # Run specific benchmark
cargo bench -- --save-baseline main   # Save baseline for comparison
cargo bench -- --baseline main        # Compare against saved baseline
```

---

## Interpreting Results

### What to Look For

**✅ Good Signs:**
- Competitive with or faster than established libraries (within 2x)
- Consistent performance across runs (low variance)
- Reasonable scaling (large inputs ~10x slower than small, not 100x)

**⚠️ Warning Signs:**
- 10x+ slower than competitors (investigate before release)
- High variance (>20% coefficient of variation) - unstable measurements
- Superlinear scaling (O(n²) when O(n) expected) - algorithmic issue

### Example Output Interpretation

**tatami-ng output:**
```
benchmark                                  time/iter       iters/s
----------------------------------------------------------------
• Validation
primitive: string (valid)              220.18 ns ± 1.07 %  5629749
primitive: number (valid)              232.24 ns ± 0.93 %  5415341

Validation summary
  primitive: string (valid)
    1.05 ± 1.01 % times faster than primitive: number (valid)
```

**Interpretation:**
- **time/iter:** Average time per iteration (lower is better)
- **± X.XX %:** Error margin / variance (lower is better, <5% target)
- **iters/s:** Operations per second (higher is better)
- **summary:** Relative performance vs baseline (first benchmark in group)

**Criterion output:**
```
validate simple object  time:   [810.2 ns 815.5 ns 821.3 ns]
                        change: [-2.5% -0.8% +1.2%] (p = 0.12 > 0.05)
                        No change in performance detected.
```

**Interpretation:**
- **time:** [lower bound, estimate, upper bound] with 95% confidence
- **change:** Performance delta vs baseline (if available)
- **p-value:** Statistical significance (p > 0.05 = no significant change)

---

## Reporting Results

### benchmarks/README.md Template

````markdown
# Benchmark Results

**Last Updated:** 2026-01-02
**Tool Version:** v0.4.0
**Node.js:** v20.11.0 / **Rust:** 1.75.0
**Hardware:** MacBook Pro M2, 16GB RAM

## Summary

Our tool performs competitively with established libraries:

- **Simple validation:** ~20% faster than zod, ~30% faster than yup
- **Complex nested:** Comparable to zod (within 5%)
- **Large datasets:** Scales linearly, no performance cliffs

## Detailed Results

### Small Input (10 items)

| Library | ops/sec | Average (ns) | Relative |
|---------|---------|--------------|----------|
| **our-tool** | 1,234,567 | 810 | 1.0x (baseline) |
| zod | 987,654 | 1,012 | 0.8x |
| yup | 856,432 | 1,168 | 0.69x |

### Medium Input (100 items)

| Library | ops/sec | Average (ns) | Relative |
|---------|---------|--------------|----------|
| **our-tool** | 123,456 | 8,100 | 1.0x |
| zod | 120,345 | 8,310 | 0.97x |
| yup | 98,234 | 10,180 | 0.80x |

## Methodology

- **Tool:** tatami-ng v0.8.18 (criterion-equivalent for Node.js)
- **Configuration:**
  - 256 samples per benchmark
  - 2 seconds per benchmark (vs tinybench's 100ms)
  - Automatic warmup for JIT optimization
  - Automatic outlier detection and removal
- **Target variance:** <5%
- **Why the change:** Previous tool (tinybench) showed ±19.4% variance, making optimization work unreliable

## How to Reproduce

```bash
cd benchmarks/
npm install
npm run bench
```

## Analysis

**Strengths:**
- Fast primitive validation due to zero-dependency implementation
- Efficient object validation with pre-compiled validators
- Minimal overhead for simple schemas

**Trade-offs:**
- Slightly slower than AJV for very large objects (1000+ properties)
- No async validation support (by design)

**Conclusion:** Competitive performance for typical use cases (small-medium schemas). For massive schemas (100+ properties), consider AJV if performance is critical.
````

---

## Scaffold Integration

### Tool Template Updates

**For `templates/tool-repo-template/` (TypeScript):**

Add `benchmarks/` directory with:
- `benchmarks/README.md` (template above)
- `benchmarks/package.json` (tatami-ng dependency)
- `benchmarks/index.bench.ts` (tatami-ng template with group() and run())
- `benchmarks/fixtures/` (empty directory)

**For `templates/rust-tool-template/` (Rust):**

Add `benches/` directory with:
- `benches/README.md` (template above)
- `benches/main.rs` (placeholder criterion benchmark)
- `benches/fixtures/` (empty directory)

Update `Cargo.toml`:
```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "main"
harness = false
```

### /new-tool Command Integration

When creating a new tool, ask:
```
Does this tool require performance benchmarking? (y/N)
```

If yes:
- Include benchmark scaffolding
- Add benchmark setup to README
- Include benchmark run in CI (optional step)

---

## CI Integration (Optional)

Benchmarks typically DON'T run in CI by default (too slow, too noisy). However, for critical tools:

**GitHub Actions example:**
```yaml
name: Benchmark
on:
  workflow_dispatch: # Manual trigger only

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 20
      - run: cd benchmarks && npm install
      - run: cd benchmarks && npm run bench
      - uses: actions/upload-artifact@v4
        with:
          name: benchmark-results
          path: benchmarks/results.txt
```

**When to enable CI benchmarks:**
- Performance regressions have happened before
- Tool is performance-critical (e.g., used in hot paths)
- Competitor benchmarks are a key selling point

---

## Versioning & History

**Baseline Tracking:**

Create `benchmarks/baselines/` directory:
```
benchmarks/baselines/
├── v0.1.0.json      # Benchmark results for v0.1.0
├── v0.2.0.json      # Benchmark results for v0.2.0
└── v0.4.0.json      # Current version
```

**When to update baseline:**
- After each minor version release (v0.2.0, v0.3.0, etc.)
- After significant performance work
- When adding new benchmark scenarios

**Comparison across versions:**
```bash
npm run bench:compare-versions
# Compare v0.4.0 vs v0.3.0 baseline
```

---

## Maintenance

**When to update benchmarks:**
- ✅ Adding new core features (add corresponding benchmark)
- ✅ Significant API changes (update affected benchmarks)
- ✅ Performance optimization PRs (verify improvements)
- ❌ Minor bug fixes (unless performance-related)
- ❌ Documentation updates (no benchmark impact)

**Benchmark maintenance checklist:**
- [ ] Update fixtures if data format changes
- [ ] Update competitor versions annually (check for new releases)
- [ ] Re-run baselines after major Node.js/Rust version upgrades
- [ ] Archive old baselines (keep last 3 major versions)

---

## References

**Migration Documentation:**
- [property-validator's BENCHMARKING_MIGRATION.md](https://github.com/tuulbelt/property-validator/blob/main/docs/BENCHMARKING_MIGRATION.md) - Why we switched from tinybench to tatami-ng (±19.4% variance → ±1.54%)

**Research Sources:**
- [Zod vs Yup Comparison 2025](https://dev.to/dataformathub/zod-vs-yup-vs-typebox-the-ultimate-schema-validation-guide-for-2025-1l4l)
- [tatami-ng Documentation](https://github.com/poolifier/tatami-ng) - Criterion-equivalent benchmarking for Node.js
- [mitata benchmarking article](https://steve-adams.me/typescript-benchmarking-mitata.html) - Background on mitata (tatami-ng's predecessor)
- [Node.js Benchmarking State](https://webpro.nl/articles/the-state-of-benchmarking-in-nodejs)
- [Criterion.rs Guide](https://bheisler.github.io/criterion.rs/book/getting_started.html)
- [Microbenchmarking Pitfalls](https://www.oracle.com/technical-resources/articles/java/architect-benchmarking.html)

---

**Next:** See property-validator's `benchmarks/README.md` for a concrete implementation example.
