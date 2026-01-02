#!/usr/bin/env -S npx tsx
/**
 * TOOL_NAME Benchmarks
 *
 * Measures performance of core operations to:
 * 1. Validate competitive performance
 * 2. Detect performance regressions
 * 3. Guide optimization efforts
 *
 * See: /docs/BENCHMARKING_STANDARDS.md
 */

import { Bench } from 'tinybench';

// Import your tool's functions
// import { yourFunction } from '../src/index.ts';

const bench = new Bench({
  time: 100,           // Minimum 100ms per benchmark
  warmupIterations: 5, // Warm up the V8 JIT
  warmupTime: 100,     // 100ms warmup
});

// Prevent dead code elimination
let result: any;

// ============================================================================
// Core Operations Benchmarks
// ============================================================================

bench.add('operation: basic case', () => {
  // result = yourFunction('input');
  result = 'placeholder'; // Replace with actual function call
});

bench.add('operation: edge case', () => {
  // result = yourFunction('');
  result = 'placeholder'; // Replace with actual function call
});

// ============================================================================
// Run Benchmarks
// ============================================================================

await bench.warmup();
await bench.run();

console.table(
  bench.tasks.map((task) => ({
    'Benchmark': task.name,
    'ops/sec': task.result?.hz ? task.result.hz.toFixed(0).replace(/\B(?=(\d{3})+(?!\d))/g, ',') : 'N/A',
    'Average (ns)': task.result?.mean ? (task.result.mean * 1_000_000).toFixed(2) : 'N/A',
    'Margin': task.result?.rme ? `±${task.result.rme.toFixed(2)}%` : 'N/A',
    'Samples': task.result?.samples?.length || 'N/A',
  }))
);

console.log('\n✨ Benchmarks complete!');
console.log('💡 Add competitor benchmarks in competitors/ directory');
console.log('📖 See docs/BENCHMARKING_STANDARDS.md for guidelines');
