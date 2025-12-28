import { test, describe, beforeEach, afterEach } from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, unlinkSync, writeFileSync, mkdirSync, rmdirSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { pid as currentPid } from 'node:process';
import {
  Semaphore,
  createLockInfo,
  serializeLockInfo,
  parseLockInfo,
  isLockStale,
  isProcessRunning,
  type LockInfo,
  type SemaphoreConfig,
} from '../src/index.js';

// ============================================================================
// Test Utilities
// ============================================================================

function tempLockPath(name: string): string {
  return join(tmpdir(), `semats-test-${name}-${currentPid}-${Date.now()}.lock`);
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
// LockInfo Tests
// ============================================================================

describe('createLockInfo', () => {
  test('creates lock info with current PID', () => {
    const info = createLockInfo();
    assert.strictEqual(info.pid, currentPid);
    assert(info.timestamp > 0);
    assert.strictEqual(info.tag, undefined);
  });

  test('creates lock info with tag', () => {
    const info = createLockInfo('my-tag');
    assert.strictEqual(info.pid, currentPid);
    assert.strictEqual(info.tag, 'my-tag');
  });

  test('creates lock info with empty tag', () => {
    const info = createLockInfo('');
    assert.strictEqual(info.tag, '');
  });
});

describe('serializeLockInfo', () => {
  test('serializes basic lock info', () => {
    const info: LockInfo = { pid: 12345, timestamp: 1234567890 };
    const serialized = serializeLockInfo(info);
    assert(serialized.includes('pid=12345'));
    assert(serialized.includes('timestamp=1234567890'));
    assert(!serialized.includes('tag='));
  });

  test('serializes lock info with tag', () => {
    const info: LockInfo = { pid: 12345, timestamp: 1234567890, tag: 'test-tag' };
    const serialized = serializeLockInfo(info);
    assert(serialized.includes('tag=test-tag'));
  });

  test('sanitizes newlines in tag', () => {
    const info: LockInfo = { pid: 12345, timestamp: 1234567890, tag: 'tag\nwith\nnewlines' };
    const serialized = serializeLockInfo(info);
    // Newlines should be replaced with spaces
    assert(serialized.includes('tag=tag with newlines'));
    // Should not have multiple pid= lines
    const pidMatches = serialized.match(/pid=/g);
    assert.strictEqual(pidMatches?.length, 1);
  });

  test('sanitizes carriage returns in tag', () => {
    const info: LockInfo = { pid: 12345, timestamp: 1234567890, tag: 'tag\r\nwith\rcr' };
    const serialized = serializeLockInfo(info);
    assert(!serialized.includes('\r'));
  });

  test('truncates long tags', () => {
    const longTag = 'x'.repeat(20000);
    const info: LockInfo = { pid: 12345, timestamp: 1234567890, tag: longTag };
    const serialized = serializeLockInfo(info, 100);
    assert(serialized.length < 200);
  });
});

describe('parseLockInfo', () => {
  test('parses valid lock info', () => {
    const content = 'pid=12345\ntimestamp=1234567890\n';
    const result = parseLockInfo(content);
    assert(result.ok);
    assert.strictEqual(result.value.pid, 12345);
    assert.strictEqual(result.value.timestamp, 1234567890);
    assert.strictEqual(result.value.tag, undefined);
  });

  test('parses lock info with tag', () => {
    const content = 'pid=12345\ntimestamp=1234567890\ntag=my-tag\n';
    const result = parseLockInfo(content);
    assert(result.ok);
    assert.strictEqual(result.value.tag, 'my-tag');
  });

  test('handles empty lines', () => {
    const content = '\npid=12345\n\ntimestamp=1234567890\n\n';
    const result = parseLockInfo(content);
    assert(result.ok);
    assert.strictEqual(result.value.pid, 12345);
  });

  test('handles whitespace', () => {
    const content = '  pid=12345  \n  timestamp=1234567890  \n';
    const result = parseLockInfo(content);
    assert(result.ok);
    assert.strictEqual(result.value.pid, 12345);
  });

  test('ignores unknown fields', () => {
    const content = 'pid=12345\ntimestamp=1234567890\nunknown=value\n';
    const result = parseLockInfo(content);
    assert(result.ok);
    assert.strictEqual(result.value.pid, 12345);
  });

  test('fails on missing pid', () => {
    const content = 'timestamp=1234567890\n';
    const result = parseLockInfo(content);
    assert(!result.ok);
    assert.strictEqual(result.error.type, 'PARSE_ERROR');
    assert(result.error.message.includes('pid'));
  });

  test('fails on missing timestamp', () => {
    const content = 'pid=12345\n';
    const result = parseLockInfo(content);
    assert(!result.ok);
    assert.strictEqual(result.error.type, 'PARSE_ERROR');
    assert(result.error.message.includes('timestamp'));
  });

  test('fails on invalid pid', () => {
    const content = 'pid=invalid\ntimestamp=1234567890\n';
    const result = parseLockInfo(content);
    assert(!result.ok);
    assert.strictEqual(result.error.type, 'PARSE_ERROR');
  });

  test('handles tag with equals sign', () => {
    const content = 'pid=12345\ntimestamp=1234567890\ntag=key=value\n';
    const result = parseLockInfo(content);
    assert(result.ok);
    assert.strictEqual(result.value.tag, 'key=value');
  });
});

describe('isLockStale', () => {
  test('returns true for old lock', () => {
    const info: LockInfo = { pid: 12345, timestamp: 0 };
    assert(isLockStale(info, 1000)); // 1 second timeout
  });

  test('returns false for fresh lock', () => {
    const info = createLockInfo();
    assert(!isLockStale(info, 3600000)); // 1 hour timeout
  });

  test('handles edge case at timeout boundary', () => {
    const nowSeconds = Math.floor(Date.now() / 1000);
    const info: LockInfo = { pid: 12345, timestamp: nowSeconds - 10 };
    assert(!isLockStale(info, 11000)); // 11 second timeout (not stale yet)
    assert(isLockStale(info, 9000)); // 9 second timeout (stale)
  });
});

describe('isProcessRunning', () => {
  test('returns true for current process', () => {
    assert(isProcessRunning(currentPid));
  });

  test('returns false for non-existent process', () => {
    // Use a very high PID that likely doesn't exist
    assert(!isProcessRunning(999999999));
  });
});

// ============================================================================
// Semaphore Constructor Tests
// ============================================================================

describe('Semaphore constructor', () => {
  test('creates semaphore with valid path', () => {
    const path = tempLockPath('constructor');
    const sem = new Semaphore(path);
    assert.strictEqual(sem.getPath(), path);
    cleanup(path);
  });

  test('creates semaphore with custom config', () => {
    const path = tempLockPath('custom-config');
    const config: SemaphoreConfig = {
      staleTimeout: 5000,
      retryInterval: 50,
      acquireTimeout: 1000,
    };
    const sem = new Semaphore(path, config);
    assert(sem);
    cleanup(path);
  });

  test('throws on empty path', () => {
    assert.throws(() => new Semaphore(''), /empty/i);
  });

  test('throws on whitespace-only path', () => {
    assert.throws(() => new Semaphore('   '), /empty/i);
  });

  test('throws on path with null bytes', () => {
    assert.throws(() => new Semaphore('/tmp/test\x00.lock'), /null/i);
  });

  test('throws on non-existent parent directory', () => {
    assert.throws(
      () => new Semaphore('/nonexistent/dir/file.lock'),
      /parent directory/i
    );
  });

  test('static withDefaults creates semaphore', () => {
    const path = tempLockPath('with-defaults');
    const sem = Semaphore.withDefaults(path);
    assert(sem);
    cleanup(path);
  });
});

// ============================================================================
// Semaphore Acquire/Release Tests
// ============================================================================

describe('Semaphore tryAcquire', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('try-acquire');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('acquires lock when not held', () => {
    const sem = new Semaphore(lockPath);
    assert(!sem.isLocked());

    const result = sem.tryAcquire();
    assert(result.ok);
    assert.strictEqual(result.value.pid, currentPid);
    assert(sem.isLocked());
  });

  test('acquires lock with tag', () => {
    const sem = new Semaphore(lockPath);
    const result = sem.tryAcquire('my-operation');
    assert(result.ok);
    assert.strictEqual(result.value.tag, 'my-operation');
  });

  test('fails when lock already held', () => {
    const sem = new Semaphore(lockPath);

    const result1 = sem.tryAcquire();
    assert(result1.ok);

    const result2 = sem.tryAcquire();
    assert(!result2.ok);
    assert.strictEqual(result2.error.type, 'ALREADY_LOCKED');
    assert.strictEqual(result2.error.holderPid, currentPid);
  });

  test('sets file permissions to 0600', () => {
    const sem = new Semaphore(lockPath);
    sem.tryAcquire();
    // File should exist and have restrictive permissions
    assert(existsSync(lockPath));
  });
});

