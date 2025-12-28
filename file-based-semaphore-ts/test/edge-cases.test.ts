import { test, describe, beforeEach, afterEach } from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, unlinkSync, writeFileSync, mkdirSync, rmdirSync, chmodSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { pid as currentPid } from 'node:process';
import {
  Semaphore,
  parseLockInfo,
  type LockInfo,
} from '../src/index.js';

// ============================================================================
// Test Utilities
// ============================================================================

function tempLockPath(name: string): string {
  return join(tmpdir(), `semats-edge-${name}-${currentPid}-${Date.now()}.lock`);
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
// Corrupted Lock File Tests
// ============================================================================

describe('Edge Cases: Corrupted Lock Files', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('corrupted');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('handles lock file with binary garbage', () => {
    writeFileSync(lockPath, Buffer.from([0x00, 0x01, 0x02, 0xFF, 0xFE]));
    const sem = new Semaphore(lockPath);
    const info = sem.getLockInfo();
    assert.strictEqual(info, null);
  });

  test('handles lock file with partial PID', () => {
    writeFileSync(lockPath, 'pid=');
    const sem = new Semaphore(lockPath);
    const info = sem.getLockInfo();
    assert.strictEqual(info, null);
  });

  test('handles lock file with non-numeric PID', () => {
    writeFileSync(lockPath, 'pid=abc\ntimestamp=123\n');
    const sem = new Semaphore(lockPath);
    const info = sem.getLockInfo();
    assert.strictEqual(info, null);
  });

  test('handles lock file with negative PID', () => {
    writeFileSync(lockPath, 'pid=-1\ntimestamp=123\n');
    const sem = new Semaphore(lockPath);
    const info = sem.getLockInfo();
    // Should either return null or treat as invalid
    assert(info === null || info.pid === -1);
  });

  test('handles lock file with extremely large PID', () => {
    writeFileSync(lockPath, 'pid=999999999999999\ntimestamp=123\n');
    const sem = new Semaphore(lockPath);
    const info = sem.getLockInfo();
    // Should handle without crashing
    assert(info === null || info.pid === 999999999999999);
  });

  test('handles lock file with missing timestamp', () => {
    writeFileSync(lockPath, 'pid=1234\n');
    const sem = new Semaphore(lockPath);
    const info = sem.getLockInfo();
    assert.strictEqual(info, null);
  });

  test('handles lock file with timestamp=0', () => {
    writeFileSync(lockPath, 'pid=1234\ntimestamp=0\n');
    const sem = new Semaphore(lockPath);
    const info = sem.getLockInfo();
    // Timestamp 0 is valid (epoch)
    assert(info !== null);
    assert.strictEqual(info.timestamp, 0);
  });

  test('handles lock file with very old timestamp', () => {
    writeFileSync(lockPath, 'pid=1234\ntimestamp=1\n');
    const sem = new Semaphore(lockPath);
    const info = sem.getLockInfo();
    assert(info !== null);
    // Should be considered stale
    const status = sem.status();
    assert.strictEqual(status.isStale, true);
  });

  test('handles lock file with future timestamp', () => {
    const futureTime = Math.floor(Date.now() / 1000) + 86400; // 1 day in future
    writeFileSync(lockPath, `pid=1234\ntimestamp=${futureTime}\n`);
    const sem = new Semaphore(lockPath);
    const info = sem.getLockInfo();
    assert(info !== null);
    // Future timestamp should NOT be stale
    const status = sem.status();
    assert.strictEqual(status.isStale, false);
  });

  test('handles lock file with duplicate keys', () => {
    writeFileSync(lockPath, 'pid=1234\npid=5678\ntimestamp=123\n');
    const sem = new Semaphore(lockPath);
    const info = sem.getLockInfo();
    // Should parse first or last value consistently
    assert(info !== null);
    assert(info.pid === 1234 || info.pid === 5678);
  });

  test('handles lock file with unknown keys', () => {
    writeFileSync(lockPath, 'pid=1234\ntimestamp=123\nunknown_key=value\n');
    const sem = new Semaphore(lockPath);
    const info = sem.getLockInfo();
    // Should ignore unknown keys
    assert(info !== null);
    assert.strictEqual(info.pid, 1234);
  });

  test('handles lock file with Windows line endings', () => {
    writeFileSync(lockPath, 'pid=1234\r\ntimestamp=123\r\n');
    const sem = new Semaphore(lockPath);
    const info = sem.getLockInfo();
    assert(info !== null);
    assert.strictEqual(info.pid, 1234);
  });

  test('handles lock file with mixed line endings', () => {
    writeFileSync(lockPath, 'pid=1234\r\ntimestamp=123\ntag=test\r\n');
    const sem = new Semaphore(lockPath);
    const info = sem.getLockInfo();
    assert(info !== null);
    assert.strictEqual(info.pid, 1234);
  });
});

