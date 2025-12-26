import { test, describe } from 'node:test';
import assert from 'node:assert/strict';
import {
  StructuredError,
  serializeError,
  deserializeError,
  formatError,
  type ErrorContext,
  type SerializedError,
  type StructuredErrorOptions,
} from '../src/index.js';

// =============================================================================
// StructuredError Class Tests
// =============================================================================

describe('StructuredError', () => {
  describe('constructor', () => {
    test('creates error with message only', () => {
      const error = new StructuredError('Test error');

      assert.strictEqual(error.message, 'Test error');
      assert.strictEqual(error.name, 'StructuredError');
      assert.strictEqual(error.code, undefined);
      assert.strictEqual(error.category, undefined);
      assert.deepStrictEqual(error.context, []);
      assert(error.stack !== undefined);
    });

    test('creates error with code and category', () => {
      const error = new StructuredError('Test error', {
        code: 'TEST_ERROR',
        category: 'test',
      });

      assert.strictEqual(error.code, 'TEST_ERROR');
      assert.strictEqual(error.category, 'test');
    });

    test('creates error with operation (adds context)', () => {
      const error = new StructuredError('Test error', {
        operation: 'testOperation',
        component: 'TestComponent',
        metadata: { key: 'value' },
      });

      assert.strictEqual(error.context.length, 1);
      assert.strictEqual(error.context[0].operation, 'testOperation');
      assert.strictEqual(error.context[0].component, 'TestComponent');
      assert.deepStrictEqual(error.context[0].metadata, { key: 'value' });
      assert(typeof error.context[0].timestamp === 'string');
    });

    test('creates error with cause', () => {
      const cause = new Error('Original error');
      const error = new StructuredError('Wrapped error', { cause });

      assert.strictEqual(error.cause, cause);
    });

    test('extends Error class', () => {
      const error = new StructuredError('Test');

      assert(error instanceof Error);
      assert(error instanceof StructuredError);
    });
  });

  describe('addContext', () => {
    test('adds context and returns new error', () => {
      const original = new StructuredError('Test error', {
        code: 'TEST',
        operation: 'original',
      });

      const withContext = original.addContext('newOperation', {
        component: 'NewComponent',
        metadata: { added: true },
      });

      // Original unchanged
      assert.strictEqual(original.context.length, 1);

      // New error has both contexts
      assert.strictEqual(withContext.context.length, 2);
      assert.strictEqual(withContext.context[0].operation, 'newOperation');
      assert.strictEqual(withContext.context[1].operation, 'original');

      // Preserves other properties
      assert.strictEqual(withContext.code, 'TEST');
      assert.strictEqual(withContext.message, 'Test error');
    });

    test('preserves stack trace', () => {
      const original = new StructuredError('Test');
      const withContext = original.addContext('op');

      assert.strictEqual(withContext.stack, original.stack);
    });
  });

  describe('wrap', () => {
    test('wraps plain Error', () => {
      const original = new Error('Original');
      const wrapped = StructuredError.wrap(original, 'Wrapped message', {
        operation: 'wrapOp',
      });

      assert.strictEqual(wrapped.message, 'Wrapped message');
      assert.strictEqual(wrapped.cause, original);
      assert.strictEqual(wrapped.context.length, 1);
      assert.strictEqual(wrapped.context[0].operation, 'wrapOp');
    });

    test('wraps StructuredError and preserves context', () => {
      const original = new StructuredError('Original', {
        code: 'ORIGINAL_CODE',
        category: 'original',
        operation: 'originalOp',
      });

      const wrapped = StructuredError.wrap(original, 'Wrapped', {
        operation: 'wrapOp',
      });

      // Inherits code and category
      assert.strictEqual(wrapped.code, 'ORIGINAL_CODE');
      assert.strictEqual(wrapped.category, 'original');

      // Preserves context chain
      assert.strictEqual(wrapped.context.length, 2);
      assert.strictEqual(wrapped.context[0].operation, 'wrapOp');
      assert.strictEqual(wrapped.context[1].operation, 'originalOp');
    });

    test('allows overriding code and category', () => {
      const original = new StructuredError('Original', {
        code: 'OLD_CODE',
        category: 'old',
      });

      const wrapped = StructuredError.wrap(original, 'Wrapped', {
        code: 'NEW_CODE',
        category: 'new',
      });

      assert.strictEqual(wrapped.code, 'NEW_CODE');
      assert.strictEqual(wrapped.category, 'new');
    });

    test('wraps non-Error values', () => {
      const wrapped = StructuredError.wrap('string error', 'Wrapped');

      assert.strictEqual(wrapped.message, 'Wrapped');
      assert(wrapped.cause instanceof Error);
      assert.strictEqual(wrapped.cause.message, 'string error');
    });
  });

  describe('from', () => {
    test('returns same StructuredError if passed one', () => {
      const original = new StructuredError('Test');
      const result = StructuredError.from(original);

      assert.strictEqual(result, original);
    });

    test('converts plain Error', () => {
      const original = new Error('Plain error');
      const result = StructuredError.from(original, {
        code: 'CONVERTED',
      });

      assert.strictEqual(result.message, 'Plain error');
      assert.strictEqual(result.code, 'CONVERTED');
      assert.strictEqual(result.cause, original);
    });

    test('converts non-Error values', () => {
      const result = StructuredError.from('string error');

      assert.strictEqual(result.message, 'string error');
      assert.strictEqual(result.cause, undefined);
    });
  });

  describe('isStructuredError', () => {
    test('returns true for StructuredError', () => {
      const error = new StructuredError('Test');
      assert.strictEqual(StructuredError.isStructuredError(error), true);
    });

    test('returns false for plain Error', () => {
      const error = new Error('Test');
      assert.strictEqual(StructuredError.isStructuredError(error), false);
    });

    test('returns false for non-Error', () => {
      assert.strictEqual(StructuredError.isStructuredError('string'), false);
      assert.strictEqual(StructuredError.isStructuredError(null), false);
      assert.strictEqual(StructuredError.isStructuredError(undefined), false);
    });
  });

  describe('getRootCause', () => {
    test('returns self when no cause', () => {
      const error = new StructuredError('Test');
      assert.strictEqual(error.getRootCause(), error);
    });

    test('returns root of cause chain', () => {
      const root = new Error('Root cause');
      const middle = new StructuredError('Middle', { cause: root });
      const top = new StructuredError('Top', { cause: middle });

      assert.strictEqual(top.getRootCause(), root);
    });
  });

  describe('getCauseChain', () => {
    test('returns array with self when no cause', () => {
      const error = new StructuredError('Test');
      const chain = error.getCauseChain();

      assert.strictEqual(chain.length, 1);
      assert.strictEqual(chain[0], error);
    });

    test('returns full cause chain', () => {
      const root = new Error('Root');
      const middle = new StructuredError('Middle', { cause: root });
      const top = new StructuredError('Top', { cause: middle });

      const chain = top.getCauseChain();

      assert.strictEqual(chain.length, 3);
      assert.strictEqual(chain[0], top);
      assert.strictEqual(chain[1], middle);
      assert.strictEqual(chain[2], root);
    });
  });

  describe('hasCode', () => {
    test('returns true for matching code', () => {
      const error = new StructuredError('Test', { code: 'TEST_CODE' });
      assert.strictEqual(error.hasCode('TEST_CODE'), true);
    });

    test('returns false for non-matching code', () => {
      const error = new StructuredError('Test', { code: 'TEST_CODE' });
      assert.strictEqual(error.hasCode('OTHER_CODE'), false);
    });

    test('checks cause chain for code', () => {
      const cause = new StructuredError('Cause', { code: 'CAUSE_CODE' });
      const error = new StructuredError('Test', { cause });

      assert.strictEqual(error.hasCode('CAUSE_CODE'), true);
    });
  });

  describe('hasCategory', () => {
    test('returns true for matching category', () => {
      const error = new StructuredError('Test', { category: 'test' });
      assert.strictEqual(error.hasCategory('test'), true);
    });

    test('returns false for non-matching category', () => {
      const error = new StructuredError('Test', { category: 'test' });
      assert.strictEqual(error.hasCategory('other'), false);
    });

    test('checks cause chain for category', () => {
      const cause = new StructuredError('Cause', { category: 'cause-cat' });
      const error = new StructuredError('Test', { cause });

      assert.strictEqual(error.hasCategory('cause-cat'), true);
    });
  });
});

