/**
 * Tests for @tuulbelt/result-type
 */

import { test, describe } from 'node:test';
import assert from 'node:assert/strict';
import {
  ok,
  err,
  isOk,
  isErr,
  map,
  mapErr,
  andThen,
  flatMap,
  or,
  orElse,
  match,
  unwrap,
  unwrapOr,
  unwrapOrElse,
  expect,
  unwrapErr,
  all,
  any,
  partition,
  tryCatch,
  tryCatchAsync,
  fromPromise,
  toOptional,
  fromNullable,
  tap,
  tapErr,
  type Result,
  type Ok,
  type Err,
} from '../src/index.js';

// ============================================================================
// Constructor Tests
// ============================================================================

describe('ok()', () => {
  test('creates an Ok result', () => {
    const result = ok(42);
    assert.strictEqual(result.ok, true);
    assert.strictEqual(result.value, 42);
  });

  test('works with various types', () => {
    assert.strictEqual(ok('hello').value, 'hello');
    assert.deepEqual(ok([1, 2, 3]).value, [1, 2, 3]);
    assert.deepEqual(ok({ a: 1 }).value, { a: 1 });
    assert.strictEqual(ok(null).value, null);
    assert.strictEqual(ok(undefined).value, undefined);
  });
});

describe('err()', () => {
  test('creates an Err result', () => {
    const result = err('failed');
    assert.strictEqual(result.ok, false);
    assert.strictEqual(result.error, 'failed');
  });

  test('works with Error objects', () => {
    const error = new Error('Something went wrong');
    const result = err(error);
    assert.strictEqual(result.error, error);
  });

  test('works with custom error types', () => {
    type ApiError = { code: number; message: string };
    const error: ApiError = { code: 404, message: 'Not found' };
    const result = err(error);
    assert.deepEqual(result.error, error);
  });
});

// ============================================================================
// Type Guard Tests
// ============================================================================

describe('isOk()', () => {
  test('returns true for Ok result', () => {
    assert.strictEqual(isOk(ok(42)), true);
  });

  test('returns false for Err result', () => {
    assert.strictEqual(isOk(err('failed')), false);
  });

  test('narrows type correctly', () => {
    const result: Result<number, string> = ok(42);
    if (isOk(result)) {
      // TypeScript knows result.value exists here
      const value: number = result.value;
      assert.strictEqual(value, 42);
    }
  });
});

describe('isErr()', () => {
  test('returns false for Ok result', () => {
    assert.strictEqual(isErr(ok(42)), false);
  });

  test('returns true for Err result', () => {
    assert.strictEqual(isErr(err('failed')), true);
  });

  test('narrows type correctly', () => {
    const result: Result<number, string> = err('failed');
    if (isErr(result)) {
      // TypeScript knows result.error exists here
      const error: string = result.error;
      assert.strictEqual(error, 'failed');
    }
  });
});

// ============================================================================
// Transformation Tests
// ============================================================================

describe('map()', () => {
  test('transforms Ok value', () => {
    const result = map(ok(5), (x) => x * 2);
    assert.strictEqual(result.ok, true);
    if (result.ok) assert.strictEqual(result.value, 10);
  });

  test('leaves Err unchanged', () => {
    const original = err('error');
    const result = map(original, (x: number) => x * 2);
    assert.strictEqual(result.ok, false);
    if (!result.ok) assert.strictEqual(result.error, 'error');
  });

  test('can change value type', () => {
    const result = map(ok(42), (x) => x.toString());
    assert.strictEqual(result.ok, true);
    if (result.ok) assert.strictEqual(result.value, '42');
  });
});

describe('mapErr()', () => {
  test('transforms Err value', () => {
    const result = mapErr(err('error'), (e) => new Error(e));
    assert.strictEqual(result.ok, false);
    if (!result.ok) assert.strictEqual(result.error.message, 'error');
  });

  test('leaves Ok unchanged', () => {
    const original = ok(42);
    const result = mapErr(original, (e: string) => new Error(e));
    assert.strictEqual(result.ok, true);
    if (result.ok) assert.strictEqual(result.value, 42);
  });
});

