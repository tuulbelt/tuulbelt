/**
 * Tool Name
 *
 * One sentence description of what this tool does.
 */

/**
 * Configuration options for the tool
 */
export interface Config {
  /** Enable verbose output */
  verbose?: boolean;
}

/**
 * Result returned by the process function
 */
export interface Result {
  /** Whether the operation succeeded */
  success: boolean;
  /** The processed data */
  data: string;
  /** Optional error message if success is false */
  error?: string;
}

/**
 * Process the input and return a result
 *
 * @param input - The input string to process
 * @param config - Optional configuration options
 * @returns The processing result
 *
 * @example
 * ```typescript
 * const result = process('hello', { verbose: true });
 * console.log(result.data); // 'HELLO'
 * ```
 *
 * @performance
 * - Pre-size arrays/Sets/Maps when final size is known: `new Array(size)`, `new Set(items)`
 * - Use streaming for large inputs instead of loading all into memory
 * - Prefer iterators over creating intermediate arrays
 * - Profile with Node's built-in profiler: `node --prof`
 */
export function process(input: string, config: Config = {}): Result {
  if (typeof input !== 'string') {
    return {
      success: false,
      data: '',
      error: 'Input must be a string',
    };
  }

  if (config.verbose) {
    console.error(`[DEBUG] Processing input: ${input}`);
  }

  // Example implementation: convert to uppercase
  const processed = input.toUpperCase();

  return {
    success: true,
    data: processed,
  };
}

/**
 * Parse command line arguments
 */
function parseArgs(args: string[]): { input: string; config: Config } {
  let input = '';
  const config: Config = {};

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];

    if (arg === '--verbose' || arg === '-v') {
      config.verbose = true;
    } else if (arg === '--help' || arg === '-h') {
      console.log(`Usage: tool-name [options] <input>

Options:
  -v, --verbose  Enable verbose output
  -h, --help     Show this help message

Examples:
  node src/index.js "hello world"
  node src/index.js --verbose "hello"`);
      process.exit(0);
    } else if (!arg.startsWith('-')) {
      input = arg;
    }
  }

  return { input, config };
}

// CLI entry point - only runs when executed directly
function main(): void {
  const args = globalThis.process?.argv?.slice(2) ?? [];
  const { input, config } = parseArgs(args);

  if (!input) {
    console.error('Error: No input provided');
    console.error('Usage: tool-name [options] <input>');
    globalThis.process?.exit(1);
    return;
  }

  const result = process(input, config);

  if (result.success) {
    console.log(JSON.stringify(result, null, 2));
  } else {
    console.error(`Error: ${result.error}`);
    globalThis.process?.exit(1);
  }
}

// Check if this module is being run directly
const argv1 = globalThis.process?.argv?.[1];
if (argv1 && import.meta.url === `file://${argv1}`) {
  main();
}
