#!/usr/bin/env -S npx tsx
/**
 * Config File Merger / `cfgmerge`
 *
 * Merge configuration from ENV variables, config files, and CLI arguments
 * with clear precedence rules and source tracking.
 */

import { readFileSync, realpathSync, existsSync } from 'node:fs';

// ============================================================================
// Types
// ============================================================================

/**
 * Source of a configuration value
 */
export type ConfigSource = 'cli' | 'env' | 'file' | 'default';

/**
 * A configuration value with its source for provenance tracking
 */
export interface ConfigValue<T = unknown> {
  /** The configuration value */
  value: T;
  /** Where this value came from */
  source: ConfigSource;
}

/**
 * Merged configuration with source tracking
 */
export type TrackedConfig = Record<string, ConfigValue>;

/**
 * Simple merged configuration (values only)
 */
export type SimpleConfig = Record<string, unknown>;

/**
 * Options for merging configuration
 */
export interface MergeOptions {
  /** Environment variables to merge (pass process.env or subset) */
  env?: Record<string, string | undefined>;
  /** Prefix to filter environment variables (e.g., 'APP_') */
  envPrefix?: string;
  /** Whether to strip the prefix from env var names */
  stripPrefix?: boolean;
  /** Whether to convert env var names to lowercase */
  lowercaseEnv?: boolean;
  /** Config file contents (parsed JSON object) */
  file?: Record<string, unknown>;
  /** CLI arguments as key=value pairs */
  cli?: Record<string, unknown>;
  /** Default values */
  defaults?: Record<string, unknown>;
  /** Whether to track sources in output */
  trackSources?: boolean;
}

/**
 * Result of merging configuration
 */
export interface MergeResult {
  /** Whether the merge succeeded */
  ok: true;
  /** The merged configuration */
  config: TrackedConfig | SimpleConfig;
}

/**
 * Error result from merging
 */
export interface MergeError {
  /** Whether the merge succeeded */
  ok: false;
  /** Error message */
  error: string;
}

export type Result = MergeResult | MergeError;

// ============================================================================
// Core Functions
// ============================================================================

/**
 * Parse environment variables with optional prefix filtering
 *
 * @param env - Environment variables object
 * @param prefix - Optional prefix to filter by
 * @param stripPrefix - Whether to remove prefix from keys
 * @param lowercase - Whether to convert keys to lowercase
 * @returns Parsed config entries
 */
export function parseEnv(
  env: Record<string, string | undefined>,
  prefix?: string,
  stripPrefix = true,
  lowercase = true
): Record<string, ConfigValue<string>> {
  const result: Record<string, ConfigValue<string>> = {};

  for (const [key, value] of Object.entries(env)) {
    if (value === undefined) continue;

    // Filter by prefix if specified
    if (prefix && !key.startsWith(prefix)) continue;

    // Determine the output key
    let outputKey = key;
    if (prefix && stripPrefix) {
      outputKey = key.slice(prefix.length);
    }
    if (lowercase) {
      outputKey = outputKey.toLowerCase();
    }

    result[outputKey] = { value, source: 'env' };
  }

  return result;
}

/**
 * Parse a JSON config file
 *
 * @param filePath - Path to the JSON file
 * @returns Parsed config or error
 */
export function parseJsonFile(
  filePath: string
): { ok: true; data: Record<string, unknown> } | { ok: false; error: string } {
  try {
    if (!existsSync(filePath)) {
      return { ok: false, error: `File not found: ${filePath}` };
    }

    const content = readFileSync(filePath, 'utf-8');
    const parsed = JSON.parse(content);

    if (typeof parsed !== 'object' || parsed === null || Array.isArray(parsed)) {
      return { ok: false, error: 'Config file must contain a JSON object' };
    }

    return { ok: true, data: parsed as Record<string, unknown> };
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);
    return { ok: false, error: `Failed to parse config file: ${message}` };
  }
}

/**
 * Parse CLI arguments in key=value format
 *
 * @param args - CLI arguments string (e.g., "port=3000,debug=true")
 * @returns Parsed config entries
 */
