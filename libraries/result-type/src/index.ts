/**
 * @tuulbelt/result-type
 *
 * Rust-style Result<T, E> for TypeScript with zero dependencies.
 * A type-safe alternative to exceptions for representing success or failure.
 *
 * @packageDocumentation
 */

// ============================================================================
// Core Types
// ============================================================================

/**
 * A Result represents the outcome of an operation that may succeed or fail.
 * Use this instead of throwing exceptions for expected failures.
 *
 * @typeParam T - The success value type
 * @typeParam E - The error type (defaults to Error)
 *
 * @example
 * ```typescript
 * function divide(a: number, b: number): Result<number, string> {
 *   if (b === 0) return err('Division by zero');
 *   return ok(a / b);
 * }
 *
 * const result = divide(10, 2);
 * if (result.ok) {
 *   console.log(result.value); // 5
 * }
 * ```
 */
export type Result<T, E = Error> =
  | { readonly ok: true; readonly value: T }
  | { readonly ok: false; readonly error: E };

/**
 * The Ok variant of a Result containing a success value.
 */
export type Ok<T> = { readonly ok: true; readonly value: T };

/**
 * The Err variant of a Result containing an error.
 */
export type Err<E> = { readonly ok: false; readonly error: E };

// ============================================================================
// Type Utilities
// ============================================================================

/**
 * Extracts the success type T from a Result<T, E>.
 */
export type ResultValue<R> = R extends Result<infer T, unknown> ? T : never;

/**
 * Extracts the error type E from a Result<T, E>.
 */
export type ResultError<R> = R extends Result<unknown, infer E> ? E : never;

/**
 * A function that produces a Result.
 */
export type ResultFn<T, E = Error> = () => Result<T, E>;

/**
 * An async function that produces a Result.
 */
export type AsyncResultFn<T, E = Error> = () => Promise<Result<T, E>>;

// ============================================================================
// Constructors
// ============================================================================

/**
 * Creates a successful Result containing the given value.
 *
 * @param value - The success value
 * @returns An Ok Result
 *
 * @example
 * ```typescript
 * const result = ok(42);
 * console.log(result.value); // 42
 * ```
 */
export function ok<T>(value: T): Ok<T> {
  return { ok: true, value };
}

/**
 * Creates a failed Result containing the given error.
 *
 * @param error - The error value
 * @returns An Err Result
 *
 * @example
 * ```typescript
 * const result = err(new Error('Something went wrong'));
 * console.log(result.error.message); // 'Something went wrong'
 * ```
 */
export function err<E>(error: E): Err<E> {
  return { ok: false, error };
}

// ============================================================================
// Type Guards
// ============================================================================

/**
 * Checks if a Result is Ok.
 *
 * @example
 * ```typescript
 * if (isOk(result)) {
 *   console.log(result.value); // TypeScript knows value exists
 * }
 * ```
 */
export function isOk<T, E>(result: Result<T, E>): result is Ok<T> {
  return result.ok;
}

/**
 * Checks if a Result is Err.
 *
 * @example
 * ```typescript
 * if (isErr(result)) {
 *   console.log(result.error); // TypeScript knows error exists
 * }
 * ```
 */
export function isErr<T, E>(result: Result<T, E>): result is Err<E> {
  return !result.ok;
}

// ============================================================================
// Transformation Functions
// ============================================================================

/**
 * Maps a Result<T, E> to Result<U, E> by applying a function to the success value.
 * Leaves Err values unchanged.
 *
 * @param result - The Result to transform
 * @param fn - The function to apply to the success value
 * @returns A new Result with the transformed value
 *
 * @example
 * ```typescript
 * const result = ok(5);
 * const doubled = map(result, x => x * 2);
 * // { ok: true, value: 10 }
 * ```
 */
export function map<T, U, E>(
  result: Result<T, E>,
  fn: (value: T) => U
): Result<U, E> {
  if (result.ok) {
    return ok(fn(result.value));
  }
  return result;
}

