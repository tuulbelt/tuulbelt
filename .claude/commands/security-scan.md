---
description: Run comprehensive security analysis on codebase
---

Perform security scanning on the current state of the repository. This command is called by `/quality-check` and can also be run standalone for deeper analysis.

## Security Checks

### 1. Secret Detection

Scan for potential hardcoded secrets:

```bash
# Check staged changes (pre-commit)
echo "→ Checking staged changes for secrets..."
git diff --cached | grep -iE '(password|api_key|secret|token|private_key|credentials|bearer|authorization)' && echo "⚠️  Potential secrets detected in staged files" || echo "✓ No obvious secrets in staged changes"

# Check all source files
echo "→ Checking source files for hardcoded secrets..."
grep -r -iE '(password\s*[:=]|api_key\s*[:=]|secret\s*[:=]|token\s*[:=]|private_key|aws_access_key|aws_secret)' src/ 2>/dev/null | grep -v 'test' | grep -v '.md' | grep -v 'example' && echo "⚠️  Potential secrets in source files" || echo "✓ No hardcoded secrets detected"
```

### 2. Protected File Check

Verify no sensitive files are committed:

```bash
echo "→ Checking for tracked sensitive files..."

# Check for .env files
find . -name ".env*" -not -path "*/node_modules/*" -not -path "*/.git/*" -not -path "*/target/*" 2>/dev/null | while read file; do
  if git ls-files --error-unmatch "$file" >/dev/null 2>&1; then
    echo "⚠️  WARNING: $file is tracked by git - remove immediately"
  fi
done

# Check for other sensitive files
for pattern in "*.pem" "*.key" "*credentials*" "*secrets*"; do
  git ls-files | grep -i "$pattern" && echo "⚠️  Sensitive file pattern '$pattern' found in git"
done
```

### 3. Dependency Vulnerability Scan

Check for known vulnerabilities in dependencies:

```bash
echo "→ Running dependency vulnerability scan..."

# TypeScript dependencies
if [ -f package.json ]; then
  echo "  TypeScript: Running npm audit..."
  npm audit --audit-level=high 2>&1
  if [ $? -ne 0 ]; then
    echo "⚠️  High-severity vulnerabilities found - review and fix"
  else
    echo "✓ No high-severity vulnerabilities in npm dependencies"
  fi
fi

# Rust dependencies
if [ -f Cargo.toml ]; then
  echo "  Rust: Running cargo audit..."
  if command -v cargo-audit &> /dev/null; then
    cargo audit 2>&1
  else
    echo "ℹ️  cargo-audit not installed. Install with: cargo install cargo-audit"
  fi
fi
```

### 4. Zero-Dependency Validation

Ensure zero-dependency principle (Tuulbelt-specific):

```bash
echo "→ Validating zero-dependency principle..."

# TypeScript: Check package.json dependencies
if [ -f package.json ]; then
  deps=$(cat package.json | grep -A 50 '"dependencies"' | grep -E '^\s+"[^"]+":' | wc -l)
  if [ "$deps" -gt 0 ]; then
    echo "⚠️  WARNING: Found $deps runtime dependencies (should be 0)"
    cat package.json | grep -A 50 '"dependencies"' | head -20
  else
    echo "✓ Zero runtime dependencies (TypeScript)"
  fi
fi

# Rust: Check Cargo.toml dependencies
if [ -f Cargo.toml ]; then
  deps=$(grep -A 100 "^\[dependencies\]" Cargo.toml 2>/dev/null | grep -v "^\[" | grep -v "^#" | grep -v "^$" | grep -v "^\[dev-dependencies\]" | head -20 | wc -l)
  if [ "$deps" -gt 0 ]; then
    echo "⚠️  WARNING: Found runtime dependencies in Cargo.toml (should be 0)"
    grep -A 10 "^\[dependencies\]" Cargo.toml
  else
    echo "✓ Zero runtime dependencies (Rust)"
  fi
fi
```

---

## Language-Specific Checks

### TypeScript/Node.js

```bash
echo "→ TypeScript-specific security checks..."

# Check for eval() usage
grep -r "eval(" src/ 2>/dev/null && echo "⚠️  eval() usage detected - potential code injection" || echo "✓ No eval() usage"

# Check for Function constructor
grep -r "new Function(" src/ 2>/dev/null && echo "⚠️  Function constructor detected - potential code injection" || echo "✓ No Function constructor usage"

# Check for child_process with user input
grep -r "exec\|execSync\|spawn" src/ 2>/dev/null | grep -v test && echo "ℹ️  child_process usage - verify input sanitization" || echo "✓ No child_process concerns"

# Check for unsafe innerHTML
grep -r "innerHTML\|dangerouslySetInnerHTML" src/ 2>/dev/null && echo "⚠️  innerHTML usage - potential XSS" || echo "✓ No innerHTML usage"
```

### Rust

```bash
echo "→ Rust-specific security checks..."

# Check for unsafe blocks
grep -r "unsafe {" src/ 2>/dev/null && echo "ℹ️  unsafe blocks detected - verify safety invariants" || echo "✓ No unsafe blocks"

# Check for unwrap() in non-test code
grep -r "\.unwrap()" src/ 2>/dev/null | grep -v test | grep -v "#\[cfg(test)\]" && echo "⚠️  unwrap() in production code - use ? operator or proper error handling" || echo "✓ No unwrap() in production paths"

# Check for panic! in non-test code
grep -r "panic!" src/ 2>/dev/null | grep -v test && echo "⚠️  panic! in production code - consider Result types" || echo "✓ No panic! in production paths"

# Check for format string injection
grep -r 'format!.*{.*}' src/ 2>/dev/null | grep -v test && echo "ℹ️  format! usage - verify no user input in format strings" || echo "✓ No format string concerns"
```

---

## Output Format

Provide a clear summary:

```
🔒 Security Scan Summary

Location: [current directory]
Language: [TypeScript|Rust|Both]

PASSED:
  ✓ No hardcoded secrets
  ✓ No tracked sensitive files
  ✓ No high-severity vulnerabilities
  ✓ Zero runtime dependencies

WARNINGS:
  ⚠ [Any warnings found]

ACTIONS REQUIRED:
  [List of items that must be fixed before commit]

SECURITY STATUS: [PASS|WARN|FAIL]
```

## Severity Levels

| Finding | Severity | Action |
|---------|----------|--------|
| Hardcoded secrets | 🔴 Critical | Remove immediately, rotate credentials |
| Tracked .env files | 🔴 Critical | Remove from git, add to .gitignore |
| High-severity CVE | 🟠 High | Update dependency before release |
| eval()/Function() | 🟠 High | Refactor to avoid dynamic code execution |
| unwrap() in prod | 🟡 Medium | Replace with proper error handling |
| unsafe blocks | 🟡 Medium | Document safety invariants |

## Integration

This command is automatically run as part of `/quality-check`. For pre-release security review, run it standalone for the full analysis.

## References

- @docs/security-guidelines.md - Security best practices
- @docs/QUALITY_CHECKLIST.md - Pre-commit checklist
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