export function parseCliArgs(args: string): Record<string, ConfigValue> {
  const result: Record<string, ConfigValue> = {};

  if (!args.trim()) return result;

  // Split by comma, but handle quoted values
  const pairs = args.split(',').map(p => p.trim()).filter(Boolean);

  for (const pair of pairs) {
    const eqIndex = pair.indexOf('=');
    if (eqIndex === -1) continue;

    const key = pair.slice(0, eqIndex).trim();
    const rawValue = pair.slice(eqIndex + 1).trim();

    if (!key) continue;

    // Parse value type
    const value = parseValue(rawValue);
    result[key] = { value, source: 'cli' };
  }

  return result;
}

/**
 * Parse a string value into appropriate type
 */
export function parseValue(raw: string): unknown {
  // Remove quotes if present
  if ((raw.startsWith('"') && raw.endsWith('"')) ||
      (raw.startsWith("'") && raw.endsWith("'"))) {
    return raw.slice(1, -1);
  }

  // Boolean
  if (raw === 'true') return true;
  if (raw === 'false') return false;

  // Null
  if (raw === 'null') return null;

  // Number
  const num = Number(raw);
  if (!isNaN(num) && raw !== '') return num;

  // String
  return raw;
}

/**
 * Convert a plain object to tracked config entries
 */
function toTrackedConfig(
  obj: Record<string, unknown>,
  source: ConfigSource
): Record<string, ConfigValue> {
  const result: Record<string, ConfigValue> = {};

  for (const [key, value] of Object.entries(obj)) {
    result[key] = { value, source };
  }

  return result;
}

/**
 * Merge configuration from multiple sources with precedence
 *
 * Precedence (highest to lowest):
 * 1. CLI arguments
 * 2. Environment variables
 * 3. Config file values
 * 4. Default values
 *
 * @param options - Merge options specifying sources
 * @returns Merged configuration
 */
export function mergeConfig(options: MergeOptions): Result {
  const {
    env,
    envPrefix,
    stripPrefix = true,
    lowercaseEnv = true,
    file,
    cli,
    defaults,
    trackSources = false,
  } = options;

  // Build config from each source (lowest to highest precedence)
  const merged: TrackedConfig = {};

  // 1. Defaults (lowest precedence)
  if (defaults) {
    const defaultEntries = toTrackedConfig(defaults, 'default');
    Object.assign(merged, defaultEntries);
  }

  // 2. Config file
  if (file) {
    const fileEntries = toTrackedConfig(file, 'file');
    Object.assign(merged, fileEntries);
  }

  // 3. Environment variables
  if (env) {
    const envEntries = parseEnv(env, envPrefix, stripPrefix, lowercaseEnv);
    Object.assign(merged, envEntries);
  }

  // 4. CLI arguments (highest precedence)
  if (cli) {
    const cliEntries = toTrackedConfig(cli, 'cli');
    Object.assign(merged, cliEntries);
  }

  // Return with or without source tracking
  if (trackSources) {
    return { ok: true, config: merged };
  }

  // Strip source tracking for simple output
  const simple: SimpleConfig = {};
  for (const [key, entry] of Object.entries(merged)) {
    simple[key] = entry.value;
  }

  return { ok: true, config: simple };
}

/**
 * Get a typed value from merged config
 */
export function getValue<T>(
  config: TrackedConfig | SimpleConfig,
  key: string,
  defaultValue?: T
): T | undefined {
  if (key in config) {
    const entry = config[key];
    // Handle both tracked and simple config
    if (typeof entry === 'object' && entry !== null && 'value' in entry) {
      return (entry as ConfigValue).value as T;
    }
    return entry as T;
  }
  return defaultValue;
}

// ============================================================================
// CLI Interface
// ============================================================================

interface CliOptions {
  envEnabled: boolean;
  envPrefix?: string;
  stripPrefix: boolean;
  lowercaseEnv: boolean;
  configFile?: string;
  defaultsFile?: string;
  cliArgs?: string;
  trackSources: boolean;
  help: boolean;
  version: boolean;
}

