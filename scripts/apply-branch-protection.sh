#!/bin/bash
# Apply comprehensive branch protection rules to Tuulbelt repositories
# Usage: ./scripts/apply-branch-protection.sh [repo-name]
#   If no repo-name provided, applies to all repos

set -e

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || echo "$HOME/tuulbelt")"
source "$REPO_ROOT/scripts/lib/load-credentials.sh"

# Repository owner
ORG="${GITHUB_ORG:-tuulbelt}"

# Branch protection rules
PROTECTED_BRANCH="main"

# Function to apply branch protection to a single repo
apply_protection() {
  local repo=$1

  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "Applying branch protection to: $ORG/$repo"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  # Check if repo exists
  if ! gh repo view "$ORG/$repo" &>/dev/null; then
    echo "❌ Repository $ORG/$repo not found or not accessible"
    return 1
  fi

  # Check if main branch exists
  if ! gh api "repos/$ORG/$repo/branches/$PROTECTED_BRANCH" &>/dev/null; then
    echo "⚠️  Branch '$PROTECTED_BRANCH' does not exist in $repo"
    echo "   Skipping protection (apply after first push)"
    return 0
  fi

  echo "📋 Applying protection rules..."

  # Apply branch protection using GitHub API v3
  # See: https://docs.github.com/en/rest/branches/branch-protection
  gh api \
    --method PUT \
    -H "Accept: application/vnd.github+json" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    "/repos/$ORG/$repo/branches/$PROTECTED_BRANCH/protection" \
    -f required_status_checks='{"strict":true,"contexts":["test"]}' \
    -f enforce_admins=false \
    -f required_pull_request_reviews='{"dismissal_restrictions":{},"dismiss_stale_reviews":true,"require_code_owner_reviews":false,"required_approving_review_count":1,"require_last_push_approval":false,"bypass_pull_request_allowances":{}}' \
    -f restrictions=null \
    -F required_linear_history=true \
    -F allow_force_pushes=false \
    -F allow_deletions=false \
    -F block_creations=false \
    -F required_conversation_resolution=true \
    -F lock_branch=false \
    -F allow_fork_syncing=true \
    > /dev/null

  if [ $? -eq 0 ]; then
    echo "✅ Branch protection applied successfully"
    echo ""
    echo "Protection rules:"
    echo "  ✓ Require pull request before merging"
    echo "  ✓ Require 1 approval"
    echo "  ✓ Dismiss stale reviews when new commits pushed"
    echo "  ✓ Require status checks to pass (test workflow)"
    echo "  ✓ Require branches to be up to date before merging"
    echo "  ✓ Require linear history (no merge commits)"
    echo "  ✓ Require conversation resolution before merging"
    echo "  ✓ Block force pushes"
    echo "  ✓ Block deletions"
    echo ""
  else
    echo "❌ Failed to apply branch protection"
    return 1
  fi
}

# Main execution
if [ -n "$1" ]; then
  # Apply to single repo
  apply_protection "$1"
else
  # Apply to all Tuulbelt repos
  echo "🔍 Discovering all tuulbelt repositories..."

  # Get list of all repos in org
  repos=$(gh repo list "$ORG" --limit 100 --json name --jq '.[].name')

  if [ -z "$repos" ]; then
    echo "❌ No repositories found in $ORG"
    exit 1
  fi

  echo "Found $(echo "$repos" | wc -l) repositories"
  echo ""

  # Apply to each repo
  success_count=0
  skip_count=0
  fail_count=0

  for repo in $repos; do
    if apply_protection "$repo"; then
      if grep -q "Skipping protection" <<< "$(apply_protection "$repo" 2>&1)"; then
        ((skip_count++))
      else
        ((success_count++))
      fi
    else
      ((fail_count++))
    fi
    echo ""
  done

  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "Summary:"
  echo "  ✅ Successfully protected: $success_count repos"
  echo "  ⚠️  Skipped (no main branch): $skip_count repos"
  echo "  ❌ Failed: $fail_count repos"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
fi
