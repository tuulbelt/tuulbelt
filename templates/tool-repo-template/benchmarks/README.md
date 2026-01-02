# TOOL_NAME Benchmarks

Performance benchmarks for TOOL_NAME.

## Quick Start

```bash
cd benchmarks
npm install
npm run bench
```

## Benchmark Results

### Overall Performance

| Operation | ops/sec | Average (ns) | Margin |
|-----------|---------|--------------|--------|
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
   import { Bench } from 'tinybench';
   import competitorFunction from 'competitor-name';

   const bench = new Bench({ time: 100 });
   let result: any;

   bench.add('competitor: basic case', () => {
     result = competitorFunction('input');
   });

   await bench.run();
   console.table(/* ... */);
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