describe('andThen() / flatMap()', () => {
  function parseNumber(s: string): Result<number, string> {
    const n = parseInt(s, 10);
    return isNaN(n) ? err('Not a number') : ok(n);
  }

  test('chains successful operations', () => {
    const result = andThen(ok('42'), parseNumber);
    assert.strictEqual(result.ok, true);
    if (result.ok) assert.strictEqual(result.value, 42);
  });

  test('short-circuits on error', () => {
    const result = andThen(err('initial error'), parseNumber);
    assert.strictEqual(result.ok, false);
    if (!result.ok) assert.strictEqual(result.error, 'initial error');
  });

  test('returns error from chained function', () => {
    const result = andThen(ok('not-a-number'), parseNumber);
    assert.strictEqual(result.ok, false);
    if (!result.ok) assert.strictEqual(result.error, 'Not a number');
  });

  test('flatMap is an alias for andThen', () => {
    assert.strictEqual(flatMap, andThen);
  });
});

describe('or()', () => {
  test('returns Ok if first is Ok', () => {
    const result = or(ok(1), ok(2));
    assert.strictEqual(result.ok, true);
    if (result.ok) assert.strictEqual(result.value, 1);
  });

  test('returns alternative if first is Err', () => {
    const result = or(err('a'), ok(2));
    assert.strictEqual(result.ok, true);
    if (result.ok) assert.strictEqual(result.value, 2);
  });

  test('returns alternative Err if both are Err', () => {
    const result = or(err('a'), err('b'));
    assert.strictEqual(result.ok, false);
    if (!result.ok) assert.strictEqual(result.error, 'b');
  });
});

describe('orElse()', () => {
  test('returns Ok unchanged', () => {
    const result = orElse(ok(1), () => ok(2));
    assert.strictEqual(result.ok, true);
    if (result.ok) assert.strictEqual(result.value, 1);
  });

  test('calls function for Err and returns result', () => {
    const result = orElse(err('error'), (e) => ok(e.length));
    assert.strictEqual(result.ok, true);
    if (result.ok) assert.strictEqual(result.value, 5);
  });

  test('receives error in callback', () => {
    let receivedError: string | null = null;
    orElse(err('test-error'), (e) => {
      receivedError = e;
      return ok(0);
    });
    assert.strictEqual(receivedError, 'test-error');
  });
});

// ============================================================================
// Pattern Matching Tests
// ============================================================================

describe('match()', () => {
  test('calls ok handler for Ok result', () => {
    const result = match(ok(42), {
      ok: (v) => `value: ${v}`,
      err: (e) => `error: ${e}`,
    });
    assert.strictEqual(result, 'value: 42');
  });

  test('calls err handler for Err result', () => {
    const result = match(err('failed'), {
      ok: (v) => `value: ${v}`,
      err: (e) => `error: ${e}`,
    });
    assert.strictEqual(result, 'error: failed');
  });

  test('can return different types', () => {
    const okResult = match(ok(1), {
      ok: () => 'yes',
      err: () => 'no',
    });
    const errResult = match(err('x'), {
      ok: () => 'yes',
      err: () => 'no',
    });
    assert.strictEqual(okResult, 'yes');
    assert.strictEqual(errResult, 'no');
  });
});

// ============================================================================
// Unwrapping Tests
// ============================================================================

describe('unwrap()', () => {
  test('returns value for Ok', () => {
    assert.strictEqual(unwrap(ok(42)), 42);
  });

  test('throws error for Err', () => {
    const error = new Error('test error');
    assert.throws(() => unwrap(err(error)), error);
  });

  test('throws the error itself for Err', () => {
    const result = err('string error');
    try {
      unwrap(result);
      assert.fail('Should have thrown');
    } catch (e) {
      assert.strictEqual(e, 'string error');
    }
  });
});

describe('unwrapOr()', () => {
  test('returns value for Ok', () => {
    assert.strictEqual(unwrapOr(ok(42), 0), 42);
  });

  test('returns default for Err', () => {
    assert.strictEqual(unwrapOr(err('error'), 0), 0);
  });
});

