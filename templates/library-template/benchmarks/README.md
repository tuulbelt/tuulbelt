# Benchmark Results

**Last Updated:** YYYY-MM-DD
**Library Version:** v0.1.0
**Node.js:** v20.x

## Summary

| Operation | ops/sec | Average | Notes |
|-----------|---------|---------|-------|
| process (valid) | TBD | TBD | Baseline |
| validate (valid) | TBD | TBD | |
| ok() | TBD | TBD | |
| err() | TBD | TBD | |

## How to Run

```bash
cd benchmarks
npm install
npm run bench
```

## Methodology

- **Tool:** tatami-ng v0.8.18 (criterion-equivalent for Node.js)
- **Configuration:**
  - 256 samples per benchmark
  - 2 seconds per benchmark
  - Automatic warmup for JIT optimization
  - Automatic outlier detection

## Competitor Comparison

_Add competitor benchmarks here if applicable._
