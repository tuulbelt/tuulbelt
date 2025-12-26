# Changelog

All notable changes to file-based-semaphore will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-25

### Added
- Cross-platform file-based semaphore implementation
- `Semaphore` struct with RAII guards for automatic lock release
- Atomic lock acquisition using `O_CREAT | O_EXCL` flags
- Stale lock detection with configurable timeouts
- Process ID tracking in lock files
- Optional tag field for lock identification
- CLI tool (`file-semaphore`) with commands: try, acquire, release, status, cleanup
- Blocking acquisition with timeout support
- Non-blocking try-acquire operation
- Comprehensive test suite (31 unit + 39 CLI + 11 integration + 4 doctests = 85 tests)
- Protocol specification (SPEC.md)
- API documentation with examples
- Dogfooding script for test validation

### Implementation Notes
- Zero runtime dependencies (uses Rust std library only)
- Works on Linux, macOS, and Windows
- Rust Edition 2021
- Optimized release profile (LTO enabled, stripped binaries)
- Lock file format: newline-separated key=value pairs

### Security
- Safe lock file creation with exclusive access
- Process validation to detect stale locks
- Path traversal protection in lock file handling
