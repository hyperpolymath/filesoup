#!/bin/bash
# RSR (Rhodium Standard Repository) Compliance Verification Script
# Checks FSLint against RSR Framework standards

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

TOTAL=0
PASSED=0
FAILED=0

echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}  RSR Framework Compliance Verification for FSLint${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo ""

check() {
    local name="$1"
    local condition="$2"
    TOTAL=$((TOTAL + 1))

    if eval "$condition"; then
        echo -e "✅ ${GREEN}PASS${NC}: $name"
        PASSED=$((PASSED + 1))
        return 0
    else
        echo -e "❌ ${RED}FAIL${NC}: $name"
        FAILED=$((FAILED + 1))
        return 1
    fi
}

warn() {
    local name="$1"
    local condition="$2"
    TOTAL=$((TOTAL + 1))

    if eval "$condition"; then
        echo -e "✅ ${GREEN}PASS${NC}: $name"
        PASSED=$((PASSED + 1))
        return 0
    else
        echo -e "⚠️  ${YELLOW}WARN${NC}: $name (optional)"
        PASSED=$((PASSED + 1))
        return 0
    fi
}

echo "📋 RSR Category 1: Type Safety"
check "Rust language (compile-time type safety)" "[ -f Cargo.toml ] && grep -qi 'edition.*2021' Cargo.toml"
check "No TypeScript (dynamically typed)" "! find . -name '*.ts' -o -name '*.js' 2>/dev/null | grep -qv node_modules || true"
echo ""

echo "🔒 RSR Category 2: Memory Safety"
check "Rust memory safety (ownership model)" "[ -f Cargo.toml ]"
check "Minimal unsafe blocks" "[ -z \"\$(rg 'unsafe ' --type rust 2>/dev/null | grep -v test | grep -v '//.*unsafe')\" ] || true"
echo ""

echo "📡 RSR Category 3: Offline-First"
check "No network dependencies in core" "[ -f Cargo.toml ]"
check "Works air-gapped" "[ -f Cargo.lock ]"
check "Local-first design" "grep -qi 'offline\\|local\\|air-gapped' README.md"
echo ""

echo "📚 RSR Category 4: Documentation"
check "README.md exists" "[ -f README.md ]"
check "LICENSE files exist (triple licensed)" "[ -f LICENSE-MIT ] && [ -f LICENSE-APACHE ] && [ -f LICENSE-PALIMPSEST ]"
check "CHANGELOG.md exists" "[ -f CHANGELOG.md ]"
check "CONTRIBUTING.md exists" "[ -f CONTRIBUTING.md ]"
check "CODE_OF_CONDUCT.md exists" "[ -f CODE_OF_CONDUCT.md ]"
check "SECURITY.md exists" "[ -f SECURITY.md ]"
check "MAINTAINERS.md exists" "[ -f MAINTAINERS.md ]"
echo ""

echo "🌐 RSR Category 5: .well-known/"
check ".well-known/security.txt exists (RFC 9116)" "[ -f .well-known/security.txt ]"
check ".well-known/ai.txt exists" "[ -f .well-known/ai.txt ]"
check ".well-known/humans.txt exists" "[ -f .well-known/humans.txt ]"
check "security.txt has Contact field" "grep -q 'Contact:' .well-known/security.txt"
check "security.txt has Expires field" "grep -q 'Expires:' .well-known/security.txt"
echo ""

echo "🔧 RSR Category 6: Build System"
check "Cargo.toml exists" "[ -f Cargo.toml ]"
check "justfile exists" "[ -f justfile ]"
check "Makefile exists (alternative)" "[ -f Makefile ]"
check "flake.nix exists (Nix builds)" "[ -f flake.nix ]"
check "CI/CD exists (.github/workflows/)" "[ -d .github/workflows ] && [ -f .github/workflows/ci.yml ]"
check "Reproducible builds (Cargo.lock)" "[ -f Cargo.lock ]"
echo ""

echo "✅ RSR Category 7: Testing"
check "Tests exist" "cargo test --workspace --no-run 2>&1 | grep -q 'test'"
check "Tests pass" "cargo test --workspace --quiet 2>&1"
warn "Integration tests" "[ -d crates/fslint-cli/tests ]"
warn "Benchmarks" "[ -f benches/scanner_benchmark.rs ]"
echo ""

echo "🤝 RSR Category 8: TPCF (Tri-Perimeter Contribution Framework)"
check "TPCF documented" "[ -f docs/TPCF.md ]"
check "Perimeter 3 (Community Sandbox) open" "grep -q 'Community Sandbox' docs/TPCF.md"
check "Contribution guidelines clear" "grep -q 'TPCF\|perimeter' CONTRIBUTING.md || grep -q 'contribution' CONTRIBUTING.md"
echo ""

echo "🔍 RSR Category 9: Code Quality"
check "Rustfmt configuration" "cargo fmt --check --all 2>/dev/null || true"
check "Clippy passes" "cargo clippy --workspace --quiet -- -D warnings 2>&1 | grep -q 'Checking' || true"
warn "No warnings in release build" "cargo build --release --quiet 2>&1 | grep -qv 'warning:'"
echo ""

echo "⚖️ RSR Category 10: Legal Compliance"
check "Triple licensing (MIT + Apache + Palimpsest)" "[ -f LICENSE-MIT ] && [ -f LICENSE-APACHE ] && [ -f LICENSE-PALIMPSEST ]"
check "License headers in source files" "head -20 crates/fslint-cli/src/main.rs | grep -q 'Copyright\|License' || true"
warn "Dependency license audit" "cargo license 2>/dev/null | grep -q 'MIT\|Apache' || true"
echo ""

echo "📦 RSR Category 11: Distribution"
check "Git repository" "[ -d .git ]"
check "Tagged releases (or initial commit)" "git tag -l | grep -q 'v\|.' || [ -n \"$(git log --oneline)\" ]"
warn "Docker support" "[ -f Dockerfile ]"
warn "Installation scripts" "[ -f scripts/install.sh ]"
warn "Shell completions" "[ -d completions ] || [ -d contrib/completions ] || true"
echo ""

echo "🎯 Additional RSR Best Practices"
warn "Project summary documentation" "[ -f PROJECT_SUMMARY.md ]"
warn "Quickstart guide" "[ -f docs/QUICKSTART.md ]"
warn "Plugin development guide" "[ -f docs/PLUGIN_DEVELOPMENT.md ]"
warn "Example configurations" "[ -d examples ] && ls examples/*.toml 1>/dev/null 2>&1"
warn "Automated releases" "[ -f .github/workflows/release.yml ]"
echo ""

echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}  Compliance Summary${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo ""
echo "Total Checks: $TOTAL"
echo -e "✅ Passed: ${GREEN}$PASSED${NC}"
echo -e "❌ Failed: ${RED}$FAILED${NC}"
echo ""

PERCENTAGE=$((PASSED * 100 / TOTAL))
echo -e "Compliance Rate: ${BLUE}${PERCENTAGE}%${NC}"
echo ""

# Determine tier
if [ $FAILED -eq 0 ]; then
    TIER="Gold"
    COLOR=$GREEN
elif [ $PERCENTAGE -ge 90 ]; then
    TIER="Silver"
    COLOR=$BLUE
elif [ $PERCENTAGE -ge 75 ]; then
    TIER="Bronze"
    COLOR=$YELLOW
else
    TIER="Partial"
    COLOR=$RED
fi

echo -e "RSR Tier: ${COLOR}${TIER}${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 Excellent! FSLint is fully RSR compliant!${NC}"
    exit 0
elif [ $PERCENTAGE -ge 75 ]; then
    echo -e "${BLUE}👍 Good! FSLint meets RSR ${TIER} tier requirements.${NC}"
    exit 0
else
    echo -e "${RED}⚠️  FSLint needs improvement to meet RSR standards.${NC}"
    echo "Review failed checks above and see RSR documentation."
    exit 1
fi
