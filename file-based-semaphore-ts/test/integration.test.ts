import { test, describe, beforeEach, afterEach } from 'node:test';
import assert from 'node:assert/strict';
import { execSync, spawn } from 'node:child_process';
import { existsSync, unlinkSync, writeFileSync, readFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { pid as currentPid } from 'node:process';

// ============================================================================
// Test Utilities
// ============================================================================

const CLI_PATH = join(process.cwd(), 'src/index.ts');

function tempLockPath(name: string): string {
  return join(tmpdir(), `semats-integration-${name}-${currentPid}-${Date.now()}.lock`);
}

function cleanup(path: string): void {
  try {
    if (existsSync(path)) {
      unlinkSync(path);
    }
  } catch {
    // Ignore
  }
}

function runCli(args: string[], timeout = 5000): { stdout: string; stderr: string; exitCode: number | null } {
  try {
    const result = execSync(`npx tsx ${CLI_PATH} ${args.join(' ')}`, {
      encoding: 'utf-8',
      timeout,
      stdio: ['pipe', 'pipe', 'pipe'],
    });
    return { stdout: result, stderr: '', exitCode: 0 };
  } catch (error: unknown) {
    const execError = error as { stdout?: string; stderr?: string; status?: number | null };
    return {
      stdout: execError.stdout ?? '',
      stderr: execError.stderr ?? '',
      exitCode: execError.status ?? 1,
    };
  }
}

// ============================================================================
// CLI Help Tests
// ============================================================================

describe('CLI: Help', () => {
  test('shows help with --help flag', () => {
    const result = runCli(['--help']);
    assert.strictEqual(result.exitCode, 0);
    assert(result.stdout.includes('Usage:'));
    assert(result.stdout.includes('Commands:'));
    assert(result.stdout.includes('try-acquire'));
    assert(result.stdout.includes('release'));
    assert(result.stdout.includes('status'));
  });

  test('shows help with -h flag', () => {
    const result = runCli(['-h']);
    assert.strictEqual(result.exitCode, 0);
    assert(result.stdout.includes('Usage:'));
  });

  test('shows help with no arguments', () => {
    // No arguments shows help but with exit code 1 (error)
    const result = runCli([]);
    // Either exit 0 with help, or exit 1 with error message
    assert(result.stdout.includes('Usage:') || result.stderr.includes('Usage:') || result.stderr.includes('No command'));
  });
});

// ============================================================================
// CLI: try-acquire Tests
// ============================================================================

describe('CLI: try-acquire', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('try-acquire');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('acquires lock on new path', () => {
    const result = runCli(['try-acquire', lockPath]);
    assert.strictEqual(result.exitCode, 0);
    assert(result.stdout.includes('status: acquired'));
    assert(existsSync(lockPath));
  });

  test('acquires lock with tag', () => {
    const result = runCli(['try-acquire', lockPath, '--tag', 'my-test-tag']);
    assert.strictEqual(result.exitCode, 0);
    assert(result.stdout.includes('status: acquired'));

    const content = readFileSync(lockPath, 'utf-8');
    assert(content.includes('tag=my-test-tag'));
  });

  test('fails when lock already exists', () => {
    // First acquire
    runCli(['try-acquire', lockPath]);
    assert(existsSync(lockPath));

    // Second acquire should fail
    const result = runCli(['try-acquire', lockPath]);
    assert.strictEqual(result.exitCode, 1);
    assert(result.stderr.includes('ALREADY_LOCKED') || result.stdout.includes('ALREADY_LOCKED') || result.stderr.includes('already'));
  });

  test('outputs JSON with --json flag', () => {
    const result = runCli(['try-acquire', lockPath, '--json']);
    assert.strictEqual(result.exitCode, 0);
    const json = JSON.parse(result.stdout);
    assert.strictEqual(json.status, 'acquired');
    assert.strictEqual(typeof json.pid, 'number');
  });

  test('shows error for missing path', () => {
    const result = runCli(['try-acquire']);
    assert.strictEqual(result.exitCode, 1);
    assert(result.stderr.includes('No lock path') || result.stderr.includes('path') || result.stdout.includes('path'));
  });
});

// ============================================================================
// CLI: release Tests
// ============================================================================