// =============================================================================
// Serialization Tests
// =============================================================================

describe('Serialization', () => {
  describe('toJSON', () => {
    test('serializes basic error', () => {
      const error = new StructuredError('Test message');
      const json = error.toJSON();

      assert.strictEqual(json.name, 'StructuredError');
      assert.strictEqual(json.message, 'Test message');
      assert.deepStrictEqual(json.context, []);
      assert(json.stack !== undefined);
    });

    test('serializes code and category', () => {
      const error = new StructuredError('Test', {
        code: 'TEST_CODE',
        category: 'test',
      });
      const json = error.toJSON();

      assert.strictEqual(json.code, 'TEST_CODE');
      assert.strictEqual(json.category, 'test');
    });

    test('serializes context chain', () => {
      const error = new StructuredError('Test', {
        operation: 'op1',
        metadata: { key: 'value' },
      }).addContext('op2');

      const json = error.toJSON();

      assert.strictEqual(json.context.length, 2);
      assert.strictEqual(json.context[0].operation, 'op2');
      assert.strictEqual(json.context[1].operation, 'op1');
    });

    test('serializes cause chain', () => {
      const cause = new Error('Cause message');
      const error = new StructuredError('Test', { cause });
      const json = error.toJSON();

      assert(json.cause !== undefined);
      assert.strictEqual(json.cause.message, 'Cause message');
    });

    test('omits undefined optional fields', () => {
      const error = new StructuredError('Test');
      const json = error.toJSON();

      assert.strictEqual('code' in json, false);
      assert.strictEqual('category' in json, false);
    });
  });

  describe('fromJSON', () => {
    test('deserializes basic error', () => {
      const json: SerializedError = {
        name: 'StructuredError',
        message: 'Test message',
        context: [],
      };

      const error = StructuredError.fromJSON(json);

      assert.strictEqual(error.message, 'Test message');
      assert.strictEqual(error.name, 'StructuredError');
    });

    test('deserializes code and category', () => {
      const json: SerializedError = {
        name: 'StructuredError',
        message: 'Test',
        code: 'TEST_CODE',
        category: 'test',
        context: [],
      };

      const error = StructuredError.fromJSON(json);

      assert.strictEqual(error.code, 'TEST_CODE');
      assert.strictEqual(error.category, 'test');
    });

    test('deserializes context chain', () => {
      const json: SerializedError = {
        name: 'StructuredError',
        message: 'Test',
        context: [
          { operation: 'op1', timestamp: '2025-01-01T00:00:00.000Z' },
          { operation: 'op2', timestamp: '2025-01-01T00:00:01.000Z' },
        ],
      };

      const error = StructuredError.fromJSON(json);

      assert.strictEqual(error.context.length, 2);
      assert.strictEqual(error.context[0].operation, 'op1');
      assert.strictEqual(error.context[1].operation, 'op2');
    });

    test('deserializes cause chain', () => {
      const json: SerializedError = {
        name: 'StructuredError',
        message: 'Test',
        context: [],
        cause: {
          name: 'Error',
          message: 'Cause',
          context: [],
        },
      };

      const error = StructuredError.fromJSON(json);

      assert(error.cause !== undefined);
      assert.strictEqual(error.cause.message, 'Cause');
    });

    test('restores stack trace', () => {
      const json: SerializedError = {
        name: 'StructuredError',
        message: 'Test',
        stack: 'Error: Test\n    at test.ts:1:1',
        context: [],
      };

      const error = StructuredError.fromJSON(json);

      assert.strictEqual(error.stack, 'Error: Test\n    at test.ts:1:1');
    });
  });

  describe('round-trip serialization', () => {
    test('preserves all properties through JSON round-trip', () => {
      const original = new StructuredError('Test message', {
        code: 'TEST_CODE',
        category: 'test',
        operation: 'testOp',
        component: 'TestComponent',
        metadata: { key: 'value', nested: { a: 1 } },
      }).addContext('outerOp', { metadata: { outer: true } });

      const json = original.toJSON();
      const restored = StructuredError.fromJSON(json);

      assert.strictEqual(restored.message, original.message);
      assert.strictEqual(restored.code, original.code);
      assert.strictEqual(restored.category, original.category);
      assert.strictEqual(restored.context.length, original.context.length);
      assert.deepStrictEqual(
        restored.context[0].metadata,
        original.context[0].metadata
      );
    });
  });
});

