#!/bin/bash
# Clone a project and all its Tuulbelt dependencies
#
# Usage:
#   ./scripts/clone-with-deps.sh <path> [--shallow]
#
# Examples:
#   ./scripts/clone-with-deps.sh tools/snapshot-comparison
#   ./scripts/clone-with-deps.sh tools/test-flakiness-detector --shallow

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Arguments
PROJECT_PATH="$1"
SHALLOW=""

# Parse flags
for arg in "$@"; do
    case $arg in
        --shallow)
            SHALLOW="--depth 1"
            shift
            ;;
    esac
done

# Validate arguments
if [ -z "$PROJECT_PATH" ]; then
    echo -e "${RED}Error: Project path required${NC}"
    echo ""
    echo "Usage: ./scripts/clone-with-deps.sh <path> [--shallow]"
    echo ""
    echo "Examples:"
    echo "  ./scripts/clone-with-deps.sh tools/snapshot-comparison"
    echo "  ./scripts/clone-with-deps.sh tools/test-flakiness-detector --shallow"
    exit 1
fi

# Check if path exists in .gitmodules
if ! grep -q "path = $PROJECT_PATH" .gitmodules 2>/dev/null; then
    echo -e "${RED}Error: '$PROJECT_PATH' is not a registered submodule${NC}"
    exit 1
fi

PROJECT_NAME=$(basename "$PROJECT_PATH")

echo -e "${GREEN}Cloning project with dependencies: $PROJECT_NAME${NC}"
echo "  Path: $PROJECT_PATH"
if [ -n "$SHALLOW" ]; then
    echo "  Mode: shallow (--depth 1)"
fi
echo ""

# First, clone the main project
echo -e "${BLUE}[1/n]${NC} Cloning main project..."
git submodule update --init $SHALLOW "$PROJECT_PATH"

if [ ! -d "$PROJECT_PATH" ]; then
    echo -e "${RED}✗ Failed to clone main project${NC}"
    exit 1
fi
echo -e "  ${GREEN}✓${NC} $PROJECT_NAME"

# Function to find Tuulbelt dependencies
find_deps() {
    local project_dir="$1"
    local deps=""

    # Check package.json for TypeScript projects
    if [ -f "$project_dir/package.json" ]; then
        # Look for git+https://github.com/tuulbelt/ patterns
        deps=$(grep -oE 'git\+https://github\.com/tuulbelt/[^"]+' "$project_dir/package.json" 2>/dev/null | \
               sed 's|git+https://github.com/tuulbelt/||' | \
               sed 's|\.git||' || true)
    fi

    # Check Cargo.toml for Rust projects
    if [ -f "$project_dir/Cargo.toml" ]; then
        # Look for git = "https://github.com/tuulbelt/ patterns
        rust_deps=$(grep -oE 'git = "https://github\.com/tuulbelt/[^"]+' "$project_dir/Cargo.toml" 2>/dev/null | \
                   sed 's|git = "https://github.com/tuulbelt/||' | \
                   sed 's|\.git||' || true)
        deps="$deps $rust_deps"
    fi

    echo "$deps" | tr ' ' '\n' | sort -u | grep -v '^$' || true
}

# Find dependencies
echo ""
echo "Scanning for Tuulbelt dependencies..."
DEPS=$(find_deps "$PROJECT_PATH")

if [ -z "$DEPS" ]; then
    echo -e "${YELLOW}No Tuulbelt dependencies found${NC}"
    echo ""
    echo -e "${GREEN}✓ Project cloned successfully (no dependencies)${NC}"
else
    # Count dependencies
    DEP_COUNT=$(echo "$DEPS" | wc -l | tr -d ' ')
    echo "Found $DEP_COUNT Tuulbelt dependencies"
    echo ""

    # Clone each dependency
    STEP=2
    for DEP in $DEPS; do
        # Try to find the dependency in each category
        DEP_PATH=""
        for CATEGORY in tools libraries frameworks protocols systems meta research; do
            if grep -q "path = $CATEGORY/$DEP" .gitmodules 2>/dev/null; then
                DEP_PATH="$CATEGORY/$DEP"
                break
            fi
        done

        if [ -n "$DEP_PATH" ]; then
            echo -e "${BLUE}[$STEP/n]${NC} Cloning dependency: $DEP..."
            if git submodule update --init $SHALLOW "$DEP_PATH" 2>/dev/null; then
                echo -e "  ${GREEN}✓${NC} $DEP"
            else
                echo -e "  ${YELLOW}⚠${NC} $DEP (already cloned or failed)"
            fi
        else
            echo -e "  ${YELLOW}⚠${NC} $DEP (not found in ecosystem - may be external)"
        fi
        STEP=$((STEP + 1))
    done
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}✓ Clone complete${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Main project: $PROJECT_PATH"
if [ -n "$DEPS" ]; then
    echo "Dependencies cloned:"
    for DEP in $DEPS; do
        for CATEGORY in tools libraries frameworks protocols systems meta research; do
            if [ -d "$CATEGORY/$DEP" ]; then
                echo "  - $CATEGORY/$DEP"
                break
            fi
        done
    done
fi
echo ""
echo "Next steps:"
echo "  cd $PROJECT_PATH"
if [ -f "$PROJECT_PATH/package.json" ]; then
    echo "  npm install"
    echo "  npm test"
elif [ -f "$PROJECT_PATH/Cargo.toml" ]; then
    echo "  cargo build"
    echo "  cargo test"
fi
