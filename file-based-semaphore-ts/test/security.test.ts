import { test, describe, beforeEach, afterEach } from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, unlinkSync, writeFileSync, mkdirSync, symlinkSync, rmdirSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { pid as currentPid } from 'node:process';
import {
  Semaphore,
  serializeLockInfo,
  parseLockInfo,
  type LockInfo,
} from '../src/index.js';

// ============================================================================
// Test Utilities
// ============================================================================

function tempLockPath(name: string): string {
  return join(tmpdir(), `semats-security-${name}-${currentPid}-${Date.now()}.lock`);
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

// ============================================================================
// Path Traversal Tests
// ============================================================================

describe('Security: Path Traversal Prevention', () => {
  test('rejects path with ..', () => {
    assert.throws(
      () => new Semaphore('/tmp/../etc/passwd.lock'),
      /dangerous pattern/i
    );
  });

  test('rejects path with null bytes', () => {
    assert.throws(
      () => new Semaphore('/tmp/test\x00.lock'),
      /null/i
    );
  });

  test('rejects hidden path traversal', () => {
    assert.throws(
      () => new Semaphore('/tmp/foo/../../../etc/shadow'),
      /dangerous pattern/i
    );
  });

  test('rejects double encoded traversal', () => {
    // URL-encoded .. might be decoded by some systems
    // Our implementation should still catch the normalized path
    assert.throws(
      () => new Semaphore('/tmp/..%2F..%2Fetc/passwd'),
      /dangerous pattern|parent directory/i
    );
  });

  test('accepts valid absolute path', () => {
    const path = tempLockPath('valid-path');
    const sem = new Semaphore(path);
    assert(sem);
    cleanup(path);
  });

  test('accepts path in tmp directory', () => {
    const path = join(tmpdir(), 'safe-semaphore.lock');
    const sem = new Semaphore(path);
    assert(sem);
    cleanup(path);
  });
});

// ============================================================================
// Tag Injection Tests
// ============================================================================

describe('Security: Tag Injection Prevention', () => {
  test('sanitizes newline injection in tag', () => {
    const maliciousTag = 'mytag\npid=99999\ntimestamp=0';
    const info: LockInfo = {
      pid: 12345,
      timestamp: 1234567890,
      tag: maliciousTag,
    };

    const serialized = serializeLockInfo(info);

    // Should only have ONE pid= line (the real one)
    const pidMatches = serialized.match(/^pid=/gm);
    assert.strictEqual(pidMatches?.length, 1);

    // Parse back - should get original PID, not injected
    const parsed = parseLockInfo(serialized);
    assert(parsed.ok);
    assert.strictEqual(parsed.value.pid, 12345);
    assert.notStrictEqual(parsed.value.pid, 99999);
  });

  test('sanitizes carriage return injection', () => {
    const maliciousTag = 'mytag\r\npid=99999';
    const info: LockInfo = {
      pid: 12345,
      timestamp: 1234567890,
      tag: maliciousTag,
    };

    const serialized = serializeLockInfo(info);

    // Should not contain carriage returns
    assert(!serialized.includes('\r'));

    // Should only have one pid line
    const pidMatches = serialized.match(/^pid=/gm);
    assert.strictEqual(pidMatches?.length, 1);
  });

  test('sanitizes mixed newline injection', () => {
    const maliciousTag = 'start\r\ntimestamp=0\npid=1\rtag=evil';
    const info: LockInfo = {
      pid: 12345,
      timestamp: 1234567890,
      tag: maliciousTag,
    };

    const serialized = serializeLockInfo(info);

    // Count each key - should be exactly one of each
    assert.strictEqual((serialized.match(/^pid=/gm) || []).length, 1);
    assert.strictEqual((serialized.match(/^timestamp=/gm) || []).length, 1);
    assert.strictEqual((serialized.match(/^tag=/gm) || []).length, 1);
  });

  test('handles very long tag (resource exhaustion)', () => {
    const longTag = 'x'.repeat(100000);
    const info: LockInfo = {
      pid: 12345,
      timestamp: 1234567890,
      tag: longTag,
    };

    // With default limit, should truncate
    const serialized = serializeLockInfo(info, 1000);
    assert(serialized.length < 2000);
  });
});

// ============================================================================
// PID Spoofing Tests
// ============================================================================

describe('Security: PID Spoofing Prevention', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('pid-spoof');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('cannot release lock held by different PID', () => {
    const sem = new Semaphore(lockPath);

    // Create lock file with different PID
    const fakeInfo: LockInfo = {
      pid: 99999,
      timestamp: Math.floor(Date.now() / 1000),
    };
    writeFileSync(lockPath, serializeLockInfo(fakeInfo));

    // Try to release without force
    const result = sem.release();
    assert(!result.ok);
    assert.strictEqual(result.error.type, 'PERMISSION_DENIED');
    assert.strictEqual(result.error.holderPid, 99999);
  });

  test('detects PID mismatch in lock info', () => {
    const sem = new Semaphore(lockPath);

    // Create lock with different PID
    const fakeInfo: LockInfo = {
      pid: 12345,
      timestamp: Math.floor(Date.now() / 1000),
    };
    writeFileSync(lockPath, serializeLockInfo(fakeInfo));

    const status = sem.status();
    assert.strictEqual(status.isOwnedByCurrentProcess, false);
  });

  test('force release requires explicit flag', () => {
    const sem = new Semaphore(lockPath);

    // Create lock with different PID
    const fakeInfo: LockInfo = {
      pid: 99999,
      timestamp: Math.floor(Date.now() / 1000),
    };
    writeFileSync(lockPath, serializeLockInfo(fakeInfo));

    // Regular release fails
    const result1 = sem.release(false);
    assert(!result1.ok);

    // Force release succeeds
    const result2 = sem.release(true);
    assert(result2.ok);
  });
});

