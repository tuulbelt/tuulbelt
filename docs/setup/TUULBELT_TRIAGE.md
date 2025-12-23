# Software Toolbelt Project Triage

## CLI/DevTools

| Tool | Complexity | Notes |
|------|-----------|-------|
| CLI progress/status reporting | Quick | Concurrent-safe. Proven pattern from existing tools. |
| Structured error/panic handling | Quick | Format + utilities for useful stack traces. Well-defined scope. |
| Cross-platform path handling | Quick | Normalize paths consistently, especially Windows. Standard library base. |
| Configuration file merger/resolver | Medium | ENV + config file + CLI arg merging. Clear precedence rules. |

## Testing/Observability

| Tool | Complexity | Notes |
|------|-----------|-------|
| Test flakiness detector | Quick | Run tests N times, identify unreliable ones. Proven approach. |
| Output diffing utility | Quick | Better than textual diffs for JSON, binary, structured data in assertions. |
| Snapshot comparison | Medium | Binary/structured data snapshots in tests, not just strings. |
| Test output isolation (port conflicts) | Medium | Concurrent test port collision detection. Assign free ports to parallel tests. |

## Frontend

| Tool | Complexity | Notes |
|------|-----------|-------|
| Component prop validation at runtime | Medium | TypeScript types + runtime validation codec. Generated code has zero deps. |
| Exhaustiveness checker | Medium | Catch unhandled union cases at compile time. TypeScript/JavaScript focused. |

## Data & Protocol Level

| Tool | Complexity | Notes |
|------|-----------|-------|
| Content-Addressable Blob Store | Medium | SHA-256 hash-based storage. Deduplication built-in. Git model. |
| Unified Schema Converter (YAML ↔ JSON first) | Medium | Phased approach: start YAML ↔ JSON, add TOML ↔ JSON next. Separate tools, shared parsing primitives. |

## Communication & Networking Protocols

| Tool | Complexity | Notes |
|------|-----------|-------|
| Minimalist Pub-Sub Protocol | Medium | Wire protocol for service messaging. Low-bandwidth, high-reliability. No heavy brokers. |
| Self-Describing Binary Wire Protocol | Medium | Tag-Length-Value format. Basic nesting + type-hinting. Parsable state machine, zero deps. |
| Dead-Simple Peer Discovery | Medium | UDP Multicast for local service discovery. Outputs IP/Port to stdout. No central registry. |
| One-File Reverse Proxy | Medium | Basic HTTP mapping. Single mapping file or CLI flags. No plugin system. |

## Data/Schema & APIs

| Tool | Complexity | Notes |
|------|-----------|-------|
| Request/response envelope codec | Medium | Standard RPC/HTTP response wrapping (errors, metadata, versioning). Cross-language. |
| API versioning helper | Medium | Router-agnostic logic for handling multiple API versions cleanly. |
| JSON Schema validator | Medium | Compliance checking without heavy deps. Code gen or runtime validator option. |
| Streaming JSON parser | Medium | Handle incomplete/large payloads without full memory load. Rust-focused. |

## Web Frontier & Security

| Tool | Complexity | Notes |
|------|-----------|-------|
| Stateless Identity Generator (Ed25519 first) | Medium | Ed25519 token generation + verification. Design for algorithm-agnostic swaps (PQC-ready later). |
| Static Site Search Indexer | Medium | Crawls HTML/Markdown, produces compressed index (Bloom filter model). <5KB vanilla JS query script. |

## Developer Productivity

| Tool | Complexity | Notes |
|------|-----------|-------|
| Universal Log Normalizer | Medium | CLI tool. Pipes unstructured logs from various sources to standard JSON stream. Regex-based recipes. |

## Storage & State Management

| Tool | Complexity | Notes |
|------|-----------|-------|
| File-Based Semaphore | Quick | Cross-platform locking utility. Manages concurrency for shell scripts/distributed processes. Handles stale locks. |
| Manifest-First Sync Tool | Medium | Generates text manifest (hash, path, size). Compares manifests, outputs diff as action list. User decides transport. |

## Execution & System Utilities

| Tool | Complexity | Notes |
|------|-----------|-------|
| Universal Health-Check Probe | Medium | Multi-check abstraction (HTTP 200, TCP connect, File exists, Script exit 0). Bridge to orchestration layers. |
| Secret Injector | Medium | Reads encrypted file (AES-GCM/ChaCha20), injects into child process environment. Never writes plaintext to disk. |
| Deterministic Task Runner | Medium | Explicit DAG executor with file-hash skipping. YAML/TOML task manifest. Skip tasks if source files unchanged. |

## Build/Performance

| Tool | Complexity | Notes |
|------|-----------|-------|
| Zero-overhead timing | Medium | Inject timing instrumentation, remove at compile-time if unused. |
| Deterministic build artifact generator | Medium | Same input = same output always. Useful for reproducibility. |

## Reliability & Observability

| Tool | Complexity | Notes |
|------|-----------|-------|
| Structured Trace-to-SVG | Medium | Events (Timestamp, EventName, Duration) → static SVG Flame Graph. No web dashboard or heavy tracing library. |
| Backpressure Proxy | Medium | Sits in front of service. Returns 503/drops packets on slow responses. Prevents cascading failures. |

## Interop

| Tool | Complexity | Notes |
|------|-----------|-------|
| FFI binding generator | Medium | Given C header, generate Rust FFI boilerplate. Code gen, no interpretation. |

---

## Summary

**Quick (days):** 5 tools
**Medium (1-2 weeks):** 28 tools
**Total:** 33 tools across 12 categories

---

## Removed

- Test output isolation (repurposed to port conflicts, narrowed scope)
- Incremental build cache invalidation (too vague, overlaps existing solutions)
- Local secret/env management (scope unclear)
- Form state management (too broad)
- Dependency graph visualization (subjective heuristics)
- Module resolution debugger (Node.js-only, too narrow)
- Ordered Key-Value Log (LSM-lite) (unuseful without compaction; defer indefinitely)
- Zero-Config Binary Bundler (language-specific, violates portability)
- Deterministic Task Runner (repurposed to explicit DAG with file-hash skipping)

---

## Design Principles

These tools stay focused by following:

| Principle | Application |
|-----------|-------------|
| **Single Problem** | Each tool solves one painful problem, not many. |
| **Standard Library Only** | No mandatory package dependencies. |
| **Portable Interface** | CLI/Files/Sockets, not proprietary APIs. |
| **Composable** | Works via pipes, not plugins. |
