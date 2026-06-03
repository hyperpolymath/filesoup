#!/bin/bash
# Development helper script for FSLint

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

show_help() {
    echo "FSLint Development Helper"
    echo ""
    echo "Usage: ./scripts/dev.sh <command>"
    echo ""
    echo "Commands:"
    echo "  build       - Build all crates"
    echo "  test        - Run all tests"
    echo "  bench       - Run benchmarks"
    echo "  lint        - Run clippy and format check"
    echo "  fix         - Auto-fix formatting and clippy issues"
    echo "  clean       - Clean build artifacts"
    echo "  docs        - Generate documentation"
    echo "  run         - Run FSLint CLI"
    echo "  check       - Quick check (faster than build)"
    echo "  ci          - Run all CI checks"
    echo ""
}

case "$1" in
    build)
        echo -e "${GREEN}Building all crates...${NC}"
        cargo build --workspace
        ;;

    test)
        echo -e "${GREEN}Running tests...${NC}"
        cargo test --workspace
        ;;

    bench)
        echo -e "${GREEN}Running benchmarks...${NC}"
        cargo bench
        ;;

    lint)
        echo -e "${GREEN}Running linters...${NC}"
        cargo fmt --all -- --check
        cargo clippy --workspace -- -D warnings
        ;;

    fix)
        echo -e "${GREEN}Auto-fixing issues...${NC}"
        cargo fmt --all
        cargo clippy --workspace --fix --allow-dirty
        ;;

    clean)
        echo -e "${GREEN}Cleaning build artifacts...${NC}"
        cargo clean
        ;;

    docs)
        echo -e "${GREEN}Generating documentation...${NC}"
        cargo doc --workspace --no-deps --open
        ;;

    run)
        echo -e "${GREEN}Running FSLint...${NC}"
        shift
        cargo run --release -- "$@"
        ;;

    check)
        echo -e "${GREEN}Quick check...${NC}"
        cargo check --workspace
        ;;

    ci)
        echo -e "${GREEN}Running CI checks...${NC}"
        cargo fmt --all -- --check
        cargo clippy --workspace -- -D warnings
        cargo test --workspace
        cargo build --release
        echo -e "${GREEN}All CI checks passed!${NC}"
        ;;

    *)
        show_help
        exit 1
        ;;
esac
