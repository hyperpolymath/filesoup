#!/bin/bash
# FSLint installation script for Unix-like systems

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}FSLint Installer${NC}"
echo "================="
echo ""

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     MACHINE=linux;;
    Darwin*)    MACHINE=mac;;
    *)          MACHINE="UNKNOWN:${OS}"
esac

if [ "$MACHINE" == "UNKNOWN:${OS}" ]; then
    echo -e "${RED}Unsupported operating system: ${OS}${NC}"
    exit 1
fi

echo "Detected OS: ${MACHINE}"

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}Rust not found. Installing Rust...${NC}"
# WARNING: Pipe-to-shell is unsafe — download and verify first
# WARNING: Pipe-to-shell is unsafe — download and verify first
# WARNING: Pipe-to-shell is unsafe — download and verify first
# WARNING: Pipe-to-shell is unsafe — download and verify first
# WARNING: Pipe-to-shell is unsafe — download and verify first
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust found: $(cargo --version)"
fi

# Install method
echo ""
echo "Choose installation method:"
echo "1) Install from crates.io (recommended)"
echo "2) Build from source"
read -p "Enter choice [1-2]: " choice

case $choice in
    1)
        echo -e "${GREEN}Installing from crates.io...${NC}"
        cargo install fslint
        ;;
    2)
        echo -e "${GREEN}Building from source...${NC}"

        # Check if we're in the repo
        if [ ! -f "Cargo.toml" ]; then
            echo "Cloning repository..."
            git clone https://github.com/Hyperpolymath/file-soup.git
            cd file-soup
        fi

        echo "Building release binary..."
        cargo build --release

        echo "Installing binary..."
        sudo cp target/release/fslint /usr/local/bin/

        echo "Cleaning up..."
        cargo clean
        ;;
    *)
        echo -e "${RED}Invalid choice${NC}"
        exit 1
        ;;
esac

# Verify installation
if command -v fslint &> /dev/null; then
    echo ""
    echo -e "${GREEN}✓ FSLint installed successfully!${NC}"
    echo ""
    echo "Version: $(fslint --version 2>/dev/null || echo 'unknown')"
    echo ""
    echo "Try it out:"
    echo "  fslint scan ."
    echo "  fslint plugins"
    echo ""
    echo "For more information, visit: https://github.com/Hyperpolymath/file-soup"
else
    echo -e "${RED}✗ Installation failed${NC}"
    exit 1
fi
