/**
 * Structured Error Handler
 *
 * Structured error format with context preservation and serialization.
 * Enables rich error information to flow through call stacks without losing context.
 *
 * @module structured-error-handler
 * @version 0.1.0
 */

// =============================================================================
// Core Types
// =============================================================================

/**
 * Error context - metadata about the operation that failed
 */
export interface ErrorContext {
  /** The operation that was being performed */
  operation: string;
  /** Optional component or module name */
  component?: string;
  /** Additional metadata (must be JSON-serializable) */
  metadata?: Record<string, unknown>;
  /** Timestamp when the error occurred */
  timestamp: string;
}

/**
 * Serialized error format for JSON output
 */
export interface SerializedError {
  /** Error name/type */
  name: string;
  /** Error message */
  message: string;
  /** Error code for programmatic handling */
  code?: string;
  /** Error category for grouping */
  category?: string;
  /** Stack trace */
  stack?: string;
  /** Context chain (most recent first) */
  context: ErrorContext[];
  /** Cause chain (serialized) */
  cause?: SerializedError;
}

/**
 * Options for creating a StructuredError
 */
export interface StructuredErrorOptions {
  /** Error code for programmatic handling (e.g., 'ENOENT', 'VALIDATION_FAILED') */
  code?: string;
  /** Error category for grouping (e.g., 'io', 'validation', 'network') */
  category?: string;
  /** The operation being performed when error occurred */
  operation?: string;
  /** Component or module name */
  component?: string;
  /** Additional metadata */
  metadata?: Record<string, unknown>;
  /** The underlying cause (another error) */
  cause?: Error;
}

// =============================================================================
// StructuredError Class
// =============================================================================

/**
 * A structured error with context preservation and serialization support.
 *
 * @example
 * ```typescript
 * // Create a new structured error
 * const error = new StructuredError('File not found', {
 *   code: 'ENOENT',
 *   category: 'io',
 *   operation: 'readConfig',
 *   metadata: { path: '/etc/config.json' }
 * });
 *
 * // Wrap an existing error with context
 * try {
 *   await readFile(path);
 * } catch (err) {
 *   throw StructuredError.wrap(err, 'Failed to load configuration', {
 *     operation: 'loadConfig',
 *     metadata: { path }
 *   });
 * }
 *
 * // Serialize for logging
 * console.log(JSON.stringify(error.toJSON(), null, 2));
 * ```
 */
export class StructuredError extends Error {
  /** Error code for programmatic handling */
  readonly code?: string;

  /** Error category for grouping */
  readonly category?: string;

  /** Context chain (most recent first) */
  readonly context: ErrorContext[];

  /** The underlying cause */
  override readonly cause?: Error;

  /**
   * Create a new StructuredError
   */
  constructor(message: string, options: StructuredErrorOptions = {}) {
    super(message);
    this.name = 'StructuredError';
    this.code = options.code;
    this.category = options.category;
    this.cause = options.cause;

    // Initialize context chain
    this.context = [];

    // Add initial context if operation is provided
    if (options.operation) {
      this.context.push({
        operation: options.operation,
        component: options.component,
        metadata: options.metadata,
        timestamp: new Date().toISOString(),
      });
    }

    // Preserve stack trace
    if (Error.captureStackTrace) {
      Error.captureStackTrace(this, StructuredError);
    }
  }

  /**
   * Add context to the error (returns a new StructuredError)
   */
  addContext(
    operation: string,
    options: { component?: string; metadata?: Record<string, unknown> } = {}
  ): StructuredError {
    const newError = new StructuredError(this.message, {
      code: this.code,
      category: this.category,
      cause: this.cause,
    });

    // Copy existing context
    newError.context.push(...this.context);

    // Add new context at the beginning (most recent first)
    newError.context.unshift({
      operation,
      component: options.component,
      metadata: options.metadata,
      timestamp: new Date().toISOString(),
    });

    // Preserve original stack
    newError.stack = this.stack;

    return newError;
  }

  /**
   * Wrap an existing error with structured context
   */
  static wrap(
    error: unknown,
    message: string,
    options: StructuredErrorOptions = {}
  ): StructuredError {
    const cause = error instanceof Error ? error : new Error(String(error));

    // If wrapping a StructuredError, preserve its context
    if (error instanceof StructuredError) {
      const wrapped = new StructuredError(message, {
        ...options,
        cause,
        code: options.code ?? error.code,
        category: options.category ?? error.category,
      });

      // Copy the original context chain
      wrapped.context.push(...error.context);

      return wrapped;
    }

    return new StructuredError(message, {
      ...options,
      cause,
    });
  }

