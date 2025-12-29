# Tuulbelt GitHub MCP Server

Model Context Protocol (MCP) server for GitHub API operations in the Tuulbelt meta repository.

## Purpose

This MCP server provides GitHub repository management capabilities to Claude Code, enabling automated tool creation and configuration. It's used by:
- `/new-tool` command for creating new tool repositories
- `tool-creator` agent for comprehensive tool setup
- Other automation workflows requiring GitHub API access

## Features

### Available Tools

| Tool | Description | Parameters |
|------|-------------|------------|
| `check_repo_exists` | Check if repository exists | `name: string` |
| `create_tool_repo` | Create new tool repository | `name, description, language, is_private` |
| `configure_repo_settings` | Set up branch protection, labels | `repo: string` |
| `create_github_labels` | Create issue labels for tool | `repo, labels[]` |
| `get_repo_info` | Get repository metadata | `repo: string` |
| `delete_repo` | Delete repository (careful!) | `repo: string, confirm: boolean` |

## Setup

### Prerequisites

1. **GitHub Personal Access Token**
   - Go to GitHub Settings → Developer settings → Personal access tokens → Tokens (classic)
   - Generate new token with these scopes:
     - `repo` (full repository access)
     - `admin:org` (for creating repos in tuulbelt org)
   - Copy the token

2. **Environment Variable**
   ```bash
   export GITHUB_TOKEN=your_token_here
   export GITHUB_ORG=tuulbelt  # Optional, defaults to "tuulbelt"
   ```

   Add to your shell profile (`~/.zshrc`, `~/.bashrc`, etc.) for persistence:
   ```bash
   echo 'export GITHUB_TOKEN=your_token_here' >> ~/.zshrc
   source ~/.zshrc
   ```

### Installation

From the tuulbelt meta repository root:

```bash
cd .claude/mcp/tuulbelt-github
npm install
```

### Configuration

The MCP server is configured via `.mcp.json` at the project root:

```json
{
  "mcpServers": {
    "tuulbelt-github": {
      "type": "stdio",
      "command": "node",
      "args": ["./.claude/mcp/tuulbelt-github/index.js"],
      "env": {
        "GITHUB_TOKEN": "${GITHUB_TOKEN}",
        "GITHUB_ORG": "tuulbelt"
      }
    }
  }
}
```

Claude Code will automatically load this configuration when you open the project.

## Usage

### In Claude Code CLI

Once configured, you can use GitHub operations in your prompts:

```
Create a new repository called "test-utility" for a TypeScript tool
```

Claude will use the MCP server to execute GitHub API calls automatically.

### Direct Testing (Optional)

You can test the MCP server directly:

```bash
cd .claude/mcp/tuulbelt-github
export GITHUB_TOKEN=your_token
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | node index.js
```

## Security

**IMPORTANT:**
- Never commit your GitHub token to version control
- Keep `.env` files out of git (already in `.gitignore`)
- Use minimal required scopes for your token
- Rotate tokens periodically
- Each developer uses their own token

The MCP server:
- Reads `GITHUB_TOKEN` from environment only
- Does not log or store credentials
- Validates all operations before execution
- Requires `confirm=true` for destructive operations (delete)

## Troubleshooting

### "GITHUB_TOKEN not set" Error

```bash
# Verify token is set
echo $GITHUB_TOKEN

# If empty, set it
export GITHUB_TOKEN=your_token_here
```

### "Permission denied" Errors

Check your token scopes:
- `repo` - Required for all repository operations
- `admin:org` - Required for creating repos in organization

### MCP Server Not Loading

1. Verify `.mcp.json` exists at project root
2. Check syntax: `cat .mcp.json | jq .`
3. Restart Claude Code CLI
4. Check Claude Code logs for errors

### Repository Creation Fails

Common issues:
- Token lacks `admin:org` scope
- Repository name already exists (check with `check_repo_exists`)
- Invalid repository name (must be kebab-case, lowercase)

## Development

### Adding New Tools

1. Add tool definition to `ListToolsRequestSchema` handler
2. Implement tool function
3. Add case to `CallToolRequestSchema` handler
4. Update README.md with tool documentation
5. Test with `/test-all`

### Testing Changes

```bash
# Run npm install after changes
npm install

# Test tool listing
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | node index.js | jq .

# Test specific tool
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"check_repo_exists","arguments":{"name":"test-flakiness-detector"}}}' | node index.js | jq .
```

## References

- [MCP SDK Documentation](https://github.com/modelcontextprotocol/sdk)
- [Octokit REST API](https://octokit.github.io/rest.js/)
- [GitHub API Documentation](https://docs.github.com/en/rest)
- [Tuulbelt Migration Plan](../../../docs/MIGRATION_TO_META_REPO.md)

## License

MIT