describe('unwrapOrElse()', () => {
  test('returns value for Ok', () => {
    assert.strictEqual(unwrapOrElse(ok(42), () => 0), 42);
  });

  test('calls function for Err', () => {
    assert.strictEqual(
      unwrapOrElse(err('error'), (e) => e.length),
      5
    );
  });
});

describe('expect()', () => {
  test('returns value for Ok', () => {
    assert.strictEqual(expect(ok(42), 'expected value'), 42);
  });

  test('throws Error with custom message for Err', () => {
    assert.throws(
      () => expect(err('original'), 'custom message'),
      { message: 'custom message' }
    );
  });
});

describe('unwrapErr()', () => {
  test('returns error for Err', () => {
    assert.strictEqual(unwrapErr(err('error')), 'error');
  });

  test('throws value for Ok', () => {
    try {
      unwrapErr(ok(42));
      assert.fail('Should have thrown');
    } catch (e) {
      assert.strictEqual(e, 42);
    }
  });
});

// ============================================================================
// Collection Tests
// ============================================================================

describe('all()', () => {
  test('returns Ok with array for all Ok results', () => {
    const result = all([ok(1), ok(2), ok(3)]);
    assert.strictEqual(result.ok, true);
    if (result.ok) assert.deepEqual(result.value, [1, 2, 3]);
  });

  test('returns first Err for mixed results', () => {
    const result = all([ok(1), err('error'), ok(3)]);
    assert.strictEqual(result.ok, false);
    if (!result.ok) assert.strictEqual(result.error, 'error');
  });

  test('returns Ok with empty array for empty input', () => {
    const result = all([]);
    assert.strictEqual(result.ok, true);
    if (result.ok) assert.deepEqual(result.value, []);
  });

  test('returns first Err when multiple Errs', () => {
    const result = all([err('first'), err('second')]);
    assert.strictEqual(result.ok, false);
    if (!result.ok) assert.strictEqual(result.error, 'first');
  });
});

describe('any()', () => {
  test('returns first Ok result', () => {
    const result = any([err('a'), ok(2), err('c')]);
    assert.strictEqual(result.ok, true);
    if (result.ok) assert.strictEqual(result.value, 2);
  });

  test('returns Err with all errors when all fail', () => {
    const result = any([err('a'), err('b'), err('c')]);
    assert.strictEqual(result.ok, false);
    if (!result.ok) assert.deepEqual(result.error, ['a', 'b', 'c']);
  });

  test('returns Err with empty array for empty input', () => {
    const result = any([]);
    assert.strictEqual(result.ok, false);
    if (!result.ok) assert.deepEqual(result.error, []);
  });
});

describe('partition()', () => {
  test('partitions mixed results', () => {
    const [oks, errs] = partition([ok(1), err('a'), ok(2), err('b')]);
    assert.deepEqual(oks, [1, 2]);
    assert.deepEqual(errs, ['a', 'b']);
  });

  test('handles all Ok results', () => {
    const [oks, errs] = partition([ok(1), ok(2), ok(3)]);
    assert.deepEqual(oks, [1, 2, 3]);
    assert.deepEqual(errs, []);
  });

  test('handles all Err results', () => {
    const [oks, errs] = partition([err('a'), err('b')]);
    assert.deepEqual(oks, []);
    assert.deepEqual(errs, ['a', 'b']);
  });

  test('handles empty array', () => {
    const [oks, errs] = partition([]);
    assert.deepEqual(oks, []);
    assert.deepEqual(errs, []);
  });
});

// ============================================================================
// Try/Catch Tests
// ============================================================================

