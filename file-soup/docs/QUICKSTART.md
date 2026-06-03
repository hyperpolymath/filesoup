# FSLint Quickstart Guide

Get up and running with FSLint in 5 minutes!

## Installation

### From crates.io (Recommended)

```bash
cargo install fslint
```

### From Source

```bash
git clone https://github.com/Hyperpolymath/file-soup.git
cd file-soup
cargo build --release
sudo cp target/release/fslint /usr/local/bin/
```

### Using Installation Script

**Linux/macOS:**
```bash
curl -sSL https://raw.githubusercontent.com/Hyperpolymath/file-soup/main/scripts/install.sh | bash
```

**Windows (PowerShell):**
```powershell
iwr https://raw.githubusercontent.com/Hyperpolymath/file-soup/main/scripts/install.ps1 | iex
```

### Using Docker

```bash
docker pull fslint/fslint:latest
docker run -v $(pwd):/scan fslint/fslint scan /scan
```

## First Steps

### 1. Scan a Directory

```bash
# Scan current directory
fslint scan

# Scan specific directory
fslint scan ~/projects/myproject

# Different output formats
fslint scan --format json
fslint scan --format simple
```

### 2. List Available Plugins

```bash
fslint plugins
```

Output:
```
Plugin                    Status     Description
----------------------------------------------------------------------------------------
git-status                enabled    Shows git repository status and branch info
file-age                  enabled    Highlights recently modified files (< 7 days)
grouping                  enabled    Categorizes files (node_modules, media, etc.)
version-detection         disabled   Detects versioned files
...
```

### 3. Enable/Disable Plugins

```bash
# Enable a plugin
fslint enable secret-scanner

# Disable a plugin
fslint disable grouping

# View current configuration
fslint config
```

### 4. Query Files

```bash
# Find all TypeScript files
fslint query "ext:ts"

# Find recently modified files
fslint query "tag:age"

# Find newest version of a file
fslint query "name:report newest:true"

# Combine filters
fslint query "ext:pdf size_gt:1048576"
```

## Common Use Cases

### Security Audit

```bash
# Enable secret scanner
fslint enable secret-scanner

# Scan for exposed secrets
fslint scan --format table

# Check specific directory
fslint scan ./src --format json > security-audit.json
```

### Find Duplicates

```bash
# Enable duplicate finder
fslint enable duplicate-finder

# Scan for duplicates
fslint scan

# Find large duplicates only
fslint query "tag:duplicate size_gt:10485760"
```

### Git Status Overview

```bash
# Default scan shows git status
fslint scan

# Find modified files
fslint query "git-status:Modified"

# Find new files
fslint query "git-status:New"
```

### Find AI-Generated Images

```bash
# Enable AI detection
fslint enable ai-detection

# Scan for AI-generated images
fslint scan ./images

# Query for AI content
fslint query "tag:ai"
```

### Clean Up Old Files

```bash
# Find recent files (modified in last 7 days)
fslint query "tag:age"

# Find old files
fslint scan --format table
# Look for files without the "Recent" tag
```

## Configuration

### Configuration File Location

FSLint stores configuration at:
- Linux/macOS: `~/.config/fslint/config.toml`
- Windows: `%APPDATA%\fslint\config.toml`

### Example Configurations

Copy example configs:

```bash
# Minimal configuration
cp examples/config-minimal.toml ~/.config/fslint/config.toml

# Full configuration (all plugins)
cp examples/config-full.toml ~/.config/fslint/config.toml

# Security-focused configuration
cp examples/config-security.toml ~/.config/fslint/config.toml

# Development configuration
cp examples/config-development.toml ~/.config/fslint/config.toml
```

### Custom Configuration

Edit `~/.config/fslint/config.toml`:

```toml
enabled_plugins = ["git-status", "file-age", "secret-scanner"]

[scanner]
max_depth = 15
include_hidden = false
respect_gitignore = true

[plugin_config.secret-scanner]
max_file_size = "5242880"
```

## Tips & Tricks

### 1. Use Aliases

Add to your `.bashrc` or `.zshrc`:

```bash
alias fs='fslint scan'
alias fsp='fslint plugins'
alias fsq='fslint query'
```

### 2. Pipe to Other Tools

```bash
# Count files by type
fslint scan --format simple | grep '\.rs$' | wc -l

# Find and open modified files
fslint query "git-status:Modified" --format simple | xargs code
```

### 3. Integrate with Git Hooks

Create `.git/hooks/pre-commit`:

```bash
#!/bin/bash
fslint enable secret-scanner
fslint scan --format simple | grep "secret" && exit 1
```

### 4. CI/CD Integration

Add to your CI pipeline:

```yaml
# GitHub Actions example
- name: Scan for secrets
  run: |
    cargo install fslint
    fslint enable secret-scanner
    fslint scan --format json > scan-results.json

- name: Upload results
  uses: actions/upload-artifact@v3
  with:
    name: fslint-results
    path: scan-results.json
```

### 5. Docker Compose for Projects

Add `docker-compose.yml`:

```yaml
version: '3.8'
services:
  fslint:
    image: fslint/fslint:latest
    volumes:
      - ./:/scan:ro
    command: scan /scan --format table
```

Run with:
```bash
docker-compose run fslint
```

## Next Steps

- Read the [full documentation](../README.md)
- Learn about [plugin development](PLUGIN_DEVELOPMENT.md)
- Check out [contributing guidelines](../CONTRIBUTING.md)
- Report issues on [GitHub](https://github.com/Hyperpolymath/file-soup/issues)

## Getting Help

```bash
# General help
fslint --help

# Command-specific help
fslint scan --help
fslint query --help

# Version information
fslint --version
```

## Troubleshooting

### "Permission denied" errors

```bash
# Run with sudo if needed
sudo fslint scan /system/directory

# Or scan with user permissions
fslint scan ~/myproject
```

### Slow scanning

```bash
# Reduce max depth
fslint scan --max-depth 5

# Disable expensive plugins
fslint disable duplicate-finder
fslint disable ai-detection
```

### Plugin not working

```bash
# Check plugin status
fslint plugins

# Enable plugin
fslint enable plugin-name

# Check configuration
fslint config
```

---

**Happy scanning! 🚀**
