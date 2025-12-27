# Examples

Real-world usage scenarios for Cross-Platform Path Normalizer.

## CLI Examples

### Basic Path Conversion

```bash
# Convert Windows path to Unix
normpath --format unix "C:\Users\Documents\file.txt"
# Output: /c/Users/Documents/file.txt

# Convert Unix path to Windows
normpath --format windows "/home/user/project/src/index.ts"
# Output: \home\user\project\src\index.ts
```

### Auto-Detection

```bash
# Let the tool detect the format
normpath "C:\Program Files\app.exe"
# Detected format: windows
# Output: C:\Program Files\app.exe

normpath "/usr/local/bin/node"
# Detected format: unix
# Output: /usr/local/bin/node
```

### Batch Processing

```bash
# Process multiple paths
for path in "C:\Windows\System32" "/usr/bin" "\\\\server\\share"; do
  normpath --format unix "$path"
done
```

## Library Usage

### Test Fixtures

Normalize paths in cross-platform tests:

```typescript
import { normalizeToUnix } from './src/index.js';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import assert from 'node:assert/strict';

test('should load test fixture on any platform', () => {
  // Works whether TEST_FILE is Windows or Unix format
  const testFile = normalizeToUnix(
    process.env.TEST_FILE || 'C:\\tests\\fixture.txt'
  );

  const content = readFileSync(testFile, 'utf-8');
  assert(content.length > 0);
});
```

### Build Tools

Handle artifact paths from different CI environments:

```typescript
import { normalizePath } from './src/index.js';

function processArtifact(artifactPath: string): Promise<void> {
  // Normalize to Unix for consistent handling
  const result = normalizePath(artifactPath, { format: 'unix' });

  if (!result.success) {
    throw new Error(`Invalid artifact path: ${result.error}`);
  }

  // Upload to S3 (expects Unix-style paths)
  return uploadToS3(result.path);
}

// Works in GitHub Actions (Linux), Azure Pipelines (Windows), or local dev
await processArtifact(process.env.ARTIFACT_PATH);
```

### Path Migration

Convert path databases between formats:

```typescript
import { normalizeToWindows } from './src/index.js';
import { Database } from 'better-sqlite3';

async function migratePaths(db: Database): Promise<void> {
  // Get all Unix paths from database
  const unixPaths = db.prepare('SELECT id, path FROM files').all();

  console.log(`Migrating ${unixPaths.length} paths to Windows format...`);

  // Convert each to Windows format
  for (const row of unixPaths) {
    const windowsPath = normalizeToWindows(row.path);

    db.prepare('UPDATE files SET path = ? WHERE id = ?')
      .run(windowsPath, row.id);
  }

  console.log('Migration complete!');
}
```

### Configuration Files

Normalize user-provided paths in config:

```typescript
import { normalizePath } from './src/index.js';
import { readFileSync } from 'node:fs';

interface Config {
  dataDir: string;
  outputPath: string;
  logFile: string;
}

function loadConfig(configPath: string): Config {
  const result = normalizePath(configPath, { format: 'unix' });

  if (!result.success) {
    console.warn(`Invalid config path, using default`);
    return loadDefaultConfig();
  }

  const raw = JSON.parse(readFileSync(result.path, 'utf-8'));

  // Normalize all paths in config
  return {
    dataDir: normalizeToUnix(raw.dataDir),
    outputPath: normalizeToUnix(raw.outputPath),
    logFile: normalizeToUnix(raw.logFile)
  };
}
```

## CI/CD Integration

### GitHub Actions

Normalize paths from different runner environments:

```yaml
name: Build

on: [push]

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Normalize artifact path
        id: normalize
        run: |
          ARTIFACT_PATH="${{ runner.temp }}/dist.zip"
          NORMALIZED=$(normpath --format unix "$ARTIFACT_PATH")
          echo "path=$NORMALIZED" >> $GITHUB_OUTPUT

      - name: Build
        run: npm run build

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: dist-${{ matrix.os }}
          path: ${{ steps.normalize.outputs.path }}
```

### Pre-commit Hook

Validate paths in committed files:

```bash
#!/bin/bash
# .git/hooks/pre-commit

echo "🔍 Checking for mixed path formats..."

# Get staged files
STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep -E '\.(ts|js|json)$')

if [ -z "$STAGED_FILES" ]; then
  exit 0
fi

# Check each file for mixed path formats
for file in $STAGED_FILES; do
  # Extract string literals that look like paths
  PATHS=$(grep -o '["\x27][^"]*\\[^"]*["\x27]' "$file" || true)

  if [ -n "$PATHS" ]; then
    echo "⚠️  Found Windows-style paths in $file"
    echo "$PATHS"
    echo ""
    echo "💡 Normalize paths using:"
    echo "   npx tsx path-normalizer/src/index.ts --format unix \"<path>\""
    exit 1
  fi
done

echo "✅ No path format issues found"
exit 0
```

## Advanced Examples

### Mixed Path Detection

Detect and report mixed formats in a codebase:

