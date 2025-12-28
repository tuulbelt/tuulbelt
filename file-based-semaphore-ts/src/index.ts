#!/usr/bin/env -S npx tsx
/**
 * File-Based Semaphore (TypeScript)
 *
 * Cross-platform file-based semaphore for process synchronization in Node.js.
 * Compatible with the Rust version (same lock file format).
 */

import {
  existsSync,
  readFileSync,
  writeFileSync,
  unlinkSync,
  renameSync,
  realpathSync,
  lstatSync,
  readlinkSync,
  readdirSync,
} from 'node:fs';
import { dirname, resolve, normalize, basename } from 'node:path';
import { pid as currentPid } from 'node:process';
import { randomBytes } from 'node:crypto';

// ============================================================================
// Types and Interfaces
// ============================================================================

/**
 * Error types for semaphore operations
 */
export type SemaphoreErrorType =
  | 'ALREADY_LOCKED'
  | 'INVALID_PATH'
  | 'IO_ERROR'
  | 'NOT_LOCKED'
  | 'TIMEOUT'
  | 'PARSE_ERROR'
  | 'PERMISSION_DENIED'
  | 'PATH_TRAVERSAL';

/**
 * Semaphore error with structured information
 */
export interface SemaphoreError {
  type: SemaphoreErrorType;
  message: string;
  holderPid?: number;
  lockedSince?: number;
  cause?: Error;
}

/**
 * Result type for semaphore operations
 */
export type SemaphoreResult<T> =
  | { ok: true; value: T }
  | { ok: false; error: SemaphoreError };

/**
 * Configuration for semaphore behavior
 */
export interface SemaphoreConfig {
  /**
   * Timeout for stale lock detection in milliseconds.
   * If a lock is older than this, it can be forcibly removed.
   * Set to null to disable stale detection.
   * Default: 3600000 (1 hour)
   */
  staleTimeout?: number | null;

  /**
   * Retry interval when waiting for a lock in milliseconds.
   * Default: 100
   */
  retryInterval?: number;

  /**
   * Maximum time to wait when acquiring a lock in milliseconds.
   * Set to null to wait forever.
   * Default: null
   */
  acquireTimeout?: number | null;

  /**
   * Maximum tag length to prevent resource exhaustion.
   * Default: 10000
   */
  maxTagLength?: number;
}

/**
 * Lock file contents
 */
export interface LockInfo {
  /** Process ID that holds the lock */
  pid: number;
  /** Unix timestamp (seconds) when lock was acquired */
  timestamp: number;
  /** Optional description/tag */
  tag?: string;
}

/**
 * Default configuration values
 */
const DEFAULT_CONFIG: Required<SemaphoreConfig> = {
  staleTimeout: 3600000, // 1 hour
  retryInterval: 100,
  acquireTimeout: null,
  maxTagLength: 10000,
};

// ============================================================================
// Security Utilities
// ============================================================================

/**
 * Dangerous path patterns that could indicate path traversal attacks
 */
const DANGEROUS_PATH_PATTERNS = ['..', '\x00'];

/**
 * Validate path for security issues
 */
