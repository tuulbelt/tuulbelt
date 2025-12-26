# Changelog

All notable changes to Output Diffing Utility will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project scaffolding from Rust template
- Cargo configuration for both lib and bin targets
- Basic project structure with examples
- Integration test setup

## [0.1.0] - YYYY-MM-DD (Not Released)

### Added
- Core functionality (TODO: describe main features)
- Comprehensive test suite (unit + integration)
- API documentation with doc comments
- Usage examples

### Implementation Notes
- Zero runtime dependencies
- Uses Rust standard library only
- Rust Edition 2021
- Optimized release profile (LTO enabled)

---

## Template Instructions

When releasing versions, follow this format:

### Version 0.1.0 - Initial Release

**Added:**
- List new features
- New functions or capabilities
- New documentation

**Changed:**
- List modifications to existing features
- API changes

**Deprecated:**
- List features marked for removal

**Removed:**
- List removed features
- Breaking changes

**Fixed:**
- List bug fixes

**Security:**
- List security fixes or improvements

### Version Numbering

- **MAJOR (X.0.0)**: Breaking changes, incompatible API changes
- **MINOR (0.X.0)**: New features, backwards-compatible
- **PATCH (0.0.X)**: Bug fixes, backwards-compatible

### Example Entry

```markdown
## [1.2.3] - 2025-01-15

### Added
- New `process_data()` function with validation
- Support for UTF-8 input (#42)
- Binary tool can now accept stdin

### Changed
- Improved error messages for `ProcessError::InvalidInput`
- Updated `Config` struct to use builder pattern

### Fixed
- Handle empty string input correctly (#38)
- Memory leak in parsing loop (#40)

### Security
- Validate file paths to prevent traversal attacks
- Use secure RNG for token generation
```

### Publishing to crates.io

Before publishing:

```bash
# Dry run
cargo publish --dry-run

# Actual publish
cargo publish
```

Version bumps:
```toml
# In Cargo.toml
[package]
version = "0.2.0"  # Update this
```

---

*Remove these instructions before the first release.*