```typescript
import { detectPathFormat } from './src/index.js';
import { readFileSync, readdirSync, statSync } from 'node:fs';
import { join } from 'node:path';

function scanForMixedPaths(dir: string): void {
  const results = {
    windows: [] as string[],
    unix: [] as string[],
    files: new Set<string>()
  };

  function scanFile(filePath: string): void {
    const content = readFileSync(filePath, 'utf-8');

    // Find string literals that look like paths
    const pathPattern = /["']([^"']*[\/\\][^"']*)["']/g;
    let match;

    while ((match = pathPattern.exec(content)) !== null) {
      const path = match[1];
      const format = detectPathFormat(path);

      if (format === 'windows') {
        results.windows.push(path);
        results.files.add(filePath);
      } else {
        results.unix.push(path);
      }
    }
  }

  function walkDir(currentDir: string): void {
    for (const entry of readdirSync(currentDir)) {
      const fullPath = join(currentDir, entry);
      const stat = statSync(fullPath);

      if (stat.isDirectory() && !entry.startsWith('.')) {
        walkDir(fullPath);
      } else if (entry.match(/\.(ts|js|json)$/)) {
        scanFile(fullPath);
      }
    }
  }

  walkDir(dir);

  // Report findings
  console.log('\n📊 Path Format Analysis:');
  console.log(`   Windows paths: ${results.windows.length}`);
  console.log(`   Unix paths: ${results.unix.length}`);
  console.log(`   Files with Windows paths: ${results.files.size}`);

  if (results.files.size > 0) {
    console.log('\n⚠️  Files containing Windows paths:');
    for (const file of results.files) {
      console.log(`   ${file}`);
    }
  }
}

scanForMixedPaths('./src');
```

### Cross-Platform File Watcher

Watch files with normalized paths:

```typescript
import { watch } from 'node:fs';
import { normalizeToUnix } from './src/index.js';

function watchCrossPlatform(pathFromUser: string, callback: () => void): void {
  // Normalize path for consistent handling
  const normalized = normalizeToUnix(pathFromUser);

  console.log(`👀 Watching: ${normalized}`);

  watch(normalized, { recursive: true }, (eventType, filename) => {
    if (filename) {
      const eventPath = normalizeToUnix(filename);
      console.log(`📝 ${eventType}: ${eventPath}`);
      callback();
    }
  });
}

// Works with Windows or Unix input
watchCrossPlatform('C:\\Projects\\myapp\\src', () => {
  console.log('Files changed, rebuilding...');
});
```

### Path Sanitization for APIs

Sanitize user input before API calls:

```typescript
import { normalizePath } from './src/index.js';

async function uploadFile(userPath: string, apiUrl: string): Promise<void> {
  // Validate and normalize user input
  const result = normalizePath(userPath, { format: 'unix' });

  if (!result.success) {
    throw new Error(`Invalid file path: ${result.error}`);
  }

  // APIs typically expect Unix-style paths
  const response = await fetch(apiUrl, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ filePath: result.path })
  });

  if (!response.ok) {
    throw new Error(`Upload failed: ${response.statusText}`);
  }
}

// Safely handle paths from any source
await uploadFile(process.argv[2], 'https://api.example.com/upload');
```

### Monorepo Tool Paths

Normalize paths across monorepo packages:

```typescript
import { normalizeToUnix } from './src/index.js';
import { join, resolve } from 'node:path';

interface PackageInfo {
  name: string;
  path: string;
  normalizedPath: string;
}

function getMonorepoPackages(rootDir: string): PackageInfo[] {
  const packages: PackageInfo[] = [];

  // Find all package.json files
  const packageDirs = [
    'packages/core',
    'packages/ui',
    'packages/utils'
  ];

  for (const dir of packageDirs) {
    const absolutePath = resolve(rootDir, dir);
    const normalized = normalizeToUnix(absolutePath);

    packages.push({
      name: dir.split('/').pop() || '',
      path: absolutePath,
      normalizedPath: normalized
    });
  }

  return packages;
}

// Works on Windows, Mac, Linux
const packages = getMonorepoPackages(process.cwd());
console.table(packages);
```

## Integration with Popular Tools

### Webpack Configuration

```typescript
import { normalizeToUnix } from './src/index.js';
import { resolve } from 'node:path';

export default {
  entry: normalizeToUnix(resolve(__dirname, 'src/index.ts')),
  output: {
    path: normalizeToUnix(resolve(__dirname, 'dist')),
    filename: 'bundle.js'
  },
  resolve: {
    alias: {
      '@': normalizeToUnix(resolve(__dirname, 'src'))
    }
  }
};
```

### ESLint Plugin

```typescript
import { detectPathFormat } from './src/index.js';

export default {
  rules: {
    'consistent-path-format': {
      create(context) {
        return {
          Literal(node) {
            if (typeof node.value === 'string' && node.value.includes('\\')) {
              const format = detectPathFormat(node.value);
              if (format === 'windows') {
                context.report({
                  node,
                  message: 'Use Unix-style paths for consistency',
                  fix(fixer) {
                    const normalized = normalizeToUnix(node.value);
                    return fixer.replaceText(node, `"${normalized}"`);
                  }
                });
              }
            }
          }
        };
      }
    }
  }
};
```

## Troubleshooting

### Debugging Path Issues

```typescript
import { detectPathFormat, normalizePath } from './src/index.js';

function debugPath(path: string): void {
  console.log('\n🔍 Path Debug Info:');
  console.log(`   Input: ${path}`);
  console.log(`   Detected format: ${detectPathFormat(path)}`);

  const toUnix = normalizePath(path, { format: 'unix' });
  console.log(`   As Unix: ${toUnix.path}`);

  const toWindows = normalizePath(path, { format: 'windows' });
  console.log(`   As Windows: ${toWindows.path}`);

  console.log(`   Length: ${path.length} chars`);
  console.log(`   Contains backslashes: ${path.includes('\\')}`);
  console.log(`   Contains drive letter: ${/^[A-Za-z]:/.test(path)}`);
}

debugPath(process.argv[2]);
```

## See Also

- [Getting Started](/tools/cross-platform-path-normalizer/getting-started) — Installation and setup
- [CLI Usage](/tools/cross-platform-path-normalizer/cli-usage) — Command-line interface
- [Library Usage](/tools/cross-platform-path-normalizer/library-usage) — TypeScript/JavaScript API
- [API Reference](/tools/cross-platform-path-normalizer/api-reference) — Complete API documentation
