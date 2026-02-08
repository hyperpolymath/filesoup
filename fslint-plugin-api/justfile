# Rust crate justfile

default:
    @just --list

# Build the project
build:
    cargo build

# Run tests
test:
    cargo test

# Check formatting
fmt-check:
    cargo fmt --check

# Format code
fmt:
    cargo fmt

# Run clippy
lint:
    cargo clippy -- -D warnings

# Build documentation
doc:
    cargo doc --no-deps

# Clean build artifacts
clean:
    cargo clean
