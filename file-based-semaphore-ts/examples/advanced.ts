#!/usr/bin/env -S npx tsx
/**
 * Advanced usage patterns for File-Based Semaphore (TypeScript)
 *
 * This example demonstrates:
 * - Stale lock detection and recovery
 * - Cross-process coordination patterns
 * - Concurrent worker protection
 * - Custom configuration options
 * - Error handling strategies
 *
 * Run this example:
 *   npx tsx examples/advanced.ts
 */

import {
  Semaphore,
  type SemaphoreConfig,
  type SemaphoreResult,
  type LockInfo,
} from '../src/index.js';
import { mkdtempSync, rmSync, writeFileSync, readFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { spawn } from 'node:child_process';

// Create temporary directory for examples
const tmpDir = mkdtempSync(join(tmpdir(), 'semats-advanced-'));

console.log('File-Based Semaphore (TypeScript) - Advanced Patterns\n');

// Example 1: Custom configuration with short stale timeout
console.log('Example 1: Custom stale timeout configuration');
console.log('=' .repeat(50));

const config: SemaphoreConfig = {
  staleTimeout: 5000,      // 5 seconds (very short for demo)
  retryInterval: 100,      // 100ms between retries
  maxTagLength: 1000,      // Maximum tag length
};

const sem1 = new Semaphore(join(tmpDir, 'custom.lock'), config);
sem1.tryAcquire('short-stale-timeout');
const status1 = sem1.status();
console.log(`Stale timeout: ${config.staleTimeout}ms`);
console.log(`Lock is stale: ${status1.isStale}`);
sem1.release();

console.log();

// Example 2: Stale lock detection and cleanup
console.log('Example 2: Stale lock detection');
console.log('=' .repeat(50));

const staleLockPath = join(tmpDir, 'stale.lock');

// Simulate a stale lock by writing an old timestamp
const oldTimestamp = Math.floor(Date.now() / 1000) - 7200; // 2 hours ago
writeFileSync(staleLockPath, `pid=99999\ntimestamp=${oldTimestamp}\ntag=crashed-process\n`);

const sem2 = new Semaphore(staleLockPath, { staleTimeout: 3600000 }); // 1 hour
const status2 = sem2.status();
console.log(`Lock exists: ${status2.locked}`);
console.log(`Is stale: ${status2.isStale}`);
console.log(`Original holder: PID ${status2.info?.pid}`);

// Clean the stale lock
if (sem2.cleanStale()) {
  console.log('✅ Stale lock cleaned');
}

// Now we can acquire
const result2 = sem2.tryAcquire('new-process');
if (result2.ok) {
  console.log(`✅ Acquired after cleaning: PID ${result2.value.pid}`);
  sem2.release();
}

console.log();

// Example 3: Concurrent worker protection pattern
console.log('Example 3: Concurrent worker protection');
console.log('=' .repeat(50));

async function protectedWorker(
  workerId: number,
  sem: Semaphore,
  sharedResourcePath: string,
): Promise<void> {
  console.log(`Worker ${workerId}: Requesting lock...`);

  const result = await sem.acquire({ timeout: 10000, tag: `worker-${workerId}` });
  if (!result.ok) {
    console.log(`Worker ${workerId}: Failed to acquire - ${result.error.message}`);
    return;
  }

  try {
    console.log(`Worker ${workerId}: Lock acquired, working...`);

    // Simulate reading, modifying, writing shared resource
    const current = parseInt(readFileSync(sharedResourcePath, 'utf-8') || '0', 10);
    await new Promise((r) => setTimeout(r, 50)); // Simulate work
    writeFileSync(sharedResourcePath, String(current + 1));

    console.log(`Worker ${workerId}: Incremented counter to ${current + 1}`);
  } finally {
    sem.release();
    console.log(`Worker ${workerId}: Lock released`);
  }
}

const workerLockPath = join(tmpDir, 'worker.lock');
const counterPath = join(tmpDir, 'counter.txt');
writeFileSync(counterPath, '0');

const workerSem = new Semaphore(workerLockPath);

// Run 3 workers concurrently
await Promise.all([
  protectedWorker(1, workerSem, counterPath),
  protectedWorker(2, workerSem, counterPath),
  protectedWorker(3, workerSem, counterPath),
]);

const finalCount = readFileSync(counterPath, 'utf-8');
console.log(`Final counter value: ${finalCount} (expected: 3)`);

console.log();

// Example 4: Error handling with result pattern
console.log('Example 4: Comprehensive error handling');
console.log('=' .repeat(50));

function handleResult<T>(result: SemaphoreResult<T>, operation: string): boolean {
  if (result.ok) {
    console.log(`✅ ${operation}: Success`);
    return true;
  }

  switch (result.error.type) {
    case 'ALREADY_LOCKED':
      console.log(`⏳ ${operation}: Already locked by PID ${result.error.holderPid}`);
      if (result.error.holderTag) {
        console.log(`   Tag: ${result.error.holderTag}`);
      }
      break;
    case 'NOT_LOCKED':
      console.log(`⚠️ ${operation}: Lock doesn't exist`);
      break;
    case 'TIMEOUT':
      console.log(`⏱️ ${operation}: Timed out waiting for lock`);
      break;
    case 'PERMISSION_DENIED':
      console.log(`🚫 ${operation}: Permission denied`);
      break;
    case 'PATH_TRAVERSAL':
      console.log(`🔒 ${operation}: Security violation - path traversal`);
      break;
    case 'IO_ERROR':
      console.log(`💥 ${operation}: I/O error - ${result.error.message}`);
      break;
    default:
      console.log(`❌ ${operation}: ${result.error.message}`);
  }
  return false;
}

const sem4 = new Semaphore(join(tmpDir, 'error-demo.lock'));

handleResult(sem4.tryAcquire('test'), 'First acquire');
handleResult(sem4.tryAcquire('test2'), 'Second acquire (should fail)');
handleResult(sem4.release(), 'Release');
handleResult(sem4.release(), 'Double release (should fail)');

console.log();

// Example 5: Cross-language compatibility check
console.log('Example 5: Cross-language lock format');
console.log('=' .repeat(50));

const crossLangPath = join(tmpDir, 'cross-lang.lock');
const sem5 = new Semaphore(crossLangPath);
sem5.tryAcquire('typescript-process');

// Read the raw lock file content
const lockContent = readFileSync(crossLangPath, 'utf-8');
console.log('Lock file content (compatible with Rust sema):');
console.log('---');
console.log(lockContent);
console.log('---');
console.log('This format can be read by: Rust sema, shell scripts, other languages');

sem5.release();

console.log();

// Example 6: Disable stale timeout (infinite lock)
console.log('Example 6: Infinite lock (no stale timeout)');
console.log('=' .repeat(50));

const infiniteSem = new Semaphore(join(tmpDir, 'infinite.lock'), {
  staleTimeout: null, // Disable stale detection
});

infiniteSem.tryAcquire('permanent-lock');
const infiniteStatus = infiniteSem.status();
console.log(`Stale detection disabled: ${!infiniteStatus.isStale}`);
console.log('Lock will never be considered stale (manual cleanup required)');
infiniteSem.release();

console.log();

// Cleanup
console.log('Cleanup');
console.log('=' .repeat(50));
rmSync(tmpDir, { recursive: true });
console.log('Temporary directory removed');

console.log('\nAdvanced Examples Complete!');
console.log('\nKey Takeaways:');
console.log('  - Use short stale timeouts for quick recovery');
console.log('  - Always use try/finally or Result pattern for cleanup');
console.log('  - The lock format is cross-language compatible');
console.log('  - cleanStale() recovers from crashed processes');
console.log('  - Disable stale timeout for permanent locks');
