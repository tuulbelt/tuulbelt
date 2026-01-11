/**
 * Benchmarks for @tuulbelt/library-name
 *
 * Run with: npm run bench
 */

import { bench, baseline, group, run } from 'tatami-ng';
import { process, validate, ok, err } from '../src/index.js';

// Prevent dead code elimination
let result: unknown;

group('Core API', () => {
  baseline('process (valid input)', () => {
    result = process('hello world');
  });

  bench('process (with options)', () => {
    result = process('hello world', { debug: false });
  });

  bench('validate (valid)', () => {
    result = validate('hello');
  });

  bench('validate (invalid)', () => {
    result = validate(null);
  });
});

group('Result Helpers', () => {
  baseline('ok()', () => {
    result = ok('value');
  });

  bench('err()', () => {
    result = err(new Error('error'));
  });
});

// Run benchmarks
await run({
  units: false,
  silent: false,
  json: false,
  samples: 256,
  time: 2_000_000_000, // 2 seconds
  warmup: true,
  latency: true,
  throughput: true,
});
