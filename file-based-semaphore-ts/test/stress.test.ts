import { test, describe, beforeEach, afterEach } from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, unlinkSync, readFileSync } from 'node:fs';
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
  return join(tmpdir(), `semats-stress-${name}-${currentPid}-${Date.now()}.lock`);
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
// Rapid Acquire/Release Stress Tests
// ============================================================================

describe('Stress: Rapid Acquire/Release', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('rapid');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('handles 100 rapid acquire-release cycles', () => {
    const sem = new Semaphore(lockPath);

    for (let i = 0; i < 100; i++) {
      const result = sem.tryAcquire(`cycle-${i}`);
      assert(result.ok, `Failed to acquire on cycle ${i}`);
      const releaseResult = sem.release();
      assert(releaseResult.ok, `Failed to release on cycle ${i}`);
    }

    assert(!existsSync(lockPath));
  });

  test('handles 50 acquire-release with status checks', () => {
    const sem = new Semaphore(lockPath);

    for (let i = 0; i < 50; i++) {
      assert(!sem.status().locked);

      const result = sem.tryAcquire();
      assert(result.ok);
      assert(sem.status().locked);
      assert(sem.status().isOwnedByCurrentProcess);

      sem.release();
      assert(!sem.status().locked);
    }
  });

  test('handles alternating acquire attempts', () => {
    const sem1 = new Semaphore(lockPath);
    const sem2 = new Semaphore(lockPath);

    for (let i = 0; i < 25; i++) {
      // sem1 acquires
      const result1 = sem1.tryAcquire(`sem1-${i}`);
      assert(result1.ok);

      // sem2 fails
      const result2 = sem2.tryAcquire(`sem2-${i}`);
      assert(!result2.ok);

      // sem1 releases
      sem1.release();

      // sem2 acquires
      const result3 = sem2.tryAcquire(`sem2-${i}`);
      assert(result3.ok);

      // sem1 fails
      const result4 = sem1.tryAcquire(`sem1-${i}`);
      assert(!result4.ok);

      // sem2 releases
      sem2.release();
    }
  });
});

// ============================================================================
// Serialization Stress Tests
// ============================================================================

describe('Stress: Serialization', () => {
  test('handles 1000 serialize-parse round trips', () => {
    for (let i = 0; i < 1000; i++) {
      const info: LockInfo = {
        pid: 1000 + i,
        timestamp: Math.floor(Date.now() / 1000) + i,
        tag: `test-tag-${i}`,
      };

      const serialized = serializeLockInfo(info);
      const parsed = parseLockInfo(serialized);

      assert(parsed.ok);
      if (parsed.ok) {
        assert.strictEqual(parsed.value.pid, info.pid);
        assert.strictEqual(parsed.value.timestamp, info.timestamp);
        assert.strictEqual(parsed.value.tag, info.tag);
      }
    }
  });

  test('handles varying tag sizes', () => {
    const sizes = [0, 1, 10, 100, 1000, 5000];

    for (const size of sizes) {
      const tag = 'x'.repeat(size);
      const info: LockInfo = {
        pid: 12345,
        timestamp: 67890,
        tag,
      };

      const serialized = serializeLockInfo(info);
      const parsed = parseLockInfo(serialized);

      assert(parsed.ok);
      if (parsed.ok) {
        assert.strictEqual(parsed.value.pid, 12345);
        assert.strictEqual(parsed.value.timestamp, 67890);
        // Tag might be truncated at maxLength
        assert(parsed.value.tag?.length !== undefined);
      }
    }
  });

  test('handles special characters in tags', () => {
    const specialTags = [
      'simple',
      'with spaces',
      'with\ttabs',
      'with=equals',
      'with:colons',
      'with/slashes',
      'with\\backslashes',
      'with"quotes',
      "with'apostrophes",
      'with<>brackets',
      'with日本語unicode',
      'with🎉emoji',
      'with\x00nullbyte',
    ];

    for (const tag of specialTags) {
      const info: LockInfo = {
        pid: 12345,
        timestamp: 67890,
        tag,
      };

      const serialized = serializeLockInfo(info);
      const parsed = parseLockInfo(serialized);

      assert(parsed.ok, `Failed to parse tag: ${tag}`);
      if (parsed.ok) {
        assert.strictEqual(parsed.value.pid, 12345);
        assert.strictEqual(parsed.value.timestamp, 67890);
      }
    }
  });
});

// ============================================================================
// Concurrent Instance Stress Tests
// ============================================================================

describe('Stress: Multiple Instances', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('multi');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('handles 10 semaphore instances on same path', () => {
    const instances: Semaphore[] = [];
    for (let i = 0; i < 10; i++) {
      instances.push(new Semaphore(lockPath));
    }

    // First one wins
    const firstResult = instances[0].tryAcquire('first');
    assert(firstResult.ok);

    // Others fail
    for (let i = 1; i < 10; i++) {
      const result = instances[i].tryAcquire(`attempt-${i}`);
      assert(!result.ok);
      if (!result.ok) {
        assert.strictEqual(result.error.type, 'ALREADY_LOCKED');
      }
    }

    // All see consistent status
    for (const sem of instances) {
      const status = sem.status();
      assert.strictEqual(status.locked, true);
      assert(status.info !== null);
      assert.strictEqual(status.info!.tag, 'first');
    }

    instances[0].release();
  });

  test('handles round-robin acquire-release', () => {
    const instances: Semaphore[] = [];
    for (let i = 0; i < 5; i++) {
      instances.push(new Semaphore(lockPath));
    }

    // Each instance takes a turn
    for (let round = 0; round < 3; round++) {
      for (let i = 0; i < instances.length; i++) {
        const result = instances[i].tryAcquire(`round-${round}-instance-${i}`);
        assert(result.ok, `Round ${round}, instance ${i} failed to acquire`);
        instances[i].release();
      }
    }
  });
});

