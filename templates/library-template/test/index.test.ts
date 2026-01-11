/**
 * Tests for @tuulbelt/library-name
 */

import { test, describe } from 'node:test';
import assert from 'node:assert/strict';
import { process, validate, ok, err } from '../src/index.js';

describe('process', () => {
  test('processes valid input', () => {
    const result = process('hello');
    assert.strictEqual(result.ok, true);
    if (result.ok) {
      assert.strictEqual(result.value, 'HELLO');
    }
  });

  test('returns error for empty input', () => {
    const result = process('');
    assert.strictEqual(result.ok, false);
    if (!result.ok) {
      assert.strictEqual(result.error.message, 'Input is required');
    }
  });

  test('respects debug option', () => {
    // Debug mode doesn't change output, just logs
    const result = process('test', { debug: true });
    assert.strictEqual(result.ok, true);
  });
});

describe('validate', () => {
  test('returns true for valid string', () => {
    assert.strictEqual(validate('hello'), true);
  });

  test('returns false for empty string', () => {
    assert.strictEqual(validate(''), false);
  });

  test('returns false for non-string', () => {
    assert.strictEqual(validate(123), false);
    assert.strictEqual(validate(null), false);
    assert.strictEqual(validate(undefined), false);
    assert.strictEqual(validate({}), false);
  });
});

describe('Result helpers', () => {
  test('ok creates success result', () => {
    const result = ok('value');
    assert.strictEqual(result.ok, true);
    assert.strictEqual(result.value, 'value');
  });

  test('err creates failure result', () => {
    const error = new Error('test');
    const result = err(error);
    assert.strictEqual(result.ok, false);
    assert.strictEqual(result.error, error);
  });
});
