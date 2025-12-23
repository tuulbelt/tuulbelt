---
description: Run comprehensive security analysis on codebase
---

Perform security scanning on the current state of the repository.

## Security Checks

### 1. Dependency Vulnerability Scan

Check for known vulnerabilities in dependencies:

```bash
# TypeScript dependencies (if package.json exists)
if [ -f package.json ]; then
  npm audit --audit-level=moderate 2>&1
fi

# Rust dependencies (if Cargo.toml exists)
if [ -f Cargo.toml ]; then
  cargo audit 2>&1 || echo "cargo-audit not installed, skipping"
fi
```

### 2. Secret Detection

Scan for potential hardcoded secrets:

```bash
# Check staged changes
git diff --cached | grep -iE '(password|api_key|secret|token|private_key|credentials)' && echo "⚠️  Potential secrets detected in staged files" || echo "✓ No obvious secrets found in staged changes"

# Check all source files
grep -r -iE '(password.*=|api_key.*=|secret.*=|token.*=)' src/ 2>/dev/null | grep -v 'test' | grep -v '.md' && echo "⚠️  Potential secrets in source files" || echo "✓ No hardcoded secrets detected"
```

### 3. Protected File Check

Verify no sensitive files are committed:

```bash
# Check for .env files
find . -name ".env*" -not -path "*/node_modules/*" -not -path "*/.git/*" 2>/dev/null | while read file; do
  if git ls-files --error-unmatch "$file" >/dev/null 2>&1; then
    echo "⚠️  WARNING: $file is tracked by git"
  fi
done
```

### 4. Dependency Count Validation

Ensure zero-dependency principle:

```bash
# TypeScript: Check package.json dependencies
if [ -f package.json ]; then
  deps=$(jq -r '.dependencies // {} | length' package.json)
  if [ "$deps" -gt 0 ]; then
    echo "⚠️  WARNING: Found $deps runtime dependencies (should be 0)"
    jq -r '.dependencies // {}' package.json
  else
    echo "✓ Zero runtime dependencies (TypeScript)"
  fi
fi

# Rust: Check Cargo.toml dependencies
if [ -f Cargo.toml ]; then
  deps=$(grep -A 100 "^\[dependencies\]" Cargo.toml | grep -v "^\[" | grep -v "^#" | grep -v "^$" | wc -l)
  if [ "$deps" -gt 0 ]; then
    echo "⚠️  WARNING: Found runtime dependencies in Cargo.toml (should be 0)"
  else
    echo "✓ Zero runtime dependencies (Rust)"
  fi
fi
```

## Analysis

Review the security scan output and address any findings:

1. **High-severity vulnerabilities** - Update dependencies immediately
2. **Exposed credentials** - Remove and rotate
3. **Tracked sensitive files** - Remove from git history
4. **Dependencies violations** - Remove non-dev dependencies

Report a summary of findings and recommended actions.
