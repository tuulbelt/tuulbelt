#!/bin/bash
# Create PRs for web session (branch-based workflow)

set -e

MODE="${1:-all}"  # all, --meta, --submodules

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(git rev-parse --show-toplevel)"

# Load GitHub credentials
source "$REPO_ROOT/scripts/lib/load-credentials.sh"

CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

source "$SCRIPT_DIR/tracking-lib.sh"

echo "Creating pull requests for web session..."
echo ""

# Check we're on a feature branch
if [ "$CURRENT_BRANCH" = "main" ]; then
  echo "❌ ERROR: Cannot create PR from main branch"
  echo "Please create a feature branch with /work-init first"
  exit 1
fi

# Function: Create PR for a repository
create_pr() {
  local repo_path="$1"
  local branch="$2"
  local repo_name="$3"

  cd "$repo_path"

  # Check if there are commits ahead of main
  COMMITS_AHEAD=$(git rev-list --count HEAD ^origin/main 2>/dev/null || echo "0")

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
    # Generate PR title from first commit
    PR_TITLE=$(git log --format=%s -1)

    # Generate PR body
    PR_BODY="## Changes

$(git log origin/main..HEAD --format="- %s")

## Related

Part of web session workflow

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
  create_pr "$REPO_ROOT" "$CURRENT_BRANCH" "meta"
fi

# Submodule PRs
if [ "$MODE" = "all" ] || [ "$MODE" = "--submodules" ]; then
  echo "Submodules:"

  # Process submodules from repo root
  cd "$REPO_ROOT"

  while read -r submodule; do
    echo "  $submodule"

    # Check if submodule has the same branch
    cd "$REPO_ROOT/$submodule"
    SUBMODULE_BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "main")

    if [ "$SUBMODULE_BRANCH" = "main" ]; then
      echo "    → On main branch, skipping"
      echo ""
      cd "$REPO_ROOT"  # Return to repo root for next iteration
      continue
    fi

    # Extract repo name from submodule path
    REPO_NAME=$(basename "$submodule")

    create_pr "$REPO_ROOT/$submodule" "$SUBMODULE_BRANCH" "$REPO_NAME"

    # Return to repo root for next iteration
    cd "$REPO_ROOT"
  done < <(git submodule foreach --quiet 'echo $path')
fi

echo "✓ Pull request workflow completed!"
