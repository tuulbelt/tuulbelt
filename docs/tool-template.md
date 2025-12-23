# Tool Template

This document provides step-by-step guidance for creating a new Tuulbelt tool.

## Quick Start

1. Copy the template: `cp -r templates/tool-repo-template/ ../my-new-tool/`
2. Update `package.json` with your tool's name and description
3. Implement core logic in `src/index.ts`
4. Write tests in `test/index.test.ts`
5. Run `npm test` to verify
6. Update `README.md` with usage documentation

## Template Structure

```
tool-repo-template/
├── src/
│   └── index.ts          # Main implementation
├── test/
│   └── index.test.ts     # Test suite
├── examples/
│   └── basic.ts          # Usage example
├── package.json          # Package metadata
├── tsconfig.json         # TypeScript config
├── README.md             # Documentation
├── SPEC.md               # Specification (optional)
├── LICENSE               # MIT license
└── .github/
    └── workflows/
        └── test.yml      # CI/CD workflow
```

## Customization Checklist

- [ ] Update `package.json` name to `@tuulbelt/your-tool-name`
- [ ] Update `package.json` description
- [ ] Update `package.json` repository URL
- [ ] Implement core functionality in `src/index.ts`
- [ ] Write comprehensive tests in `test/index.test.ts`
- [ ] Create usage example in `examples/basic.ts`
- [ ] Write clear documentation in `README.md`
- [ ] (Optional) Add formal specification in `SPEC.md`

## See Also

- [CONTRIBUTING.md](../CONTRIBUTING.md) — Full contribution workflow
- [PRINCIPLES.md](../PRINCIPLES.md) — Design principles
- [testing-standards.md](testing-standards.md) — Testing requirements
- [claude-code-workflow.md](claude-code-workflow.md) — Claude Code best practices
