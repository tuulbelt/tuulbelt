# Changelog

All notable changes to Config File Merger will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-27

### Added

- **Core merging functionality**
  - Merge configuration from multiple sources (ENV, files, CLI args, defaults)
  - Clear precedence rules: CLI > ENV > File > Defaults
  - Optional source tracking to know where each value came from

- **Environment variable handling**
  - Prefix filtering (`--prefix APP_`)
  - Case normalization (`--no-lowercase` to preserve case)
  - Prefix stripping (`--no-strip-prefix` to keep prefix)

- **Type coercion**
  - Automatic parsing of booleans (`true`, `false`)
  - Automatic parsing of numbers (integers, floats, negative)
  - Automatic parsing of null
  - Quoted strings preserve string type

- **CLI interface**
  - Short name: `cfgmerge`
  - Long name: `config-file-merger`
  - All options have short flags (`-e`, `-f`, `-a`, `-t`, etc.)

- **Library API**
  - `mergeConfig()` - merge configuration from multiple sources
  - `parseJsonFile()` - parse JSON config file with error handling
  - `parseCliArgs()` - parse key=value,key2=value2 format
  - `parseEnv()` - parse environment variables with filtering
  - `getValue()` - typed value retrieval with defaults

- **Documentation**
  - Complete README with examples
  - SPEC.md with full specification
  - GitHub Pages documentation (7 pages)
  - Basic and advanced usage examples

- **Testing**
  - 135 tests covering all functionality
  - Edge case coverage (unicode, special characters, etc.)
  - Determinism validation
  - CLI integration tests

- **Dogfooding**
  - `scripts/dogfood-flaky.sh` - validate test determinism
  - `scripts/dogfood-diff.sh` - validate output determinism

### Implementation Notes

- Zero runtime dependencies
- Uses only Node.js built-in modules (`fs`, `path`)
- TypeScript with strict mode
- Result pattern for error handling (no thrown exceptions)
- Fully deterministic output

---

## [Unreleased]

### Planned

- YAML/TOML config file support (under consideration)
- Deep merge for nested objects (under consideration)
- Schema validation (under consideration)
