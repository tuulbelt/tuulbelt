# TOOL_NAME Benchmarks

Performance benchmarks using [criterion.rs](https://github.com/bheisler/criterion.rs).

## Quick Start

```bash
cargo bench
```

## Benchmark Results

### Overall Performance

| Operation | Time | Throughput | Notes |
|-----------|------|------------|-------|
| Basic operation | TBD | TBD | |
| Edge case | TBD | TBD | |

### Competitor Comparison

| Tool | Performance | Notes |
|------|-------------|-------|
| TOOL_NAME | TBD | (this tool) |
| competitor-1 | TBD | |
| competitor-2 | TBD | |

**Winner:** TBD

## Adding Benchmarks

See the [Tuulbelt Benchmarking Standards](../../docs/BENCHMARKING_STANDARDS.md) for guidelines.

### Benchmark Structure

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_name(c: &mut Criterion) {
    c.bench_function("operation description", |b| {
        b.iter(|| {
            black_box(your_function(black_box(input)));
        });
    });
}

criterion_group!(benches, benchmark_name);
criterion_main!(benches);
```

### Advanced Features

**Parameterized benchmarks:**
```rust
fn benchmark_sizes(c: &mut Criterion) {
    for size in [10, 100, 1000].iter() {
        c.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &size| {
                b.iter(|| your_function(black_box(size)));
            },
        );
    }
}
```

**Throughput measurement:**
```rust
fn benchmark_throughput(c: &mut Criterion) {
    let data = vec![0u8; 1024];
    c.bench_function("process_1kb", |b| {
        b.iter(|| your_function(black_box(&data)));
    })
    .throughput(Throughput::Bytes(1024));
}
```

## CI Integration

To run benchmarks in CI (optional):

```yaml
- name: Run benchmarks
  run: cargo bench --no-fail-fast
```

**Note:** Benchmarks are for development reference and regression detection, not for CI gating.

## Output

Criterion generates detailed reports in `target/criterion/`:
- HTML reports with graphs
- Statistical analysis
- Comparison with previous runs

View reports:
```bash
open target/criterion/report/index.html
```
