# /new-protocol Command

Create a new Tuulbelt protocol specification with reference implementation.

## Usage

```
/new-protocol <protocol-name> [description]
```

## Parameters

| Parameter | Required | Description | Example |
|-----------|----------|-------------|---------|
| protocol-name | Yes | Full kebab-case name | `wire-protocol` |
| description | No | One-line description | `"Self-describing binary TLV format"` |

## Examples

```bash
# Full specification
/new-protocol wire-protocol "Self-describing binary TLV format"

# Minimal (auto-generates description)
/new-protocol message-envelope
```

## What This Command Does

This command automates protocol creation with the following steps:

### Phase 1: Repository Creation

1. Check if repo already exists
2. Create GitHub repository: `tuulbelt/<protocol-name>`
3. Configure repo settings (description, topics, issues → meta repo)
4. Clone to local workspace

### Phase 2: Scaffolding

5. Copy `templates/protocol-template/`
6. Customize `Cargo.toml` with protocol info
7. Generate README.md with all sections
8. Create SPEC.md template (REQUIRED)
9. Set up reference implementation in `src/lib.rs`
10. Create test vectors in `tests/vectors.rs`

### Phase 3: Meta Repo Integration

11. Add as git submodule to `protocols/`
12. Update `docs/.vitepress/config.ts` (sidebar)
13. Update `docs/protocols/index.md` (count, card)
14. Update `README.md` (protocol list)

### Phase 4: Verification

15. Run `cargo build`
16. Run `cargo test`
17. Run `cargo clippy -- -D warnings`
18. Verify zero runtime dependencies

### Phase 5: Commit

19. Commit protocol repo: `feat: initialize {protocol-name} from template`
20. Push protocol repo to origin
21. Commit meta repo: `chore: add {protocol-name} submodule`

## Protocol Template Structure

```
protocol-name/
├── SPEC.md               # Protocol specification (REQUIRED)
├── src/
│   ├── lib.rs            # Reference implementation
│   └── main.rs           # CLI for testing
├── tests/
│   └── vectors.rs        # Compliance test vectors
├── examples/
│   └── basic.rs
├── Cargo.toml            # Zero dependencies
├── README.md
├── CLAUDE.md
├── LICENSE
└── .github/workflows/test.yml
```

## SPEC.md Requirements

The specification document must include:

1. **Abstract** - One paragraph summary
2. **Wire Format** - Byte-level layout with diagrams
3. **Message Types** - All message type definitions
4. **Encoding Rules** - Endianness, string encoding, etc.
5. **Error Handling** - All error codes and meanings
6. **Security Considerations** - Authentication, validation
7. **Test Vectors** - Example inputs and expected outputs

## Quality Requirements

Protocols must have:

- [ ] Complete SPEC.md specification
- [ ] Zero runtime dependencies in reference implementation
- [ ] Compliance test vectors
- [ ] All error cases handled
- [ ] CLI for testing/debugging

## Key Differences from Other Categories

| Aspect | Protocol | Tool | Library |
|--------|----------|------|---------|
| Primary Artifact | SPEC.md | CLI | API |
| Language | Rust (typically) | TS or Rust | TS (typically) |
| SPEC.md | Required | Optional | Not applicable |
| Cross-language | Goal | Not required | Not required |

## Governance

Protocols have **high governance** requirements:
- Specification changes require review
- Wire format changes are breaking changes
- Test vectors must be comprehensive

## Post-Creation Tasks

After the protocol is created:

1. **Complete the specification**
   ```bash
   cd protocols/wire-protocol
   # Edit SPEC.md with full specification
   ```

2. **Implement reference**
   ```bash
   # Edit src/lib.rs to match spec
   cargo test
   ```

3. **Add test vectors**
   ```bash
   # Edit tests/vectors.rs with compliance tests
   cargo test vectors
   ```

4. **Create first release**
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

## Related Commands

- `/new-tool` - Create CLI tools
- `/new-library` - Create programmatic APIs
- `/new-research` - Create experimental projects
