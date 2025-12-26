/**
 * Basic usage example for Structured Error Handler
 *
 * Run this example:
 *   npx tsx examples/basic.ts
 */

import { StructuredError, serializeError, formatError } from '../src/index.js';

// Example 1: Create a basic structured error
console.log('Example 1: Create a basic structured error');
console.log('=' .repeat(50));

const error1 = new StructuredError('File not found', {
  code: 'ENOENT',
  category: 'io',
  operation: 'readConfig',
  metadata: { path: '/etc/config.json' }
});

console.log('Error message:', error1.message);
console.log('Error code:', error1.code);
console.log('Error category:', error1.category);
console.log('Context:', error1.context);
console.log();

// Example 2: Serialize to JSON
console.log('Example 2: Serialize to JSON');
console.log('=' .repeat(50));

const json = error1.toJSON();
console.log(JSON.stringify(json, null, 2));
console.log();

// Example 3: Format for human-readable output
console.log('Example 3: Format for human-readable output');
console.log('=' .repeat(50));

console.log(error1.toString());
console.log();

// Example 4: Check error properties programmatically
console.log('Example 4: Check error properties programmatically');
console.log('=' .repeat(50));

if (error1.hasCode('ENOENT')) {
  console.log('✓ Error has code ENOENT');
}

if (error1.hasCategory('io')) {
  console.log('✓ Error has category io');
}
console.log();

// Example 5: Wrap an existing error
console.log('Example 5: Wrap an existing error');
console.log('=' .repeat(50));

const originalError = new Error('Connection refused');
const wrappedError = StructuredError.wrap(originalError, 'Failed to connect to database', {
  code: 'DB_ERROR',
  category: 'database',
  operation: 'connect',
  component: 'DatabaseClient',
  metadata: { host: 'localhost', port: 5432 }
});

console.log(wrappedError.toString());
console.log();

// Example 6: Convert any error to StructuredError
console.log('Example 6: Convert any error to StructuredError');
console.log('=' .repeat(50));

const converted = StructuredError.from(new TypeError('Invalid input'), {
  operation: 'validateInput',
  component: 'Validator'
});

console.log('Converted error:', converted.message);
console.log('Context:', converted.context);
console.log();

// Example 7: Serialize any error (not just StructuredError)
console.log('Example 7: Serialize any error');
console.log('=' .repeat(50));

const plainError = new Error('Something went wrong');
const serialized = serializeError(plainError);
console.log('Serialized plain error:', JSON.stringify(serialized, null, 2));
console.log();

// Example 8: Format error with stack trace
console.log('Example 8: Format error with stack trace');
console.log('=' .repeat(50));

console.log(formatError(error1, { includeStack: true }));