// ============================================================================
// Status Check Stress Tests
// ============================================================================

describe('Stress: Status Checks', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('status-stress');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('handles 500 rapid status checks', () => {
    const sem = new Semaphore(lockPath);
    sem.tryAcquire('stress-test');

    for (let i = 0; i < 500; i++) {
      const status = sem.status();
      assert.strictEqual(status.locked, true);
      assert.strictEqual(status.isOwnedByCurrentProcess, true);
      assert(status.info !== null);
    }

    sem.release();
  });

  test('handles interleaved status and operations', () => {
    const sem = new Semaphore(lockPath);

    for (let i = 0; i < 50; i++) {
      // Check status before acquire
      let status = sem.status();
      assert.strictEqual(status.locked, false);

      // Acquire
      sem.tryAcquire(`iteration-${i}`);

      // Check status after acquire
      status = sem.status();
      assert.strictEqual(status.locked, true);
      assert(status.info !== null);
      assert.strictEqual(status.info!.tag, `iteration-${i}`);

      // Get lock info directly
      const info = sem.getLockInfo();
      assert(info !== null);
      assert.strictEqual(info.tag, `iteration-${i}`);

      // Release
      sem.release();

      // Check status after release
      status = sem.status();
      assert.strictEqual(status.locked, false);
    }
  });
});

// ============================================================================
// Memory and Resource Stress Tests
// ============================================================================

describe('Stress: Resource Usage', () => {
  test('handles creating many semaphore instances', () => {
    const instances: Semaphore[] = [];
    const paths: string[] = [];

    // Create 100 different semaphore instances on different paths
    for (let i = 0; i < 100; i++) {
      const path = tempLockPath(`instance-${i}`);
      paths.push(path);
      instances.push(new Semaphore(path));
    }

    // Acquire half of them
    for (let i = 0; i < 50; i++) {
      const result = instances[i].tryAcquire(`tag-${i}`);
      assert(result.ok);
    }

    // Release all acquired locks
    for (let i = 0; i < 50; i++) {
      instances[i].release();
    }

    // Cleanup
    for (const path of paths) {
      cleanup(path);
    }
  });

  test('handles repeated large tag serialization', () => {
    const largeTag = 'x'.repeat(9000); // Near default maxTagLength

    for (let i = 0; i < 100; i++) {
      const info: LockInfo = {
        pid: 12345,
        timestamp: 67890,
        tag: largeTag,
      };

      const serialized = serializeLockInfo(info);
      const parsed = parseLockInfo(serialized);

      assert(parsed.ok);
    }
  });
});

// ============================================================================
// Edge Case Stress Tests
// ============================================================================

describe('Stress: Edge Case Combinations', () => {
  let lockPath: string;

  beforeEach(() => {
    lockPath = tempLockPath('edge-stress');
    cleanup(lockPath);
  });

  afterEach(() => {
    cleanup(lockPath);
  });

  test('handles acquire-check-release-check cycles', () => {
    const sem = new Semaphore(lockPath);

    for (let i = 0; i < 30; i++) {
      // Acquire
      const acquireResult = sem.tryAcquire(`cycle-${i}`);
      assert(acquireResult.ok);

      // Verify acquired
      assert(existsSync(lockPath));
      const content = readFileSync(lockPath, 'utf-8');
      assert(content.includes(`pid=${currentPid}`));
      assert(content.includes(`tag=cycle-${i}`));

      // Check status
      const status = sem.status();
      assert(status.locked);
      assert(status.isOwnedByCurrentProcess);
      assert.strictEqual(status.info?.tag, `cycle-${i}`);

      // Release
      const releaseResult = sem.release();
      assert(releaseResult.ok);

      // Verify released
      assert(!existsSync(lockPath));

      // Check status
      const statusAfter = sem.status();
      assert(!statusAfter.locked);
    }
  });

  test('handles failed acquire followed by successful acquire', () => {
    const sem1 = new Semaphore(lockPath);
    const sem2 = new Semaphore(lockPath);

    for (let i = 0; i < 20; i++) {
      // sem1 acquires
      sem1.tryAcquire();

      // sem2 fails
      const failResult = sem2.tryAcquire();
      assert(!failResult.ok);

      // sem1 releases
      sem1.release();

      // sem2 succeeds
      const successResult = sem2.tryAcquire();
      assert(successResult.ok);

      // sem2 releases
      sem2.release();
    }
  });

  test('handles mixed operations under contention', async () => {
    const instances = [
      new Semaphore(lockPath),
      new Semaphore(lockPath),
      new Semaphore(lockPath),
    ];

    for (let round = 0; round < 10; round++) {
      // Random instance tries to acquire
      for (const sem of instances) {
        const result = sem.tryAcquire(`round-${round}`);
        if (result.ok) {
          // Check status from all instances
          for (const checker of instances) {
            const status = checker.status();
            assert(status.locked);
          }
          sem.release();
          break;
        }
      }
    }
  });
});
