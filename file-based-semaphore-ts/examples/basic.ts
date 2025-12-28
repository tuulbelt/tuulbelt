#!/usr/bin/env -S npx tsx
/**
 * Basic usage example for File-Based Semaphore (TypeScript)
 *
 * Run this example:
 *   npx tsx examples/basic.ts
 */

import { Semaphore } from '../src/index.js';
import { mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

// Create temporary directory for examples
const tmpDir = mkdtempSync(join(tmpdir(), 'semats-example-'));
const lockPath = join(tmpDir, 'example.lock');

console.log('File-Based Semaphore (TypeScript) - Basic Examples\n');
console.log(`Lock file: ${lockPath}\n`);

// Example 1: Basic tryAcquire and release
console.log('Example 1: Basic tryAcquire and release');
console.log('=' .repeat(50));

const sem = new Semaphore(lockPath);

const result1 = sem.tryAcquire('example-process');
if (result1.ok) {
  console.log('✅ Lock acquired successfully');
  console.log(`   PID: ${result1.value.pid}`);
  console.log(`   Timestamp: ${new Date(result1.value.timestamp * 1000).toISOString()}`);
  console.log(`   Tag: ${result1.value.tag}`);

  // Do some work...
  console.log('   Simulating work...');

  // Release the lock
  const releaseResult = sem.release();
  if (releaseResult.ok) {
    console.log('✅ Lock released');
  }
} else {
  console.log(`❌ Failed to acquire: ${result1.error.message}`);
}

console.log();

// Example 2: Check lock status
console.log('Example 2: Check lock status');
console.log('=' .repeat(50));

// Acquire again
sem.tryAcquire('status-check');
const status = sem.status();
console.log(`Locked: ${status.locked}`);
console.log(`Stale: ${status.isStale}`);
console.log(`Owned by us: ${status.isOwnedByCurrentProcess}`);
if (status.info) {
  console.log(`Holder PID: ${status.info.pid}`);
}
sem.release();

console.log();

// Example 3: Handle already-locked scenario
console.log('Example 3: Already locked error handling');
console.log('=' .repeat(50));

// First process acquires
sem.tryAcquire('process-1');
console.log('Process 1 acquired the lock');

// Second attempt fails (same process, but demonstrates the error)
const sem2 = new Semaphore(lockPath);
const result2 = sem2.tryAcquire('process-2');
if (!result2.ok) {
  console.log(`Expected failure: ${result2.error.type}`);
  console.log(`Message: ${result2.error.message}`);
  if (result2.error.holderPid) {
    console.log(`Current holder: PID ${result2.error.holderPid}`);
  }
}

// Clean up
sem.release();

console.log();

// Example 4: Blocking acquire with timeout
console.log('Example 4: Blocking acquire with timeout');
console.log('=' .repeat(50));

async function blockingExample() {
  // Acquire lock with blocking wait (will succeed immediately since unlocked)
  const result = await sem.acquire({ timeout: 5000, tag: 'blocking-example' });
  if (result.ok) {
    console.log('✅ Acquired after waiting (immediately in this case)');
    sem.release();
  } else {
    console.log(`❌ Timeout or error: ${result.error.message}`);
  }
}

await blockingExample();

console.log();

// Example 5: Using getLockInfo()
console.log('Example 5: Reading lock info directly');
console.log('=' .repeat(50));

sem.tryAcquire('info-example');
const info = sem.getLockInfo();
if (info) {
  console.log('Lock info from file:');
  console.log(`  pid=${info.pid}`);
  console.log(`  timestamp=${info.timestamp}`);
  console.log(`  tag=${info.tag ?? '(none)'}`);
}
sem.release();

console.log();

// Cleanup
console.log('Cleanup');
console.log('=' .repeat(50));
rmSync(tmpDir, { recursive: true });
console.log('Temporary directory removed');
console.log('\nDone!');
