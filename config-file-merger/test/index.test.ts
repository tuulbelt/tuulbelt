/**
 * Config File Merger Tests
 */

import { test, describe } from 'node:test';
import assert from 'node:assert/strict';
import { writeFileSync, unlinkSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { execSync } from 'node:child_process';

import {
  parseEnv,
  parseJsonFile,
  parseCliArgs,
  parseValue,
  mergeConfig,
  getValue,
  type ConfigSource,
} from '../src/index.ts';

// ============================================================================
// parseValue Tests
// ============================================================================

describe('parseValue', () => {
  test('parses boolean true', () => {
    assert.strictEqual(parseValue('true'), true);
  });

  test('parses boolean false', () => {
    assert.strictEqual(parseValue('false'), false);
  });

  test('parses null', () => {
    assert.strictEqual(parseValue('null'), null);
  });

  test('parses integer', () => {
    assert.strictEqual(parseValue('42'), 42);
  });

  test('parses negative integer', () => {
    assert.strictEqual(parseValue('-10'), -10);
  });

  test('parses float', () => {
    assert.strictEqual(parseValue('3.14'), 3.14);
  });

  test('parses zero', () => {
    assert.strictEqual(parseValue('0'), 0);
  });

  test('parses string', () => {
    assert.strictEqual(parseValue('hello'), 'hello');
  });

  test('parses empty string', () => {
    assert.strictEqual(parseValue(''), '');
  });

  test('strips double quotes', () => {
    assert.strictEqual(parseValue('"hello world"'), 'hello world');
  });

  test('strips single quotes', () => {
    assert.strictEqual(parseValue("'hello world'"), 'hello world');
  });

  test('preserves string that looks like number with quotes', () => {
    assert.strictEqual(parseValue('"42"'), '42');
  });

  test('preserves whitespace in quoted strings', () => {
    assert.strictEqual(parseValue('"  spaces  "'), '  spaces  ');
  });
});

// ============================================================================
// parseEnv Tests
// ============================================================================

describe('parseEnv', () => {
  test('parses all env vars without prefix', () => {
    const env = { FOO: 'bar', BAZ: 'qux' };
    const result = parseEnv(env);

    assert.strictEqual(result['foo'].value, 'bar');
    assert.strictEqual(result['foo'].source, 'env');
    assert.strictEqual(result['baz'].value, 'qux');
  });

  test('filters by prefix', () => {
    const env = { APP_PORT: '3000', OTHER_VAR: 'value' };
    const result = parseEnv(env, 'APP_');

    assert.ok('port' in result);
    assert.ok(!('other_var' in result));
    assert.strictEqual(result['port'].value, '3000');
  });

  test('keeps prefix when stripPrefix is false', () => {
    const env = { APP_PORT: '3000' };
    const result = parseEnv(env, 'APP_', false);

    assert.ok('app_port' in result);
    assert.ok(!('port' in result));
  });

  test('preserves case when lowercase is false', () => {
    const env = { APP_PORT: '3000' };
    const result = parseEnv(env, 'APP_', true, false);

    assert.ok('PORT' in result);
    assert.ok(!('port' in result));
  });

  test('skips undefined values', () => {
    const env: Record<string, string | undefined> = { FOO: 'bar', BAZ: undefined };
    const result = parseEnv(env);

    assert.ok('foo' in result);
    assert.ok(!('baz' in result));
  });

  test('returns empty object for empty env', () => {
    const result = parseEnv({});
    assert.deepStrictEqual(result, {});
  });

  test('handles complex prefix filtering', () => {
    const env = {
      MY_APP_HOST: 'localhost',
      MY_APP_PORT: '8080',
      MY_APP_DEBUG: 'true',
      OTHER_VAR: 'ignored',
    };
    const result = parseEnv(env, 'MY_APP_');

    assert.strictEqual(Object.keys(result).length, 3);
    assert.strictEqual(result['host'].value, 'localhost');
    assert.strictEqual(result['port'].value, '8080');
    assert.strictEqual(result['debug'].value, 'true');
  });
});

// ============================================================================
// parseCliArgs Tests
// ============================================================================

describe('parseCliArgs', () => {
  test('parses single key=value', () => {
    const result = parseCliArgs('port=3000');

    assert.ok('port' in result);
    assert.strictEqual(result['port'].value, 3000);
    assert.strictEqual(result['port'].source, 'cli');
  });

  test('parses multiple key=value pairs', () => {
    const result = parseCliArgs('host=localhost,port=3000,debug=true');

    assert.strictEqual(result['host'].value, 'localhost');
    assert.strictEqual(result['port'].value, 3000);
    assert.strictEqual(result['debug'].value, true);
  });

  test('handles spaces around commas', () => {
    const result = parseCliArgs('a=1, b=2 , c=3');

    assert.strictEqual(result['a'].value, 1);
    assert.strictEqual(result['b'].value, 2);
    assert.strictEqual(result['c'].value, 3);
  });

  test('handles empty string', () => {
    const result = parseCliArgs('');
    assert.deepStrictEqual(result, {});
  });

  test('handles whitespace-only string', () => {
    const result = parseCliArgs('   ');
    assert.deepStrictEqual(result, {});
  });

  test('skips pairs without equals sign', () => {
    const result = parseCliArgs('valid=1,invalid,also_valid=2');

    assert.ok('valid' in result);
    assert.ok('also_valid' in result);
    assert.ok(!('invalid' in result));
  });

  test('handles values with equals signs', () => {
    const result = parseCliArgs('url=http://example.com?foo=bar');

    assert.strictEqual(result['url'].value, 'http://example.com?foo=bar');
  });

  test('parses boolean and null values', () => {
    const result = parseCliArgs('enabled=true,disabled=false,empty=null');

    assert.strictEqual(result['enabled'].value, true);
    assert.strictEqual(result['disabled'].value, false);
    assert.strictEqual(result['empty'].value, null);
  });

  test('handles quoted string values', () => {
    const result = parseCliArgs('message="hello world"');

    assert.strictEqual(result['message'].value, 'hello world');
  });
});

// ============================================================================
// parseJsonFile Tests
// ============================================================================

describe('parseJsonFile', () => {
  let tmpDir: string;

  test('parses valid JSON file', () => {
    tmpDir = mkdtempSync(join(tmpdir(), 'cfgmerge-test-'));
    const filePath = join(tmpDir, 'config.json');
    writeFileSync(filePath, '{"port": 3000, "host": "localhost"}');

    const result = parseJsonFile(filePath);

    assert.ok(result.ok);
    assert.deepStrictEqual(result.data, { port: 3000, host: 'localhost' });

    unlinkSync(filePath);
  });

  test('returns error for non-existent file', () => {
    const result = parseJsonFile('/non/existent/file.json');

    assert.ok(!result.ok);
    assert.ok(result.error.includes('File not found'));
  });

  test('returns error for invalid JSON', () => {
    tmpDir = mkdtempSync(join(tmpdir(), 'cfgmerge-test-'));
    const filePath = join(tmpDir, 'invalid.json');
    writeFileSync(filePath, '{ invalid json }');

    const result = parseJsonFile(filePath);

    assert.ok(!result.ok);
    assert.ok(result.error.includes('Failed to parse'));

    unlinkSync(filePath);
  });

  test('returns error for JSON array', () => {
    tmpDir = mkdtempSync(join(tmpdir(), 'cfgmerge-test-'));
    const filePath = join(tmpDir, 'array.json');
    writeFileSync(filePath, '[1, 2, 3]');

    const result = parseJsonFile(filePath);

    assert.ok(!result.ok);
    assert.ok(result.error.includes('must contain a JSON object'));

    unlinkSync(filePath);
  });

  test('returns error for JSON null', () => {
    tmpDir = mkdtempSync(join(tmpdir(), 'cfgmerge-test-'));
    const filePath = join(tmpDir, 'null.json');
    writeFileSync(filePath, 'null');

    const result = parseJsonFile(filePath);

    assert.ok(!result.ok);
    assert.ok(result.error.includes('must contain a JSON object'));

    unlinkSync(filePath);
  });

  test('parses nested JSON', () => {
    tmpDir = mkdtempSync(join(tmpdir(), 'cfgmerge-test-'));
    const filePath = join(tmpDir, 'nested.json');
    writeFileSync(filePath, '{"server": {"port": 3000}, "logging": {"level": "debug"}}');

    const result = parseJsonFile(filePath);

    assert.ok(result.ok);
    assert.deepStrictEqual(result.data, {
      server: { port: 3000 },
      logging: { level: 'debug' },
    });

    unlinkSync(filePath);
  });
});

// ============================================================================
// mergeConfig Tests
// ============================================================================

describe('mergeConfig', () => {
  test('merges defaults only', () => {
    const result = mergeConfig({
      defaults: { port: 8080, host: 'localhost' },
    });

    assert.ok(result.ok);
    assert.deepStrictEqual(result.config, { port: 8080, host: 'localhost' });
  });

  test('file overrides defaults', () => {
    const result = mergeConfig({
      defaults: { port: 8080, host: 'localhost' },
      file: { port: 3000 },
    });

    assert.ok(result.ok);
    assert.deepStrictEqual(result.config, { port: 3000, host: 'localhost' });
  });

  test('env overrides file', () => {
    const result = mergeConfig({
      file: { port: 3000, host: 'localhost' },
      env: { PORT: '8080' },
      envPrefix: undefined,
      lowercaseEnv: true,
    });

    assert.ok(result.ok);
    assert.strictEqual((result.config as Record<string, unknown>)['port'], '8080');
  });

  test('cli overrides everything', () => {
    const result = mergeConfig({
      defaults: { port: 8080 },
      file: { port: 3000 },
      env: { PORT: '9000' },
      cli: { port: 4000 },
      lowercaseEnv: true,
    });

    assert.ok(result.ok);
    assert.strictEqual((result.config as Record<string, unknown>)['port'], 4000);
  });

  test('tracks sources when enabled', () => {
    const result = mergeConfig({
      defaults: { a: 1 },
      file: { b: 2 },
      cli: { c: 3 },
      trackSources: true,
    });

    assert.ok(result.ok);
    const config = result.config as Record<string, { value: unknown; source: ConfigSource }>;

    assert.strictEqual(config['a'].value, 1);
    assert.strictEqual(config['a'].source, 'default');
    assert.strictEqual(config['b'].value, 2);
    assert.strictEqual(config['b'].source, 'file');
    assert.strictEqual(config['c'].value, 3);
    assert.strictEqual(config['c'].source, 'cli');
  });

  test('handles empty options', () => {
    const result = mergeConfig({});

    assert.ok(result.ok);
    assert.deepStrictEqual(result.config, {});
  });

  test('handles env prefix filtering', () => {
    const result = mergeConfig({
      env: { APP_PORT: '3000', OTHER: 'ignored' },
      envPrefix: 'APP_',
    });

    assert.ok(result.ok);
    assert.ok('port' in (result.config as Record<string, unknown>));
    assert.ok(!('other' in (result.config as Record<string, unknown>)));
  });

  test('preserves all keys from all sources', () => {
    const result = mergeConfig({
      defaults: { a: 1 },
      file: { b: 2 },
      env: { C: '3' },
      cli: { d: 4 },
      lowercaseEnv: true,
    });

    assert.ok(result.ok);
    const config = result.config as Record<string, unknown>;

    assert.strictEqual(config['a'], 1);
    assert.strictEqual(config['b'], 2);
    assert.strictEqual(config['c'], '3');
    assert.strictEqual(config['d'], 4);
  });
});

// ============================================================================
// getValue Tests
// ============================================================================

describe('getValue', () => {
  test('gets value from simple config', () => {
    const config = { port: 3000, host: 'localhost' };

    assert.strictEqual(getValue(config, 'port'), 3000);
    assert.strictEqual(getValue(config, 'host'), 'localhost');
  });

  test('gets value from tracked config', () => {
    const config = {
      port: { value: 3000, source: 'cli' as ConfigSource },
    };

    assert.strictEqual(getValue(config, 'port'), 3000);
  });

  test('returns default for missing key', () => {
    const config = { port: 3000 };

    assert.strictEqual(getValue(config, 'missing', 'default'), 'default');
  });

  test('returns undefined for missing key without default', () => {
    const config = { port: 3000 };

    assert.strictEqual(getValue(config, 'missing'), undefined);
  });

  test('returns typed value', () => {
    const config = { count: 42 };

    const value = getValue<number>(config, 'count');
    assert.strictEqual(value, 42);
  });
});

// ============================================================================
// CLI Integration Tests
// ============================================================================

describe('CLI', () => {
  const cliPath = join(import.meta.dirname, '..', 'src', 'index.ts');
  let tmpDir: string;

  function runCli(args: string, env: Record<string, string> = {}): string {
    const fullEnv = { ...process.env, ...env };
    return execSync(`npx tsx "${cliPath}" ${args}`, {
      encoding: 'utf-8',
      env: fullEnv,
    }).trim();
  }

  test('shows help', () => {
    const output = runCli('--help');

    assert.ok(output.includes('cfgmerge'));
    assert.ok(output.includes('USAGE'));
    assert.ok(output.includes('OPTIONS'));
  });

  test('shows version', () => {
    const output = runCli('--version');

    assert.ok(output.includes('0.1.0'));
  });

  test('merges CLI args', () => {
    const output = runCli('--args "port=3000,debug=true"');
    const config = JSON.parse(output);

    assert.strictEqual(config.port, 3000);
    assert.strictEqual(config.debug, true);
  });

  test('merges config file', () => {
    tmpDir = mkdtempSync(join(tmpdir(), 'cfgmerge-cli-'));
    const configPath = join(tmpDir, 'config.json');
    writeFileSync(configPath, '{"host": "localhost", "port": 8080}');

    const output = runCli(`--file "${configPath}"`);
    const config = JSON.parse(output);

    assert.strictEqual(config.host, 'localhost');
    assert.strictEqual(config.port, 8080);

    unlinkSync(configPath);
  });

  test('cli args override file', () => {
    tmpDir = mkdtempSync(join(tmpdir(), 'cfgmerge-cli-'));
    const configPath = join(tmpDir, 'config.json');
    writeFileSync(configPath, '{"port": 8080}');

    const output = runCli(`--file "${configPath}" --args "port=3000"`);
    const config = JSON.parse(output);

    assert.strictEqual(config.port, 3000);

    unlinkSync(configPath);
  });

  test('tracks sources', () => {
    tmpDir = mkdtempSync(join(tmpdir(), 'cfgmerge-cli-'));
    const configPath = join(tmpDir, 'config.json');
    writeFileSync(configPath, '{"from_file": true}');

    const output = runCli(`--file "${configPath}" --args "from_cli=true" --track-sources`);
    const config = JSON.parse(output);

    assert.strictEqual(config.from_file.value, true);
    assert.strictEqual(config.from_file.source, 'file');
    assert.strictEqual(config.from_cli.value, true);
    assert.strictEqual(config.from_cli.source, 'cli');

    unlinkSync(configPath);
  });

  test('merges env vars with prefix', () => {
    const output = runCli('--env --prefix TEST_CFG_', {
      TEST_CFG_PORT: '9000',
      TEST_CFG_HOST: 'example.com',
      OTHER_VAR: 'ignored',
    });
    const config = JSON.parse(output);

    assert.strictEqual(config.port, '9000');
    assert.strictEqual(config.host, 'example.com');
    assert.ok(!('other_var' in config));
  });

  test('empty output without sources', () => {
    const output = runCli('');
    const config = JSON.parse(output);

    assert.deepStrictEqual(config, {});
  });

  test('returns error for missing file', () => {
    try {
      runCli('--file /non/existent/file.json');
      assert.fail('Should have thrown');
    } catch (err) {
      const error = err as Error & { stderr?: string };
      assert.ok(error.stderr?.includes('File not found') || error.message.includes('File not found'));
    }
  });
});

// ============================================================================
// Edge Cases
// ============================================================================

describe('Edge Cases', () => {
  test('handles special characters in values', () => {
    const result = parseCliArgs('url=https://example.com?a=1&b=2');

    assert.strictEqual(result['url'].value, 'https://example.com?a=1&b=2');
  });

  test('handles empty env prefix', () => {
    const env = { FOO: 'bar' };
    const result = parseEnv(env, '');

    assert.strictEqual(result['foo'].value, 'bar');
  });

  test('handles unicode in values', () => {
    const result = parseCliArgs('emoji=🎉,japanese=日本語');

    assert.strictEqual(result['emoji'].value, '🎉');
    assert.strictEqual(result['japanese'].value, '日本語');
  });

  test('handles very long values', () => {
    const longValue = 'x'.repeat(10000);
    const result = parseCliArgs(`long=${longValue}`);

    assert.strictEqual(result['long'].value, longValue);
  });

  test('handles numeric-looking strings', () => {
    const result = parseCliArgs('version="1.0.0",phone="555-1234"');

    assert.strictEqual(result['version'].value, '1.0.0');
    assert.strictEqual(result['phone'].value, '555-1234');
  });

  test('preserves nested objects from file', () => {
    const result = mergeConfig({
      file: {
        server: { host: 'localhost', port: 3000 },
        database: { url: 'postgres://...' },
      },
    });

    assert.ok(result.ok);
    const config = result.config as Record<string, unknown>;

    assert.deepStrictEqual(config['server'], { host: 'localhost', port: 3000 });
  });

  test('handles scientific notation numbers', () => {
    const result = parseCliArgs('big=1e10,small=1e-5');

    assert.strictEqual(result['big'].value, 1e10);
    assert.strictEqual(result['small'].value, 1e-5);
  });
});
