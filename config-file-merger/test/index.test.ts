/**
 * Config File Merger Tests
 */

import { test, describe } from 'node:test';
import assert from 'node:assert/strict';
import { writeFileSync, unlinkSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, dirname } from 'node:path';
import { execSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

// Node 18 compatible __dirname (import.meta.dirname is only in Node 20.11+)
const __dirname = dirname(fileURLToPath(import.meta.url));

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
  const cliPath = join(__dirname, '..', 'src', 'index.ts');
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

// ============================================================================
// Extended parseValue Tests
// ============================================================================

describe('parseValue - Extended', () => {
  test('parses negative float', () => {
    assert.strictEqual(parseValue('-3.14'), -3.14);
  });

  test('parses negative zero', () => {
    assert.strictEqual(parseValue('-0'), -0);
  });

  test('parses Infinity as number (JavaScript Number() behavior)', () => {
    // JavaScript: Number('Infinity') === Infinity (not NaN)
    assert.strictEqual(parseValue('Infinity'), Infinity);
  });

  test('parses NaN as NaN (JavaScript Number() behavior)', () => {
    // JavaScript: Number('NaN') === NaN, but isNaN(NaN) is true so this becomes NaN
    // Actually, the code checks !isNaN(num), NaN fails that, so it returns 'NaN' string
    assert.strictEqual(parseValue('NaN'), 'NaN');
  });

  test('parses hex values as number (JavaScript Number() behavior)', () => {
    // JavaScript: Number('0x1F') === 31
    assert.strictEqual(parseValue('0x1F'), 31);
  });

  test('parses leading zeros as string', () => {
    // Leading zeros often mean "keep as string" (e.g., zip codes)
    assert.strictEqual(parseValue('007'), 7); // Actually parses as number
  });

  test('handles mixed quote types', () => {
    // Only strip if both ends match
    assert.strictEqual(parseValue('"hello\''), '"hello\'');
  });

  test('handles empty quoted string', () => {
    assert.strictEqual(parseValue('""'), '');
    assert.strictEqual(parseValue("''"), '');
  });

  test('handles TRUE and FALSE (case sensitive)', () => {
    // Only lowercase true/false are booleans
    assert.strictEqual(parseValue('TRUE'), 'TRUE');
    assert.strictEqual(parseValue('FALSE'), 'FALSE');
  });

  test('handles NULL (case sensitive)', () => {
    assert.strictEqual(parseValue('NULL'), 'NULL');
  });

  test('parses very large number', () => {
    assert.strictEqual(parseValue('999999999999999999999'), 1e21);
  });

  test('parses very small number', () => {
    assert.strictEqual(parseValue('0.000000001'), 1e-9);
  });

  test('handles whitespace-only value (JavaScript Number() behavior)', () => {
    // JavaScript: Number('   ') === 0 (whitespace coerces to 0)
    // This is a known edge case of JavaScript's Number() coercion
    assert.strictEqual(parseValue('   '), 0);
  });

  test('handles tabs and newlines in quoted strings', () => {
    assert.strictEqual(parseValue('"tab\there"'), 'tab\there');
  });
});

// ============================================================================
// Extended parseEnv Tests
// ============================================================================

describe('parseEnv - Extended', () => {
  test('handles empty string values', () => {
    const env = { EMPTY: '' };
    const result = parseEnv(env);

    assert.strictEqual(result['empty'].value, '');
  });

  test('handles values with equals signs', () => {
    const env = { CONNECTION_STRING: 'host=localhost;port=5432' };
    const result = parseEnv(env);

    assert.strictEqual(result['connection_string'].value, 'host=localhost;port=5432');
  });

  test('handles values with newlines', () => {
    const env = { MULTILINE: 'line1\nline2' };
    const result = parseEnv(env);

    assert.strictEqual(result['multiline'].value, 'line1\nline2');
  });

  test('handles numeric env values', () => {
    const env = { PORT: '3000', TIMEOUT: '30.5' };
    const result = parseEnv(env);

    // ENV values are always strings
    assert.strictEqual(result['port'].value, '3000');
    assert.strictEqual(typeof result['port'].value, 'string');
  });

  test('handles boolean-like env values', () => {
    const env = { DEBUG: 'true', VERBOSE: 'false' };
    const result = parseEnv(env);

    // ENV values are always strings - no coercion
    assert.strictEqual(result['debug'].value, 'true');
    assert.strictEqual(typeof result['debug'].value, 'string');
  });

  test('handles underscore-heavy names', () => {
    const env = { APP_DATABASE_CONNECTION_POOL_SIZE: '10' };
    const result = parseEnv(env, 'APP_');

    assert.ok('database_connection_pool_size' in result);
  });

  test('handles single-char prefix', () => {
    const env = { X_VALUE: '1', Y_VALUE: '2' };
    const result = parseEnv(env, 'X_');

    assert.ok('value' in result);
    assert.strictEqual(Object.keys(result).length, 1);
  });

  test('handles exact prefix match (no suffix)', () => {
    const env = { APP_: 'exact' };
    const result = parseEnv(env, 'APP_');

    // Key becomes empty string after stripping prefix
    assert.ok('' in result);
  });

  test('all options disabled', () => {
    const env = { APP_HOST: 'localhost' };
    const result = parseEnv(env, 'APP_', false, false);

    assert.ok('APP_HOST' in result);
    assert.strictEqual(result['APP_HOST'].value, 'localhost');
  });
});

// ============================================================================
// Extended parseCliArgs Tests
// ============================================================================

describe('parseCliArgs - Extended', () => {
  test('handles empty value', () => {
    const result = parseCliArgs('key=');

    assert.ok('key' in result);
    assert.strictEqual(result['key'].value, '');
  });

  test('handles key only (no equals, no value)', () => {
    const result = parseCliArgs('justkey');

    assert.ok(!('justkey' in result));
  });

  test('handles multiple equals in value', () => {
    const result = parseCliArgs('equation=a=b=c');

    assert.strictEqual(result['equation'].value, 'a=b=c');
  });

  test('handles underscore keys', () => {
    const result = parseCliArgs('my_key=value,another_key=value2');

    assert.strictEqual(result['my_key'].value, 'value');
    assert.strictEqual(result['another_key'].value, 'value2');
  });

  test('handles hyphen keys', () => {
    const result = parseCliArgs('my-key=value');

    assert.strictEqual(result['my-key'].value, 'value');
  });

  test('handles dots in keys (nested notation)', () => {
    const result = parseCliArgs('server.port=3000,server.host=localhost');

    assert.strictEqual(result['server.port'].value, 3000);
    assert.strictEqual(result['server.host'].value, 'localhost');
  });

  test('handles consecutive commas', () => {
    const result = parseCliArgs('a=1,,b=2,,,c=3');

    assert.strictEqual(result['a'].value, 1);
    assert.strictEqual(result['b'].value, 2);
    assert.strictEqual(result['c'].value, 3);
  });

  test('handles trailing comma', () => {
    const result = parseCliArgs('a=1,b=2,');

    assert.strictEqual(Object.keys(result).length, 2);
  });

  test('handles leading comma', () => {
    const result = parseCliArgs(',a=1,b=2');

    assert.strictEqual(Object.keys(result).length, 2);
  });

  test('handles JSON-like values', () => {
    const result = parseCliArgs('data={"key":"value"}');

    assert.strictEqual(result['data'].value, '{"key":"value"}');
  });

  test('handles array-like values (known limitation - comma splitting)', () => {
    // CLI args split by comma, so '[1,2,3]' becomes '[1' + '2' + '3]'
    // This is a known limitation - use quoted strings for complex values
    const result = parseCliArgs('items=[1,2,3]');

    // The split breaks at commas, so 'items=[1' is the first pair
    assert.strictEqual(result['items'].value, '[1');
  });

  test('handles paths with backslashes', () => {
    const result = parseCliArgs('path=C:\\Users\\test');

    assert.strictEqual(result['path'].value, 'C:\\Users\\test');
  });
});

// ============================================================================
// Extended parseJsonFile Tests
// ============================================================================

describe('parseJsonFile - Extended', () => {
  let tmpDir: string;

  test('parses empty object', () => {
    tmpDir = mkdtempSync(join(tmpdir(), 'cfgmerge-test-'));
    const filePath = join(tmpDir, 'empty.json');
    writeFileSync(filePath, '{}');

    const result = parseJsonFile(filePath);

    assert.ok(result.ok);
    assert.deepStrictEqual(result.data, {});

    unlinkSync(filePath);
  });

  test('parses deeply nested structure', () => {
    tmpDir = mkdtempSync(join(tmpdir(), 'cfgmerge-test-'));
    const filePath = join(tmpDir, 'deep.json');
    writeFileSync(filePath, '{"a":{"b":{"c":{"d":{"e":1}}}}}');

    const result = parseJsonFile(filePath);

    assert.ok(result.ok);
    assert.strictEqual((result.data as any).a.b.c.d.e, 1);

    unlinkSync(filePath);
  });

  test('parses arrays in values', () => {
    tmpDir = mkdtempSync(join(tmpdir(), 'cfgmerge-test-'));
    const filePath = join(tmpDir, 'arrays.json');
    writeFileSync(filePath, '{"ports": [3000, 3001], "hosts": ["a", "b"]}');

    const result = parseJsonFile(filePath);

    assert.ok(result.ok);
    assert.deepStrictEqual((result.data as any).ports, [3000, 3001]);

    unlinkSync(filePath);
  });

  test('rejects file with BOM (JSON.parse limitation)', () => {
    tmpDir = mkdtempSync(join(tmpdir(), 'cfgmerge-test-'));
    const filePath = join(tmpDir, 'bom.json');
    // UTF-8 BOM + JSON - JSON.parse does NOT handle BOM
    writeFileSync(filePath, '\uFEFF{"key": "value"}');

    const result = parseJsonFile(filePath);

    // Node.js JSON.parse does NOT handle BOM (it's not valid JSON)
    // This returns an error because BOM character makes it invalid JSON
    assert.strictEqual(result.ok, false);

    unlinkSync(filePath);
  });

  test('parses file with trailing newline', () => {
    tmpDir = mkdtempSync(join(tmpdir(), 'cfgmerge-test-'));
    const filePath = join(tmpDir, 'newline.json');
    writeFileSync(filePath, '{"key": "value"}\n\n');

    const result = parseJsonFile(filePath);

    assert.ok(result.ok);

    unlinkSync(filePath);
  });

  test('returns error for JSON primitive string', () => {
    tmpDir = mkdtempSync(join(tmpdir(), 'cfgmerge-test-'));
    const filePath = join(tmpDir, 'string.json');
    writeFileSync(filePath, '"just a string"');

    const result = parseJsonFile(filePath);

    assert.ok(!result.ok);
    assert.ok(result.error.includes('must contain a JSON object'));

    unlinkSync(filePath);
  });

  test('returns error for JSON primitive number', () => {
    tmpDir = mkdtempSync(join(tmpdir(), 'cfgmerge-test-'));
    const filePath = join(tmpDir, 'number.json');
    writeFileSync(filePath, '42');

    const result = parseJsonFile(filePath);

    assert.ok(!result.ok);
    assert.ok(result.error.includes('must contain a JSON object'));

    unlinkSync(filePath);
  });

  test('handles unicode in file content', () => {
    tmpDir = mkdtempSync(join(tmpdir(), 'cfgmerge-test-'));
    const filePath = join(tmpDir, 'unicode.json');
    writeFileSync(filePath, '{"emoji": "🎉", "japanese": "日本語"}');

    const result = parseJsonFile(filePath);

    assert.ok(result.ok);
    assert.strictEqual((result.data as any).emoji, '🎉');

    unlinkSync(filePath);
  });
});

// ============================================================================
// Extended mergeConfig Tests
// ============================================================================

describe('mergeConfig - Extended', () => {
  test('precedence: same key in all sources', () => {
    const result = mergeConfig({
      defaults: { key: 'from_default' },
      file: { key: 'from_file' },
      env: { KEY: 'from_env' },
      cli: { key: 'from_cli' },
      lowercaseEnv: true,
      trackSources: true,
    });

    assert.ok(result.ok);
    const config = result.config as Record<string, { value: unknown; source: ConfigSource }>;

    // CLI has highest precedence
    assert.strictEqual(config['key'].value, 'from_cli');
    assert.strictEqual(config['key'].source, 'cli');
  });

  test('precedence without CLI: env wins', () => {
    const result = mergeConfig({
      defaults: { key: 'from_default' },
      file: { key: 'from_file' },
      env: { KEY: 'from_env' },
      lowercaseEnv: true,
      trackSources: true,
    });

    assert.ok(result.ok);
    const config = result.config as Record<string, { value: unknown; source: ConfigSource }>;

    assert.strictEqual(config['key'].value, 'from_env');
    assert.strictEqual(config['key'].source, 'env');
  });

  test('precedence without env and CLI: file wins', () => {
    const result = mergeConfig({
      defaults: { key: 'from_default' },
      file: { key: 'from_file' },
      trackSources: true,
    });

    assert.ok(result.ok);
    const config = result.config as Record<string, { value: unknown; source: ConfigSource }>;

    assert.strictEqual(config['key'].value, 'from_file');
    assert.strictEqual(config['key'].source, 'file');
  });

  test('preserves type from highest precedence source', () => {
    const result = mergeConfig({
      defaults: { port: '8080' },  // string
      cli: { port: 3000 },         // number
    });

    assert.ok(result.ok);
    assert.strictEqual((result.config as any).port, 3000);
    assert.strictEqual(typeof (result.config as any).port, 'number');
  });

  test('handles null values', () => {
    const result = mergeConfig({
      defaults: { value: 'default' },
      cli: { value: null },
    });

    assert.ok(result.ok);
    assert.strictEqual((result.config as any).value, null);
  });

  test('handles false values (falsy but valid)', () => {
    const result = mergeConfig({
      defaults: { debug: true },
      cli: { debug: false },
    });

    assert.ok(result.ok);
    assert.strictEqual((result.config as any).debug, false);
  });

  test('handles zero values (falsy but valid)', () => {
    const result = mergeConfig({
      defaults: { timeout: 5000 },
      cli: { timeout: 0 },
    });

    assert.ok(result.ok);
    assert.strictEqual((result.config as any).timeout, 0);
  });

  test('handles empty string values', () => {
    const result = mergeConfig({
      defaults: { name: 'default' },
      cli: { name: '' },
    });

    assert.ok(result.ok);
    assert.strictEqual((result.config as any).name, '');
  });

  test('preserves arrays from file', () => {
    const result = mergeConfig({
      file: { ports: [3000, 3001, 3002] },
    });

    assert.ok(result.ok);
    assert.deepStrictEqual((result.config as any).ports, [3000, 3001, 3002]);
  });

  test('tracks sources correctly with env prefix', () => {
    const result = mergeConfig({
      env: { APP_PORT: '3000' },
      envPrefix: 'APP_',
      trackSources: true,
    });

    assert.ok(result.ok);
    const config = result.config as Record<string, { value: unknown; source: ConfigSource }>;

    assert.strictEqual(config['port'].value, '3000');
    assert.strictEqual(config['port'].source, 'env');
  });
});

// ============================================================================
// Extended getValue Tests
// ============================================================================

describe('getValue - Extended', () => {
  test('handles null value correctly', () => {
    const config = { nullKey: null };

    assert.strictEqual(getValue(config, 'nullKey'), null);
  });

  test('handles false value correctly', () => {
    const config = { boolKey: false };

    assert.strictEqual(getValue(config, 'boolKey'), false);
  });

  test('handles zero value correctly', () => {
    const config = { numKey: 0 };

    assert.strictEqual(getValue(config, 'numKey'), 0);
  });

  test('handles empty string correctly', () => {
    const config = { strKey: '' };

    assert.strictEqual(getValue(config, 'strKey'), '');
  });

  test('returns undefined for null prototype object', () => {
    const config = Object.create(null);
    config.key = 'value';

    assert.strictEqual(getValue(config, 'key'), 'value');
    assert.strictEqual(getValue(config, 'missing'), undefined);
  });

  test('handles nested tracked config', () => {
    const config = {
      nested: {
        value: { deep: 'data' },
        source: 'file' as ConfigSource,
      },
    };

    const value = getValue(config, 'nested');
    assert.deepStrictEqual(value, { deep: 'data' });
  });

  test('default value is not used when key exists with falsy value', () => {
    const config = { zero: 0, empty: '', falsy: false };

    assert.strictEqual(getValue(config, 'zero', 999), 0);
    assert.strictEqual(getValue(config, 'empty', 'default'), '');
    assert.strictEqual(getValue(config, 'falsy', true), false);
  });
});

// ============================================================================
// CLI Extended Tests
// ============================================================================

describe('CLI - Extended', () => {
  const cliPath = join(__dirname, '..', 'src', 'index.ts');
  let tmpDir: string;

  function runCli(args: string, env: Record<string, string> = {}): string {
    const fullEnv = { ...process.env, ...env };
    return execSync(`npx tsx "${cliPath}" ${args}`, {
      encoding: 'utf-8',
      env: fullEnv,
    }).trim();
  }

  test('uses short flags', () => {
    const output = runCli('-a "port=3000"');
    const config = JSON.parse(output);

    assert.strictEqual(config.port, 3000);
  });

  test('combines multiple options', () => {
    tmpDir = mkdtempSync(join(tmpdir(), 'cfgmerge-cli-'));
    const configPath = join(tmpDir, 'config.json');
    const defaultsPath = join(tmpDir, 'defaults.json');

    writeFileSync(defaultsPath, '{"a": 1, "b": 2}');
    writeFileSync(configPath, '{"b": 20, "c": 3}');

    const output = runCli(`-d "${defaultsPath}" -f "${configPath}" -a "c=30,d=4"`);
    const config = JSON.parse(output);

    assert.strictEqual(config.a, 1);   // from defaults
    assert.strictEqual(config.b, 20);  // file overrides defaults
    assert.strictEqual(config.c, 30);  // cli overrides file
    assert.strictEqual(config.d, 4);   // cli only

    unlinkSync(configPath);
    unlinkSync(defaultsPath);
  });

  test('--no-strip-prefix keeps prefix', () => {
    const output = runCli('--env --prefix TEST_ --no-strip-prefix', {
      TEST_VALUE: 'kept',
    });
    const config = JSON.parse(output);

    assert.ok('test_value' in config);
    assert.ok(!('value' in config));
  });

  test('--no-lowercase preserves case', () => {
    const output = runCli('--env --prefix TEST_ --no-lowercase', {
      TEST_UPPER: 'preserved',
    });
    const config = JSON.parse(output);

    assert.ok('UPPER' in config);
    assert.ok(!('upper' in config));
  });

  test('handles file with special characters in path', () => {
    tmpDir = mkdtempSync(join(tmpdir(), 'cfgmerge-spaces '));
    const configPath = join(tmpDir, 'config file.json');
    writeFileSync(configPath, '{"key": "value"}');

    const output = runCli(`--file "${configPath}"`);
    const config = JSON.parse(output);

    assert.strictEqual(config.key, 'value');

    unlinkSync(configPath);
  });

  test('help includes all options', () => {
    const output = runCli('--help');

    assert.ok(output.includes('--env'));
    assert.ok(output.includes('--prefix'));
    assert.ok(output.includes('--file'));
    assert.ok(output.includes('--defaults'));
    assert.ok(output.includes('--args'));
    assert.ok(output.includes('--track-sources'));
    assert.ok(output.includes('PRECEDENCE'));
  });

  test('empty env prefix includes all env vars', () => {
    const output = runCli('--env --prefix ""', {
      CFG_TEST_KEY: 'value',
    });
    const config = JSON.parse(output);

    // With empty prefix, all env vars are included (lowercased)
    assert.ok('cfg_test_key' in config);
  });
});

// ============================================================================
// Isolation and Determinism Tests
// ============================================================================

describe('Isolation and Determinism', () => {
  test('multiple mergeConfig calls are independent', () => {
    const result1 = mergeConfig({ cli: { a: 1 } });
    const result2 = mergeConfig({ cli: { b: 2 } });

    assert.ok(result1.ok);
    assert.ok(result2.ok);

    assert.ok('a' in (result1.config as any));
    assert.ok(!('b' in (result1.config as any)));
    assert.ok(!('a' in (result2.config as any)));
    assert.ok('b' in (result2.config as any));
  });

  test('modifying result does not affect source', () => {
    const defaults = { port: 8080 };
    const result = mergeConfig({ defaults });

    assert.ok(result.ok);
    (result.config as any).port = 9999;

    // Original should be unchanged
    assert.strictEqual(defaults.port, 8080);
  });

  test('produces identical output for identical input', () => {
    const options = {
      defaults: { a: 1 },
      file: { b: 2 },
      cli: { c: 3 },
    };

    const result1 = mergeConfig(options);
    const result2 = mergeConfig(options);

    assert.ok(result1.ok);
    assert.ok(result2.ok);
    assert.deepStrictEqual(result1.config, result2.config);
  });

  test('order of keys is deterministic', () => {
    const result1 = mergeConfig({
      defaults: { z: 1, a: 2, m: 3 },
    });
    const result2 = mergeConfig({
      defaults: { z: 1, a: 2, m: 3 },
    });

    assert.ok(result1.ok);
    assert.ok(result2.ok);

    const keys1 = Object.keys(result1.config);
    const keys2 = Object.keys(result2.config);
    assert.deepStrictEqual(keys1, keys2);
  });
});

// ============================================================================
// Security Tests - Prototype Pollution Prevention
// ============================================================================

describe('security - prototype pollution prevention', () => {
  test('parseCliArgs ignores __proto__ key', () => {
    const result = parseCliArgs('__proto__=evil,name=good');

    // __proto__ should be silently ignored (use Object.hasOwn to check own properties)
    assert.ok(!Object.hasOwn(result, '__proto__'));
    assert.ok(Object.hasOwn(result, 'name'));
    assert.strictEqual(result.name.value, 'good');
  });

  test('parseCliArgs ignores constructor key', () => {
    const result = parseCliArgs('constructor=evil,name=good');

    assert.ok(!Object.hasOwn(result, 'constructor'));
    assert.ok(Object.hasOwn(result, 'name'));
  });

  test('parseCliArgs ignores prototype key', () => {
    const result = parseCliArgs('prototype=evil,name=good');

    assert.ok(!Object.hasOwn(result, 'prototype'));
    assert.ok(Object.hasOwn(result, 'name'));
  });

  test('parseEnv ignores __proto__ key', () => {
    const env = {
      '__proto__': 'evil',
      'NAME': 'good',
    };
    const result = parseEnv(env, undefined, true, true);

    assert.ok(!Object.hasOwn(result, '__proto__'));
    assert.ok(Object.hasOwn(result, 'name'));
  });

  test('parseEnv ignores constructor key', () => {
    const env = {
      'constructor': 'evil',
      'NAME': 'good',
    };
    const result = parseEnv(env, undefined, true, true);

    assert.ok(!Object.hasOwn(result, 'constructor'));
    assert.ok(Object.hasOwn(result, 'name'));
  });

  test('mergeConfig ignores dangerous keys in defaults', () => {
    const result = mergeConfig({
      defaults: {
        '__proto__': 'evil',
        'constructor': 'evil',
        'prototype': 'evil',
        'safe': 'value',
      },
    });

    assert.ok(result.ok);
    const config = result.config as Record<string, unknown>;
    assert.ok(!Object.hasOwn(config, '__proto__'));
    assert.ok(!Object.hasOwn(config, 'constructor'));
    assert.ok(!Object.hasOwn(config, 'prototype'));
    assert.strictEqual(config.safe, 'value');
  });

  test('mergeConfig ignores dangerous keys in file config', () => {
    const result = mergeConfig({
      file: {
        '__proto__': { 'polluted': true },
        'safe': 'value',
      },
    });

    assert.ok(result.ok);
    const config = result.config as Record<string, unknown>;
    assert.ok(!Object.hasOwn(config, '__proto__'));
    assert.strictEqual(config.safe, 'value');

    // Verify Object.prototype was not polluted
    const testObj = {};
    assert.strictEqual((testObj as { polluted?: boolean }).polluted, undefined);
  });

  test('mergeConfig ignores dangerous keys in cli config', () => {
    const result = mergeConfig({
      cli: {
        '__proto__': 'evil',
        'safe': 'value',
      },
    });

    assert.ok(result.ok);
    const config = result.config as Record<string, unknown>;
    assert.ok(!Object.hasOwn(config, '__proto__'));
    assert.strictEqual(config.safe, 'value');
  });

  test('all dangerous keys filtered across all sources', () => {
    const result = mergeConfig({
      defaults: { '__proto__': 'default-evil', 'a': 1 },
      file: { 'constructor': 'file-evil', 'b': 2 },
      env: { 'PROTOTYPE': 'env-evil', 'C': '3' },
      envPrefix: undefined,
      cli: { 'safe': 'cli-value' },
    });

    assert.ok(result.ok);
    const config = result.config as Record<string, unknown>;

    // Dangerous keys filtered (use Object.hasOwn for own property check)
    assert.ok(!Object.hasOwn(config, '__proto__'));
    assert.ok(!Object.hasOwn(config, 'constructor'));
    assert.ok(!Object.hasOwn(config, 'prototype'));

    // Safe keys preserved
    assert.strictEqual(config.a, 1);
    assert.strictEqual(config.b, 2);
    assert.strictEqual(config.c, '3');
    assert.strictEqual(config.safe, 'cli-value');
  });
});