function validatePath(pathInput: string): SemaphoreResult<string> {
  if (!pathInput || pathInput.trim() === '') {
    return {
      ok: false,
      error: {
        type: 'INVALID_PATH',
        message: 'Path cannot be empty',
      },
    };
  }

  // Check for null bytes FIRST (before any processing)
  if (pathInput.includes('\x00')) {
    return {
      ok: false,
      error: {
        type: 'PATH_TRAVERSAL',
        message: 'Path contains null bytes',
      },
    };
  }

  // Check for path traversal patterns in the ORIGINAL input
  // This must happen BEFORE normalization which removes .. segments
  if (pathInput.includes('..')) {
    return {
      ok: false,
      error: {
        type: 'PATH_TRAVERSAL',
        message: 'Path contains dangerous pattern: parent directory traversal (..) not allowed',
      },
    };
  }

  // Normalize and resolve
  const normalized = normalize(pathInput);
  let resolved = resolve(pathInput);

  // Double-check normalized path doesn't contain suspicious patterns
  for (const pattern of DANGEROUS_PATH_PATTERNS) {
    if (normalized.includes(pattern)) {
      return {
        ok: false,
        error: {
          type: 'PATH_TRAVERSAL',
          message: `Path contains dangerous pattern: ${pattern}`,
        },
      };
    }
  }

  // Follow symlinks to their actual target
  // This ensures we know the real path we're writing to
  try {
    if (existsSync(resolved)) {
      // File/symlink exists - use realpathSync to follow all symlinks
      resolved = realpathSync(resolved);
    } else {
      // Path doesn't exist - check if it's a dangling symlink
      try {
        const stats = lstatSync(resolved);
        if (stats.isSymbolicLink()) {
          // It's a dangling symlink - read where it points and resolve that
          const linkTarget = readlinkSync(resolved);
          // Resolve relative to the symlink's directory
          const symlinkDir = dirname(resolved);
          resolved = resolve(symlinkDir, linkTarget);
        }
      } catch {
        // lstatSync failed - path truly doesn't exist, use resolved path
      }
    }
  } catch {
    // realpathSync failed - use resolved path as-is
  }

  return { ok: true, value: resolved };
}

/**
 * Sanitize tag to prevent injection attacks
 * Replaces all control characters (including newlines) with spaces
 */
function sanitizeTag(tag: string, maxLength: number): string {
  // Remove all control characters (0x00-0x1F and 0x7F)
  let sanitized = tag.replace(/[\x00-\x1F\x7F]/g, ' ');
  if (sanitized.length > maxLength) {
    sanitized = sanitized.slice(0, maxLength);
  }
  return sanitized;
}

// ============================================================================
// LockInfo Functions
// ============================================================================

/**
 * Create a new LockInfo for the current process
 */
export function createLockInfo(tag?: string): LockInfo {
  return {
    pid: currentPid,
    timestamp: Math.floor(Date.now() / 1000),
    tag,
  };
}

/**
 * Serialize LockInfo to string for writing to lock file
 * Note: Newlines in tags are sanitized to prevent injection attacks
 */
export function serializeLockInfo(
  info: LockInfo,
  maxTagLength: number = DEFAULT_CONFIG.maxTagLength
): string {
  let content = `pid=${info.pid}\ntimestamp=${info.timestamp}\n`;
  if (info.tag !== undefined) {
    const sanitizedTag = sanitizeTag(info.tag, maxTagLength);
    content += `tag=${sanitizedTag}\n`;
  }
  return content;
}

/**
 * Parse LockInfo from file contents
 */
export function parseLockInfo(content: string): SemaphoreResult<LockInfo> {
  let pid: number | undefined;
  let timestamp: number | undefined;
  let tag: string | undefined;

  const lines = content.split('\n');
  for (const line of lines) {
    const trimmed = line.trim();
    if (!trimmed) continue;

    const eqIndex = trimmed.indexOf('=');
    if (eqIndex === -1) continue;

    const key = trimmed.slice(0, eqIndex);
    const value = trimmed.slice(eqIndex + 1);

    switch (key) {
      case 'pid':
        pid = parseInt(value, 10);
        if (isNaN(pid)) pid = undefined;
        break;
      case 'timestamp':
        timestamp = parseInt(value, 10);
        if (isNaN(timestamp)) timestamp = undefined;
        break;
      case 'tag':
        tag = value;
        break;
      // Ignore unknown keys for forward compatibility
    }
  }

  if (pid === undefined) {
    return {
      ok: false,
      error: {
        type: 'PARSE_ERROR',
        message: 'Missing pid in lock file',
      },
    };
  }

  if (timestamp === undefined) {
    return {
      ok: false,
      error: {
        type: 'PARSE_ERROR',
        message: 'Missing timestamp in lock file',
      },
    };
  }

  return {
    ok: true,
    value: { pid, timestamp, tag },
  };
}