describe('Semaphore release', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('release');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('releases held lock', () => {
    const sem = new Semaphore(lockPath);
    sem.tryAcquire();
    assert(sem.isLocked());

    const result = sem.release();
    assert(result.ok);
    assert(!sem.isLocked());
  });

  test('fails when lock not held', () => {
    const sem = new Semaphore(lockPath);
    const result = sem.release();
    assert(!result.ok);
    assert.strictEqual(result.error.type, 'NOT_LOCKED');
  });

  test('fails when lock held by different process', () => {
    const sem = new Semaphore(lockPath);
    // Manually create a lock file with different PID
    const fakeInfo: LockInfo = { pid: 99999, timestamp: Math.floor(Date.now() / 1000) };
    writeFileSync(lockPath, serializeLockInfo(fakeInfo));

    const result = sem.release();
    assert(!result.ok);
    assert.strictEqual(result.error.type, 'PERMISSION_DENIED');
  });

  test('forceRelease removes lock from any process', () => {
    const sem = new Semaphore(lockPath);
    // Manually create a lock file with different PID
    const fakeInfo: LockInfo = { pid: 99999, timestamp: Math.floor(Date.now() / 1000) };
    writeFileSync(lockPath, serializeLockInfo(fakeInfo));

    const result = sem.forceRelease();
    assert(result.ok);
    assert(!sem.isLocked());
  });
});

