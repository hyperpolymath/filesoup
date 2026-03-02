#!/bin/bash
# FSLint uninstall script

set -e

echo "FSLint Uninstaller"
echo "=================="
echo ""

# Remove binary
if command -v fslint &> /dev/null; then
    BINARY_PATH=$(which fslint)
    echo "Removing binary: $BINARY_PATH"
    sudo rm -f "$BINARY_PATH"
fi

# Remove config
CONFIG_DIR="$HOME/.config/fslint"
if [ -d "$CONFIG_DIR" ]; then
    read -p "Remove configuration directory ($CONFIG_DIR)? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rm -rf "$CONFIG_DIR"
        echo "Configuration removed"
    fi
fi

# Remove cargo installation
if cargo install --list | grep -q "^fslint"; then
    echo "Removing cargo installation..."
    cargo uninstall fslint
fi

echo ""
echo "✓ FSLint uninstalled"
