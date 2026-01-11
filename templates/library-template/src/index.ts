/**
 * @tuulbelt/library-name
 *
 * One sentence description of what this library does.
 *
 * @packageDocumentation
 */

// ============================================================================
// Types
// ============================================================================

/**
 * Result type for operations that can fail.
 * Use this instead of throwing exceptions.
 */
export type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E };

/**
 * Configuration options for the library.
 */
export interface LibraryOptions {
  /**
   * Enable debug mode for verbose logging.
   * @default false
   */
  debug?: boolean;
}

// ============================================================================
// Core API
// ============================================================================

/**
 * Main entry point for the library.
 *
 * @example
 * ```typescript
 * import { process } from '@tuulbelt/library-name';
 *
 * const result = process('input');
 * if (result.ok) {
 *   console.log(result.value);
 * }
 * ```
 *
 * @param input - The input to process
 * @param options - Optional configuration
 * @returns A Result containing the processed output or an error
 */
export function process(
  input: string,
  options: LibraryOptions = {}
): Result<string> {
  if (!input) {
    return { ok: false, error: new Error('Input is required') };
  }

  if (options.debug) {
    console.log('[DEBUG] Processing input:', input);
  }

  // TODO: Implement actual logic
  const output = input.toUpperCase();

  return { ok: true, value: output };
}

/**
 * Validates input before processing.
 *
 * @param input - The input to validate
 * @returns true if valid, false otherwise
 */
export function validate(input: unknown): input is string {
  return typeof input === 'string' && input.length > 0;
}

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Creates a successful result.
 */
export function ok<T>(value: T): Result<T, never> {
  return { ok: true, value };
}

/**
 * Creates a failed result.
 */
export function err<E>(error: E): Result<never, E> {
  return { ok: false, error };
}
