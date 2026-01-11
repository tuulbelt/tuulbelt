#!/bin/bash
# Clone a single project from the Tuulbelt ecosystem
#
# Usage:
#   ./scripts/clone-project.sh <path> [--shallow]
#
# Examples:
#   ./scripts/clone-project.sh tools/test-flakiness-detector
#   ./scripts/clone-project.sh libraries/result-type --shallow

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
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
    echo "Usage: ./scripts/clone-project.sh <path> [--shallow]"
    echo ""
    echo "Examples:"
    echo "  ./scripts/clone-project.sh tools/test-flakiness-detector"
    echo "  ./scripts/clone-project.sh libraries/result-type --shallow"
    echo ""
    echo "Available categories:"
    echo "  tools/       - CLI utilities"
    echo "  libraries/   - Programmatic APIs"
    echo "  frameworks/  - Application structures"
    echo "  protocols/   - Wire formats & specs"
    echo "  systems/     - Complex infrastructure"
    echo "  meta/        - Tools for building tools"
    echo "  research/    - Experimental projects"
    exit 1
fi

# Check if path exists in .gitmodules
if ! grep -q "path = $PROJECT_PATH" .gitmodules 2>/dev/null; then
    echo -e "${RED}Error: '$PROJECT_PATH' is not a registered submodule${NC}"
    echo ""
    echo "Check .gitmodules for available projects, or verify the path."
    exit 1
fi

# Extract category
CATEGORY=$(echo "$PROJECT_PATH" | cut -d'/' -f1)
PROJECT_NAME=$(basename "$PROJECT_PATH")

echo -e "${GREEN}Cloning project: $PROJECT_NAME${NC}"
echo "  Category: $CATEGORY"
echo "  Path: $PROJECT_PATH"
if [ -n "$SHALLOW" ]; then
    echo "  Mode: shallow (--depth 1)"
fi
echo ""

# Initialize and update the specific submodule
echo "Initializing submodule..."
git submodule update --init $SHALLOW "$PROJECT_PATH"

# Verify success
if [ -d "$PROJECT_PATH" ] && [ -f "$PROJECT_PATH/.git" -o -d "$PROJECT_PATH/.git" ]; then
    echo ""
    echo -e "${GREEN}✓ Successfully cloned $PROJECT_NAME${NC}"
    echo ""
    echo "Next steps:"
    echo "  cd $PROJECT_PATH"

    # Show language-specific next steps
    if [ -f "$PROJECT_PATH/package.json" ]; then
        echo "  npm install"
        echo "  npm test"
    elif [ -f "$PROJECT_PATH/Cargo.toml" ]; then
        echo "  cargo build"
        echo "  cargo test"
    fi
else
    echo -e "${RED}✗ Failed to clone $PROJECT_NAME${NC}"
    exit 1
fi