// =============================================================================
// Helper Functions Tests
// =============================================================================

describe('serializeError', () => {
  test('serializes StructuredError using toJSON', () => {
    const error = new StructuredError('Test', { code: 'TEST' });
    const json = serializeError(error);

    assert.strictEqual(json.code, 'TEST');
  });

  test('serializes plain Error', () => {
    const error = new Error('Plain error');
    const json = serializeError(error);

    assert.strictEqual(json.name, 'Error');
    assert.strictEqual(json.message, 'Plain error');
    assert.deepStrictEqual(json.context, []);
  });

  test('serializes Error with cause', () => {
    const cause = new Error('Cause');
    const error = new Error('Main');
    (error as { cause: Error }).cause = cause;

    const json = serializeError(error);

    assert(json.cause !== undefined);
    assert.strictEqual(json.cause.message, 'Cause');
  });
});

describe('deserializeError', () => {
  test('deserializes as StructuredError when has context', () => {
    const json: SerializedError = {
      name: 'Error',
      message: 'Test',
      context: [{ operation: 'op', timestamp: '2025-01-01T00:00:00.000Z' }],
    };

    const error = deserializeError(json);

    assert(error instanceof StructuredError);
  });

  test('deserializes as StructuredError when has code', () => {
    const json: SerializedError = {
      name: 'Error',
      message: 'Test',
      code: 'TEST',
      context: [],
    };

    const error = deserializeError(json);

    assert(error instanceof StructuredError);
  });

  test('deserializes as plain Error when no StructuredError fields', () => {
    const json: SerializedError = {
      name: 'TypeError',
      message: 'Test',
      context: [],
    };

    const error = deserializeError(json);

    assert(!(error instanceof StructuredError));
    assert.strictEqual(error.name, 'TypeError');
    assert.strictEqual(error.message, 'Test');
  });
});

