#!/bin/bash
# Install pre-commit hooks in meta repo and all submodules

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(git rev-parse --show-toplevel)"

META_HOOK_TEMPLATE="$SCRIPT_DIR/templates/meta-pre-commit-hook.sh"
SUBMODULE_HOOK_TEMPLATE="$SCRIPT_DIR/templates/submodule-pre-commit-hook.sh"

# Install meta repo hook
echo "Installing pre-commit hook in meta repo..."
META_HOOK_PATH="$REPO_ROOT/.git/hooks/pre-commit"

if [ ! -d "$REPO_ROOT/.git/hooks" ]; then
  mkdir -p "$REPO_ROOT/.git/hooks"
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
