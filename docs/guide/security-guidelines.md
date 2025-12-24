# Security Guidelines

This document outlines security considerations for Tuulbelt tool development.

## Core Principles

### 1. Zero External Dependencies

The primary security benefit of Tuulbelt tools is **no external runtime dependencies**.

- No `node_modules` in production
- No supply chain attacks via compromised packages
- No transitive dependency vulnerabilities

### 2. Input Validation

All tools must validate input at system boundaries:

```typescript
// Good: Validate at entry point
export function process(input: unknown): Result {
  if (typeof input !== 'string') {
    throw new TypeError('Input must be a string');
  }
  if (input.length > MAX_INPUT_SIZE) {
    throw new RangeError('Input exceeds maximum size');
  }
  // ... process validated input
}
```

### 3. No Secrets in Code

- Never hardcode API keys, passwords, or tokens
- Use environment variables for configuration
- Document required environment variables in README

```typescript
// Good: Read from environment
const apiKey = process.env.API_KEY;
if (!apiKey) {
  throw new Error('API_KEY environment variable required');
}

// Bad: Hardcoded secret
const apiKey = 'sk-1234567890abcdef';
```

### 4. Safe File Operations

When tools read or write files:

```typescript
import { resolve, normalize } from 'path';

// Prevent path traversal
function safePath(userPath: string, baseDir: string): string {
  const resolved = resolve(baseDir, userPath);
  const normalized = normalize(resolved);

  if (!normalized.startsWith(baseDir)) {
    throw new Error('Path traversal detected');
  }

  return normalized;
}
```

### 5. Command Injection Prevention

Never pass user input directly to shell commands:

```typescript
import { execFile } from 'child_process';

// Good: Use execFile with arguments array
execFile('git', ['status', '--porcelain'], (error, stdout) => {
  // ...
});

// Bad: String interpolation in exec
exec(`git status ${userInput}`, (error, stdout) => {
  // Vulnerable to command injection
});
```

## Tool-Specific Guidelines

### CLI Tools

- Validate all command-line arguments
- Use allowlists for flag values when possible
- Exit with appropriate error codes (non-zero for errors)

### File Processing Tools

- Limit file sizes to prevent resource exhaustion
- Validate file formats before processing
- Clean up temporary files

### Network Tools

- Validate URLs and hostnames
- Use timeouts for all network requests
- Handle connection failures gracefully

## Reporting Security Issues

If you discover a security vulnerability in a Tuulbelt tool:

1. **Do not** open a public GitHub issue
2. Email the maintainer directly (see CODEOWNERS)
3. Include:
   - Tool name and version
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact

## Security Review Checklist

Before releasing a tool:

- [ ] No hardcoded secrets
- [ ] All user input validated
- [ ] No shell command injection vectors
- [ ] File paths sanitized (no path traversal)
- [ ] Network requests have timeouts
- [ ] Error messages don't leak sensitive information
- [ ] Dependencies reviewed (dev-only, TypeScript compiler)

## See Also

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Node.js Security Best Practices](https://nodejs.org/en/docs/guides/security/)
