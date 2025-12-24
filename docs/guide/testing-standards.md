# Tuulbelt Testing Standards

All tools must have automated tests. This guide ensures consistency across the tuulbelt.

## Test Framework

Use the language's native test runner:
- **Node.js/TypeScript:** `node:test`
- **Rust:** `cargo test`

No external test libraries (except TypeScript compiler).

## Test Structure

```typescript
// test/index.test.ts
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { myFunction } from '../src/index.ts';

// Group related tests
test('myFunction', async (t) => {
  await t.test('basic case', () => {
    const result = myFunction('input');
    assert.strictEqual(result, 'expected');
  });

  await t.test('edge case: empty input', () => {
    const result = myFunction('');
    assert(result !== null);
  });

  await t.test('error case: invalid input', () => {
    assert.throws(() => myFunction('bad'), TypeError);
  });
});
```

## Test Categories

### 1. Unit Tests (Core Logic)

Test individual functions in isolation:

```typescript
test('parseConfig', () => {
  const config = parseConfig('{ "key": "value" }');
  assert.strictEqual(config.key, 'value');
});
```

Minimum: 1 unit test per exported function

### 2. Integration Tests (CLI/API Behavior)

Test end-to-end behavior:

```typescript
import { execSync } from 'child_process';

test('CLI produces correct output', () => {
  const output = execSync('node src/index.js --format json').toString();
  const parsed = JSON.parse(output);
  assert(parsed.success);
});
```

Minimum: 1 integration test per major feature

### 3. Edge Case Tests

Test boundary conditions, empty input, malformed data:

```typescript
test('edge cases', () => {
  assert.doesNotThrow(() => myFunction(''));       // Empty
  assert.doesNotThrow(() => myFunction('   '));    // Whitespace
  assert.throws(() => myFunction(null), TypeError); // Invalid type
});
```

Minimum: 1 edge case test per major feature

### 4. Error Handling Tests

Ensure errors are clear and recoverable:

```typescript
test('error handling', () => {
  const error = assert.throws(
    () => myFunction('invalid'),
    Error
  );
  assert(error.message.includes('expected'));
});
```

Minimum: 1 error test per error case

## Test Naming

Be specific and descriptive:

```typescript
// Good
test('parseConfig parses valid JSON', () => {});
test('parseConfig throws on invalid JSON', () => {});
test('parseConfig handles empty string', () => {});

// Bad
test('it works', () => {});
test('test 1', () => {});
test('parseConfig', () => {}); // Vague
```

## Assertion Patterns

Use `node:assert/strict`:

```typescript
import assert from 'node:assert/strict';

// Equality
assert.strictEqual(a, b);           // ===
assert.deepEqual(obj1, obj2);       // Deep comparison

// Existence
assert(value);                       // Truthy
assert.ok(value);                    // Same as above

// Type checking
assert.throws(() => fn(), TypeError);
assert.doesNotThrow(() => fn());

// Inclusion
assert(array.includes(item));
assert(string.includes('substring'));
```

## Test Coverage Target

- **Core logic:** 80%+ coverage
- **CLI:** 70%+ coverage
- **Examples:** Don't need to be tested

Check coverage:
```bash
npm test -- --coverage  # If your runner supports it
```

For Node test runner, count tests manually or use a coverage tool as dev dependency.

## Running Tests

```bash
npm test                 # Run all tests
npm test -- --grep "pattern"  # Run specific test
npm test -- --watch     # Rerun on file change
```

## CI/CD (GitHub Actions)

Every tool repo has `.github/workflows/test.yml`:

```yaml
name: Test
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        node-version: [18, 20]
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: ${{ matrix.node-version }}
      - run: npm test
```

This runs tests on every commit and pull request.

## Test Data

Keep test data small and self-contained:

```typescript
// Good: Data in test file
test('parseCSV', () => {
  const data = 'a,b,c\n1,2,3';
  const result = parseCSV(data);
  assert.deepEqual(result, [['a', 'b', 'c'], ['1', '2', '3']]);
});

// Okay: Data in separate file if large
import testData from './fixtures/large-data.json';

test('parseLargeFile', () => {
  const result = parse(testData);
  // ...
});
```

## Performance Tests (Optional)

If your tool is performance-critical:

```typescript
test('performance: handles 10MB file', () => {
  const start = performance.now();
  const result = process(largeData);
  const elapsed = performance.now() - start;
  assert(elapsed < 1000, `Took ${elapsed}ms, should be < 1s`);
});
```

But keep them optional; don't slow down test suite.

## Debugging Tests

Add logging temporarily:

```typescript
test('debug example', () => {
  const input = 'data';
  console.log('[DEBUG] Input:', input);
  const result = process(input);
  console.log('[DEBUG] Output:', result);
  assert(result);
});
```

Then remove before committing.

## Test Review Checklist

Before pushing:
- [ ] All tests pass locally
- [ ] No `console.log()` or `debugger;`
- [ ] Tests cover happy path + edge cases + errors
- [ ] Test names are descriptive
- [ ] Test data is minimal and clear
- [ ] No hardcoded paths or system-specific values
- [ ] CI passes on GitHub