describe('tryCatch()', () => {
  test('returns Ok for successful function', () => {
    const result = tryCatch(() => JSON.parse('{"a": 1}'));
    assert.strictEqual(result.ok, true);
    if (result.ok) assert.deepEqual(result.value, { a: 1 });
  });

  test('returns Err for throwing function', () => {
    const result = tryCatch(() => JSON.parse('invalid json'));
    assert.strictEqual(result.ok, false);
    if (!result.ok) assert(result.error instanceof SyntaxError);
  });

  test('wraps non-Error throws in Error', () => {
    const result = tryCatch(() => {
      throw 'string error';
    });
    assert.strictEqual(result.ok, false);
    if (!result.ok) {
      assert(result.error instanceof Error);
      assert.strictEqual(result.error.message, 'string error');
    }
  });
});

describe('tryCatchAsync()', async () => {
  test('returns Ok for successful async function', async () => {
    const result = await tryCatchAsync(async () => {
      return Promise.resolve(42);
    });
    assert.strictEqual(result.ok, true);
    if (result.ok) assert.strictEqual(result.value, 42);
  });

  test('returns Err for rejecting async function', async () => {
    const result = await tryCatchAsync(async () => {
      throw new Error('async error');
    });
    assert.strictEqual(result.ok, false);
    if (!result.ok) assert.strictEqual(result.error.message, 'async error');
  });
});

describe('fromPromise()', async () => {
  test('returns Ok for resolved promise', async () => {
    const result = await fromPromise(Promise.resolve(42));
    assert.strictEqual(result.ok, true);
    if (result.ok) assert.strictEqual(result.value, 42);
  });

  test('returns Err for rejected promise', async () => {
    const result = await fromPromise(Promise.reject(new Error('rejected')));
    assert.strictEqual(result.ok, false);
    if (!result.ok) assert.strictEqual(result.error.message, 'rejected');
  });
});

// ============================================================================
// Optional Interop Tests
// ============================================================================

describe('toOptional()', () => {
  test('returns value for Ok', () => {
    assert.strictEqual(toOptional(ok(42)), 42);
  });

  test('returns undefined for Err', () => {
    assert.strictEqual(toOptional(err('error')), undefined);
  });
});

describe('fromNullable()', () => {
  test('returns Ok for non-null value', () => {
    const result = fromNullable(42, 'no value');
    assert.strictEqual(result.ok, true);
    if (result.ok) assert.strictEqual(result.value, 42);
  });

  test('returns Err for null', () => {
    const result = fromNullable(null, 'no value');
    assert.strictEqual(result.ok, false);
    if (!result.ok) assert.strictEqual(result.error, 'no value');
  });

  test('returns Err for undefined', () => {
    const result = fromNullable(undefined, 'no value');
    assert.strictEqual(result.ok, false);
    if (!result.ok) assert.strictEqual(result.error, 'no value');
  });

  test('returns Ok for falsy values that are not null/undefined', () => {
    const result1 = fromNullable(0, 'error');
    const result2 = fromNullable('', 'error');
    const result3 = fromNullable(false, 'error');

    assert.strictEqual(result1.ok, true);
    assert.strictEqual(result2.ok, true);
    assert.strictEqual(result3.ok, true);
  });
});

// ============================================================================
// Tap/Inspect Tests
// ============================================================================

describe('tap()', () => {
  test('calls function for Ok', () => {
    let called = false;
    let receivedValue: number | null = null;

    const result = tap(ok(42), (v) => {
      called = true;
      receivedValue = v;
    });

    assert.strictEqual(called, true);
    assert.strictEqual(receivedValue, 42);
    assert.deepEqual(result, ok(42));
  });

  test('does not call function for Err', () => {
    let called = false;

    const result = tap(err('error'), () => {
      called = true;
    });

    assert.strictEqual(called, false);
    assert.deepEqual(result, err('error'));
  });

  test('returns original result unchanged', () => {
    const original = ok({ a: 1 });
    const result = tap(original, () => {});
    assert.strictEqual(result, original);
  });
});

describe('tapErr()', () => {
  test('calls function for Err', () => {
    let called = false;
    let receivedError: string | null = null;

    const result = tapErr(err('error'), (e) => {
      called = true;
      receivedError = e;
    });

    assert.strictEqual(called, true);
    assert.strictEqual(receivedError, 'error');
    assert.deepEqual(result, err('error'));
  });

  test('does not call function for Ok', () => {
    let called = false;

    const result = tapErr(ok(42), () => {
      called = true;
    });

    assert.strictEqual(called, false);
    assert.deepEqual(result, ok(42));
  });
});

