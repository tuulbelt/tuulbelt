# Tuulbelt

A curated collection of focused, zero-dependency tools and utilities for modern software development.

## Philosophy

Each tool in Tuulbelt:
- **Solves one problem** — Narrow, well-defined scope
- **Zero external dependencies** — Uses standard library only
- **Portable interface** — CLI, files, sockets; not proprietary APIs
- **Composable** — Works via pipes, environment variables, file I/O
- **Independently cloneable** — Each tool is a standalone repository
- **Proven implementation** — No moonshots, no "works 80%" solutions

## Current Tools

### CLI/DevTools
- **[Structured Error Handler](https://github.com/tuulbelt/structured-error-handler)** — Error format + serialization with context preservation 🟢 v0.1.0 🐕 | [📖 Docs](https://github.com/tuulbelt/structured-error-handler#readme) | [🚀 Examples](https://github.com/tuulbelt/structured-error-handler/tree/main/examples/)
- **[CLI Progress Reporting](https://github.com/tuulbelt/cli-progress-reporting)** — Concurrent-safe progress updates 🟢 v0.1.0 🐕 | [📖 Docs](https://github.com/tuulbelt/cli-progress-reporting#readme) | [🚀 Examples](https://github.com/tuulbelt/cli-progress-reporting/tree/main/examples/)
- **[Configuration File Merger](https://github.com/tuulbelt/config-file-merger)** — ENV + config + CLI arg merging 🟢 v0.1.0 🐕 | [📖 Docs](https://github.com/tuulbelt/config-file-merger#readme) | [🚀 Examples](https://github.com/tuulbelt/config-file-merger/tree/main/examples/)
- **[Cross-Platform Path Normalizer](https://github.com/tuulbelt/cross-platform-path-normalizer)** — Windows/Unix path consistency 🟢 v0.1.0 🐕 | [📖 Docs](https://github.com/tuulbelt/cross-platform-path-normalizer#readme) | [🚀 Examples](https://github.com/tuulbelt/cross-platform-path-normalizer/tree/main/examples/)

### Testing & Observability
- **[Test Flakiness Detector](https://github.com/tuulbelt/test-flakiness-detector)** — Identify unreliable tests 🟢 v0.1.0 🐕 | [📖 Docs](https://github.com/tuulbelt/test-flakiness-detector#readme) | [🚀 Examples](https://github.com/tuulbelt/test-flakiness-detector/tree/main/examples/)
- **[Output Diffing Utility](https://github.com/tuulbelt/output-diffing-utility)** — Semantic diff for JSON, text, binary files 🟢 v0.1.0 🐕 | [📖 Docs](https://github.com/tuulbelt/output-diffing-utility#readme) | [🚀 Examples](https://github.com/tuulbelt/output-diffing-utility/tree/main/examples/)
- **[Snapshot Comparison](https://github.com/tuulbelt/snapshot-comparison)** — Snapshot testing with integrated diffs 🟢 v0.1.0 🐕 | [📖 Docs](https://github.com/tuulbelt/snapshot-comparison#readme) | [🚀 Examples](https://github.com/tuulbelt/snapshot-comparison/tree/main/examples/)
- **[Test Port Resolver](https://github.com/tuulbelt/port-resolver)** — Concurrent test port allocation 🟢 v0.1.0 🐕 | [📖 Docs](https://github.com/tuulbelt/port-resolver#readme) | [🚀 Examples](https://github.com/tuulbelt/port-resolver/tree/main/examples/)

### Frontend
- **[Component Prop Validator](https://github.com/tuulbelt/component-prop-validator)** — TypeScript runtime validation (TBD)
- **[Exhaustiveness Checker](https://github.com/tuulbelt/exhaustiveness-checker)** — Union case coverage for TS/JS (TBD)

### Data & Protocol
- **[Content-Addressable Blob Store](https://github.com/tuulbelt/content-addressable-blob-store)** — SHA-256 hash-based storage (TBD)
- **[Schema Converter (YAML ↔ JSON)](https://github.com/tuulbelt/schema-converter-yaml-json)** — Format conversion, no deps (TBD)
- **[Minimalist Pub-Sub Protocol](https://github.com/tuulbelt/pub-sub-protocol)** — Wire format for service messaging (TBD)
- **[Self-Describing Binary Wire Protocol](https://github.com/tuulbelt/wire-protocol)** — TLV format for RPC (TBD)

### APIs & Integration
- **[Request/Response Envelope Codec](https://github.com/tuulbelt/envelope-codec)** — Standard RPC response wrapping (TBD)
- **[API Versioning Helper](https://github.com/tuulbelt/api-versioning)** — Multi-version API logic (TBD)
- **[JSON Schema Validator](https://github.com/tuulbelt/json-schema-validator)** — Schema compliance checking (TBD)
- **[Streaming JSON Parser](https://github.com/tuulbelt/streaming-json-parser)** — Memory-efficient JSON parsing (TBD)

### Security & Networking
- **[Stateless Identity Generator](https://github.com/tuulbelt/identity-generator)** — Ed25519 token generation (TBD)
- **[Static Site Search Indexer](https://github.com/tuulbelt/static-search-indexer)** — HTML/Markdown → compressed index (TBD)
- **[Peer Discovery (UDP Multicast)](https://github.com/tuulbelt/peer-discovery)** — Local service discovery (TBD)
- **[One-File Reverse Proxy](https://github.com/tuulbelt/reverse-proxy)** — Minimal HTTP mapping (TBD)

### Utilities & Infrastructure
- **[Universal Log Normalizer](https://github.com/tuulbelt/log-normalizer)** — Structured log standardization (TBD)
- **[File-Based Semaphore](https://github.com/tuulbelt/file-based-semaphore)** — Cross-platform process locking (Rust) 🟢 v0.1.0 🐕 | [📖 Docs](https://github.com/tuulbelt/file-based-semaphore#readme) | [🚀 Examples](https://github.com/tuulbelt/file-based-semaphore/tree/main/examples/)
- **[File-Based Semaphore (TS)](https://github.com/tuulbelt/file-based-semaphore-ts)** — Cross-platform process locking (TypeScript) 🟢 v0.1.0 🐕 | [📖 Docs](https://github.com/tuulbelt/file-based-semaphore-ts#readme) | [🚀 Examples](https://github.com/tuulbelt/file-based-semaphore-ts/tree/main/examples/)
- **[Manifest-First Sync Tool](https://github.com/tuulbelt/manifest-sync)** — Directory sync via manifest diffs (TBD)
- **[Universal Health-Check Probe](https://github.com/tuulbelt/health-check-probe)** — Multi-check abstraction (TBD)
- **[Secret Injector](https://github.com/tuulbelt/secret-injector)** — Encrypted secret injection (TBD)
- **[Deterministic Task Runner](https://github.com/tuulbelt/task-runner)** — DAG executor with file-hash skipping (TBD)
- **[Zero-Overhead Timing](https://github.com/tuulbelt/timing-injector)** — Compile-time optional instrumentation (TBD)
- **[Deterministic Build Artifact Generator](https://github.com/tuulbelt/deterministic-builds)** — Reproducible builds (TBD)

### Observability
- **[Structured Trace-to-SVG](https://github.com/tuulbelt/trace-to-svg)** — Event → Flame Graph visualization (TBD)
- **[Backpressure Proxy](https://github.com/tuulbelt/backpressure-proxy)** — Cascading failure prevention (TBD)

### Interoperability
- **[FFI Binding Generator](https://github.com/tuulbelt/ffi-binding-generator)** — Rust FFI from C headers (TBD)

## Quick Start

Clone any tool independently:

```bash
git clone https://github.com/tuulbelt/<tool-name>.git
cd <tool-name>
npm test  # or cargo test for Rust tools
```

## Quick Examples

### Test Flakiness Detector

```bash
cd tools/test-flakiness-detector && npm install
flaky --test "npm test" --runs 10
```

[📖 Docs](https://github.com/tuulbelt/test-flakiness-detector#readme) | [🚀 Examples](https://github.com/tuulbelt/test-flakiness-detector/tree/main/examples/)

---

### CLI Progress Reporting

```bash
cd tools/cli-progress-reporting && npm install
prog init --total 100 --message "Processing files"
```

[📖 Docs](https://github.com/tuulbelt/cli-progress-reporting#readme) | [🚀 Examples](https://github.com/tuulbelt/cli-progress-reporting/tree/main/examples/)

---

### Cross-Platform Path Normalizer

```bash
cd tools/cross-platform-path-normalizer && npm install
normpath --format unix "C:\Users\file.txt"
```

[📖 Docs](https://github.com/tuulbelt/cross-platform-path-normalizer#readme) | [🚀 Examples](https://github.com/tuulbelt/cross-platform-path-normalizer/tree/main/examples/)

---

### Configuration File Merger

```bash
cd tools/config-file-merger && npm install
cfgmerge --file config.json --env --prefix APP_ --args "port=3000"
```

[📖 Docs](https://github.com/tuulbelt/config-file-merger#readme) | [🚀 Examples](https://github.com/tuulbelt/config-file-merger/tree/main/examples/)

---

### Test Port Resolver

```bash
cd tools/port-resolver && npm install
portres get --tag "api-server"
# Output: 54321
```

[📖 Docs](https://github.com/tuulbelt/port-resolver#readme) | [🚀 Examples](https://github.com/tuulbelt/port-resolver/tree/main/examples/)

_[See all 33 tools →](#current-tools)_

## Dogfooding: Tools Working Together

Tuulbelt tools validate and enhance each other through real-world composition:

**Snapshot Comparison** uses **Output Diffing Utility** as a library dependency for semantic diffs:
```rust
// First tool using library-level composition (PRINCIPLES.md Exception 2)
use output_diffing_utility::{diff_text, diff_json, diff_binary};

// When snapshots don't match, odiff provides rich diff output
let diff = match file_type {
    FileType::Text => diff_text(expected, actual, &config),
    FileType::Json => diff_json(expected, actual, &config),
    FileType::Binary => diff_binary(expected, actual, &config),
};
```

**Test Flakiness Detector** integrates **CLI Progress Reporting** for real-time progress tracking:
```bash
cd tools/test-flakiness-detector
flaky --test "npm test" --runs 20 --verbose
# [INFO] Progress tracking enabled (dogfooding cli-progress-reporting)
# Shows live run counts and pass/fail status
```

**CLI Progress Reporting** and **Cross-Platform Path Normalizer** use **Test Flakiness Detector** to validate their test suites:
```bash
cd tools/cli-progress-reporting
npm run test:dogfood
# ✅ NO FLAKINESS DETECTED (111 tests × 20 runs = 2,220 executions)

cd tools/cross-platform-path-normalizer
npm run test:dogfood
# ✅ NO FLAKINESS DETECTED (128 tests × 10 runs = 1,280 executions)
```

This creates a **bidirectional validation network** where tools prove their reliability by using each other in production workflows.

## Development

- Read [PRINCIPLES.md](PRINCIPLES.md) for design philosophy
- Read [ARCHITECTURE.md](ARCHITECTURE.md) for repo structure
- Read [CONTRIBUTING.md](CONTRIBUTING.md) for contribution workflow
- See [docs/](docs/) for detailed guides

## Status

🟢 = Implemented (10/33)
🟡 = In progress (0/33)
🔴 = TBD (23/33)

**Recently Completed:** Test Port Resolver v0.1.0 (2025-12-29)

**Next Up:** Component Prop Validator 🎯

**Progress:** 10 of 33 tools implemented (30%) | Phase 1 Quick Tools: 5/5 (100% ✅) | Phase 2: 5/28

## License

All tools are MIT licensed unless otherwise specified.

## Support

Found a bug? Have an idea? Open an issue at https://github.com/tuulbelt/tuulbelt/issues

Use labels to indicate which tool: `test-flakiness-detector`, `cli-progress`, etc.