/**
 * Check if a lock is stale based on the given timeout
 */
export function isLockStale(info: LockInfo, timeoutMs: number): boolean {
  const nowSeconds = Math.floor(Date.now() / 1000);
  const ageSeconds = nowSeconds - info.timestamp;
  return ageSeconds > timeoutMs / 1000;
}

// ============================================================================
// Process Utilities
// ============================================================================

/**
 * Check if a process with the given PID is still running
 * This is a best-effort check that works on Unix-like systems
 */
export function isProcessRunning(pid: number): boolean {
  try {
    // On Unix, sending signal 0 checks if process exists without affecting it
    process.kill(pid, 0);
    return true;
  } catch {
    return false;
  }
}

// ============================================================================
// Semaphore Class
// ============================================================================

/**
 * File-based semaphore for cross-platform process coordination
 *
 * @example
 * ```typescript
 * const semaphore = new Semaphore('/tmp/my-lock.lock');
 *
 * // Try to acquire (non-blocking)
 * const result = semaphore.tryAcquire();
 * if (result.ok) {
 *   try {
 *     // Do work while holding lock
 *   } finally {
 *     semaphore.release();
 *   }
 * }
 *
 * // Or use acquire with timeout
 * const acquired = await semaphore.acquire({ timeout: 5000 });
 * ```
 */
export class Semaphore {
  private readonly path: string;
  private readonly config: Required<SemaphoreConfig>;

  /**
   * Create a new semaphore with the given lock file path
   *
   * @param lockPath - Path to the lock file
   * @param config - Optional configuration
   */
  constructor(lockPath: string, config: SemaphoreConfig = {}) {
    // Validate path
    const pathResult = validatePath(lockPath);
    if (!pathResult.ok) {
      throw new Error(pathResult.error.message);
    }

    this.path = pathResult.value;
    this.config = { ...DEFAULT_CONFIG, ...config };

    // Check parent directory exists
    const parentDir = dirname(this.path);
    if (parentDir && parentDir !== '.' && parentDir !== '/') {
      if (!existsSync(parentDir)) {
        throw new Error(`Parent directory does not exist: ${parentDir}`);
      }
    }
  }

  /**
   * Create a semaphore with default configuration
   */
  static withDefaults(lockPath: string): Semaphore {
    return new Semaphore(lockPath);
  }

  /**
   * Get the path to the lock file
   */
  getPath(): string {
    return this.path;
  }

  /**
   * Check if the lock is currently held
   */
  isLocked(): boolean {
    return existsSync(this.path);
  }

  /**
   * Get information about the current lock holder (if any)
   */
  getLockInfo(): LockInfo | null {
    if (!existsSync(this.path)) {
      return null;
    }

    try {
      const content = readFileSync(this.path, 'utf-8');
      const result = parseLockInfo(content);
      return result.ok ? result.value : null;
    } catch {
      return null;
    }
  }

  /**
   * Try to acquire the lock without blocking
   *
   * @param tag - Optional tag/description for the lock
   * @returns Result indicating success or failure
   */
  tryAcquire(tag?: string): SemaphoreResult<LockInfo> {
    return this.tryAcquireWithInfo(createLockInfo(tag));
  }

