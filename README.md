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
- **[Structured Error Handler](https://github.com/tuulbelt/structured-error-handler)** — Error format + serialization with context preservation (TBD)
- **[CLI Progress Reporting](cli-progress-reporting/)** — Concurrent-safe progress updates 🟢 v0.1.0 | [📖 Docs](cli-progress-reporting/) | [🚀 Examples](cli-progress-reporting/examples/)
- **[Configuration File Merger](https://github.com/tuulbelt/config-file-merger)** — ENV + config + CLI arg merging (TBD)
- **[Cross-Platform Path Normalizer](cross-platform-path-normalizer/)** — Windows/Unix path consistency 🟢 v0.1.0 | [📖 Docs](cross-platform-path-normalizer/)

### Testing & Observability
- **[Test Flakiness Detector](test-flakiness-detector/)** — Identify unreliable tests 🟢 v0.1.0 | [📖 Docs](test-flakiness-detector/docs/) | [🚀 Examples](test-flakiness-detector/examples/)
- **[Output Diffing Utility](https://github.com/tuulbelt/output-diffing)** — Semantic diffs for JSON/binary in assertions (TBD)
- **[Snapshot Comparison](https://github.com/tuulbelt/snapshot-comparison)** — Binary/structured data snapshots (TBD)
- **[Test Port Conflict Resolver](https://github.com/tuulbelt/test-port-resolver)** — Concurrent test port allocation (TBD)

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
- **[File-Based Semaphore](https://github.com/tuulbelt/file-semaphore)** — Cross-platform process locking (TBD)
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

### Try Our Tools Now

**Test Flakiness Detector** — Identify unreliable tests

```bash
cd test-flakiness-detector
npm install
npx tsx src/index.ts --test "npm test" --runs 10
```

**Features:**
- 🎯 Framework agnostic - works with any test command
- 🔍 Detects flaky tests through repeated execution
- 📊 Comprehensive JSON reports with failure rates
- ⚡ Zero runtime dependencies
- 🚀 107+ tests with 80%+ coverage
- 📖 [Full Documentation](test-flakiness-detector/docs/)
- 🎮 [Example Outputs](test-flakiness-detector/examples/)

---

**CLI Progress Reporting** — Concurrent-safe progress updates

```bash
cd cli-progress-reporting
npm install

# Initialize progress tracker
npx tsx src/index.ts init --total 100 --message "Processing files"

# Update progress
npx tsx src/index.ts increment --amount 10

# Check current status
npx tsx src/index.ts get
```

**Features:**
- 🔒 Concurrent-safe with file-based atomic writes
- 🆔 Multiple independent progress trackers
- 🛠️ CLI and library API
- 💾 State persistence across processes
- ⚡ Zero runtime dependencies
- 🚀 93 tests, dogfooding validated
- 📖 [Full Documentation](cli-progress-reporting/)

## Development

- Read [PRINCIPLES.md](PRINCIPLES.md) for design philosophy
- Read [ARCHITECTURE.md](ARCHITECTURE.md) for repo structure
- Read [CONTRIBUTING.md](CONTRIBUTING.md) for contribution workflow
- See [docs/](docs/) for detailed guides

## Status

🟢 = Implemented (3/33)
🟡 = In progress (0/33)
🔴 = TBD (30/33)

**Recently Completed:** Cross-Platform Path Normalizer v0.1.0 (2025-12-24)

**Next Up:** File-Based Semaphore or Output Diffing Utility 🎯

**Progress:** 3 of 33 tools implemented (9%) | Phase 1 Quick Tools: 3/5 (60%)

## License

All tools are MIT licensed unless otherwise specified.

## Support

Found a bug? Have an idea? Open an issue at https://github.com/tuulbelt/tuulbelt/issues

Use labels to indicate which tool: `test-flakiness-detector`, `cli-progress`, etc.
