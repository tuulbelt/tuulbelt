# Config File Merger

Merge configuration from ENV variables, config files, and CLI arguments with clear precedence rules and source tracking.

## Overview

Config File Merger (`cfgmerge`) solves the common problem of combining configuration from multiple sources with explicit precedence. It handles environment variables, JSON config files, CLI arguments, and default values—all with zero external dependencies.

**Status:** <img src="/icons/check-circle.svg" class="inline-icon" alt=""> Production Ready (v0.1.0)

**Language:** TypeScript

**Repository:** [tuulbelt/tuulbelt/config-file-merger](https://github.com/tuulbelt/tuulbelt/tree/main/config-file-merger)

## Features

### <img src="/icons/layers.svg" class="inline-icon" alt=""> Clear Precedence

Explicit merge order: CLI arguments > Environment variables > Config file > Defaults. No ambiguity about which source wins.

### <img src="/icons/search.svg" class="inline-icon" alt=""> Source Tracking

Optionally track where each configuration value came from. Know instantly whether a setting came from the command line, environment, file, or defaults.

### <img src="/icons/code.svg" class="inline-icon" alt=""> Automatic Type Coercion

CLI arguments are automatically parsed: `"true"` → `true`, `"42"` → `42`, `"null"` → `null`. Quoted strings preserve their type.

### <img src="/icons/hash.svg" class="inline-icon" alt=""> Prefix Filtering

Filter environment variables by prefix (e.g., `APP_`) with optional case normalization and prefix stripping.

### <img src="/icons/zap.svg" class="inline-icon" alt=""> Zero Runtime Dependencies

Uses only Node.js built-ins. No `npm install` required in production.

## Quick Start

```bash
# Clone the repository
git clone https://github.com/tuulbelt/tuulbelt.git
cd tuulbelt/config-file-merger

# Install dev dependencies (for TypeScript)
npm install

# Enable global CLI
npm link

# Merge configuration sources
cfgmerge --file config.json --env --prefix APP_ --args "port=3000"
```

## Use Cases

- **Application Configuration:** Merge defaults, config files, and runtime overrides
- **Environment-Aware Deployment:** Use env vars in containers, files locally
- **CLI Tool Configuration:** Accept command-line overrides for any setting
- **Configuration Debugging:** Track which source set each value
- **Twelve-Factor Apps:** Environment-based configuration with fallbacks

## Precedence Rules

Values are merged in this order (highest precedence first):

1. **CLI arguments** (`--args`) — explicit overrides
2. **Environment variables** (`--env`) — deployment config
3. **Config file** (`--file`) — application defaults
4. **Default values** (`--defaults`) — fallback values

## Demo

See the tool in action:

![Config File Merger Demo](/config-file-merger/demo.gif)

**[▶ View interactive recording on asciinema.org](https://asciinema.org/a/Z6MnFzDhqu6fVAGYA4veOlGk6)**

<div style="margin: 20px 0;">
  <span style="display: inline-block; vertical-align: middle; margin-right: 8px;">
    <strong>Try it online:</strong>
  </span>
  <a href="https://stackblitz.com/github/tuulbelt/tuulbelt/tree/main/config-file-merger" style="display: inline-block; vertical-align: middle;">
    <img src="https://developer.stackblitz.com/img/open_in_stackblitz.svg" alt="Open in StackBlitz" style="vertical-align: middle;">
  </a>
</div>

Merge configurations directly in your browser with zero setup. Experiment with different sources, precedence, and output formats.

> Demos are automatically generated via GitHub Actions when demo scripts are updated.

## Next Steps

- [Getting Started Guide](/tools/config-file-merger/getting-started) - Installation and setup
- [CLI Usage](/tools/config-file-merger/cli-usage) - Command-line interface
- [Library Usage](/tools/config-file-merger/library-usage) - TypeScript/JavaScript API
- [Examples](/tools/config-file-merger/examples) - Real-world usage patterns
- [API Reference](/tools/config-file-merger/api-reference) - Complete API documentation

## License

MIT License - see repository for details.
