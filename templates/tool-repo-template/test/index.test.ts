import { test, describe } from 'node:test';
import assert from 'node:assert/strict';
import { process, type Config, type Result } from '../src/index.js';

describe('process', () => {
  test('converts input to uppercase', () => {
    const result = process('hello');

    assert.strictEqual(result.success, true);
    assert.strictEqual(result.data, 'HELLO');
    assert.strictEqual(result.error, undefined);
  });

  test('handles empty string', () => {
    const result = process('');

    assert.strictEqual(result.success, true);
    assert.strictEqual(result.data, '');
  });

  test('handles mixed case input', () => {
    const result = process('HeLLo WoRLd');

    assert.strictEqual(result.success, true);
    assert.strictEqual(result.data, 'HELLO WORLD');
  });

  test('handles special characters', () => {
    const result = process('hello! @#$% 123');

    assert.strictEqual(result.success, true);
    assert.strictEqual(result.data, 'HELLO! @#$% 123');
  });

  test('handles unicode characters', () => {
    const result = process('café résumé');

    assert.strictEqual(result.success, true);
    assert.strictEqual(result.data, 'CAFÉ RÉSUMÉ');
  });
});

describe('process with config', () => {
  test('accepts verbose option', () => {
    const config: Config = { verbose: true };
    const result = process('test', config);

    assert.strictEqual(result.success, true);
    assert.strictEqual(result.data, 'TEST');
  });

  test('works with empty config', () => {
    const result = process('test', {});

    assert.strictEqual(result.success, true);
    assert.strictEqual(result.data, 'TEST');
  });
});

describe('error handling', () => {
  test('returns error for non-string input', () => {
    // @ts-expect-error Testing invalid input
    const result = process(123);

    assert.strictEqual(result.success, false);
    assert.strictEqual(result.data, '');
    assert.strictEqual(result.error, 'Input must be a string');
  });

  test('returns error for null input', () => {
    // @ts-expect-error Testing invalid input
    const result = process(null);

    assert.strictEqual(result.success, false);
    assert.strictEqual(result.error, 'Input must be a string');
  });

  test('returns error for undefined input', () => {
    // @ts-expect-error Testing invalid input
    const result = process(undefined);

    assert.strictEqual(result.success, false);
    assert.strictEqual(result.error, 'Input must be a string');
  });
});

describe('Result type', () => {
  test('has correct shape on success', () => {
    const result: Result = process('test');

    assert('success' in result);
    assert('data' in result);
    assert.strictEqual(typeof result.success, 'boolean');
    assert.strictEqual(typeof result.data, 'string');
  });

  test('has correct shape on failure', () => {
    // @ts-expect-error Testing invalid input
    const result: Result = process(null);

    assert('success' in result);
    assert('data' in result);
    assert('error' in result);
    assert.strictEqual(typeof result.error, 'string');
  });
});
