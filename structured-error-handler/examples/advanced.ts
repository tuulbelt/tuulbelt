/**
 * Advanced usage patterns for Structured Error Handler
 *
 * This example demonstrates:
 * - Context chaining through call stacks
 * - Cause chain analysis
 * - Serialization/deserialization round-trips
 * - HTTP error handling patterns
 * - Logging integration patterns
 *
 * Run this example:
 *   npx tsx examples/advanced.ts
 */

import {
  StructuredError,
  serializeError,
  deserializeError,
  formatError,
  type SerializedError,
} from '../src/index.js';

// =============================================================================
// Simulated Application Layers
// =============================================================================

/**
 * Database layer - lowest level
 */
function queryDatabase(query: string, params: Record<string, unknown>): never {
  // Simulate a database error
  throw new Error(`Connection timeout after 5000ms: ${query}`);
}

/**
 * Repository layer - wraps database errors
 */
function findUserById(userId: string): never {
  try {
    queryDatabase('SELECT * FROM users WHERE id = $1', { id: userId });
  } catch (err) {
    throw StructuredError.wrap(err, 'Failed to fetch user from database', {
      code: 'DB_QUERY_FAILED',
      category: 'database',
      operation: 'findUserById',
      component: 'UserRepository',
      metadata: { userId, table: 'users' },
    });
  }
  throw new Error('Unreachable'); // TypeScript needs this
}

/**
 * Service layer - adds business context
 */
function getUserProfile(userId: string, options: { includePreferences?: boolean } = {}): never {
  try {
    findUserById(userId);
  } catch (err) {
    if (err instanceof StructuredError) {
      throw err.addContext('getUserProfile', {
        component: 'UserService',
        metadata: { includePreferences: options.includePreferences ?? false },
      });
    }
    throw err;
  }
  throw new Error('Unreachable');
}

/**
 * Controller layer - top level
 */
function handleGetUserRequest(endpoint: string, userId: string): StructuredError {
  try {
    getUserProfile(userId, { includePreferences: true });
    throw new Error('Unreachable');
  } catch (err) {
    if (err instanceof StructuredError) {
      return err.addContext('handleGetUserRequest', {
        component: 'UserController',
        metadata: { endpoint, method: 'GET' },
      });
    }
    return StructuredError.from(err, {
      operation: 'handleGetUserRequest',
      component: 'UserController',
      metadata: { endpoint, method: 'GET' },
    });
  }
}

// =============================================================================
// HTTP Error Handling Pattern
// =============================================================================

interface HttpResponse {
  status: number;
  body: { error: string; code?: string; details?: unknown };
}

function errorToHttpResponse(error: StructuredError): HttpResponse {
  // Route errors to appropriate HTTP status codes
  if (error.hasCategory('validation')) {
    return {
      status: 400,
      body: { error: error.message, code: error.code },
    };
  }

  if (error.hasCategory('authorization')) {
    return {
      status: 403,
      body: { error: 'Access denied', code: 'FORBIDDEN' },
    };
  }

  if (error.hasCategory('not_found')) {
    return {
      status: 404,
      body: { error: error.message, code: error.code },
    };
  }

  // Default to 500 for database/internal errors
  return {
    status: 500,
    body: { error: 'Internal server error' },
  };
}

// =============================================================================
// Logging Integration Pattern
// =============================================================================

interface LogEntry {
  level: 'error' | 'warn' | 'info';
  message: string;
  error?: SerializedError;
  context: Record<string, unknown>;
}

function createLogEntry(error: StructuredError, requestId: string, userId?: string): LogEntry {
  return {
    level: 'error',
    message: error.message,
    error: error.toJSON(),
    context: {
      requestId,
      userId,
      code: error.code,
      category: error.category,
      rootCause: error.getRootCause().message,
      contextDepth: error.context.length,
    },
  };
}

// =============================================================================
// Main Examples
// =============================================================================

console.log('Structured Error Handler - Advanced Usage Patterns\n');

// Example 1: Context chaining through call stack
console.log('Example 1: Context chaining through call stack');
console.log('=' .repeat(60));

const apiError = handleGetUserRequest('/api/users/12345', '12345');