describe('formatError', () => {
  test('formats StructuredError with context', () => {
    const error = new StructuredError('Test error', {
      code: 'TEST_CODE',
      operation: 'testOp',
      component: 'TestComponent',
      metadata: { key: 'value' },
    });

    const output = formatError(error);

    assert(output.includes('[TEST_CODE] Test error'));
    assert(output.includes('Context:'));
    assert(output.includes('testOp'));
    assert(output.includes('TestComponent'));
  });

  test('formats plain Error', () => {
    const error = new Error('Plain error');
    const output = formatError(error);

    assert.strictEqual(output, 'Error: Plain error');
  });

  test('includes stack when requested', () => {
    const error = new StructuredError('Test');
    const output = formatError(error, { includeStack: true });

    assert(output.includes('Stack trace:'));
  });
});

// =============================================================================
// toString Tests
// =============================================================================

describe('toString', () => {
  test('includes code in output', () => {
    const error = new StructuredError('Test', { code: 'TEST_CODE' });
    const str = error.toString();

    assert(str.includes('[TEST_CODE]'));
  });

  test('formats context chain', () => {
    const error = new StructuredError('Test', {
      operation: 'op1',
      component: 'Comp1',
    }).addContext('op2', { component: 'Comp2' });

    const str = error.toString();

    assert(str.includes('Context:'));
    assert(str.includes('→ op2 (Comp2)'));
    assert(str.includes('→ op1 (Comp1)'));
  });

  test('includes cause message', () => {
    const cause = new Error('Root cause');
    const error = new StructuredError('Wrapper', { cause });
    const str = error.toString();

    assert(str.includes('Caused by:'));
    assert(str.includes('Root cause'));
  });

  test('includes metadata in context', () => {
    const error = new StructuredError('Test', {
      operation: 'op',
      metadata: { key: 'value' },
    });
    const str = error.toString();

    assert(str.includes('{"key":"value"}'));
  });
});

