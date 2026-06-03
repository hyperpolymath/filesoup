#!/bin/bash
# Release helper script for FSLint

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

if [ -z "$1" ]; then
    echo -e "${RED}Usage: ./scripts/release.sh <version>${NC}"
    echo "Example: ./scripts/release.sh 0.2.0"
    exit 1
fi

VERSION=$1

echo -e "${GREEN}Preparing release v${VERSION}${NC}"

# Check git status
if [[ -n $(git status -s) ]]; then
    echo -e "${RED}Working directory is not clean. Commit or stash changes first.${NC}"
    exit 1
fi

# Run CI checks
echo -e "${YELLOW}Running CI checks...${NC}"
./scripts/dev.sh ci

# Update version in Cargo.toml
echo -e "${YELLOW}Updating version in Cargo.toml...${NC}"
sed -i.bak "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
rm Cargo.toml.bak

# Update CHANGELOG
echo -e "${YELLOW}Update CHANGELOG.md manually, then press Enter to continue...${NC}"
read

# Build release
echo -e "${YELLOW}Building release...${NC}"
cargo build --release

# Create git tag
echo -e "${YELLOW}Creating git tag v${VERSION}...${NC}"
git add Cargo.toml CHANGELOG.md
git commit -m "chore: Release v${VERSION}"
git tag -a "v${VERSION}" -m "Release v${VERSION}"

echo ""
echo -e "${GREEN}Release v${VERSION} prepared!${NC}"
echo ""
echo "Next steps:"
echo "1. Review the changes: git show v${VERSION}"
echo "2. Push to remote: git push && git push --tags"
echo "3. GitHub Actions will build and create the release"
echo ""
