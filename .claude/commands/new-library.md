# /new-library Command

Create a new Tuulbelt library with full automation.

## Usage

```
/new-library <library-name> [description]
```

## Parameters

| Parameter | Required | Description | Example |
|-----------|----------|-------------|---------|
| library-name | Yes | Full kebab-case name | `result-type` |
| description | No | One-line description | `"Rust-style Result for TypeScript"` |

## Examples

```bash
# Full specification
/new-library result-type "Rust-style Result<T,E> for TypeScript"

# Minimal (auto-generates description)
/new-library option-type
```

## What This Command Does

This command automates library creation with the following steps:

### Phase 1: Repository Creation

1. Check if repo already exists
2. Create GitHub repository: `tuulbelt/<library-name>`
3. Configure repo settings (description, topics, issues → meta repo)
4. Clone to local workspace

### Phase 2: Scaffolding

5. Copy `templates/library-template/`
6. Customize `package.json` with library info
7. Generate README.md with all sections
8. Create CHANGELOG.md with initial entry
9. Set up API.md template
10. Create test boilerplate

### Phase 3: Meta Repo Integration

11. Add as git submodule to `libraries/`
12. Update `docs/.vitepress/config.ts` (sidebar)
13. Update `docs/libraries/index.md` (count, card)
14. Update `README.md` (library list)

### Phase 4: Verification

15. Run `npm install`
16. Run `npm test`
17. Run `npm run build`
18. Verify zero runtime dependencies

### Phase 5: Commit

19. Commit library repo: `feat: initialize {library-name} from template`
20. Push library repo to origin
21. Commit meta repo: `chore: add {library-name} submodule`

## Library Template Structure

```
library-name/
├── src/
│   ├── index.ts          # Main exports (public API)
│   └── types.ts          # Additional type exports
├── test/
│   └── index.test.ts     # Tests
├── examples/
│   ├── basic.ts
│   └── advanced.ts
├── benchmarks/           # Optional performance benchmarks
├── package.json          # No bin entry (not CLI)
├── tsconfig.json
├── API.md                # Required API documentation
├── README.md
├── CLAUDE.md
├── LICENSE
└── .github/workflows/test.yml
```

## Key Differences from Tools

| Aspect | Tool | Library |
|--------|------|---------|
| Interface | CLI-first | API-first |
| `bin` in package.json | Required | Not present |
| API.md | Optional | Required |
| Short name | Required | Not applicable |

## Quality Requirements

Libraries must have:

- [ ] Zero runtime dependencies
- [ ] Comprehensive TypeScript types
- [ ] API.md documentation
- [ ] 80%+ test coverage
- [ ] Examples for common use cases

## Post-Creation Tasks

After the library is created:

1. **Implement the library**
   ```bash
   cd libraries/result-type
   npm install
   npm test
   ```

2. **Add real implementation**
   - Replace placeholder code in `src/index.ts`
   - Add real tests
   - Update API.md with actual API

3. **Create first release**
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

## Related Commands

- `/new-tool` - Create CLI tools
- `/new-protocol` - Create wire format specifications
- `/new-research` - Create experimental projects