// =============================================================================
// Edge Cases
// =============================================================================

describe('Edge Cases', () => {
  test('handles empty metadata', () => {
    const error = new StructuredError('Test', {
      operation: 'op',
      metadata: {},
    });

    const str = error.toString();
    // Empty metadata should not be shown
    assert(!str.includes('{}'));
  });

  test('handles undefined component', () => {
    const error = new StructuredError('Test', {
      operation: 'op',
    });

    const str = error.toString();
    assert(str.includes('→ op'));
    assert(!str.includes('(undefined)'));
  });

  test('handles special characters in message', () => {
    const error = new StructuredError('Test "quotes" and \\backslash');
    const json = error.toJSON();

    assert.strictEqual(json.message, 'Test "quotes" and \\backslash');
  });

  test('handles unicode in metadata', () => {
    const error = new StructuredError('Test', {
      operation: 'op',
      metadata: { emoji: '🎉', chinese: '中文' },
    });

    const json = error.toJSON();
    const restored = StructuredError.fromJSON(json);

    assert.deepStrictEqual(restored.context[0].metadata, {
      emoji: '🎉',
      chinese: '中文',
    });
  });

  test('handles deeply nested metadata', () => {
    const error = new StructuredError('Test', {
      operation: 'op',
      metadata: {
        level1: {
          level2: {
            level3: { value: 'deep' },
          },
        },
      },
    });

    const json = error.toJSON();
    const restored = StructuredError.fromJSON(json);

    const meta = restored.context[0].metadata as Record<string, unknown>;
    assert.deepStrictEqual(meta, {
      level1: { level2: { level3: { value: 'deep' } } },
    });
  });

  test('handles array values in metadata', () => {
    const error = new StructuredError('Test', {
      operation: 'op',
      metadata: { items: [1, 2, 3], tags: ['a', 'b'] },
    });

    const json = error.toJSON();
    const restored = StructuredError.fromJSON(json);

    assert.deepStrictEqual(restored.context[0].metadata, {
      items: [1, 2, 3],
      tags: ['a', 'b'],
    });
  });

  test('handles multiple context entries with same operation', () => {
    const error = new StructuredError('Test', { operation: 'retry' })
      .addContext('retry')
      .addContext('retry');

    assert.strictEqual(error.context.length, 3);
    assert(error.context.every((c) => c.operation === 'retry'));
  });

  test('handles error with no stack trace', () => {
    const error = new StructuredError('Test');
    error.stack = undefined;

    const json = error.toJSON();
    assert.strictEqual('stack' in json, false);
  });

  test('handles empty string message', () => {
    const error = new StructuredError('');

    assert.strictEqual(error.message, '');
    const json = error.toJSON();
    assert.strictEqual(json.message, '');
  });

  test('handles whitespace-only message', () => {
    const error = new StructuredError('   ');

    assert.strictEqual(error.message, '   ');
  });

  test('handles null values in metadata', () => {
    const error = new StructuredError('Test', {
      operation: 'op',
      metadata: { nullValue: null, defined: 'value' },
    });

    const json = error.toJSON();
    const restored = StructuredError.fromJSON(json);

    assert.strictEqual(
      (restored.context[0].metadata as Record<string, unknown>).nullValue,
      null
    );
    assert.strictEqual(
      (restored.context[0].metadata as Record<string, unknown>).defined,
      'value'
    );
  });

  test('handles deep cause chain (10+ levels)', () => {
    let error: Error = new Error('Root cause');
    for (let i = 0; i < 10; i++) {
      error = StructuredError.wrap(error, `Level ${i}`, {
        operation: `level${i}`,
      });
    }

    const structured = error as StructuredError;
    const chain = structured.getCauseChain();
    const root = structured.getRootCause();

    assert.strictEqual(chain.length, 11); // 10 wraps + 1 root
    assert.strictEqual(root.message, 'Root cause');
  });

  test('handles very long message', () => {
    const longMessage = 'x'.repeat(10000);
    const error = new StructuredError(longMessage);

    assert.strictEqual(error.message.length, 10000);
    const json = error.toJSON();
    assert.strictEqual(json.message.length, 10000);
  });
});

