/**
 * Advanced usage example for @tuulbelt/library-name
 *
 * Run with: npx tsx examples/advanced.ts
 */

import { process, validate, ok, err, type Result } from '../src/index.js';

// Example 1: Chaining Results
console.log('Example 1: Chaining Results');

function chain<T, U, E>(
  result: Result<T, E>,
  fn: (value: T) => Result<U, E>
): Result<U, E> {
  if (!result.ok) return result;
  return fn(result.value);
}

const chained = chain(
  process('hello'),
  (value) => ok(`Processed: ${value}`)
);
console.log('  Chained result:', chained);

// Example 2: Collecting Results
console.log('\nExample 2: Collecting Results');

function collectResults<T, E>(results: Result<T, E>[]): Result<T[], E> {
  const values: T[] = [];
  for (const result of results) {
    if (!result.ok) return result;
    values.push(result.value);
  }
  return ok(values);
}

const inputs = ['hello', 'world', 'test'];
const results = inputs.map(input => process(input));
const collected = collectResults(results);
console.log('  Collected:', collected);

// Example 3: Custom Error Types
console.log('\nExample 3: Custom Error Types');

interface ValidationError {
  field: string;
  message: string;
}

function validateUser(data: unknown): Result<{ name: string }, ValidationError> {
  if (!data || typeof data !== 'object') {
    return err({ field: 'root', message: 'Expected object' });
  }

  const obj = data as Record<string, unknown>;

  if (!obj.name || typeof obj.name !== 'string') {
    return err({ field: 'name', message: 'Name is required' });
  }

  return ok({ name: obj.name });
}

const userData = { name: 'Alice' };
const userResult = validateUser(userData);
console.log('  User validation:', userResult);

// Example 4: Async with Results
console.log('\nExample 4: Async with Results');

async function asyncProcess(input: string): Promise<Result<string>> {
  // Simulate async operation
  await new Promise(resolve => setTimeout(resolve, 10));

  if (!validate(input)) {
    return err(new Error('Invalid input'));
  }

  return process(input);
}

const asyncResult = await asyncProcess('async hello');
console.log('  Async result:', asyncResult);
