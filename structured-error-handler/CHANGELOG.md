# Changelog

All notable changes to Structured Error Handler will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-26

### Added

- **StructuredError class** - Extends Error with context chain preservation
  - `code` and `category` properties for programmatic error handling
  - `context` array storing operation history (most recent first)
  - Immutable enrichment via `addContext()` method

- **Static factory methods**
  - `StructuredError.wrap()` - Wrap existing errors with additional context
  - `StructuredError.from()` - Convert any value to StructuredError
  - `StructuredError.fromJSON()` - Deserialize from JSON

- **Instance methods**
  - `addContext()` - Add operation context (returns new instance)
  - `toJSON()` - Serialize to JSON-compatible object
  - `toString()` - Human-readable formatted output
  - `hasCode()` - Check if error or cause has specific code
  - `hasCategory()` - Check if error or cause has specific category
  - `getRootCause()` - Get deepest error in cause chain
  - `getCauseChain()` - Get all errors as array

- **Helper functions**
  - `serializeError()` - Serialize any Error to JSON format
  - `deserializeError()` - Deserialize JSON back to Error
  - `formatError()` - Format any error for display

- **TypeScript types**
  - `ErrorContext` - Context entry with operation, component, metadata, timestamp
  - `SerializedError` - JSON-serializable error representation
  - `StructuredErrorOptions` - Constructor options

- **CLI interface**
  - `demo` command - Show structured error examples
  - `parse` command - Parse and validate JSON errors
  - `validate` command - Validate error JSON structure
  - `--format` flag - Output as `json` or `text`
  - `--help` and `--version` flags

- **Documentation**
  - SPEC.md - Technical specification (403 lines)
  - DOGFOODING_STRATEGY.md - Composition patterns
  - 6 VitePress documentation pages
  - 2 example files (basic.ts, advanced.ts)

- **Test suite**
  - 68 tests covering core, serialization, and CLI
  - 80%+ code coverage
  - Dogfooding script for flakiness detection

### Implementation Notes

- Zero runtime dependencies
- Uses Node.js built-in modules only
- TypeScript with strict type checking
- Context stored most-recent-first (stack order)
- Code and category inherited through wrap() unless overridden