/**
 * Maps a Result<T, E> to Result<T, F> by applying a function to the error value.
 * Leaves Ok values unchanged.
 *
 * @param result - The Result to transform
 * @param fn - The function to apply to the error value
 * @returns A new Result with the transformed error
 *
 * @example
 * ```typescript
 * const result = err('not found');
 * const wrapped = mapErr(result, msg => new Error(msg));
 * // { ok: false, error: Error('not found') }
 * ```
 */
export function mapErr<T, E, F>(
  result: Result<T, E>,
  fn: (error: E) => F
): Result<T, F> {
  if (!result.ok) {
    return err(fn(result.error));
  }
  return result;
}

/**
 * Chains Result-returning functions. If the Result is Ok, applies the function
 * and returns its result. If Err, returns the Err unchanged.
 *
 * This is similar to Rust's and_then or flatMap in other languages.
 *
 * @param result - The Result to chain from
 * @param fn - A function that takes the success value and returns a new Result
 * @returns The Result from fn, or the original Err
 *
 * @example
 * ```typescript
 * function parseNumber(s: string): Result<number, string> {
 *   const n = parseInt(s, 10);
 *   return isNaN(n) ? err('Not a number') : ok(n);
 * }
 *
 * function double(n: number): Result<number, string> {
 *   return ok(n * 2);
 * }
 *
 * const result = andThen(parseNumber('5'), double);
 * // { ok: true, value: 10 }
 * ```
 */
export function andThen<T, U, E>(
  result: Result<T, E>,
  fn: (value: T) => Result<U, E>
): Result<U, E> {
  if (result.ok) {
    return fn(result.value);
  }
  return result;
}

/**
 * Alias for andThen. Chains Result-returning functions.
 */
export const flatMap = andThen;

/**
 * Returns the result if Ok, otherwise returns the provided alternative Result.
 *
 * @param result - The Result to check
 * @param alternative - The Result to return if Err
 * @returns The original Result if Ok, otherwise the alternative
 *
 * @example
 * ```typescript
 * const result = err('failed');
 * const fallback = or(result, ok(0));
 * // { ok: true, value: 0 }
 * ```
 */
export function or<T, E, F>(
  result: Result<T, E>,
  alternative: Result<T, F>
): Result<T, F> {
  if (result.ok) {
    return result;
  }
  return alternative;
}

/**
 * Returns the result if Ok, otherwise calls the function to produce an alternative.
 *
 * @param result - The Result to check
 * @param fn - A function to call if Err, receiving the error
 * @returns The original Result if Ok, otherwise the result of fn
 *
 * @example
 * ```typescript
 * const result = err('failed');
 * const recovered = orElse(result, (e) => ok(0));
 * // { ok: true, value: 0 }
 * ```
 */
export function orElse<T, E, F>(
  result: Result<T, E>,
  fn: (error: E) => Result<T, F>
): Result<T, F> {
  if (result.ok) {
    return result;
  }
  return fn(result.error);
}

// ============================================================================
// Pattern Matching
// ============================================================================

/**
 * Pattern matches on a Result, calling the appropriate handler.
 *
 * @param result - The Result to match on
 * @param handlers - Object with ok and err handler functions
 * @returns The return value of the called handler
 *
 * @example
 * ```typescript
 * const result = ok(42);
 * const message = match(result, {
 *   ok: (value) => `Got ${value}`,
 *   err: (error) => `Failed: ${error}`
 * });
 * // "Got 42"
 * ```
 */
export function match<T, E, U>(
  result: Result<T, E>,
  handlers: {
    ok: (value: T) => U;
    err: (error: E) => U;
  }
): U {
  if (result.ok) {
    return handlers.ok(result.value);
  }
  return handlers.err(result.error);
}

// ============================================================================
// Unwrapping Functions
// ============================================================================

/**
 * Returns the contained Ok value. Throws if the Result is Err.
 *
 * @param result - The Result to unwrap
 * @returns The success value
 * @throws The error value if Result is Err
 *
 * @example
 * ```typescript
 * const value = unwrap(ok(42)); // 42
 * unwrap(err('oops')); // throws 'oops'
 * ```
 */
export function unwrap<T, E>(result: Result<T, E>): T {
  if (result.ok) {
    return result.value;
  }
  throw result.error;
}

