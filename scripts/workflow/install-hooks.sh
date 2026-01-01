#!/bin/bash
# Install pre-commit hooks in meta repo and all submodules

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Get main repo root (works from worktrees too)
COMMON_GIT_DIR="$(git rev-parse --git-common-dir)"
# Convert to absolute path if relative
if [[ "$COMMON_GIT_DIR" != /* ]]; then
  COMMON_GIT_DIR="$(pwd)/$COMMON_GIT_DIR"
fi
REPO_ROOT="$(dirname "$COMMON_GIT_DIR")"

META_HOOK_TEMPLATE="$SCRIPT_DIR/templates/meta-pre-commit-hook.sh"
SUBMODULE_HOOK_TEMPLATE="$SCRIPT_DIR/templates/submodule-pre-commit-hook.sh"

# Install meta repo hook
echo "Installing pre-commit hook in meta repo..."
META_HOOK_PATH="$COMMON_GIT_DIR/hooks/pre-commit"

if [ ! -d "$COMMON_GIT_DIR/hooks" ]; then
  mkdir -p "$COMMON_GIT_DIR/hooks"
fi

cp "$META_HOOK_TEMPLATE" "$META_HOOK_PATH"
chmod +x "$META_HOOK_PATH"
echo "✓ Installed hook in meta repo"
echo ""

# Install submodule hooks
SUBMODULES=$(git submodule status | awk '{print $2}')

if [ -z "$SUBMODULES" ]; then
  echo "No submodules found"
  exit 0
fi

echo "Installing pre-commit hooks in submodules..."
echo ""

for submodule in $SUBMODULES; do
  # Handle git submodule gitdir pointer
  if [ -f "$submodule/.git" ]; then
    # Read gitdir path from pointer file
    GITDIR=$(grep "gitdir:" "$submodule/.git" | cut -d' ' -f2)
    GIT_DIR="$submodule/$GITDIR"
  else
    GIT_DIR="$submodule/.git"
  fi

  HOOK_PATH="$GIT_DIR/hooks/pre-commit"

  if [ ! -d "$GIT_DIR/hooks" ]; then
    mkdir -p "$GIT_DIR/hooks"
  fi

  cp "$SUBMODULE_HOOK_TEMPLATE" "$HOOK_PATH"
  chmod +x "$HOOK_PATH"

  echo "✓ Installed hook in $submodule"
done

echo ""
echo "✓ All hooks installed (meta + submodules)"
