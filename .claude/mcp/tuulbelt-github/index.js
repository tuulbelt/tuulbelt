#!/usr/bin/env node
/**
 * Tuulbelt GitHub MCP Server
 *
 * Provides GitHub API operations for tool repository management.
 * Used by /new-tool command and tool-creator agent.
 */

import { Server } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";
import { Octokit } from "octokit";
import { readFileSync, existsSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";

// Load .env file from project root (3 levels up from this file)
const __dirname = dirname(fileURLToPath(import.meta.url));
const envPath = join(__dirname, '../../..', '.env');

if (existsSync(envPath)) {
  const envContent = readFileSync(envPath, 'utf-8');
  envContent.split('\n').forEach(line => {
    const trimmed = line.trim();
    // Skip empty lines and comments
    if (!trimmed || trimmed.startsWith('#')) return;

    const match = trimmed.match(/^([^=]+)=(.*)$/);
    if (match) {
      const key = match[1].trim();
      let value = match[2].trim();

      // Remove quotes if present
      if ((value.startsWith('"') && value.endsWith('"')) ||
          (value.startsWith("'") && value.endsWith("'"))) {
        value = value.slice(1, -1);
      }

      // Only set if not already defined (allow override from .mcp.json)
      if (!process.env[key]) {
        process.env[key] = value;
      }
    }
  });
}

const server = new Server(
  {
    name: "tuulbelt-github",
    version: "1.0.0",
  },
  {
    capabilities: {
      tools: {},
    },
  }
);

const octokit = new Octokit({ auth: process.env.GITHUB_TOKEN });
const org = process.env.GITHUB_ORG || "tuulbelt";

// Tool: check_repo_exists
async function checkRepoExists(name) {
  try {
    await octokit.rest.repos.get({ owner: org, repo: name });
    return {
      content: [{ type: "text", text: JSON.stringify({ exists: true }, null, 2) }]
    };
  } catch (e) {
    if (e.status === 404) {
      return {
        content: [{ type: "text", text: JSON.stringify({ exists: false }, null, 2) }]
      };
    }
    return {
      content: [{ type: "text", text: JSON.stringify({ error: e.message }, null, 2) }],
      isError: true
    };
  }
}

// Tool: create_tool_repo
async function createToolRepo({ name, description, language, is_private = false }) {
  try {
    const repo = await octokit.rest.repos.createInOrg({
      org,
      name,
      description: description || `Tuulbelt tool: ${name}`,
      private: is_private,
      auto_init: false,  // We'll push our own content
      has_issues: true,
      has_projects: false,
      has_wiki: false,
    });

    return {
      content: [{
        type: "text",
        text: JSON.stringify({
          success: true,
          url: repo.data.html_url,
          clone_url: repo.data.clone_url,
          ssh_url: repo.data.ssh_url
        }, null, 2)
      }]
    };
  } catch (e) {
    return {
      content: [{ type: "text", text: JSON.stringify({ error: e.message }, null, 2) }],
      isError: true
    };
  }
}

// Tool: configure_repo_settings
async function configureRepoSettings(repo) {
  try {
    // Create standard labels
    const labels = [
      { name: "bug", color: "d73a4a", description: "Something isn't working" },
      { name: "enhancement", color: "a2eeef", description: "New feature or request" },
      { name: "documentation", color: "0075ca", description: "Documentation improvements" },
      { name: "good first issue", color: "7057ff", description: "Good for newcomers" },
      { name: "security", color: "ee0701", description: "Security-related issue" },
      { name: "performance", color: "bfd4f2", description: "Performance improvement" }
    ];

    for (const label of labels) {
      try {
        await octokit.rest.issues.createLabel({ owner: org, repo, ...label });
      } catch (e) {
        // Label may already exist, that's fine
      }
    }

    return {
      content: [{ type: "text", text: JSON.stringify({
        configured: true,
        note: "Branch protection should be applied after first push using apply_branch_protection tool"
      }, null, 2) }]
    };
  } catch (e) {
    return {
      content: [{ type: "text", text: JSON.stringify({ error: e.message }, null, 2) }],
      isError: true
    };
  }
}

// Tool: apply_branch_protection
async function applyBranchProtection({ repo, branch = "main" }) {
  try {
    // Check if branch exists first
    try {
      await octokit.rest.repos.getBranch({ owner: org, repo, branch });
    } catch (e) {
      if (e.status === 404) {
        return {
          content: [{ type: "text", text: JSON.stringify({
            error: `Branch '${branch}' does not exist in ${org}/${repo}. Push to main first.`
          }, null, 2) }],
          isError: true
        };
      }
      throw e;
    }

    // Apply comprehensive branch protection
    await octokit.rest.repos.updateBranchProtection({
      owner: org,
      repo,
      branch,
      required_status_checks: {
        strict: true,
        contexts: ["test"]
      },
      enforce_admins: false,
      required_pull_request_reviews: {
        dismissal_restrictions: {},
        dismiss_stale_reviews: true,
        require_code_owner_reviews: false,
        required_approving_review_count: 1,
        require_last_push_approval: false,
        bypass_pull_request_allowances: {}
      },
      restrictions: null,
      required_linear_history: true,
      allow_force_pushes: false,
      allow_deletions: false,
      block_creations: false,
      required_conversation_resolution: true,
      lock_branch: false,
      allow_fork_syncing: true
    });

    return {
      content: [{ type: "text", text: JSON.stringify({
        success: true,
        repo,
        branch,
        protection_rules: {
          require_pull_request: true,
          required_approving_reviews: 1,
          dismiss_stale_reviews: true,
          require_status_checks: ["test"],
          require_branches_up_to_date: true,
          require_linear_history: true,
          require_conversation_resolution: true,
          block_force_pushes: true,
          block_deletions: true
        }
      }, null, 2) }]
    };
  } catch (e) {
    return {
      content: [{ type: "text", text: JSON.stringify({
        error: e.message,
        status: e.status,
        repo,
        branch
      }, null, 2) }],
      isError: true
    };
  }
}

// Tool: create_github_labels
async function createGitHubLabels({ repo, labels }) {
  try {
    const results = [];
    for (const label of labels) {
      try {
        await octokit.rest.issues.createLabel({
          owner: org,
          repo,
          name: label.name,
          color: label.color || "cccccc",
          description: label.description || ""
        });
        results.push({ name: label.name, status: "created" });
      } catch (e) {
        if (e.status === 422) {
          results.push({ name: label.name, status: "already_exists" });
        } else {
          results.push({ name: label.name, status: "error", error: e.message });
        }
      }
    }

    return {
      content: [{ type: "text", text: JSON.stringify({ results }, null, 2) }]
    };
  } catch (e) {
    return {
      content: [{ type: "text", text: JSON.stringify({ error: e.message }, null, 2) }],
      isError: true
    };
  }
}

// Tool: get_repo_info
async function getRepoInfo(repo) {
  try {
    const repoData = await octokit.rest.repos.get({ owner: org, repo });
    return {
      content: [{
        type: "text",
        text: JSON.stringify({
          name: repoData.data.name,
          full_name: repoData.data.full_name,
          description: repoData.data.description,
          url: repoData.data.html_url,
          clone_url: repoData.data.clone_url,
          ssh_url: repoData.data.ssh_url,
          default_branch: repoData.data.default_branch,
          created_at: repoData.data.created_at,
          updated_at: repoData.data.updated_at
        }, null, 2)
      }]
    };
  } catch (e) {
    return {
      content: [{ type: "text", text: JSON.stringify({ error: e.message }, null, 2) }],
      isError: true
    };
  }
}

// Tool: delete_repo
async function deleteRepo({ repo, confirm = false }) {
  if (!confirm) {
    return {
      content: [{
        type: "text",
        text: JSON.stringify({
          error: "Must set confirm=true to delete repository. This action is irreversible."
        }, null, 2)
      }],
      isError: true
    };
  }

  try {
    await octokit.rest.repos.delete({ owner: org, repo });
    return {
      content: [{ type: "text", text: JSON.stringify({ deleted: true, repo }, null, 2) }]
    };
  } catch (e) {
    return {
      content: [{ type: "text", text: JSON.stringify({ error: e.message }, null, 2) }],
      isError: true
    };
  }
}

// Register tools with MCP server
server.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [
    {
      name: "check_repo_exists",
      description: "Check if a GitHub repository exists in the tuulbelt organization",
      inputSchema: {
        type: "object",
        properties: {
          name: {
            type: "string",
            description: "Repository name (e.g., 'test-flakiness-detector')"
          }
        },
        required: ["name"]
      }
    },
    {
      name: "create_tool_repo",
      description: "Create a new tool repository in the tuulbelt organization",
      inputSchema: {
        type: "object",
        properties: {
          name: {
            type: "string",
            description: "Repository name (kebab-case)"
          },
          description: {
            type: "string",
            description: "Repository description"
          },
          language: {
            type: "string",
            enum: ["typescript", "rust"],
            description: "Primary programming language"
          },
          is_private: {
            type: "boolean",
            description: "Whether the repository should be private (default: false)",
            default: false
          }
        },
        required: ["name", "language"]
      }
    },
    {
      name: "configure_repo_settings",
      description: "Configure repository settings (branch protection, labels)",
      inputSchema: {
        type: "object",
        properties: {
          repo: {
            type: "string",
            description: "Repository name"
          }
        },
        required: ["repo"]
      }
    },
    {
      name: "apply_branch_protection",
      description: "Apply comprehensive branch protection rules to a repository (call after first push)",
      inputSchema: {
        type: "object",
        properties: {
          repo: {
            type: "string",
            description: "Repository name"
          },
          branch: {
            type: "string",
            description: "Branch to protect (default: main)",
            default: "main"
          }
        },
        required: ["repo"]
      }
    },
    {
      name: "create_github_labels",
      description: "Create issue labels for a repository",
      inputSchema: {
        type: "object",
        properties: {
          repo: {
            type: "string",
            description: "Repository name"
          },
          labels: {
            type: "array",
            description: "Array of label objects with name, color, description",
            items: {
              type: "object",
              properties: {
                name: { type: "string" },
                color: { type: "string" },
                description: { type: "string" }
              },
              required: ["name"]
            }
          }
        },
        required: ["repo", "labels"]
      }
    },
    {
      name: "get_repo_info",
      description: "Get information about a repository",
      inputSchema: {
        type: "object",
        properties: {
          repo: {
            type: "string",
            description: "Repository name"
          }
        },
        required: ["repo"]
      }
    },
    {
      name: "delete_repo",
      description: "Delete a repository (use with caution!)",
      inputSchema: {
        type: "object",
        properties: {
          repo: {
            type: "string",
            description: "Repository name"
          },
          confirm: {
            type: "boolean",
            description: "Must be true to actually delete",
            default: false
          }
        },
        required: ["repo", "confirm"]
      }
    }
  ]
}));

// Handle tool calls
server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args } = request.params;

  switch (name) {
    case "check_repo_exists":
      return checkRepoExists(args.name);

    case "create_tool_repo":
      return createToolRepo(args);

    case "configure_repo_settings":
      return configureRepoSettings(args.repo);

    case "apply_branch_protection":
      return applyBranchProtection(args);

    case "create_github_labels":
      return createGitHubLabels(args);

    case "get_repo_info":
      return getRepoInfo(args.repo);

    case "delete_repo":
      return deleteRepo(args);

    default:
      return {
        content: [{ type: "text", text: JSON.stringify({ error: `Unknown tool: ${name}` }, null, 2) }],
        isError: true
      };
  }
});

// Start the server
async function main() {
  if (!process.env.GITHUB_TOKEN) {
    console.error("ERROR: GITHUB_TOKEN environment variable not set");
    console.error("Please set your GitHub personal access token:");
    console.error("  export GITHUB_TOKEN=your_token_here");
    process.exit(1);
  }

  const transport = server.connect({
    readable: process.stdin,
    writable: process.stdout,
  });

  console.error("Tuulbelt GitHub MCP Server running...");

  await transport;
}

main().catch((error) => {
  console.error("Fatal error:", error);
  process.exit(1);
});