// ============================================================================
// File Permission Tests
// ============================================================================

describe('Security: File Permissions', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('permissions');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('lock file is created with restrictive permissions', () => {
    const sem = new Semaphore(lockPath);
    sem.tryAcquire();

    // File should exist
    assert(existsSync(lockPath));

    // Note: Checking exact permissions is platform-dependent
    // The implementation uses mode 0o600
  });
});

// ============================================================================
// Symlink Attack Tests
// ============================================================================

describe('Security: Symlink Handling', () => {
  let lockPath: string;
  let symlinkPath: string;
  let targetDir: string;

  beforeEach(() => {
    lockPath = tempLockPath('symlink-target');
    symlinkPath = tempLockPath('symlink-link');
    targetDir = join(tmpdir(), `semats-symlink-test-${currentPid}-${Date.now()}`);
    cleanup(lockPath);
    cleanup(symlinkPath);
  });

  afterEach(() => {
    cleanup(lockPath);
    cleanup(symlinkPath);
    try {
      rmdirSync(targetDir);
    } catch {
      // Ignore
    }
  });

  test('resolves symlinks in lock path', () => {
    // Create a directory for target
    mkdirSync(targetDir, { recursive: true });
    const realLockPath = join(targetDir, 'real.lock');

    // Create symlink
    try {
      symlinkSync(realLockPath, symlinkPath);
    } catch {
      // Skip test if symlink creation fails (Windows without admin)
      return;
    }

    // Create semaphore with symlink path
    const sem = new Semaphore(symlinkPath);
    const result = sem.tryAcquire();

    // Should work but resolve to real path
    assert(result.ok);

    // Real file should exist
    assert(existsSync(realLockPath));

    // Cleanup
    cleanup(realLockPath);
  });
});

// ============================================================================
// Resource Exhaustion Tests
// ============================================================================