function parseCliOptions(args: string[]): CliOptions {
  const options: CliOptions = {
    envEnabled: false,
    stripPrefix: true,
    lowercaseEnv: true,
    trackSources: false,
    help: false,
    version: false,
  };

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];

    switch (arg) {
      case '--env':
      case '-e':
        options.envEnabled = true;
        break;

      case '--prefix':
      case '-p':
        options.envPrefix = args[++i];
        break;

      case '--no-strip-prefix':
        options.stripPrefix = false;
        break;

      case '--no-lowercase':
        options.lowercaseEnv = false;
        break;

      case '--file':
      case '-f':
        options.configFile = args[++i];
        break;

      case '--defaults':
      case '-d':
        options.defaultsFile = args[++i];
        break;

      case '--args':
      case '-a':
        options.cliArgs = args[++i];
        break;

      case '--track-sources':
      case '-t':
        options.trackSources = true;
        break;

      case '--help':
      case '-h':
        options.help = true;
        break;

      case '--version':
      case '-V':
        options.version = true;
        break;
    }
  }

  return options;
}

function showHelp(): void {
  console.log(`cfgmerge - Merge configuration from multiple sources

USAGE:
  cfgmerge [OPTIONS]

OPTIONS:
  -e, --env              Include environment variables
  -p, --prefix PREFIX    Filter env vars by prefix (e.g., APP_)
      --no-strip-prefix  Keep prefix in output keys
      --no-lowercase     Keep original env var case
  -f, --file FILE        Load config from JSON file
  -d, --defaults FILE    Load default values from JSON file
  -a, --args ARGS        CLI arguments as key=value,key2=value2
  -t, --track-sources    Show source of each value in output
  -h, --help             Show this help message
  -V, --version          Show version

PRECEDENCE (highest to lowest):
  1. CLI arguments (--args)
  2. Environment variables (--env)
  3. Config file (--file)
  4. Default values (--defaults)

EXAMPLES:
  # Merge env vars with APP_ prefix and a config file
  cfgmerge --env --prefix APP_ --file config.json

  # Override with CLI args and track sources
  cfgmerge --file config.json --args "port=3000,debug=true" --track-sources

  # Use defaults with env overrides
  cfgmerge --defaults defaults.json --env --prefix MY_APP_

OUTPUT:
  Without --track-sources:
    {"port": 3000, "debug": true, "host": "localhost"}

  With --track-sources:
    {"port": {"value": 3000, "source": "cli"}, ...}
`);
}

function showVersion(): void {
  console.log('cfgmerge 0.1.0');
}

function main(): void {
  const args = globalThis.process?.argv?.slice(2) ?? [];
  const options = parseCliOptions(args);

  if (options.help) {
    showHelp();
    return;
  }

  if (options.version) {
    showVersion();
    return;
  }

  // Build merge options
  const mergeOptions: MergeOptions = {
    trackSources: options.trackSources,
    stripPrefix: options.stripPrefix,
    lowercaseEnv: options.lowercaseEnv,
  };

  // Load defaults file if specified
  if (options.defaultsFile) {
    const parsed = parseJsonFile(options.defaultsFile);
    if (!parsed.ok) {
      console.error(`Error loading defaults: ${parsed.error}`);
      globalThis.process?.exit(1);
      return;
    }
    mergeOptions.defaults = parsed.data;
  }

  // Load config file if specified
  if (options.configFile) {
    const parsed = parseJsonFile(options.configFile);
    if (!parsed.ok) {
      console.error(`Error loading config: ${parsed.error}`);
      globalThis.process?.exit(1);
      return;
    }
    mergeOptions.file = parsed.data;
  }

  // Include environment if enabled
  if (options.envEnabled) {
    mergeOptions.env = globalThis.process?.env ?? {};
    mergeOptions.envPrefix = options.envPrefix;
  }

  // Parse CLI args if provided
  if (options.cliArgs) {
    const parsed = parseCliArgs(options.cliArgs);
    // Convert to plain object for cli option
    const cli: Record<string, unknown> = {};
    for (const [key, entry] of Object.entries(parsed)) {
      cli[key] = entry.value;
    }
    mergeOptions.cli = cli;
  }

  // Perform merge
  const result = mergeConfig(mergeOptions);

  if (!result.ok) {
    console.error(`Error: ${result.error}`);
    globalThis.process?.exit(1);
    return;
  }

  // Output merged config
  console.log(JSON.stringify(result.config, null, 2));
}

// Check if this module is being run directly
const argv1 = globalThis.process?.argv?.[1];
if (argv1) {
  try {
    const realPath = realpathSync(argv1);
    if (import.meta.url === `file://${realPath}`) {
      main();
    }
  } catch {
    if (import.meta.url === `file://${argv1}`) {
      main();
    }
  }
}
