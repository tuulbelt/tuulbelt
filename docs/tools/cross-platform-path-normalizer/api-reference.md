# API Reference
Complete API documentation for Cross-Platform Path Normalizer.
## Functions
### Main Functions
- [`normalizePath()`](/tools/cross-platform-path-normalizer/api-reference#normalize-path) - Normalize path with options
- [`normalizeToUnix()`](/tools/cross-platform-path-normalizer/api-reference#normalize-to-unix) - Convert to Unix format
- [`normalizeToWindows()`](/tools/cross-platform-path-normalizer/api-reference#normalize-to-windows) - Convert to Windows format
- [`detectPathFormat()`](/tools/cross-platform-path-normalizer/api-reference#detect-path-format) - Detect path format
### Type Definitions
- [`NormalizeOptions`](/tools/cross-platform-path-normalizer/api-reference#types#normalizeoptions) - Configuration options
- [`NormalizeResult`](/tools/cross-platform-path-normalizer/api-reference#types#normalizeresult) - Result object
- [`PathFormat`](/tools/cross-platform-path-normalizer/api-reference#types#pathformat) - Format type
## Quick Reference
\`\`\`typescript
import {
  normalizePath,
  normalizeToUnix,
  normalizeToWindows,
  detectPathFormat,
  type NormalizeOptions,
  type NormalizeResult,
  type PathFormat
} from './src/index.js';
\`\`\`