  /**
   * Create a StructuredError from a plain error
   */
  static from(error: unknown, options: StructuredErrorOptions = {}): StructuredError {
    if (error instanceof StructuredError) {
      return error;
    }

    const message = error instanceof Error ? error.message : String(error);
    const cause = error instanceof Error ? error : undefined;

    return new StructuredError(message, {
      ...options,
      cause,
    });
  }

  /**
   * Serialize the error to a JSON-compatible object
   */
  toJSON(): SerializedError {
    const serialized: SerializedError = {
      name: this.name,
      message: this.message,
      context: this.context,
    };

    if (this.code) {
      serialized.code = this.code;
    }

    if (this.category) {
      serialized.category = this.category;
    }

    if (this.stack) {
      serialized.stack = this.stack;
    }

    if (this.cause) {
      serialized.cause = serializeError(this.cause);
    }

    return serialized;
  }

  /**
   * Convert to a formatted string representation
   */
  override toString(): string {
    const parts: string[] = [];

    // Header
    if (this.code) {
      parts.push(`[${this.code}] ${this.message}`);
    } else {
      parts.push(this.message);
    }

    // Context chain
    if (this.context.length > 0) {
      parts.push('');
      parts.push('Context:');
      for (const ctx of this.context) {
        let line = `  → ${ctx.operation}`;
        if (ctx.component) {
          line += ` (${ctx.component})`;
        }
        if (ctx.metadata && Object.keys(ctx.metadata).length > 0) {
          line += ` ${JSON.stringify(ctx.metadata)}`;
        }
        parts.push(line);
      }
    }

    // Cause chain
    if (this.cause) {
      parts.push('');
      parts.push('Caused by:');
      parts.push(`  ${this.cause.message}`);
    }

    return parts.join('\n');
  }

  /**
   * Create a StructuredError from a serialized representation
   */
  static fromJSON(json: SerializedError): StructuredError {
    const error = new StructuredError(json.message, {
      code: json.code,
      category: json.category,
    });

    // Restore context
    error.context.push(...json.context);

    // Restore cause chain
    if (json.cause) {
      (error as { cause: Error }).cause = deserializeError(json.cause);
    }

    // Restore stack
    if (json.stack) {
      error.stack = json.stack;
    }

    return error;
  }

  /**
   * Check if an error is a StructuredError
   */
  static isStructuredError(error: unknown): error is StructuredError {
    return error instanceof StructuredError;
  }

  /**
   * Get the root cause of the error chain
   */
  getRootCause(): Error {
    let current: Error = this;
    while (current.cause instanceof Error) {
      current = current.cause;
    }
    return current;
  }

  /**
   * Get all errors in the cause chain
   */
  getCauseChain(): Error[] {
    const chain: Error[] = [this];
    let current: Error = this;
    while (current.cause instanceof Error) {
      chain.push(current.cause);
      current = current.cause;
    }
    return chain;
  }

  /**
   * Check if the error or any cause matches a code
   */
  hasCode(code: string): boolean {
    if (this.code === code) {
      return true;
    }
    if (this.cause instanceof StructuredError) {
      return this.cause.hasCode(code);
    }
    return false;
  }

  /**
   * Check if the error or any cause matches a category
   */
  hasCategory(category: string): boolean {
    if (this.category === category) {
      return true;
    }
    if (this.cause instanceof StructuredError) {
      return this.cause.hasCategory(category);
    }
    return false;
  }
}

// =============================================================================
// Helper Functions
// =============================================================================

/**
 * Serialize any error to a JSON-compatible format
 */
export function serializeError(error: Error): SerializedError {
  if (error instanceof StructuredError) {
    return error.toJSON();
  }

  const serialized: SerializedError = {
    name: error.name,
    message: error.message,
    context: [],
  };

  if (error.stack) {
    serialized.stack = error.stack;
  }

  if (error.cause instanceof Error) {
    serialized.cause = serializeError(error.cause);
  }

  return serialized;
}

/**
 * Deserialize a SerializedError back to an Error
 */
export function deserializeError(json: SerializedError): Error {
  // If it has StructuredError-specific fields, restore as StructuredError
  if (json.context.length > 0 || json.code || json.category) {
    return StructuredError.fromJSON(json);
  }

  // Otherwise, create a plain Error
  const error = new Error(json.message);
  error.name = json.name;
  if (json.stack) {
    error.stack = json.stack;
  }
  if (json.cause) {
    (error as { cause: Error }).cause = deserializeError(json.cause);
  }
  return error;
}

/**
 * Format an error for human-readable output
 */