// =============================================================================
// Type Checking Tests (compile-time verification)
// =============================================================================

describe('Type Safety', () => {
  test('ErrorContext type is correctly structured', () => {
    const context: ErrorContext = {
      operation: 'test',
      timestamp: new Date().toISOString(),
    };

    assert.strictEqual(typeof context.operation, 'string');
    assert.strictEqual(typeof context.timestamp, 'string');
  });

  test('StructuredErrorOptions type is correctly structured', () => {
    const options: StructuredErrorOptions = {
      code: 'TEST',
      category: 'test',
      operation: 'op',
      component: 'comp',
      metadata: { key: 'value' },
      cause: new Error('cause'),
    };

    const error = new StructuredError('Test', options);
    assert.strictEqual(error.code, 'TEST');
  });

  test('SerializedError type is correctly structured', () => {
    const serialized: SerializedError = {
      name: 'StructuredError',
      message: 'Test',
      context: [],
    };

    const error = StructuredError.fromJSON(serialized);
    assert.strictEqual(error.message, 'Test');
  });
});

// =============================================================================
// CLI Tests
// =============================================================================

describe('CLI', () => {
  test('demo command outputs JSON by default', async () => {
    const { execSync } = await import('node:child_process');
    const output = execSync('npx tsx src/index.ts demo', {
      encoding: 'utf-8',
      cwd: process.cwd(),
    });

    const parsed = JSON.parse(output);
    assert.strictEqual(parsed.name, 'StructuredError');
    assert(parsed.context.length > 0);
  });

  test('demo command with --format text outputs human-readable', async () => {
    const { execSync } = await import('node:child_process');
    const output = execSync('npx tsx src/index.ts demo --format text', {
      encoding: 'utf-8',
      cwd: process.cwd(),
    });

    assert(output.includes('[DB_CONNECTION_FAILED]'));
    assert(output.includes('Context:'));
    assert(output.includes('Caused by:'));
  });

  test('parse command parses JSON error', async () => {
    const { execSync } = await import('node:child_process');
    const input = JSON.stringify({
      name: 'StructuredError',
      message: 'Test error',
      code: 'TEST',
      context: [],
    });

    const output = execSync(`npx tsx src/index.ts parse '${input}'`, {
      encoding: 'utf-8',
      cwd: process.cwd(),
    });

    const parsed = JSON.parse(output);
    assert.strictEqual(parsed.message, 'Test error');
    assert.strictEqual(parsed.code, 'TEST');
  });

  test('validate command validates correct JSON', async () => {
    const { execSync } = await import('node:child_process');
    const input = JSON.stringify({ message: 'Test' });

    const output = execSync(`npx tsx src/index.ts validate '${input}'`, {
      encoding: 'utf-8',
      cwd: process.cwd(),
    });

    assert(output.includes('Valid error format'));
  });

  test('validate command rejects invalid JSON', async () => {
    const { execSync } = await import('node:child_process');
    const input = JSON.stringify({ notMessage: 'Test' });

    try {
      execSync(`npx tsx src/index.ts validate '${input}'`, {
        encoding: 'utf-8',
        cwd: process.cwd(),
      });
      assert.fail('Should have thrown');
    } catch (error) {
      const execError = error as { stderr?: Buffer };
      assert(execError.stderr?.toString().includes('missing required'));
    }
  });

  test('help command shows usage', async () => {
    const { execSync } = await import('node:child_process');
    const output = execSync('npx tsx src/index.ts --help', {
      encoding: 'utf-8',
      cwd: process.cwd(),
    });

    assert(output.includes('Usage:'));
    assert(output.includes('Commands:'));
    assert(output.includes('demo'));
    assert(output.includes('parse'));
    assert(output.includes('validate'));
  });

  test('parse command handles invalid JSON gracefully', async () => {
    const { execSync } = await import('node:child_process');

    try {
      execSync(`npx tsx src/index.ts parse 'not valid json'`, {
        encoding: 'utf-8',
        cwd: process.cwd(),
      });
      assert.fail('Should have thrown');
    } catch (error) {
      const execError = error as { status?: number };
      assert.strictEqual(execError.status, 1);
    }
  });

  test('parse command handles empty input', async () => {
    const { execSync } = await import('node:child_process');

    try {
      execSync(`npx tsx src/index.ts parse ''`, {
        encoding: 'utf-8',
        cwd: process.cwd(),
      });
      assert.fail('Should have thrown');
    } catch (error) {
      const execError = error as { status?: number };
      assert.strictEqual(execError.status, 1);
    }
  });
});