describe('Semaphore getLockInfo', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('lock-info');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('returns null when not locked', () => {
    const sem = new Semaphore(lockPath);
    assert.strictEqual(sem.getLockInfo(), null);
  });

  test('returns info when locked', () => {
    const sem = new Semaphore(lockPath);
    sem.tryAcquire('test-tag');

    const info = sem.getLockInfo();
    assert(info);
    assert.strictEqual(info.pid, currentPid);
    assert.strictEqual(info.tag, 'test-tag');
  });

  test('returns null for corrupted lock file', () => {
    const sem = new Semaphore(lockPath);
    writeFileSync(lockPath, 'invalid content');

    const info = sem.getLockInfo();
    assert.strictEqual(info, null);
  });
});

describe('Semaphore status', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('status');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('returns correct status when not locked', () => {
    const sem = new Semaphore(lockPath);
    const status = sem.status();

    assert.strictEqual(status.locked, false);
    assert.strictEqual(status.info, null);
    assert.strictEqual(status.isStale, false);
    assert.strictEqual(status.isOwnedByCurrentProcess, false);
  });

  test('returns correct status when locked by current process', () => {
    const sem = new Semaphore(lockPath);
    sem.tryAcquire();
    const status = sem.status();

    assert.strictEqual(status.locked, true);
    assert(status.info);
    assert.strictEqual(status.isStale, false);
    assert.strictEqual(status.isOwnedByCurrentProcess, true);
  });

  test('detects stale lock', () => {
    const sem = new Semaphore(lockPath, { staleTimeout: 1000 });
    // Create old lock
    const oldInfo: LockInfo = { pid: 99999, timestamp: 0 };
    writeFileSync(lockPath, serializeLockInfo(oldInfo));

    const status = sem.status();
    assert.strictEqual(status.isStale, true);
  });
});

