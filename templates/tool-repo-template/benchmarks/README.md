# TOOL_NAME Benchmarks

Performance benchmarks for TOOL_NAME using tatami-ng for statistical rigor.

## Quick Start

```bash
cd benchmarks
npm install
npm run bench
```

## Benchmark Environment

- **Tool:** tatami-ng v0.8.18 (criterion-equivalent for Node.js)
- **Configuration:**
  - 256 samples per benchmark
  - 2 seconds per benchmark
  - Automatic warmup for JIT optimization
  - Automatic outlier detection and removal
  - Target variance: <5%
- **Why tatami-ng:** Replaces tinybench for statistical rigor. See [rationale](#why-tatami-ng) below.

## Benchmark Results

### Overall Performance

| Operation | ops/sec | time/iter | Variance |
|-----------|---------|-----------|----------|
| Basic operation | TBD | TBD | TBD |
| Edge case | TBD | TBD | TBD |

### Competitor Comparison

| Tool | ops/sec | Notes |
|------|---------|-------|
| TOOL_NAME | TBD | (this tool) |
| competitor-1 | TBD | |
| competitor-2 | TBD | |

**Winner:** TBD

## Adding Benchmarks

See the [Tuulbelt Benchmarking Standards](../../docs/BENCHMARKING_STANDARDS.md) for guidelines.

### Adding Competitor Benchmarks

1. Add competitor as devDependency:
   ```bash
   npm install --save-dev competitor-name
   ```

2. Create `competitors/competitor-name.bench.ts`:
   ```typescript
   import { bench, group, run } from 'tatami-ng';
   import competitorFunction from 'competitor-name';

   let result: any;

   group('Competitor: competitor-name', () => {
     bench('competitor: basic case', () => {
       result = competitorFunction('input');
     });
   });

   await run({
     samples: 256,
     time: 2_000_000_000,
     warmup: true,
   });
   ```

3. Run comparison:
   ```bash
   npm run bench:compare
   ```

## Fixtures

Test data for benchmarks should be placed in `fixtures/`:
- `small.json` - Minimal test case
- `medium.json` - Typical workload
- `large.json` - Stress test

## CI Integration

To run benchmarks in CI (optional):

```yaml
- name: Run benchmarks
  run: |
    cd benchmarks
    npm install
    npm run bench
```

**Note:** Benchmarks are for development reference and regression detection, not for CI gating.

## Why tatami-ng?

**Problem with tinybench:** During property-validator optimization work, we discovered tinybench showed ±19.4% variance for unions and ±10.4% for arrays. This variance was LARGER than the optimization effects we were trying to measure, making performance work unreliable.

**Solution:** tatami-ng (criterion-equivalent for Node.js) provides:
- ✅ Statistical significance testing (p-values, confidence intervals)
- ✅ Automatic outlier detection and removal
- ✅ Variance, standard deviation, error margin built-in
- ✅ Target variance: <5% (vs tinybench's ±10-20%)
- ✅ 20x longer benchmarks (2s vs 100ms) for stable averages
- ✅ Zero dependencies (aligns with Tuulbelt principles)

**Trade-off:** Longer benchmark runs (2s per benchmark vs 100ms) in exchange for reliable, reproducible results.

See [property-validator's BENCHMARKING_MIGRATION.md](../../tools/property-validator/docs/BENCHMARKING_MIGRATION.md) for the full rationale and variance analysis.