// ============================================================================
// Concurrent Access Tests
// ============================================================================

describe('Edge Cases: Concurrent Access', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('concurrent');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('multiple semaphores on same path behave correctly', () => {
    const sem1 = new Semaphore(lockPath);
    const sem2 = new Semaphore(lockPath);

    const result1 = sem1.tryAcquire('first');
    assert(result1.ok);

    const result2 = sem2.tryAcquire('second');
    assert(!result2.ok);
    if (!result2.ok) {
      assert.strictEqual(result2.error.type, 'ALREADY_LOCKED');
    }

    sem1.release();
  });

  test('release by one semaphore is visible to another', () => {
    const sem1 = new Semaphore(lockPath);
    const sem2 = new Semaphore(lockPath);

    sem1.tryAcquire();
    assert(existsSync(lockPath));

    sem1.release();
    assert(!existsSync(lockPath));

    // Second semaphore should now be able to acquire
    const result = sem2.tryAcquire();
    assert(result.ok);

    sem2.release();
  });

  test('status is consistent across semaphore instances', () => {
    const sem1 = new Semaphore(lockPath);
    const sem2 = new Semaphore(lockPath);

    sem1.tryAcquire('test-tag');

    const status1 = sem1.status();
    const status2 = sem2.status();

    assert.strictEqual(status1.isLocked, status2.isLocked);
    assert.strictEqual(status1.holderPid, status2.holderPid);
    assert.strictEqual(status1.tag, status2.tag);

    sem1.release();
  });

  test('handles rapid acquire-release cycles', async () => {
    const sem = new Semaphore(lockPath);

    for (let i = 0; i < 50; i++) {
      const result = sem.tryAcquire(`cycle-${i}`);
      assert(result.ok);
      assert(existsSync(lockPath));
      sem.release();
      assert(!existsSync(lockPath));
    }
  });
});

// ============================================================================
// File System Edge Cases
// ============================================================================

describe('Edge Cases: File System', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('fs-edge');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('handles path with spaces', () => {
    const pathWithSpaces = join(tmpdir(), `semats space test ${Date.now()}.lock`);
    cleanup(pathWithSpaces);

    try {
      const sem = new Semaphore(pathWithSpaces);
      const result = sem.tryAcquire();
      assert(result.ok);
      assert(existsSync(pathWithSpaces));
      sem.release();
    } finally {
      cleanup(pathWithSpaces);
    }
  });

  test('handles path with special characters', () => {
    const pathWithSpecial = join(tmpdir(), `semats-test@#$%^&()${Date.now()}.lock`);
    cleanup(pathWithSpecial);

    try {
      const sem = new Semaphore(pathWithSpecial);
      const result = sem.tryAcquire();
      assert(result.ok);
      assert(existsSync(pathWithSpecial));
      sem.release();
    } finally {
      cleanup(pathWithSpecial);
    }
  });

  test('handles very long path', () => {
    // Create a long path name (within OS limits)
    const longName = 'a'.repeat(200) + '.lock';
    const longPath = join(tmpdir(), longName);
    cleanup(longPath);

    try {
      const sem = new Semaphore(longPath);
      const result = sem.tryAcquire();
      assert(result.ok);
      sem.release();
    } finally {
      cleanup(longPath);
    }
  });

  test('handles path in non-existent directory gracefully', () => {
    const nonExistentDir = join(tmpdir(), `non-existent-${Date.now()}`, 'test.lock');

    assert.throws(() => new Semaphore(nonExistentDir));
  });

  test('handles lock file deleted externally', () => {
    const sem = new Semaphore(lockPath);
    sem.tryAcquire();
    assert(existsSync(lockPath));

    // Externally delete the lock file
    unlinkSync(lockPath);
    assert(!existsSync(lockPath));

    // Status should reflect that lock is gone
    const status = sem.status();
    assert.strictEqual(status.locked, false);
  });

  test('handles lock file replaced externally', () => {
    const sem = new Semaphore(lockPath);
    sem.tryAcquire();

    // Externally replace with different PID
    writeFileSync(lockPath, 'pid=99999\ntimestamp=123\n');

    // Status should reflect the external change
    const status = sem.status();
    assert(status.info !== null);
    assert.strictEqual(status.info!.pid, 99999);
    assert.strictEqual(status.isOwnedByCurrentProcess, false);
  });
});