describe('Semaphore cleanStale', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('clean-stale');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('cleans stale lock from dead process', () => {
    const sem = new Semaphore(lockPath, { staleTimeout: 1000 });
    // Create stale lock from non-existent process
    const staleInfo: LockInfo = { pid: 999999999, timestamp: 0 };
    writeFileSync(lockPath, serializeLockInfo(staleInfo));

    const cleaned = sem.cleanStale();
    assert(cleaned);
    assert(!sem.isLocked());
  });

  test('does not clean lock from running process', () => {
    const sem = new Semaphore(lockPath, { staleTimeout: 1000 });
    // Create stale lock from current (running) process
    const staleInfo: LockInfo = { pid: currentPid, timestamp: 0 };
    writeFileSync(lockPath, serializeLockInfo(staleInfo));

    const cleaned = sem.cleanStale();
    assert(!cleaned);
    assert(sem.isLocked());
  });

  test('returns false when no lock exists', () => {
    const sem = new Semaphore(lockPath);
    const cleaned = sem.cleanStale();
    assert(!cleaned);
  });
});

// ============================================================================
// Semaphore Async Acquire Tests
// ============================================================================

describe('Semaphore acquire (async)', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('async-acquire');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('acquires lock immediately when available', async () => {
    const sem = new Semaphore(lockPath);
    const result = await sem.acquire();
    assert(result.ok);
    assert.strictEqual(result.value.pid, currentPid);
  });

  test('acquires with tag', async () => {
    const sem = new Semaphore(lockPath);
    const result = await sem.acquire({ tag: 'async-tag' });
    assert(result.ok);
    assert.strictEqual(result.value.tag, 'async-tag');
  });

  test('times out when lock held', async () => {
    const sem = new Semaphore(lockPath);
    sem.tryAcquire();

    const start = Date.now();
    const result = await sem.acquire({ timeout: 200 });
    const elapsed = Date.now() - start;

    assert(!result.ok);
    assert.strictEqual(result.error.type, 'TIMEOUT');
    assert(elapsed >= 150); // Should have waited approximately timeout duration
  });

  test('acquireTimeout convenience method', async () => {
    const sem = new Semaphore(lockPath);
    const result = await sem.acquireTimeout(100);
    assert(result.ok);
  });
});

// ============================================================================
// Stale Lock Acquisition Tests
// ============================================================================

describe('Semaphore stale lock handling', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('stale-handling');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('acquires over stale lock from dead process', () => {
    const sem = new Semaphore(lockPath, { staleTimeout: 1000 });
    // Create stale lock from non-existent process
    const staleInfo: LockInfo = { pid: 999999999, timestamp: 0 };
    writeFileSync(lockPath, serializeLockInfo(staleInfo));

    const result = sem.tryAcquire();
    assert(result.ok);
    assert.strictEqual(result.value.pid, currentPid);
  });

  test('does not acquire over stale lock from running process', () => {
    const sem = new Semaphore(lockPath, { staleTimeout: 1000 });
    // Create stale lock from current (running) process
    const staleInfo: LockInfo = { pid: currentPid, timestamp: 0 };
    writeFileSync(lockPath, serializeLockInfo(staleInfo));

    const result = sem.tryAcquire();
    // Should fail because process is still running
    assert(!result.ok);
    assert.strictEqual(result.error.type, 'ALREADY_LOCKED');
  });
});
