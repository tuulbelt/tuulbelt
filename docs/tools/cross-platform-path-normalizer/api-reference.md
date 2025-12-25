# API Reference

Complete API documentation for Cross-Platform Path Normalizer.

## Functions

For detailed usage examples, see [Library Usage](/tools/cross-platform-path-normalizer/library-usage).

### Core Functions

- [`normalizePath()`](#normalizepath) - Normalize path with options
- [`normalizeToUnix()`](#normalizetounix) - Convert to Unix format
- [`normalizeToWindows()`](#normalizetowindows) - Convert to Windows format
- [`detectPathFormat()`](#detectpathformat) - Detect path format

### Types

- [`NormalizeOptions`](#normalizeoptions) - Configuration options
- [`NormalizeResult`](#normalizeresult) - Result object
- [`PathFormat`](#pathformat) - Format type

---

## Functions

### `normalizePath()`

Normalize a path to the specified format with full error handling.

```typescript
function normalizePath(
  path: string,
  options?: NormalizeOptions
): NormalizeResult
```

**Parameters:**
- `path` (required) — The path string to normalize
- `options` (optional) — Normalization configuration

**Returns:** [`NormalizeResult`](#normalizeresult) object with success status and normalized path

**Example:**
```typescript
import { normalizePath } from './src/index.js'

const result = normalizePath('C:\\Users\\file.txt', { format: 'unix' })
// { success: true, path: '/c/Users/file.txt', format: 'unix' }
```

See [Library Usage - normalizePath()](/tools/cross-platform-path-normalizer/library-usage#normalizepath) for more examples.

---

### `normalizeToUnix()`

Convert any path to Unix format (forward slashes).

```typescript
function normalizeToUnix(path: string): string
```

**Parameters:**
- `path` (required) — The path string to convert

**Returns:** Path in Unix format with forward slashes

**Behavior:**
- Converts `C:\Users\file.txt` → `/c/Users/file.txt`
- Converts `\\server\share` → `//server/share` (UNC paths)
- Replaces all `\` with `/`
- Lowercases drive letters

**Example:**
```typescript
import { normalizeToUnix } from './src/index.js'

normalizeToUnix('C:\\Program Files\\app')  // '/c/Program Files/app'
normalizeToUnix('\\\\server\\share\\file') // '//server/share/file'
normalizeToUnix('/home/user/file.txt')     // '/home/user/file.txt'
```

See [Library Usage - normalizeToUnix()](/tools/cross-platform-path-normalizer/library-usage#normalizetounix) for more examples.

---

### `normalizeToWindows()`

Convert any path to Windows format (backslashes).

```typescript
function normalizeToWindows(path: string): string
```

**Parameters:**
- `path` (required) — The path string to convert

**Returns:** Path in Windows format with backslashes

**Behavior:**
- Converts `/c/Users/file.txt` → `C:\Users\file.txt`
- Converts `//server/share` → `\\server\share` (UNC paths)
- Replaces all `/` with `\`
- Uppercases drive letters

**Example:**
```typescript
import { normalizeToWindows } from './src/index.js'

normalizeToWindows('/c/Program Files/app')  // 'C:\Program Files\app'
normalizeToWindows('//server/share/file')   // '\\server\share\file'
normalizeToWindows('C:\\Users\\file.txt')   // 'C:\Users\file.txt'
```

See [Library Usage - normalizeToWindows()](/tools/cross-platform-path-normalizer/library-usage#normalizetowindows) for more examples.

---

### `detectPathFormat()`

Detect whether a path is Windows or Unix format.

```typescript
function detectPathFormat(path: string): 'windows' | 'unix'
```

**Parameters:**
- `path` (required) — The path string to analyze

**Returns:** `'windows'` or `'unix'`

**Detection Logic:**
- **Windows** if path contains:
  - Drive letter (`C:`, `D:`, etc.)
  - Backslashes (`\`)
  - UNC prefix (`\\server\share`)
- **Unix** otherwise

**Example:**
```typescript
import { detectPathFormat } from './src/index.js'

detectPathFormat('C:\\Users\\file.txt')      // 'windows'
detectPathFormat('\\\\server\\share')        // 'windows'
detectPathFormat('/home/user/file.txt')      // 'unix'
detectPathFormat('C:/mixed/separators.txt')  // 'windows' (has drive letter)
```

See [Library Usage - detectPathFormat()](/tools/cross-platform-path-normalizer/library-usage#detectpathformat) for more examples.

---

## Types

### `NormalizeOptions`

Configuration options for path normalization.

```typescript
interface NormalizeOptions {
  /** Target format (unix, windows, or auto to detect) */
  format?: PathFormat;

  /** Resolve to absolute path */
  absolute?: boolean;

  /** Enable verbose output */
  verbose?: boolean;
}
```

**Properties:**
- `format` (optional) — Target path format: `'unix'`, `'windows'`, or `'auto'`. Default: `'auto'` (auto-detects from input path)
- `absolute` (optional) — If true, resolves to absolute path. Default: `false`
- `verbose` (optional) — If true, logs debug information to stderr. Default: `false`

**Example:**
```typescript
const options: NormalizeOptions = {
  format: 'unix',
  absolute: false,
  verbose: true
}

const result = normalizePath('C:\\Users\\file.txt', options)
// Logs: [DEBUG] Detected format: windows
// Returns: { success: true, path: '/c/Users/file.txt', format: 'unix' }
```

---

### `NormalizeResult`

Result object returned by `normalizePath()`.

```typescript
interface NormalizeResult {
  /** Whether the operation succeeded */
  success: boolean;

  /** The normalized path */
  path: string;

  /** Detected or specified format */
  format: 'unix' | 'windows';

  /** Optional error message if success is false */
  error?: string;
}
```

**Properties:**
- `success` — `true` if normalization succeeded, `false` if error occurred
- `path` — The normalized path string (empty string if error)
- `format` — The resulting path format (`'unix'` or `'windows'`)
- `error` (optional) — Error message if `success` is `false`

**Example (success):**
```json
{
  "success": true,
  "path": "/c/Users/file.txt",
  "format": "unix"
}
```

**Example (error):**
```json
{
  "success": false,
  "path": "",
  "format": "unix",
  "error": "Path cannot be empty"
}
```

---

### `PathFormat`

Path format type.

```typescript
type PathFormat = 'unix' | 'windows' | 'auto';
```

**Values:**
- `'unix'` — Unix format with forward slashes (`/`)
- `'windows'` — Windows format with backslashes (`\`)
- `'auto'` — Auto-detect from input path

---

## Edge Cases

### Empty Paths

```typescript
normalizePath('')  // { success: false, error: 'Path cannot be empty' }
normalizeToUnix('') // ''
normalizeToWindows('') // ''
```

### Mixed Separators

Automatically normalizes paths with both `/` and `\`:

```typescript
normalizeToUnix('C:/Users\\Documents/file.txt')     // '/c/Users/Documents/file.txt'
normalizeToWindows('/c/Users\\Documents/file.txt')  // 'C:\Users\Documents\file.txt'
```

### UNC Paths

Preserves UNC path structure:

```typescript
normalizeToUnix('\\\\server\\share\\file')   // '//server/share/file'
normalizeToWindows('//server/share/file')    // '\\server\share\file'
```

### Drive Letters

Handles case normalization:

```typescript
normalizeToUnix('C:\\Users')    // '/c/Users' (lowercase drive)
normalizeToWindows('/c/Users')  // 'C:\Users' (uppercase drive)
normalizeToWindows('/D/data')   // 'D:\data'  (uppercase drive)
```

### Redundant Slashes

Removes duplicate separators:

```typescript
normalizeToUnix('C:\\\\Users\\\\\\file.txt')  // '/c/Users/file.txt'
normalizeToWindows('/c///Users///file.txt')  // 'C:\Users\file.txt'
```

### Relative Paths

Preserves relative path structure:

```typescript
normalizeToUnix('../parent/file.txt')       // '../parent/file.txt'
normalizeToWindows('../parent/file.txt')    // '..\\parent\\file.txt'
```

---

## Error Handling

### Invalid Input Type

```typescript
const result = normalizePath(123 as any)
// {
//   success: false,
//   path: '',
//   format: 'unix',
//   error: 'Path must be a string'
// }
```

### Empty String

```typescript
const result = normalizePath('   ')
// {
//   success: false,
//   path: '',
//   format: 'unix',
//   error: 'Path cannot be empty'
// }
```

### Handling Errors

```typescript
const result = normalizePath(userInput, { format: 'unix' })

if (result.success) {
  console.log('Normalized:', result.path)
} else {
  console.error('Error:', result.error)
}
```

---

## Usage Examples

### Convert CLI Arguments

```typescript
import { normalizePath } from './src/index.js'

const args = process.argv.slice(2)
const result = normalizePath(args[0], { format: 'unix' })

if (result.success) {
  console.log(result.path)
} else {
  console.error(result.error)
  process.exit(1)
}
```

### Normalize Test Fixtures

```typescript
import { normalizeToUnix } from './src/index.js'

const fixtures = [
  'C:\\test\\fixtures\\data.json',
  'C:\\test\\fixtures\\config.yml'
]

const normalized = fixtures.map(normalizeToUnix)
// [
//   '/c/test/fixtures/data.json',
//   '/c/test/fixtures/config.yml'
// ]
```

### Cross-Platform Path Comparison

```typescript
import { normalizeToUnix } from './src/index.js'

const path1 = 'C:\\Users\\file.txt'
const path2 = '/c/Users/file.txt'

if (normalizeToUnix(path1) === normalizeToUnix(path2)) {
  console.log('Paths are equivalent')
}
```

### Auto-Detect and Convert

```typescript
import { normalizePath } from './src/index.js'

function convertPath(input: string, targetFormat: 'unix' | 'windows') {
  const result = normalizePath(input, { format: targetFormat })

  if (!result.success) {
    throw new Error(result.error)
  }

  return result.path
}

convertPath('C:\\Users\\file.txt', 'unix')       // '/c/Users/file.txt'
convertPath('/home/user/file.txt', 'windows')    // 'home\\user\\file.txt'
```

---

## Implementation Details

### Path Format Detection

The `detectPathFormat()` function uses three heuristics:
1. **Drive letter regex:** `/^[A-Za-z]:/`
2. **Backslash check:** `path.includes('\\')`
3. **UNC prefix check:** `/^\\\\/.test(path)`

If any match, format is `'windows'`, otherwise `'unix'`.

### Conversion Algorithm

**Unix Conversion:**
1. Detect UNC paths (`\\server\share`)
2. Convert drive letters (`C:` → `/c`)
3. Convert UNC prefix (`\\` → `//`)
4. Replace all `\` with `/`
5. Remove redundant slashes (preserve UNC `//`)

**Windows Conversion:**
1. Detect Unix drive paths (`/c/` → `C:\`)
2. Convert UNC prefix (`//` → `\\`)
3. Replace all `/` with `\`
4. Remove redundant backslashes (preserve UNC `\\`)
5. Uppercase drive letters

### Normalization Process

`normalizePath()` follows this flow:
1. Validate input (string, non-empty)
2. Determine target format (auto-detect or use specified)
3. Apply format conversion
4. Return result with success status

---

## See Also

- [Getting Started](/tools/cross-platform-path-normalizer/getting-started) — Installation and setup
- [CLI Usage](/tools/cross-platform-path-normalizer/cli-usage) — Command-line interface
- [Library Usage](/tools/cross-platform-path-normalizer/library-usage) — TypeScript integration
- [Examples](/tools/cross-platform-path-normalizer/examples) — Real-world usage patterns
