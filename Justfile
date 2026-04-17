# FSLint justfile - Command runner for development tasks
# Install just: https://github.com/casey/just
# Usage: just <recipe>

# Default recipe (list all recipes)
default:
    @just --list

# Build all crates (debug)
build:
    cargo build --workspace

# Build release binary
build-release:
    cargo build --release --workspace

# Run all tests
test:
    cargo test --workspace

# Run tests with output
test-verbose:
    cargo test --workspace -- --nocapture

# Run specific crate tests
test-crate crate:
    cargo test -p {{crate}}

# Run benchmarks
bench:
    cargo bench

# Run specific benchmark
bench-one name:
    cargo bench --bench {{name}}

# Format all code
fmt:
    cargo fmt --all

# Check formatting (CI)
fmt-check:
    cargo fmt --all -- --check

# Run clippy linter
lint:
    cargo clippy --workspace -- -D warnings

# Auto-fix clippy issues
lint-fix:
    cargo clippy --workspace --fix --allow-dirty --allow-staged

# Quick check (faster than build)
check:
    cargo check --workspace

# Clean build artifacts
clean:
    cargo clean

# Generate and open documentation
docs:
    cargo doc --workspace --no-deps --open

# Generate documentation (no open)
docs-build:
    cargo doc --workspace --no-deps

# Run FSLint CLI (scan current directory)
run *ARGS:
    cargo run --release -- {{ARGS}}

# Scan current directory with table output
scan:
    cargo run --release -- scan .

# Scan with JSON output
scan-json:
    cargo run --release -- scan . --format json

# List all plugins
plugins:
    cargo run --release -- plugins

# Show configuration
config:
    cargo run --release -- config

# Enable a plugin
enable plugin:
    cargo run --release -- enable {{plugin}}

# Disable a plugin
disable plugin:
    cargo run --release -- disable {{plugin}}

# Run query
query filter:
    cargo run --release -- query "{{filter}}"

# Install FSLint to system
install:
    cargo install --path crates/fslint-cli

# Uninstall FSLint from system
uninstall:
    cargo uninstall fslint

# Run all CI checks
ci: fmt-check lint test build-release
    @echo "✓ All CI checks passed!"

# Validate RSR compliance
validate: ci
    @echo "Checking RSR compliance..."
    @test -f SECURITY.md || (echo "✗ Missing SECURITY.md" && exit 1)
    @test -f CODE_OF_CONDUCT.md || (echo "✗ Missing CODE_OF_CONDUCT.md" && exit 1)
    @test -f MAINTAINERS.md || (echo "✗ Missing MAINTAINERS.md" && exit 1)
    @test -f .well-known/security.txt || (echo "✗ Missing .well-known/security.txt" && exit 1)
    @test -f .well-known/ai.txt || (echo "✗ Missing .well-known/ai.txt" && exit 1)
    @test -f .well-known/humans.txt || (echo "✗ Missing .well-known/humans.txt" && exit 1)
    @test -f LICENSE-MIT || (echo "✗ Missing LICENSE-MIT" && exit 1)
    @test -f LICENSE-APACHE || (echo "✗ Missing LICENSE-APACHE" && exit 1)
    @test -f CHANGELOG.md || (echo "✗ Missing CHANGELOG.md" && exit 1)
    @test -f CONTRIBUTING.md || (echo "✗ Missing CONTRIBUTING.md" && exit 1)
    @test -f README.md || (echo "✗ Missing README.md" && exit 1)
    @echo "✓ RSR compliance validated!"

# Check for unsafe code blocks
check-unsafe:
    @echo "Checking for unsafe blocks..."
    @! rg "unsafe " --type rust || (echo "⚠ Found unsafe blocks" && exit 1)
    @echo "✓ No unsafe blocks found"

# Count lines of code
loc:
    @echo "Lines of code by language:"
    @tokei

# Show project statistics
stats:
    @echo "=== FSLint Project Statistics ==="
    @echo "Crates: $(ls -1 crates | wc -l)"
    @echo "Plugins: $(ls -1 plugins | wc -l)"
    @echo "Documentation files: $(ls -1 *.md docs/*.md 2>/dev/null | wc -l)"
    @echo ""
    @echo "Tests:"
    @cargo test --workspace --no-run 2>&1 | grep -i "test" | head -5
    @echo ""
    @echo "Dependencies:"
    @cargo tree --depth 1

# Security audit
audit:
    cargo audit

# Update dependencies
update:
    cargo update

# Check for outdated dependencies
outdated:
    cargo outdated

# Release preparation (check everything)
pre-release: clean ci validate audit
    @echo "✓ Ready for release!"

# Build Docker image
docker-build:
    docker build -t fslint:latest .

# Run Docker container
docker-run:
    docker run -v $(pwd):/scan:ro fslint:latest scan /scan

# Docker compose up
docker-up:
    docker-compose up

# Git: Add all and commit
commit message:
    git add -A
    git commit -m "{{message}}"

# Git: Add, commit, and push
push message:
    git add -A
    git commit -m "{{message}}"
    git push

# Create new plugin template
new-plugin name:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Creating plugin: {{name}}"
    mkdir -p plugins/{{name}}/src
    cat > plugins/{{name}}/Cargo.toml <<EOF
    [package]
    name = "fslint-plugin-{{name}}"
    version.workspace = true
    edition.workspace = true
    authors.workspace = true
    license.workspace = true
    repository.workspace = true
    description = "{{name}} plugin for FSLint"

    [dependencies]
    fslint-plugin-api = { path = "../../crates/fslint-plugin-api" }
    fslint-plugin-sdk = { path = "../../crates/fslint-plugin-sdk" }

    [lib]
    crate-type = ["cdylib", "rlib"]
    EOF
    echo "✓ Plugin template created at plugins/{{name}}/"
    echo "  Next: Edit plugins/{{name}}/src/lib.rs"

# Watch and rebuild on changes (requires cargo-watch)
watch:
    cargo watch -x check -x test

# Run in development mode with auto-reload
dev:
    cargo watch -x 'run -- scan .'

# Profile release binary
profile:
    cargo build --release
    time ./target/release/fslint scan .

# Generate flame graph (requires cargo-flamegraph)
flamegraph:
    cargo flamegraph -- scan .

# Check licenses of dependencies
license-check:
    cargo license --json | jq -r '.[] | "\(.name): \(.license)"' | sort

# Help text
help:
    @echo "FSLint Development Commands"
    @echo ""
    @echo "Build & Test:"
    @echo "  just build          - Build debug"
    @echo "  just build-release  - Build release"
    @echo "  just test           - Run tests"
    @echo "  just bench          - Run benchmarks"
    @echo ""
    @echo "Code Quality:"
    @echo "  just fmt            - Format code"
    @echo "  just lint           - Run clippy"
    @echo "  just ci             - Run all CI checks"
    @echo "  just validate       - Verify RSR compliance"
    @echo ""
    @echo "Run FSLint:"
    @echo "  just scan           - Scan current directory"
    @echo "  just plugins        - List plugins"
    @echo "  just run ARGS       - Run with custom args"
    @echo ""
    @echo "Development:"
    @echo "  just watch          - Auto-rebuild on changes"
    @echo "  just dev            - Run with auto-reload"
    @echo "  just new-plugin NAME - Create plugin template"
    @echo ""
    @echo "Release:"
    @echo "  just pre-release    - Check release readiness"
    @echo "  just install        - Install to system"
    @echo ""
    @echo "See 'just --list' for all recipes"