/**
 * Returns the contained Ok value or a default.
 *
 * @param result - The Result to unwrap
 * @param defaultValue - The value to return if Err
 * @returns The success value or the default
 *
 * @example
 * ```typescript
 * unwrapOr(ok(42), 0); // 42
 * unwrapOr(err('oops'), 0); // 0
 * ```
 */
export function unwrapOr<T, E>(result: Result<T, E>, defaultValue: T): T {
  if (result.ok) {
    return result.value;
  }
  return defaultValue;
}

/**
 * Returns the contained Ok value or computes it from the error.
 *
 * @param result - The Result to unwrap
 * @param fn - A function to compute the default from the error
 * @returns The success value or the computed default
 *
 * @example
 * ```typescript
 * unwrapOrElse(ok(42), () => 0); // 42
 * unwrapOrElse(err('oops'), (e) => e.length); // 4
 * ```
 */
export function unwrapOrElse<T, E>(
  result: Result<T, E>,
  fn: (error: E) => T
): T {
  if (result.ok) {
    return result.value;
  }
  return fn(result.error);
}

/**
 * Returns the contained Ok value. Throws with a custom message if Err.
 *
 * @param result - The Result to unwrap
 * @param message - The error message to throw
 * @returns The success value
 * @throws Error with the provided message if Result is Err
 *
 * @example
 * ```typescript
 * const value = expect(ok(42), 'Expected a value'); // 42
 * expect(err('oops'), 'Expected a value'); // throws Error('Expected a value')
 * ```
 */
export function expect<T, E>(result: Result<T, E>, message: string): T {
  if (result.ok) {
    return result.value;
  }
  throw new Error(message);
}

/**
 * Returns the contained Err value. Throws if the Result is Ok.
 *
 * @param result - The Result to unwrap
 * @returns The error value
 * @throws The success value if Result is Ok
 *
 * @example
 * ```typescript
 * unwrapErr(err('oops')); // 'oops'
 * unwrapErr(ok(42)); // throws 42
 * ```
 */
export function unwrapErr<T, E>(result: Result<T, E>): E {
  if (!result.ok) {
    return result.error;
  }
  throw result.value;
}

// ============================================================================
// Collection Utilities
// ============================================================================

/**
 * Collects an array of Results into a Result of an array.
 * If all Results are Ok, returns Ok with array of values.
 * If any Result is Err, returns the first Err.
 *
 * @param results - Array of Results to collect
 * @returns A Result containing an array of all values, or the first error
 *
 * @example
 * ```typescript
 * all([ok(1), ok(2), ok(3)]); // ok([1, 2, 3])
 * all([ok(1), err('fail'), ok(3)]); // err('fail')
 * ```
 */
export function all<T, E>(results: readonly Result<T, E>[]): Result<T[], E> {
  const values: T[] = [];
  for (const result of results) {
    if (!result.ok) {
      return result;
    }
    values.push(result.value);
  }
  return ok(values);
}

/**
 * Returns the first Ok Result, or an Err containing all errors if all fail.
 *
 * @param results - Array of Results to check
 * @returns The first Ok Result, or Err with array of all errors
 *
 * @example
 * ```typescript
 * any([err('a'), ok(2), err('c')]); // ok(2)
 * any([err('a'), err('b')]); // err(['a', 'b'])
 * ```
 */
export function any<T, E>(results: readonly Result<T, E>[]): Result<T, E[]> {
  const errors: E[] = [];
  for (const result of results) {
    if (result.ok) {
      return result;
    }
    errors.push(result.error);
  }
  return err(errors);
}

/**
 * Partitions an array of Results into Ok values and Err values.
 *
 * @param results - Array of Results to partition
 * @returns A tuple of [okValues, errValues]
 *
 * @example
 * ```typescript
 * const [oks, errs] = partition([ok(1), err('a'), ok(2)]);
 * // oks = [1, 2], errs = ['a']
 * ```
 */
export function partition<T, E>(
  results: readonly Result<T, E>[]
): [T[], E[]] {
  const oks: T[] = [];
  const errs: E[] = [];
  for (const result of results) {
    if (result.ok) {
      oks.push(result.value);
    } else {
      errs.push(result.error);
    }
  }
  return [oks, errs];
}

// ============================================================================
// Try/Catch Utilities
// ============================================================================

