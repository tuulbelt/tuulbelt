/**
 * Basic usage example for @tuulbelt/library-name
 *
 * Run with: npx tsx examples/basic.ts
 */

import { process, validate } from '../src/index.js';

// Example 1: Basic processing
console.log('Example 1: Basic processing');
const result1 = process('hello world');
if (result1.ok) {
  console.log('  Result:', result1.value);
} else {
  console.log('  Error:', result1.error.message);
}

// Example 2: Validation
console.log('\nExample 2: Input validation');
const inputs: unknown[] = ['valid string', '', 123, null];

for (const input of inputs) {
  const isValid = validate(input);
  console.log(`  validate(${JSON.stringify(input)}): ${isValid}`);
}

// Example 3: Error handling
console.log('\nExample 3: Error handling');
const result2 = process('');
if (!result2.ok) {
  console.log('  Expected error:', result2.error.message);
}
