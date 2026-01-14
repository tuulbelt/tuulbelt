/**
 * Experiment Runner
 *
 * Run experiments to validate the hypothesis.
 *
 * Usage: npx tsx experiments/main.ts
 */

import { experiment, runExperiment } from '../src/index.js';

console.log('Research Name - Experiment Runner\n');
console.log('See HYPOTHESIS.md for what we\'re testing.\n');

// Experiment 1: Basic functionality
console.log('=== Experiment 1: Basic Functionality ===');
const start1 = performance.now();

const result = experiment('test input');
console.log('Result:', result);

const elapsed1 = performance.now() - start1;
console.log(`Time: ${elapsed1.toFixed(2)}ms\n`);

// Experiment 2: Performance
console.log('=== Experiment 2: Performance ===');
const iterations = 1000;
const start2 = performance.now();

for (let i = 0; i < iterations; i++) {
  experiment('performance test');
}

const elapsed2 = performance.now() - start2;
const perOp = elapsed2 / iterations;
console.log(`Total: ${elapsed2.toFixed(2)}ms`);
console.log(`Per operation: ${perOp.toFixed(4)}ms`);
console.log(`Ops/sec: ${(1000 / perOp).toFixed(0)}\n`);

// Experiment 3: With configuration
console.log('=== Experiment 3: Configured Run ===');
runExperiment('configured input', { verbose: true, iterations: 3 });
console.log();

// Summary
console.log('=== Summary ===');
console.log('Update FINDINGS.md with these results.');
