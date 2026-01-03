#!/usr/bin/env node --import tsx
/**
 * TOOL_NAME Benchmarks
 *
 * Measures performance of core operations using tatami-ng for statistical rigor.
 *
 * Run: npm run bench
 *
 * Why tatami-ng over tinybench:
 * - Statistical significance testing (p-values, confidence intervals)
 * - Automatic outlier detection and removal
 * - Variance, standard deviation, error margin built-in
 * - Designed for <5% variance (vs tinybench's ±10-20% variance)
 * - Criterion-equivalent benchmarking for Node.js
 *
 * See: /docs/BENCHMARKING_STANDARDS.md
 */

import { bench, baseline, group, run } from 'tatami-ng';

// Import your tool's functions
// import { yourFunction } from '../src/index.ts';

// Prevent dead code elimination
let result: any;

// ============================================================================
// Core Operations Benchmarks
// ============================================================================

group('Core Operations', () => {
  baseline('operation: basic case', () => {
    // result = yourFunction('input');
    result = 'placeholder'; // Replace with actual function call
  });

  bench('operation: edge case', () => {
    // result = yourFunction('');
    result = 'placeholder'; // Replace with actual function call
  });
});

// ============================================================================
// Run Benchmarks
// ============================================================================

await run({
  units: false,        // Don't show unit reference
  silent: false,       // Show progress
  json: false,         // Human-readable output
  samples: 256,        // More samples = more stable results
  time: 2_000_000_000, // 2 seconds per benchmark (vs tinybench: 100ms)
  warmup: true,        // Enable warm-up iterations for JIT
  latency: true,       // Show time per iteration
  throughput: true,    // Show operations per second
});

console.log('\n✨ Benchmarks complete!');
console.log('💡 Target variance: <5%');
console.log('📖 See docs/BENCHMARKING_STANDARDS.md for guidelines');
