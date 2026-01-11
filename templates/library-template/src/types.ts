/**
 * Type definitions for @tuulbelt/library-name
 *
 * This file contains additional type definitions that may be useful
 * for consumers of the library.
 */

import type { Result, LibraryOptions } from './index.js';

// Re-export core types
export type { Result, LibraryOptions };

/**
 * A function that processes input and returns a Result.
 */
export type Processor<T, U, E = Error> = (input: T) => Result<U, E>;

/**
 * A function that validates input.
 */
export type Validator<T> = (input: unknown) => input is T;

/**
 * Extracts the success type from a Result.
 */
export type ResultValue<R> = R extends Result<infer T, unknown> ? T : never;

/**
 * Extracts the error type from a Result.
 */
export type ResultError<R> = R extends Result<unknown, infer E> ? E : never;