console.log('\nText format (human-readable):');
console.log(apiError.toString());

console.log('\nContext chain (most recent first):');
apiError.context.forEach((ctx, i) => {
  console.log(`  [${i}] ${ctx.operation} (${ctx.component})`);
});
console.log();

// Example 2: Cause chain analysis
console.log('Example 2: Cause chain analysis');
console.log('=' .repeat(60));

const causeChain = apiError.getCauseChain();
console.log(`\nCause chain depth: ${causeChain.length}`);

causeChain.forEach((err, i) => {
  const prefix = i === 0 ? 'Top-level' : i === causeChain.length - 1 ? 'Root cause' : `Level ${i}`;
  console.log(`  ${prefix}: ${err.message}`);
});

const rootCause = apiError.getRootCause();
console.log(`\nRoot cause message: "${rootCause.message}"`);
console.log();

// Example 3: Serialization round-trip
console.log('Example 3: Serialization round-trip');
console.log('=' .repeat(60));

const serialized = apiError.toJSON();
console.log('\nSerialized (first 500 chars):');
console.log(JSON.stringify(serialized, null, 2).slice(0, 500) + '...');

const restored = StructuredError.fromJSON(serialized);
console.log('\nRestored error message:', restored.message);
console.log('Restored error code:', restored.code);
console.log('Restored context count:', restored.context.length);
console.log('Round-trip preserves data:', restored.message === apiError.message);
console.log();

// Example 4: HTTP error handling
console.log('Example 4: HTTP error handling');
console.log('=' .repeat(60));

const httpResponse = errorToHttpResponse(apiError);
console.log(`\nHTTP Status: ${httpResponse.status}`);
console.log('Response body:', JSON.stringify(httpResponse.body, null, 2));

// Create a validation error to show different routing
const validationError = new StructuredError('Invalid email format', {
  code: 'VALIDATION_FAILED',
  category: 'validation',
  metadata: { field: 'email', value: 'not-an-email' },
});

const validationResponse = errorToHttpResponse(validationError);
console.log(`\nValidation error HTTP Status: ${validationResponse.status}`);
console.log('Response body:', JSON.stringify(validationResponse.body, null, 2));
console.log();

// Example 5: Logging integration
console.log('Example 5: Logging integration');
console.log('=' .repeat(60));

const logEntry = createLogEntry(apiError, 'req-abc123', 'user-12345');
console.log('\nStructured log entry:');
console.log(JSON.stringify(logEntry, null, 2));
console.log();

// Example 6: Serializing plain errors
console.log('Example 6: Serializing plain errors');
console.log('=' .repeat(60));

const plainError = new TypeError('Cannot read property "foo" of undefined');
const serializedPlain = serializeError(plainError);
console.log('\nPlain error serialized:');
console.log(JSON.stringify(serializedPlain, null, 2));

const deserializedPlain = deserializeError(serializedPlain);
console.log('\nDeserialized type:', deserializedPlain.constructor.name);
console.log('Message preserved:', deserializedPlain.message === plainError.message);
console.log();

// Example 7: Checking codes in cause chain
console.log('Example 7: Checking codes in cause chain');
console.log('=' .repeat(60));

console.log('\nChecking hasCode:');
console.log('  hasCode("DB_QUERY_FAILED"):', apiError.hasCode('DB_QUERY_FAILED'));
console.log('  hasCode("NOT_FOUND"):', apiError.hasCode('NOT_FOUND'));

console.log('\nChecking hasCategory:');
console.log('  hasCategory("database"):', apiError.hasCategory('database'));
console.log('  hasCategory("validation"):', apiError.hasCategory('validation'));
console.log();

// Summary
console.log('Key Patterns Demonstrated:');
console.log('=' .repeat(60));
console.log('');
console.log('1. Context Chaining: Use addContext() at each layer');
console.log('2. Error Wrapping: Use wrap() to preserve cause chain');
console.log('3. Programmatic Handling: Use hasCode()/hasCategory() for routing');
console.log('4. Serialization: Use toJSON()/fromJSON() for logging/transmission');
console.log('5. Root Cause Analysis: Use getRootCause()/getCauseChain()');
console.log('');
console.log('Done!');