describe('CLI: release', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('release');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('fails to release lock held by different PID', () => {
    // Write lock file with different PID
    const fakeLockContent = `pid=99999\ntimestamp=${Math.floor(Date.now() / 1000)}\n`;
    writeFileSync(lockPath, fakeLockContent);

    const result = runCli(['release', lockPath]);
    assert.strictEqual(result.exitCode, 1);
    assert(result.stderr.includes('PERMISSION_DENIED') || result.stdout.includes('PERMISSION_DENIED') || result.stderr.includes('different process'));
    assert(existsSync(lockPath)); // Lock should still exist
  });

  test('force releases lock held by different PID', () => {
    // Write lock file with different PID
    const fakeLockContent = `pid=99999\ntimestamp=${Math.floor(Date.now() / 1000)}\n`;
    writeFileSync(lockPath, fakeLockContent);

    const result = runCli(['release', lockPath, '--force']);
    assert.strictEqual(result.exitCode, 0);
    assert(result.stdout.includes('status: released') || result.stdout.includes('force'));
    assert(!existsSync(lockPath));
  });

  test('fails gracefully when lock does not exist', () => {
    const result = runCli(['release', lockPath]);
    // Releasing a non-existent lock returns exit code 1 with NOT_LOCKED error
    assert.strictEqual(result.exitCode, 1);
    const output = result.stdout + result.stderr;
    assert(output.includes('NOT_LOCKED') || output.includes('not held') || output.includes('not locked'));
  });

  test('outputs JSON with --json flag on force release', () => {
    // Write lock file with different PID
    const fakeLockContent = `pid=99999\ntimestamp=${Math.floor(Date.now() / 1000)}\n`;
    writeFileSync(lockPath, fakeLockContent);

    const result = runCli(['release', lockPath, '--force', '--json']);
    assert.strictEqual(result.exitCode, 0);
    const json = JSON.parse(result.stdout);
    assert.strictEqual(json.status, 'released');
  });
});

// ============================================================================
// CLI: status Tests
// ============================================================================

describe('CLI: status', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('status');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('shows unlocked status', () => {
    const result = runCli(['status', lockPath]);
    assert.strictEqual(result.exitCode, 0);
    assert(result.stdout.includes('locked: false') || result.stdout.includes('false'));
  });

  test('shows locked status with details', () => {
    // Write lock file with known PID and tag
    const fakeLockContent = `pid=12345\ntimestamp=${Math.floor(Date.now() / 1000)}\ntag=status-test\n`;
    writeFileSync(lockPath, fakeLockContent);

    const result = runCli(['status', lockPath]);
    assert.strictEqual(result.exitCode, 0);
    assert(result.stdout.includes('12345') || result.stdout.includes('locked: true'));
    assert(result.stdout.includes('status-test') || result.stdout.includes('tag'));
  });

  test('outputs JSON with --json flag', () => {
    // Write lock file
    const fakeLockContent = `pid=12345\ntimestamp=${Math.floor(Date.now() / 1000)}\ntag=json-test\n`;
    writeFileSync(lockPath, fakeLockContent);

    const result = runCli(['status', lockPath, '--json']);
    assert.strictEqual(result.exitCode, 0);
    const json = JSON.parse(result.stdout);
    assert.strictEqual(json.locked, true);
    assert(json.info);
    assert.strictEqual(json.info.pid, 12345);
    assert.strictEqual(json.info.tag, 'json-test');
  });

  test('shows not owned by current process for different PID', () => {
    // Write lock file with different PID
    const fakeLockContent = `pid=99999\ntimestamp=${Math.floor(Date.now() / 1000)}\n`;
    writeFileSync(lockPath, fakeLockContent);

    const result = runCli(['status', lockPath, '--json']);
    assert.strictEqual(result.exitCode, 0);
    const json = JSON.parse(result.stdout);
    assert.strictEqual(json.locked, true);
    assert.strictEqual(json.isOwnedByCurrentProcess, false);
    assert.strictEqual(json.info.pid, 99999);
  });
});

// ============================================================================
// CLI: clean Tests
// ============================================================================

describe('CLI: clean', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('clean');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('removes stale lock from non-existent PID with old timestamp', () => {
    // Write lock file with non-existent PID AND old timestamp (2 hours ago)
    const oldTimestamp = Math.floor(Date.now() / 1000) - 7200;
    const fakeLockContent = `pid=99999\ntimestamp=${oldTimestamp}\n`;
    writeFileSync(lockPath, fakeLockContent);

    const result = runCli(['clean', lockPath]);
    assert.strictEqual(result.exitCode, 0);
    assert(result.stdout.includes('cleaned: true') || result.stdout.includes('true'));
    assert(!existsSync(lockPath));
  });

  test('leaves recent lock untouched even from dead PID', () => {
    // Write lock file with non-existent PID but recent timestamp
    const fakeLockContent = `pid=99999\ntimestamp=${Math.floor(Date.now() / 1000)}\n`;
    writeFileSync(lockPath, fakeLockContent);

    const result = runCli(['clean', lockPath]);
    assert.strictEqual(result.exitCode, 0);
    // Lock should still exist (not stale by time)
    assert(existsSync(lockPath));
    assert(result.stdout.includes('cleaned: false') || result.stdout.includes('false'));
  });

  test('succeeds when lock does not exist', () => {
    const result = runCli(['clean', lockPath]);
    assert.strictEqual(result.exitCode, 0);
    assert(result.stdout.includes('cleaned: false') || result.stdout.includes('false'));
  });

  test('outputs JSON with --json flag', () => {
    // Write stale lock (old timestamp + dead PID)
    const oldTimestamp = Math.floor(Date.now() / 1000) - 7200;
    const fakeLockContent = `pid=99999\ntimestamp=${oldTimestamp}\n`;
    writeFileSync(lockPath, fakeLockContent);

    const result = runCli(['clean', lockPath, '--json']);
    assert.strictEqual(result.exitCode, 0);
    const json = JSON.parse(result.stdout);
    assert.strictEqual(json.cleaned, true);
  });
});