/**
 * Wraps a function that may throw into one that returns a Result.
 *
 * @param fn - The function to wrap
 * @returns A Result with the return value or caught error
 *
 * @example
 * ```typescript
 * const result = tryCatch(() => JSON.parse('{"a":1}'));
 * // ok({ a: 1 })
 *
 * const result = tryCatch(() => JSON.parse('invalid'));
 * // err(SyntaxError)
 * ```
 */
export function tryCatch<T>(fn: () => T): Result<T, Error> {
  try {
    return ok(fn());
  } catch (error) {
    if (error instanceof Error) {
      return err(error);
    }
    return err(new Error(String(error)));
  }
}

/**
 * Wraps an async function that may throw into one that returns a Promise<Result>.
 *
 * @param fn - The async function to wrap
 * @returns A Promise of a Result with the return value or caught error
 *
 * @example
 * ```typescript
 * const result = await tryCatchAsync(async () => {
 *   const response = await fetch('https://api.example.com/data');
 *   return response.json();
 * });
 * ```
 */
export async function tryCatchAsync<T>(
  fn: () => Promise<T>
): Promise<Result<T, Error>> {
  try {
    return ok(await fn());
  } catch (error) {
    if (error instanceof Error) {
      return err(error);
    }
    return err(new Error(String(error)));
  }
}

/**
 * Converts a Promise that may reject into a Promise<Result>.
 *
 * @param promise - The Promise to convert
 * @returns A Promise that always resolves to a Result
 *
 * @example
 * ```typescript
 * const result = await fromPromise(fetch('https://api.example.com'));
 * ```
 */
export async function fromPromise<T>(
  promise: Promise<T>
): Promise<Result<T, Error>> {
  try {
    return ok(await promise);
  } catch (error) {
    if (error instanceof Error) {
      return err(error);
    }
    return err(new Error(String(error)));
  }
}

// ============================================================================
// Optional Interop
// ============================================================================

/**
 * Converts a Result to an optional value (undefined if Err).
 *
 * @param result - The Result to convert
 * @returns The value if Ok, undefined if Err
 *
 * @example
 * ```typescript
 * toOptional(ok(42)); // 42
 * toOptional(err('oops')); // undefined
 * ```
 */
export function toOptional<T, E>(result: Result<T, E>): T | undefined {
  return result.ok ? result.value : undefined;
}

/**
 * Converts a nullable value to a Result.
 *
 * @param value - The value to convert
 * @param error - The error to use if value is null/undefined
 * @returns Ok with value if not null/undefined, otherwise Err
 *
 * @example
 * ```typescript
 * fromNullable(42, 'No value'); // ok(42)
 * fromNullable(null, 'No value'); // err('No value')
 * fromNullable(undefined, 'No value'); // err('No value')
 * ```
 */
export function fromNullable<T, E>(
  value: T | null | undefined,
  error: E
): Result<T, E> {
  if (value === null || value === undefined) {
    return err(error);
  }
  return ok(value);
}

// ============================================================================
// Tap/Inspect Utilities
// ============================================================================

/**
 * Calls a function with the Ok value for side effects, returns the Result unchanged.
 *
 * @param result - The Result to inspect
 * @param fn - The function to call with the value
 * @returns The original Result
 *
 * @example
 * ```typescript
 * const result = tap(ok(42), (value) => console.log('Got:', value));
 * // Logs "Got: 42", returns ok(42)
 * ```
 */
export function tap<T, E>(
  result: Result<T, E>,
  fn: (value: T) => void
): Result<T, E> {
  if (result.ok) {
    fn(result.value);
  }
  return result;
}

/**
 * Calls a function with the Err value for side effects, returns the Result unchanged.
 *
 * @param result - The Result to inspect
 * @param fn - The function to call with the error
 * @returns The original Result
 *
 * @example
 * ```typescript
 * const result = tapErr(err('oops'), (e) => console.error('Error:', e));
 * // Logs "Error: oops", returns err('oops')
 * ```
 */
export function tapErr<T, E>(
  result: Result<T, E>,
  fn: (error: E) => void
): Result<T, E> {
  if (!result.ok) {
    fn(result.error);
  }
  return result;
}