  /**
   * Try to acquire the lock with custom lock info
   */
  tryAcquireWithInfo(info: LockInfo): SemaphoreResult<LockInfo> {
    // Check for stale lock first
    if (this.config.staleTimeout !== null) {
      const existingInfo = this.getLockInfo();
      if (existingInfo && isLockStale(existingInfo, this.config.staleTimeout)) {
        // Check if the process is actually dead
        if (!isProcessRunning(existingInfo.pid)) {
          try {
            unlinkSync(this.path);
          } catch {
            // Ignore - someone else might have removed it
          }
        }
      }
    }

    // Try atomic creation using temp file + rename
    // Use cryptographic randomness for temp file names to prevent DoS
    const random = randomBytes(8).toString('hex');
    const tempPath = `${this.path}.${currentPid}.${random}.tmp`;

    try {
      // Write to temp file first
      const content = serializeLockInfo(info, this.config.maxTagLength);
      writeFileSync(tempPath, content, { flag: 'wx', mode: 0o600 });

      // Try to rename atomically
      // This will fail if target exists (on POSIX systems)
      try {
        // Check if lock file exists before rename
        if (existsSync(this.path)) {
          // Lock exists, clean up temp and fail
          unlinkSync(tempPath);
          const lockInfo = this.getLockInfo();
          return {
            ok: false,
            error: {
              type: 'ALREADY_LOCKED',
              message: 'Lock already held',
              holderPid: lockInfo?.pid,
              lockedSince: lockInfo?.timestamp,
            },
          };
        }

        renameSync(tempPath, this.path);
        return { ok: true, value: info };
      } catch (renameErr) {
        // Rename failed, clean up temp file
        try {
          unlinkSync(tempPath);
        } catch {
          // Ignore cleanup errors
        }

        // Check if it's because lock exists
        if (existsSync(this.path)) {
          const lockInfo = this.getLockInfo();
          return {
            ok: false,
            error: {
              type: 'ALREADY_LOCKED',
              message: 'Lock already held',
              holderPid: lockInfo?.pid,
              lockedSince: lockInfo?.timestamp,
            },
          };
        }

        return {
          ok: false,
          error: {
            type: 'IO_ERROR',
            message: `Failed to create lock: ${renameErr}`,
            cause: renameErr instanceof Error ? renameErr : undefined,
          },
        };
      }
    } catch (writeErr) {
      // Clean up temp file if it exists
      try {
        if (existsSync(tempPath)) {
          unlinkSync(tempPath);
        }
      } catch {
        // Ignore
      }

      return {
        ok: false,
        error: {
          type: 'IO_ERROR',
          message: `Failed to write lock file: ${writeErr}`,
          cause: writeErr instanceof Error ? writeErr : undefined,
        },
      };
    }
  }

  /**
   * Acquire the lock, waiting until it becomes available
   *
   * @param options - Optional timeout override
   * @returns Promise that resolves when lock is acquired
   */
  async acquire(options?: { timeout?: number; tag?: string }): Promise<SemaphoreResult<LockInfo>> {
    const timeout = options?.timeout ?? this.config.acquireTimeout;
    const info = createLockInfo(options?.tag);
    const startTime = Date.now();

    while (true) {
      const result = this.tryAcquireWithInfo(info);
      if (result.ok) {
        return result;
      }

      if (result.error.type !== 'ALREADY_LOCKED') {
        return result;
      }

      // Check timeout
      if (timeout !== null && Date.now() - startTime >= timeout) {
        return {
          ok: false,
          error: {
            type: 'TIMEOUT',
            message: `Timeout waiting for lock after ${timeout}ms`,
          },
        };
      }

      // Sleep and retry
      await this.sleep(this.config.retryInterval);
    }
  }

  /**
   * Acquire with a specific timeout (convenience method)
   */
  async acquireTimeout(timeoutMs: number, tag?: string): Promise<SemaphoreResult<LockInfo>> {
    return this.acquire({ timeout: timeoutMs, tag });
  }

  /**
   * Release the lock
   *
   * @param force - If true, release even if not owned by current process
   * @returns Result indicating success or failure
   */
  release(force: boolean = false): SemaphoreResult<void> {
    if (!existsSync(this.path)) {
      return {
        ok: false,
        error: {
          type: 'NOT_LOCKED',
          message: 'Lock not held',
        },
      };
    }

    // Verify ownership unless forcing
    if (!force) {
      const info = this.getLockInfo();
      if (info && info.pid !== currentPid) {
        return {
          ok: false,
          error: {
            type: 'PERMISSION_DENIED',
            message: `Lock held by different process (PID ${info.pid})`,
            holderPid: info.pid,
          },
        };
      }
    }

    try {
      unlinkSync(this.path);
      return { ok: true, value: undefined };
    } catch (err) {
      return {
        ok: false,
        error: {
          type: 'IO_ERROR',
          message: `Failed to release lock: ${err}`,
          cause: err instanceof Error ? err : undefined,
        },
      };
    }
  }