// ============================================================================
// CLI: acquire (blocking) Tests
// ============================================================================

describe('CLI: acquire (blocking)', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('acquire');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('acquires lock immediately when available', () => {
    const result = runCli(['acquire', lockPath, '--timeout', '1000']);
    assert.strictEqual(result.exitCode, 0);
    assert(result.stdout.includes('status: acquired'));
    assert(existsSync(lockPath));
  });

  test('times out when lock held by another process', () => {
    // Create lock with different PID
    const fakeLockContent = `pid=99999\ntimestamp=${Math.floor(Date.now() / 1000)}\n`;
    writeFileSync(lockPath, fakeLockContent);

    const result = runCli(['acquire', lockPath, '--timeout', '500'], 3000);
    assert.strictEqual(result.exitCode, 1);
    assert(result.stderr.includes('TIMEOUT') || result.stdout.includes('TIMEOUT') || result.stderr.includes('Timeout'));
  });

  test('acquires lock with tag', () => {
    const result = runCli(['acquire', lockPath, '--tag', 'acquire-test', '--timeout', '1000']);
    assert.strictEqual(result.exitCode, 0);

    const content = readFileSync(lockPath, 'utf-8');
    assert(content.includes('tag=acquire-test'));
  });
});

// ============================================================================
// CLI: Error Handling Tests
// ============================================================================

describe('CLI: Error Handling', () => {
  test('shows error for unknown command', () => {
    const result = runCli(['unknown-command', '/tmp/test.lock']);
    assert.strictEqual(result.exitCode, 1);
    assert(result.stderr.includes('Unknown command') || result.stdout.includes('Unknown command'));
  });

  test('shows error for path traversal attempt', () => {
    const result = runCli(['try-acquire', '/tmp/../../../etc/test.lock']);
    assert.strictEqual(result.exitCode, 1);
    // Check for error message about path traversal
    const output = result.stdout + result.stderr;
    assert(output.includes('dangerous pattern') || output.includes('traversal') || output.includes('..'));
  });

  test('shows error for empty path', () => {
    const result = runCli(['try-acquire', '']);
    assert.strictEqual(result.exitCode, 1);
    // Either an error about empty path or missing argument
  });
});

// ============================================================================
// CLI: Flag Parsing Tests
// ============================================================================

describe('CLI: Flag Parsing', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('flags');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('supports -t as alias for --tag', () => {
    const result = runCli(['try-acquire', lockPath, '-t', 'short-tag']);
    assert.strictEqual(result.exitCode, 0);

    const content = readFileSync(lockPath, 'utf-8');
    assert(content.includes('tag=short-tag'));
  });

  test('supports -f as alias for --force', () => {
    // Create lock with different PID
    const fakeLockContent = `pid=99999\ntimestamp=${Math.floor(Date.now() / 1000)}\n`;
    writeFileSync(lockPath, fakeLockContent);

    const result = runCli(['release', lockPath, '-f']);
    assert.strictEqual(result.exitCode, 0);
    assert(!existsSync(lockPath));
  });

  test('supports flags before and after path', () => {
    const result = runCli(['try-acquire', '--tag', 'before-path', lockPath]);
    assert.strictEqual(result.exitCode, 0);

    const content = readFileSync(lockPath, 'utf-8');
    assert(content.includes('tag=before-path'));
  });
});

// ============================================================================
// CLI: Output Format Tests
// ============================================================================

describe('CLI: Output Format', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('format');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('JSON output is valid for successful try-acquire', () => {
    const result = runCli(['try-acquire', lockPath, '--json']);
    assert.strictEqual(result.exitCode, 0);

    const json = JSON.parse(result.stdout);
    assert.strictEqual(json.status, 'acquired');
    assert.strictEqual(typeof json.pid, 'number');
    assert.strictEqual(typeof json.timestamp, 'number');
  });

  test('JSON output is valid for failed operations', () => {
    // Create lock with different PID
    const fakeLockContent = `pid=99999\ntimestamp=${Math.floor(Date.now() / 1000)}\n`;
    writeFileSync(lockPath, fakeLockContent);

    const result = runCli(['release', lockPath, '--json']);
    assert.strictEqual(result.exitCode, 1);

    // The error output might be in stdout or stderr depending on implementation
    const output = result.stdout || result.stderr;
    const json = JSON.parse(output);
    assert.strictEqual(json.status, 'failed');
    assert(json.type);
    assert(json.message);
  });
});