// ============================================================================
// Timeout and Blocking Tests
// ============================================================================

describe('Edge Cases: Timeout Behavior', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('timeout');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('acquire times out when lock is held', async () => {
    const sem1 = new Semaphore(lockPath);
    const sem2 = new Semaphore(lockPath);

    // First semaphore acquires
    sem1.tryAcquire();

    // Second semaphore should timeout
    const start = Date.now();
    const result = await sem2.acquire({ timeout: 200 });
    const elapsed = Date.now() - start;

    assert(!result.ok);
    if (!result.ok) {
      assert.strictEqual(result.error.type, 'TIMEOUT');
    }
    assert(elapsed >= 180); // Allow some tolerance
    assert(elapsed < 1000); // Should not take too long

    sem1.release();
  });

  test('acquire succeeds immediately when lock available', async () => {
    const sem = new Semaphore(lockPath);

    const start = Date.now();
    const result = await sem.acquire({ timeout: 5000 });
    const elapsed = Date.now() - start;

    assert(result.ok);
    assert(elapsed < 500); // Should be nearly instant

    sem.release();
  });

  test('acquire with timeout=0 behaves like tryAcquire', async () => {
    const sem1 = new Semaphore(lockPath);
    const sem2 = new Semaphore(lockPath);

    sem1.tryAcquire();

    const start = Date.now();
    const result = await sem2.acquire({ timeout: 0 });
    const elapsed = Date.now() - start;

    assert(!result.ok);
    assert(elapsed < 200); // Should fail quickly

    sem1.release();
  });
});

// ============================================================================
// Configuration Edge Cases
// ============================================================================

describe('Edge Cases: Configuration', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('config');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('respects custom stale timeout', () => {
    const sem = new Semaphore(lockPath, { staleTimeout: 1000 }); // 1 second

    // Create old lock
    const oldTimestamp = Math.floor(Date.now() / 1000) - 10; // 10 seconds ago
    writeFileSync(lockPath, `pid=99999\ntimestamp=${oldTimestamp}\n`);

    const status = sem.status();
    assert.strictEqual(status.isStale, true);
  });

  test('respects staleTimeout=null (no stale detection)', () => {
    const sem = new Semaphore(lockPath, { staleTimeout: null });

    // Create very old lock
    writeFileSync(lockPath, 'pid=99999\ntimestamp=1\n');

    const status = sem.status();
    assert.strictEqual(status.isStale, false);
  });

  test('respects custom maxTagLength', () => {
    const sem = new Semaphore(lockPath, { maxTagLength: 10 });
    const longTag = 'a'.repeat(100);

    const result = sem.tryAcquire(longTag);
    assert(result.ok);

    const info = sem.getLockInfo();
    assert(info !== null);
    // Tag should be truncated
    assert((info.tag?.length ?? 0) <= 10);

    sem.release();
  });
});

// ============================================================================
// Parser Edge Cases
// ============================================================================

describe('Edge Cases: Parser', () => {
  test('parseLockInfo handles empty string', () => {
    const result = parseLockInfo('');
    assert(!result.ok);
  });

  test('parseLockInfo handles whitespace only', () => {
    const result = parseLockInfo('   \n\n  ');
    assert(!result.ok);
  });

  test('parseLockInfo handles just equals signs', () => {
    const result = parseLockInfo('===\n===\n');
    assert(!result.ok);
  });

  test('parseLockInfo handles key without value', () => {
    const result = parseLockInfo('pid=\ntimestamp=\n');
    assert(!result.ok);
  });

  test('parseLockInfo handles value without key', () => {
    const result = parseLockInfo('=1234\n=567\n');
    assert(!result.ok);
  });

  test('parseLockInfo handles unicode tag', () => {
    const result = parseLockInfo('pid=1234\ntimestamp=567\ntag=日本語テスト🔒\n');
    assert(result.ok);
    if (result.ok) {
      assert(result.value.tag?.includes('日本語'));
      assert(result.value.tag?.includes('🔒'));
    }
  });

  test('parseLockInfo handles extremely long tag', () => {
    const longTag = 'x'.repeat(100000);
    const result = parseLockInfo(`pid=1234\ntimestamp=567\ntag=${longTag}\n`);
    assert(result.ok);
    // Should parse without crashing
  });
});