  /**
   * Force release a lock (even if held by another process)
   * Use with caution!
   */
  forceRelease(): SemaphoreResult<void> {
    return this.release(true);
  }

  /**
   * Get status information about the semaphore
   */
  status(): {
    locked: boolean;
    info: LockInfo | null;
    isStale: boolean;
    isOwnedByCurrentProcess: boolean;
  } {
    const info = this.getLockInfo();
    const locked = info !== null;
    const isStale =
      locked && this.config.staleTimeout !== null
        ? isLockStale(info!, this.config.staleTimeout)
        : false;
    const isOwnedByCurrentProcess = locked && info!.pid === currentPid;

    return {
      locked,
      info,
      isStale,
      isOwnedByCurrentProcess,
    };
  }

  /**
   * Clean stale locks and orphaned temp files
   * @returns true if a stale lock was cleaned
   */
  cleanStale(): boolean {
    let cleaned = false;

    // Clean orphaned temp files first
    try {
      const dir = dirname(this.path);
      const lockBasename = basename(this.path);
      const files = readdirSync(dir);

      for (const file of files) {
        // Match temp files: lockfile.pid.random.tmp
        if (file.startsWith(lockBasename + '.') && file.endsWith('.tmp')) {
          try {
            unlinkSync(resolve(dir, file));
            cleaned = true;
          } catch {
            // Ignore - file may be in use
          }
        }
      }
    } catch {
      // Ignore directory read errors
    }

    // Check for stale lock
    const info = this.getLockInfo();
    if (!info) return cleaned;

    if (this.config.staleTimeout !== null && isLockStale(info, this.config.staleTimeout)) {
      // Double check process is dead
      if (!isProcessRunning(info.pid)) {
        try {
          unlinkSync(this.path);
          return true;
        } catch {
          return cleaned;
        }
      }
    }

    return cleaned;
  }

  private sleep(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }
}

// ============================================================================
// CLI Implementation
// ============================================================================

interface CliOptions {
  path?: string;
  tag?: string;
  timeout?: number;
  force?: boolean;
  json?: boolean;
  verbose?: boolean;
}

function printUsage(): void {
  console.log(`Usage: semats <command> [options]

Commands:
  acquire <path>    Acquire a lock (blocks until available)
  try-acquire <path> Try to acquire a lock (non-blocking)
  release <path>    Release a lock
  status <path>     Show lock status
  clean <path>      Clean stale locks

Options:
  -t, --tag <tag>       Tag/description for the lock
  --timeout <ms>        Timeout in milliseconds (for acquire)
  -f, --force           Force operation (for release)
  -j, --json            Output in JSON format
  -v, --verbose         Verbose output
  -h, --help            Show this help message

Examples:
  semats acquire /tmp/my-lock.lock
  semats try-acquire /tmp/my-lock.lock --tag "build process"
  semats status /tmp/my-lock.lock --json
  semats release /tmp/my-lock.lock
  semats clean /tmp/my-lock.lock`);
}

function parseCliArgs(args: string[]): { command: string; options: CliOptions } {
  const options: CliOptions = {};
  let command = '';
  let i = 0;

  while (i < args.length) {
    const arg = args[i];
    if (arg === undefined) {
      i++;
      continue;
    }

    if (arg === '-h' || arg === '--help') {
      printUsage();
      globalThis.process?.exit(0);
    } else if (arg === '-t' || arg === '--tag') {
      i++;
      const nextArg = args[i];
      if (nextArg !== undefined) {
        options.tag = nextArg;
      }
    } else if (arg === '--timeout') {
      i++;
      const nextArg = args[i];
      if (nextArg !== undefined) {
        options.timeout = parseInt(nextArg, 10);
      }
    } else if (arg === '-f' || arg === '--force') {
      options.force = true;
    } else if (arg === '-j' || arg === '--json') {
      options.json = true;
    } else if (arg === '-v' || arg === '--verbose') {
      options.verbose = true;
    } else if (!arg.startsWith('-')) {
      if (!command) {
        command = arg;
      } else if (!options.path) {
        options.path = arg;
      }
    }
    i++;
  }

  return { command, options };
}