// ============================================================================
// Type Tests (compile-time only)
// ============================================================================

describe('Type correctness', () => {
  test('Result type discriminates correctly', () => {
    const result: Result<number, string> = ok(42);

    if (result.ok) {
      // TypeScript should allow accessing value
      const _value: number = result.value;
    } else {
      // TypeScript should allow accessing error
      const _error: string = result.error;
    }

    assert.ok(true); // Just checking compilation
  });

  test('Ok and Err types work', () => {
    const okResult: Ok<number> = ok(42);
    const errResult: Err<string> = err('error');

    assert.strictEqual(okResult.value, 42);
    assert.strictEqual(errResult.error, 'error');
  });

  test('Result with custom error type', () => {
    type MyError = { code: number; message: string };
    const result: Result<string, MyError> = err({ code: 404, message: 'Not found' });

    if (!result.ok) {
      assert.strictEqual(result.error.code, 404);
    }
  });
});

// ============================================================================
// Real-World Usage Tests
// ============================================================================

describe('Real-world scenarios', () => {
  test('parsing and validation chain', () => {
    interface User {
      name: string;
      age: number;
    }

    function parseJson(json: string): Result<unknown, Error> {
      return tryCatch(() => JSON.parse(json));
    }

    function validateUser(data: unknown): Result<User, Error> {
      if (typeof data !== 'object' || data === null) {
        return err(new Error('Expected object'));
      }
      const obj = data as Record<string, unknown>;
      if (typeof obj.name !== 'string') {
        return err(new Error('Expected name to be string'));
      }
      if (typeof obj.age !== 'number') {
        return err(new Error('Expected age to be number'));
      }
      return ok({ name: obj.name, age: obj.age });
    }

    // Success case
    const validJson = '{"name": "Alice", "age": 30}';
    const validResult = andThen(parseJson(validJson), validateUser);
    assert.strictEqual(validResult.ok, true);
    if (validResult.ok) {
      assert.deepEqual(validResult.value, { name: 'Alice', age: 30 });
    }

    // Failure case - invalid JSON
    const invalidJson = 'not json';
    const invalidResult = andThen(parseJson(invalidJson), validateUser);
    assert.strictEqual(invalidResult.ok, false);

    // Failure case - invalid data
    const badData = '{"name": "Bob"}';
    const badResult = andThen(parseJson(badData), validateUser);
    assert.strictEqual(badResult.ok, false);
    if (!badResult.ok) {
      assert.strictEqual(badResult.error.message, 'Expected age to be number');
    }
  });

  test('configuration with fallbacks', () => {
    interface Config {
      port: number;
      host: string;
    }

    function readEnvConfig(): Result<Config, string> {
      // Simulate env config not available
      return err('ENV_NOT_SET');
    }

    function readFileConfig(): Result<Config, string> {
      // Simulate file config available
      return ok({ port: 3000, host: 'localhost' });
    }

    function defaultConfig(): Result<Config, never> {
      return ok({ port: 8080, host: '0.0.0.0' });
    }

    // Use env config, fallback to file, fallback to defaults
    const config = orElse(
      orElse(readEnvConfig(), readFileConfig),
      defaultConfig
    );

    assert.strictEqual(config.ok, true);
    if (config.ok) {
      assert.deepEqual(config.value, { port: 3000, host: 'localhost' });
    }
  });

  test('collecting results from multiple operations', () => {
    function fetchUser(id: number): Result<string, string> {
      if (id === 2) return err(`User ${id} not found`);
      return ok(`User${id}`);
    }

    const ids = [1, 2, 3];
    const results = ids.map(fetchUser);
    const [successes, failures] = partition(results);

    assert.deepEqual(successes, ['User1', 'User3']);
    assert.deepEqual(failures, ['User 2 not found']);
  });
});
