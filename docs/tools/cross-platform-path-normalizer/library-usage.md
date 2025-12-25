# Library Usage

Use Cross-Platform Path Normalizer programmatically in your TypeScript/JavaScript projects.

## Installation

Since this is a standalone tool with zero runtime dependencies, you can either:

1. **Clone and import directly:**
```bash
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt/cross-platform-path-normalizer
```

2. **Copy the source files** into your project

## Import

```typescript
import {
  normalizePath,
  normalizeToUnix,
  normalizeToWindows,
  detectPathFormat,
  type NormalizeOptions,
  type NormalizeResult,
  type PathFormat
} from './src/index.js';
```

## Basic Usage

```typescript
// Normalize to Unix format
const result = normalizePath('C:\\Users\\Documents\\file.txt', { format: 'unix' });
if (result.success) {
  console.log(result.path);  // '/c/Users/Documents/file.txt'
} else {
  console.error('Error:', result.error);
}

// Quick conversion (no error handling)
const unixPath = normalizeToUnix('C:\\Program Files\\app');
// '/c/Program Files/app'

const winPath = normalizeToWindows('/home/user/documents');
// 'C:\\home\\user\\documents' (on Windows) or '\\home\\user\\documents' (on Unix)
```

## API Reference

### `normalizePath()`

Main function with full options and Result pattern for error handling.

```typescript
function normalizePath(
  path: string,
  options?: NormalizeOptions
): NormalizeResult
```

**Parameters:**
- `path` - The path to normalize (required)
- `options` - Optional configuration object:
  - `format?: 'unix' | 'windows' | 'auto'` - Target format (default: 'auto')
  - `absolute?: boolean` - Resolve to absolute path (default: false)
  - `verbose?: boolean` - Enable verbose output (default: false)

**Returns:** `NormalizeResult` object with:
- `success: boolean` - Whether the operation succeeded
- `path: string` - The normalized path
- `format: 'unix' | 'windows'` - Detected or specified format
- `error?: string` - Error message if success is false

**Example:**
```typescript
// Convert to Unix format
const result = normalizePath('C:\\Users\\file.txt', { format: 'unix' });
if (result.success) {
  console.log(result.path);  // '/c/Users/file.txt'
  console.log(result.format); // 'unix'
}

// Auto-detect format
const auto = normalizePath('/home/user/file.txt');
console.log(auto.format); // 'unix'

// Make absolute
const abs = normalizePath('../file.txt', { absolute: true, format: 'unix' });
```

---

### `normalizeToUnix()`

Convert any path to Unix format (forward slashes).

```typescript
function normalizeToUnix(path: string): string
```

**Parameters:**
- `path` - The path to convert

**Returns:** String in Unix format

**Example:**
```typescript
normalizeToUnix('C:\\Users\\Documents\\file.txt');
// '/c/Users/Documents/file.txt'

normalizeToUnix('/home/user/file.txt');
// '/home/user/file.txt' (unchanged)

normalizeToUnix('\\\\server\\share\\folder');
// '//server/share/folder' (UNC preserved)
```

---

### `normalizeToWindows()`

Convert any path to Windows format (backslashes).

```typescript
function normalizeToWindows(path: string): string
```

**Parameters:**
- `path` - The path to convert

**Returns:** String in Windows format

**Example:**
```typescript
normalizeToWindows('/c/Users/Documents/file.txt');
// 'C:\\Users\\Documents\\file.txt'

normalizeToWindows('C:\\Program Files');
// 'C:\\Program Files' (unchanged)

normalizeToWindows('//server/share/folder');
// '\\\\server\\share\\folder' (UNC preserved)
```

---

### `detectPathFormat()`

Detect whether a path is Windows or Unix format.

```typescript
function detectPathFormat(path: string): 'windows' | 'unix'
```

**Parameters:**
- `path` - The path to analyze

**Returns:** Either `'windows'` or `'unix'`

