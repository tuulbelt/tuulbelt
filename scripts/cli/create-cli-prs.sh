#!/bin/bash
# Create PRs for CLI workspace (worktree or branch)

set -e

MODE="${1:-all}"  # all, --meta, --submodules

# Get main repo root (works from worktrees too)
COMMON_GIT_DIR="$(git rev-parse --git-common-dir)"
# Convert to absolute path if relative
if [[ "$COMMON_GIT_DIR" != /* ]]; then
  COMMON_GIT_DIR="$(pwd)/$COMMON_GIT_DIR"
fi
REPO_ROOT="$(dirname "$COMMON_GIT_DIR")"
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

echo "Creating pull requests..."
echo ""

# Detect if we're in a worktree
if git worktree list | grep -q "$(pwd)"; then
  IN_WORKTREE=true
else
  IN_WORKTREE=false
fi

# Function: Create PR for a repository
create_pr() {
  local repo_path="$1"
  local branch="$2"
  local repo_name="$3"

  cd "$repo_path"

  # Check if there are commits ahead of main (use explicit branch, not HEAD)
  COMMITS_AHEAD=$(git rev-list --count "$branch" ^origin/main 2>/dev/null || echo "0")

  if [ "$COMMITS_AHEAD" = "0" ]; then
    echo "  → No commits, skipping"
    return 0
  fi

  # Push branch
  echo "  ✓ Pushing branch: $branch"
  git push -u origin "$branch" 2>&1 | grep -v "^remote:" || true

  # Create PR if it doesn't exist
  if gh pr view "$branch" &>/dev/null; then
    PR_URL=$(gh pr view "$branch" --json url -q .url)
    echo "  ✓ PR already exists: $PR_URL"
  else
    # Generate PR title from first commit (use explicit branch)
    PR_TITLE=$(git log "$branch" --format=%s -1)

    # Generate PR body (use explicit branch)
    PR_BODY="## Changes

$(git log origin/main.."$branch" --format="- %s")

## Related

Part of feature development workflow

See [Unified Workflow Plan](../docs/UNIFIED_WORKFLOW_PLAN.md)"

    # Create PR
    PR_URL=$(gh pr create --title "$PR_TITLE" --body "$PR_BODY" --base main --head "$branch" 2>&1 | grep "https://" | head -1)
    echo "  ✓ Created PR: $PR_URL"
  fi

  echo ""
}

# Meta repo PR
if [ "$MODE" = "all" ] || [ "$MODE" = "--meta" ]; then
  echo "Meta repo:"
  # If in worktree, use current directory; otherwise use REPO_ROOT
  if [ "$IN_WORKTREE" = true ]; then
    create_pr "$(pwd)" "$CURRENT_BRANCH" "meta"
  else
    create_pr "$REPO_ROOT" "$CURRENT_BRANCH" "meta"
  fi
fi

# Submodule PRs
if [ "$MODE" = "all" ] || [ "$MODE" = "--submodules" ]; then
  echo "Submodules:"

  # Use current directory (worktree location) for submodule paths
  WORK_DIR="$(pwd)"

  # Get list of submodules (run from work directory)
  # Use process substitution instead of pipe to avoid subshell issues
  cd "$WORK_DIR"
  while read -r submodule; do
    echo "  $submodule"

    # Check if submodule has the same branch
    cd "$WORK_DIR/$submodule"
    SUBMODULE_BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "main")

    if [ "$SUBMODULE_BRANCH" = "main" ]; then
      echo "    → On main branch, skipping"
      echo ""
      cd "$WORK_DIR"  # Return to work directory for next iteration
      continue
    fi

    # Extract repo name from submodule path
    REPO_NAME=$(basename "$submodule")

    create_pr "$WORK_DIR/$submodule" "$SUBMODULE_BRANCH" "$REPO_NAME"

    # Return to work directory for next iteration
    cd "$WORK_DIR"
  done < <(git submodule foreach --quiet 'echo $path')
fi

echo "✓ Pull request workflow completed!"
