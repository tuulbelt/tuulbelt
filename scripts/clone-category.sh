#!/bin/bash
# Clone all projects in a category from the Tuulbelt ecosystem
#
# Usage:
#   ./scripts/clone-category.sh <category> [--shallow]
#
# Examples:
#   ./scripts/clone-category.sh tools
#   ./scripts/clone-category.sh libraries --shallow

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Arguments
CATEGORY="$1"
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

# Valid categories
VALID_CATEGORIES="tools libraries frameworks protocols systems meta research"

# Validate arguments
if [ -z "$CATEGORY" ]; then
    echo -e "${RED}Error: Category required${NC}"
    echo ""
    echo "Usage: ./scripts/clone-category.sh <category> [--shallow]"
    echo ""
    echo "Available categories:"
    echo "  tools       - CLI utilities"
    echo "  libraries   - Programmatic APIs"
    echo "  frameworks  - Application structures"
    echo "  protocols   - Wire formats & specs"
    echo "  systems     - Complex infrastructure"
    echo "  meta        - Tools for building tools"
    echo "  research    - Experimental projects"
    exit 1
fi

# Validate category
if ! echo "$VALID_CATEGORIES" | grep -qw "$CATEGORY"; then
    echo -e "${RED}Error: Invalid category '$CATEGORY'${NC}"
    echo ""
    echo "Valid categories: $VALID_CATEGORIES"
    exit 1
fi

# Count projects in category
PROJECT_COUNT=$(grep -c "path = $CATEGORY/" .gitmodules 2>/dev/null || echo "0")

if [ "$PROJECT_COUNT" -eq 0 ]; then
    echo -e "${YELLOW}No projects found in category '$CATEGORY'${NC}"
    echo ""
    echo "This category may be empty or planned for future development."
    echo "Check README.md for planned projects."
    exit 0
fi

echo -e "${GREEN}Cloning category: $CATEGORY${NC}"
echo "  Projects: $PROJECT_COUNT"
if [ -n "$SHALLOW" ]; then
    echo "  Mode: shallow (--depth 1)"
fi
echo ""

# Get list of projects in category
PROJECTS=$(grep "path = $CATEGORY/" .gitmodules | sed 's/.*path = //')

# Clone each project
CLONED=0
FAILED=0

for PROJECT in $PROJECTS; do
    PROJECT_NAME=$(basename "$PROJECT")
    echo -e "${BLUE}[$((CLONED + FAILED + 1))/$PROJECT_COUNT]${NC} Cloning $PROJECT_NAME..."

    if git submodule update --init $SHALLOW "$PROJECT" 2>/dev/null; then
        echo -e "  ${GREEN}✓${NC} $PROJECT_NAME"
        CLONED=$((CLONED + 1))
    else
        echo -e "  ${RED}✗${NC} $PROJECT_NAME (failed)"
        FAILED=$((FAILED + 1))
    fi
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}✓ Cloned: $CLONED${NC}"
if [ "$FAILED" -gt 0 ]; then
    echo -e "${RED}✗ Failed: $FAILED${NC}"
fi
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if [ "$CLONED" -gt 0 ]; then
    echo "Projects are available at: $CATEGORY/"
    echo ""
    echo "List projects:"
    echo "  ls $CATEGORY/"
fi