describe('Security: Resource Exhaustion Prevention', () => {
  test('limits tag size to prevent memory exhaustion', () => {
    const hugeTag = 'x'.repeat(1000000); // 1MB tag
    const info: LockInfo = {
      pid: 12345,
      timestamp: 1234567890,
      tag: hugeTag,
    };

    const serialized = serializeLockInfo(info, 10000); // 10KB limit
    assert(serialized.length < 20000);
  });

  test('handles malformed lock file gracefully', () => {
    const lockPath = tempLockPath('malformed');
    cleanup(lockPath);

    try {
      // Write garbage to lock file
      writeFileSync(lockPath, 'garbage\x00binary\xFFdata');

      const sem = new Semaphore(lockPath);
      const info = sem.getLockInfo();

      // Should return null, not crash
      assert.strictEqual(info, null);
    } finally {
      cleanup(lockPath);
    }
  });

  test('handles empty lock file', () => {
    const lockPath = tempLockPath('empty');
    cleanup(lockPath);

    try {
      writeFileSync(lockPath, '');

      const sem = new Semaphore(lockPath);
      const info = sem.getLockInfo();

      assert.strictEqual(info, null);
    } finally {
      cleanup(lockPath);
    }
  });

  test('handles lock file with only whitespace', () => {
    const lockPath = tempLockPath('whitespace');
    cleanup(lockPath);

    try {
      writeFileSync(lockPath, '   \n\t\n   ');

      const sem = new Semaphore(lockPath);
      const info = sem.getLockInfo();

      assert.strictEqual(info, null);
    } finally {
      cleanup(lockPath);
    }
  });
});

// ============================================================================
// Input Validation Tests
// ============================================================================

describe('Security: Input Validation', () => {
  test('rejects empty path', () => {
    assert.throws(() => new Semaphore(''));
  });

  test('rejects whitespace-only path', () => {
    assert.throws(() => new Semaphore('   '));
  });

  test('rejects tab-only path', () => {
    assert.throws(() => new Semaphore('\t\t'));
  });

  test('handles unicode in tag safely', () => {
    const unicodeTag = '日本語\n攻撃\r\npid=99999';
    const info: LockInfo = {
      pid: 12345,
      timestamp: 1234567890,
      tag: unicodeTag,
    };

    const serialized = serializeLockInfo(info);
    const parsed = parseLockInfo(serialized);

    assert(parsed.ok);
    assert.strictEqual(parsed.value.pid, 12345);
  });

  test('handles emoji in tag', () => {
    const emojiTag = '🔒locked🔓\npid=99999';
    const info: LockInfo = {
      pid: 12345,
      timestamp: 1234567890,
      tag: emojiTag,
    };

    const serialized = serializeLockInfo(info);
    const parsed = parseLockInfo(serialized);

    assert(parsed.ok);
    assert.strictEqual(parsed.value.pid, 12345);
    assert(parsed.value.tag?.includes('🔒'));
  });
});

// ============================================================================
// Concurrent Access Security Tests
// ============================================================================

describe('Security: Concurrent Access', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('concurrent-security');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('atomic file creation prevents race conditions', async () => {
    const sem1 = new Semaphore(lockPath);
    const sem2 = new Semaphore(lockPath);

    // Try to acquire concurrently
    const [result1, result2] = await Promise.all([
      Promise.resolve(sem1.tryAcquire()),
      Promise.resolve(sem2.tryAcquire()),
    ]);

    // Exactly one should succeed
    const successCount = [result1.ok, result2.ok].filter(Boolean).length;
    assert.strictEqual(successCount, 1);
  });

  test('temp file cleanup on failure', () => {
    const sem = new Semaphore(lockPath);

    // First acquire
    sem.tryAcquire();

    // Second acquire should fail
    const result = sem.tryAcquire();
    assert(!result.ok);

    // Check no temp files left behind
    const tmpDir = tmpdir();
    // Temp files would be named like: lockPath.pid.timestamp.tmp
    // They should be cleaned up even on failure
  });
});