function formatOutput(data: unknown, json: boolean): string {
  if (json) {
    return JSON.stringify(data, null, 2);
  }
  if (typeof data === 'object' && data !== null) {
    return Object.entries(data)
      .map(([k, v]) => {
        if (v === null || v === undefined) {
          return `${k}: ${v}`;
        }
        if (typeof v === 'object') {
          // Format nested objects inline
          return `${k}: ${JSON.stringify(v)}`;
        }
        return `${k}: ${v}`;
      })
      .join('\n');
  }
  return String(data);
}

async function runCli(args: string[]): Promise<void> {
  const { command, options } = parseCliArgs(args);

  if (!command) {
    printUsage();
    globalThis.process?.exit(1);
    return;
  }

  if (!options.path) {
    console.error('Error: Lock path is required');
    globalThis.process?.exit(1);
    return;
  }

  try {
    const semaphore = new Semaphore(options.path);

    switch (command) {
      case 'acquire': {
        if (options.verbose) {
          console.error(`[INFO] Acquiring lock: ${options.path}`);
        }
        const result = await semaphore.acquire({
          timeout: options.timeout,
          tag: options.tag,
        });
        if (result.ok) {
          console.log(formatOutput({ status: 'acquired', ...result.value }, options.json ?? false));
        } else {
          console.error(formatOutput({ status: 'failed', ...result.error }, options.json ?? false));
          globalThis.process?.exit(1);
        }
        break;
      }

      case 'try-acquire': {
        if (options.verbose) {
          console.error(`[INFO] Trying to acquire lock: ${options.path}`);
        }
        const result = semaphore.tryAcquire(options.tag);
        if (result.ok) {
          console.log(formatOutput({ status: 'acquired', ...result.value }, options.json ?? false));
        } else {
          console.log(formatOutput({ status: 'failed', ...result.error }, options.json ?? false));
          globalThis.process?.exit(1);
        }
        break;
      }

      case 'release': {
        if (options.verbose) {
          console.error(`[INFO] Releasing lock: ${options.path}`);
        }
        const result = semaphore.release(options.force ?? false);
        if (result.ok) {
          console.log(formatOutput({ status: 'released' }, options.json ?? false));
        } else {
          console.error(formatOutput({ status: 'failed', ...result.error }, options.json ?? false));
          globalThis.process?.exit(1);
        }
        break;
      }

      case 'status': {
        const status = semaphore.status();
        console.log(formatOutput(status, options.json ?? false));
        break;
      }

      case 'clean': {
        if (options.verbose) {
          console.error(`[INFO] Cleaning stale locks: ${options.path}`);
        }
        const cleaned = semaphore.cleanStale();
        console.log(formatOutput({ cleaned }, options.json ?? false));
        break;
      }

      default:
        console.error(`Unknown command: ${command}`);
        printUsage();
        globalThis.process?.exit(1);
    }
  } catch (err) {
    console.error(`Error: ${err instanceof Error ? err.message : err}`);
    globalThis.process?.exit(1);
  }
}

// CLI entry point
const argv1 = globalThis.process?.argv?.[1];
if (argv1) {
  try {
    const realPath = realpathSync(argv1);
    if (import.meta.url === `file://${realPath}`) {
      const args = globalThis.process?.argv?.slice(2) ?? [];
      runCli(args);
    }
  } catch {
    if (import.meta.url === `file://${argv1}`) {
      const args = globalThis.process?.argv?.slice(2) ?? [];
      runCli(args);
    }
  }
}