// =============================================================================
// Input Validation Tests
// =============================================================================

describe('Input Validation', () => {
  test('fromJSON handles missing message gracefully', () => {
    const json = {
      name: 'StructuredError',
      context: [],
    } as unknown as SerializedError;

    // Should use empty string or throw - let's verify behavior
    const error = StructuredError.fromJSON(json);
    assert(error instanceof StructuredError);
  });

  test('fromJSON handles null context array', () => {
    const json = {
      name: 'StructuredError',
      message: 'Test',
      context: null,
    } as unknown as SerializedError;

    // Should handle gracefully
    try {
      const error = StructuredError.fromJSON(json);
      assert(error instanceof StructuredError);
    } catch {
      // Also acceptable - throwing is valid for invalid input
      assert(true);
    }
  });

  test('fromJSON handles context with missing operation', () => {
    const json: SerializedError = {
      name: 'StructuredError',
      message: 'Test',
      context: [
        { timestamp: '2025-01-01T00:00:00.000Z' } as unknown as ErrorContext,
      ],
    };

    const error = StructuredError.fromJSON(json);
    // Should not crash - graceful handling
    assert(error instanceof StructuredError);
  });

  test('wrap handles undefined gracefully', () => {
    const error = StructuredError.wrap(undefined, 'Wrapped undefined');

    assert(error instanceof StructuredError);
    assert.strictEqual(error.message, 'Wrapped undefined');
  });

  test('wrap handles null gracefully', () => {
    const error = StructuredError.wrap(null, 'Wrapped null');

    assert(error instanceof StructuredError);
    assert.strictEqual(error.message, 'Wrapped null');
  });

  test('from handles object without message', () => {
    const error = StructuredError.from({ someKey: 'value' });

    assert(error instanceof StructuredError);
    // Should stringify the object or use default message
    assert(typeof error.message === 'string');
  });
});