**Detection Rules:**
- **Windows** if path contains:
  - Drive letter (e.g., `C:`, `D:`)
  - Backslashes (`\`)
  - UNC prefix (`\\server`)
- **Unix** otherwise

**Example:**
```typescript
detectPathFormat('C:\\Users\\file.txt');      // 'windows'
detectPathFormat('/home/user/file.txt');      // 'unix'
detectPathFormat('\\\\server\\share');        // 'windows'
detectPathFormat('./relative/path');          // 'unix'
```

## Type Definitions

### `PathFormat`

```typescript
type PathFormat = 'unix' | 'windows' | 'auto';
```

### `NormalizeOptions`

```typescript
interface NormalizeOptions {
  format?: PathFormat;    // Target format (default: 'auto')
  absolute?: boolean;     // Resolve to absolute path (default: false)
  verbose?: boolean;      // Enable verbose output (default: false)
}
```

### `NormalizeResult`

```typescript
interface NormalizeResult {
  success: boolean;       // Whether the operation succeeded
  path: string;           // The normalized path
  format: 'unix' | 'windows'; // Detected or specified format
  error?: string;         // Optional error message
}
```

## Advanced Examples

### Batch Processing

```typescript
const paths = [
  'C:\\Users\\Documents\\report.pdf',
  '/home/user/project/src/index.ts',
  '\\\\server\\shared\\data.csv'
];

const normalized = paths.map(p => normalizeToUnix(p));
console.log(normalized);
// [
//   '/c/Users/Documents/report.pdf',
//   '/home/user/project/src/index.ts',
//   '//server/shared/data.csv'
// ]
```

### Error Handling with Result Pattern

```typescript
function processPaths(userInputs: string[]): string[] {
  const results: string[] = [];

  for (const input of userInputs) {
    const result = normalizePath(input, { format: 'unix' });

    if (!result.success) {
      console.warn(`Skipping invalid path "${input}": ${result.error}`);
      continue;
    }

    results.push(result.path);
  }

  return results;
}
```

### Cross-Platform File Operations

```typescript
import { readFileSync } from 'node:fs';
import { normalizeToUnix } from './src/index.js';

function readCrossPlatformFile(pathFromUser: string): Buffer {
  // Normalize to Unix for consistent handling
  const normalized = normalizeToUnix(pathFromUser);

  // Node.js handles Unix paths on all platforms
  return readFileSync(normalized);
}

// Works on both Windows and Unix
const content = readCrossPlatformFile('C:\\Users\\data.json');
```

### Integration with Path Operations

```typescript
import { join, dirname, basename } from 'node:path';
import { normalizeToUnix } from './src/index.js';

function buildCrossPlatformPath(base: string, ...segments: string[]): string {
  // Join using node:path (platform-specific)
  const joined = join(base, ...segments);

  // Normalize to Unix for consistency
  return normalizeToUnix(joined);
}

const projectPath = buildCrossPlatformPath('C:\\Projects', 'myapp', 'src', 'index.ts');
// '/c/Projects/myapp/src/index.ts'
```

### Detecting Mixed Formats

```typescript
function analyzePaths(paths: string[]): void {
  const formats = paths.map(p => ({
    path: p,
    format: detectPathFormat(p)
  }));

  const windowsPaths = formats.filter(f => f.format === 'windows');
  const unixPaths = formats.filter(f => f.format === 'unix');

  console.log(`Windows paths: ${windowsPaths.length}`);
  console.log(`Unix paths: ${unixPaths.length}`);

  if (windowsPaths.length > 0 && unixPaths.length > 0) {
    console.warn('⚠️  Mixed path formats detected!');
  }
}
```

### Absolute Path Resolution

```typescript
// Make relative paths absolute
const result = normalizePath('../config.json', {
  format: 'unix',
  absolute: true
});

console.log(result.path);
// '/home/user/project/config.json' (resolved from cwd)
```

### Custom Validation

```typescript
function validateAndNormalize(path: string): string | null {
  // Basic validation
  if (!path || path.trim() === '') {
    return null;
  }

  // Detect format
  const format = detectPathFormat(path);

  // Normalize to Unix
  const result = normalizePath(path, { format: 'unix' });

  if (!result.success) {
    console.error(`Invalid path: ${result.error}`);
    return null;
  }

  return result.path;
}
```

## Error Scenarios

The `normalizePath()` function returns `success: false` for:

```typescript
// Empty paths
normalizePath('');
// { success: false, error: 'Path cannot be empty', ... }

// Invalid characters (on some platforms)
normalizePath('C:\\invalid<>path');
// { success: false, error: 'Invalid path characters', ... }
```

The quick conversion functions (`normalizeToUnix`, `normalizeToWindows`) don't throw errors but may return unexpected results for invalid input - use `normalizePath()` when you need error handling.

## Performance Considerations

All functions use simple string operations with zero dependencies:

```typescript
// Fast - pure string manipulation
const paths = Array(10000).fill('C:\\Users\\file.txt');
const start = performance.now();
paths.forEach(p => normalizeToUnix(p));
const elapsed = performance.now() - start;
console.log(`Normalized 10,000 paths in ${elapsed}ms`);
// Typically < 50ms
```

## See Also

- [Getting Started](/tools/cross-platform-path-normalizer/getting-started) — Installation and setup
- [CLI Usage](/tools/cross-platform-path-normalizer/cli-usage) — Command-line interface
- [Examples](/tools/cross-platform-path-normalizer/examples) — Real-world usage patterns
- [API Reference](/tools/cross-platform-path-normalizer/api-reference) — Complete API documentation