export function formatError(error: Error, options: { includeStack?: boolean } = {}): string {
  if (error instanceof StructuredError) {
    let output = error.toString();
    if (options.includeStack && error.stack) {
      output += '\n\nStack trace:\n' + error.stack;
    }
    return output;
  }

  let output = `${error.name}: ${error.message}`;
  if (options.includeStack && error.stack) {
    output += '\n\n' + error.stack;
  }
  return output;
}

// =============================================================================
// CLI Interface
// =============================================================================

/**
 * Parse command line arguments
 */
function parseArgs(args: string[]): {
  command: string;
  input?: string;
  format: 'json' | 'text';
  includeStack: boolean;
  help: boolean;
} {
  let command = '';
  let input: string | undefined;
  let format: 'json' | 'text' = 'json';
  let includeStack = false;
  let help = false;

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    if (!arg) continue;

    if (arg === '--help' || arg === '-h') {
      help = true;
    } else if (arg === '--format' || arg === '-f') {
      const value = args[i + 1];
      if (value === 'json' || value === 'text') {
        format = value;
        i++;
      }
    } else if (arg === '--stack' || arg === '-s') {
      includeStack = true;
    } else if (!arg.startsWith('-') && !command) {
      command = arg;
    } else if (!arg.startsWith('-')) {
      input = arg;
    }
  }

  return { command, input, format, includeStack, help };
}

/**
 * Print help message
 */
function printHelp(): void {
  console.log(`Structured Error Handler - Error format with context preservation

Usage:
  structured-error-handler <command> [options] [input]

Commands:
  parse <json>     Parse a JSON error and format it
  validate <json>  Validate JSON error format
  demo             Show a demo of structured errors

Options:
  -f, --format <format>  Output format: json, text (default: json)
  -s, --stack           Include stack traces in output
  -h, --help            Show this help message

Examples:
  # Parse and format an error
  structured-error-handler parse '{"message":"test","context":[]}'

  # Validate error format
  structured-error-handler validate '{"message":"test"}'

  # Show demo
  structured-error-handler demo --format text`);
}

/**
 * Run demo showing structured error features
 */
function runDemo(format: 'json' | 'text', includeStack: boolean): void {
  // Create a chain of errors to demonstrate context preservation
  const rootError = new Error('Connection refused');

  const dbError = StructuredError.wrap(rootError, 'Failed to connect to database', {
    code: 'DB_CONNECTION_FAILED',
    category: 'database',
    operation: 'connectToDatabase',
    component: 'DatabaseClient',
    metadata: { host: 'localhost', port: 5432 },
  });

  const userError = dbError.addContext('fetchUserProfile', {
    component: 'UserService',
    metadata: { userId: '12345' },
  });

  const apiError = userError.addContext('handleGetUser', {
    component: 'UserController',
    metadata: { endpoint: '/api/users/12345' },
  });

  if (format === 'json') {
    console.log(JSON.stringify(apiError.toJSON(), null, 2));
  } else {
    console.log(formatError(apiError, { includeStack }));
  }
}

/**
 * Main CLI entry point
 */
function main(): void {
  const args = globalThis.process?.argv?.slice(2) ?? [];
  const { command, input, format, includeStack, help } = parseArgs(args);

  if (help || !command) {
    printHelp();
    return;
  }

  switch (command) {
    case 'demo':
      runDemo(format, includeStack);
      break;

    case 'parse':
      if (!input) {
        console.error('Error: parse command requires JSON input');
        globalThis.process?.exit(1);
        return;
      }
      try {
        const parsed = JSON.parse(input) as SerializedError;
        const error = StructuredError.fromJSON(parsed);
        if (format === 'json') {
          console.log(JSON.stringify(error.toJSON(), null, 2));
        } else {
          console.log(formatError(error, { includeStack }));
        }
      } catch {
        console.error('Error: Invalid JSON input');
        globalThis.process?.exit(1);
      }
      break;

    case 'validate':
      if (!input) {
        console.error('Error: validate command requires JSON input');
        globalThis.process?.exit(1);
        return;
      }
      try {
        const parsed = JSON.parse(input) as unknown;
        if (
          typeof parsed === 'object' &&
          parsed !== null &&
          'message' in parsed &&
          typeof (parsed as Record<string, unknown>).message === 'string'
        ) {
          console.log('Valid error format');
        } else {
          console.error('Invalid error format: missing required "message" field');
          globalThis.process?.exit(1);
        }
      } catch {
        console.error('Error: Invalid JSON');
        globalThis.process?.exit(1);
      }
      break;

    default:
      console.error(`Unknown command: ${command}`);
      printHelp();
      globalThis.process?.exit(1);
  }
}

// Check if this module is being run directly
const argv1 = globalThis.process?.argv?.[1];
if (argv1 && import.meta.url === `file://${argv1}`) {
  main();
}
