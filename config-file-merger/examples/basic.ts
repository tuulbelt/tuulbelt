/**
 * Basic usage example for Config File Merger
 *
 * Run this example:
 *   npx tsx examples/basic.ts
 */

import { mergeConfig, getValue, parseCliArgs, parseEnv } from '../src/index.js';

console.log('=== Config File Merger Examples ===\n');

// Example 1: Merge from defaults and CLI args
console.log('Example 1: Merge defaults with CLI overrides');
const result1 = mergeConfig({
  defaults: { port: 8080, host: 'localhost', debug: false },
  cli: { port: 3000, debug: true },
});

if (result1.ok) {
  console.log('Merged config:', JSON.stringify(result1.config, null, 2));
}
console.log();

// Example 2: Track where each value came from
console.log('Example 2: Track sources');
const result2 = mergeConfig({
  defaults: { timeout: 30000 },
  file: { host: 'api.example.com' },
  cli: { port: 443 },
  trackSources: true,
});

if (result2.ok) {
  console.log('Config with sources:', JSON.stringify(result2.config, null, 2));
}
console.log();

// Example 3: Parse environment variables with prefix
console.log('Example 3: Parse env vars with prefix');
const mockEnv = {
  APP_HOST: 'prod.example.com',
  APP_PORT: '8443',
  APP_DEBUG: 'true',
  OTHER_VAR: 'ignored',
};
const envResult = parseEnv(mockEnv, 'APP_');
console.log('Parsed env vars:', JSON.stringify(envResult, null, 2));
console.log();

// Example 4: Parse CLI arguments
console.log('Example 4: Parse CLI arguments');
const cliResult = parseCliArgs('port=3000,host=localhost,debug=true,name="My App"');
console.log('Parsed CLI args:', JSON.stringify(cliResult, null, 2));
console.log();

// Example 5: Complete merge with all sources
console.log('Example 5: Complete merge (CLI > ENV > File > Defaults)');
const result5 = mergeConfig({
  defaults: { port: 8080, host: 'localhost', timeout: 5000 },
  file: { port: 3000, host: 'file.example.com' },
  env: { PORT: '9000' },
  cli: { port: 443 },
  lowercaseEnv: true,
  trackSources: true,
});

if (result5.ok) {
  console.log('Final config:', JSON.stringify(result5.config, null, 2));

  // Get typed values
  const port = getValue<number>(result5.config, 'port');
  const host = getValue<string>(result5.config, 'host');
  console.log(`\nUsing: ${host}:${port}`);
}
