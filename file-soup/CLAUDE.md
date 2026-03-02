# CLAUDE.md

This file provides context and guidance for Claude (AI assistant) when working on FSLint (file-soup).

## Project Overview

**FSLint** is a cross-platform file system intelligence tool with a Notepad++-style plugin architecture.

**Core Purpose**: Provide contextual metadata about files (git status, age, grouping, AI detection, OCR status, etc.) through tiny, composable plugins that can be enabled/disabled individually.

**Innovation**: No existing tool provides cross-platform file intelligence with this plugin architecture. Closest competitors: exa (no plugins), fd (search only), gitleaks (git-only).

## Project Structure

```
file-soup/
├── crates/
│   ├── fslint-core/         # Scanner, plugin loader, config system
│   ├── fslint-cli/          # CLI commands and output formats
│   ├── fslint-plugin-api/   # Plugin trait definitions
│   └── fslint-plugin-sdk/   # Plugin helper utilities
├── plugins/
│   ├── git-status/          # Git repo status + branch (color-coded)
│   ├── file-age/            # Recent modification highlighting (<7 days)
│   ├── grouping/            # node_modules, .DS_Store, media sets
│   ├── version-detection/   # Find newest among file_v1, file_v2, file_final
│   ├── ocr-status/          # PDF text layer detection
│   ├── ai-detection/        # EXIF/PNG metadata for AI-generated content
│   ├── duplicate-finder/    # Hash-based duplicate detection
│   └── secret-scanner/      # Regex patterns for API keys
└── target/                  # Build output (in .gitignore)
```

## Architecture Decisions

### Rust over Ada
- Better mobile/app store support
- Stronger ecosystem
- WASM runtime available for future browser integration

### Native Plugins First, WASM Later
- Ship fast with native plugins
- Add WASM support in Phase 2 for sandboxing and web compatibility

### Trait-Based Plugin API
```rust
trait Plugin {
    fn metadata() -> PluginMetadata;
    fn check(context: &PluginContext) -> PluginResult;
}
```
Simple, focused interface - each plugin = one feature

### Tiny Core Philosophy
Like Notepad++: each plugin is one focused feature that can be enabled/disabled

### Config System
- Location: `~/.config/fslint/config.toml`
- Plugin enable/disable state
- Per-plugin configuration

## Development Guidelines

### Code Style
- Follow Rust conventions (rustfmt, clippy)
- Write clear, self-documenting code with appropriate comments
- Keep plugins focused - one feature per plugin
- Graceful error handling (no panics in plugins)

### Git Workflow
- Work on feature branches prefixed with `claude/`
- Write clear, descriptive commit messages
- Push changes to the designated branch when complete
- Commit frequently with logical chunks

### Testing
```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p fslint-core

# Run with output
cargo test -- --nocapture
```

### Dependencies
- Use version pinning for stability
- libgit2 for git operations (auto-walks to find .git/)
- wasmtime for WASM plugin runtime
- serde for serialization
- clap for CLI parsing

## Common Tasks

### Setup
```bash
# Clone the repository
git clone <repository-url>
cd file-soup

# Build debug
cargo build

# Build release
cargo build --release
```

### Development
```bash
# Run FSLint
cargo run --release -- scan .

# List plugins
cargo run --release -- plugins

# Enable/disable plugins
cargo run --release -- enable git-status
cargo run --release -- disable file-age

# Query syntax
cargo run --release -- query "name:myfile ext:tiff newest:true"

# Output formats
cargo run --release -- scan . --format table
cargo run --release -- scan . --format json
cargo run --release -- scan . --format simple
```

### Build & Test
```bash
# Format code
cargo fmt --all

# Lint
cargo clippy --workspace -- -D warnings

# Test
cargo test --workspace

# Build release binary
cargo build --release
# Binary at: ./target/release/fslint
```

## Key Features

### Phase 1 (Core)
- ✅ Plugin API trait system
- ✅ File scanner with directory walker
- ✅ CLI with table/JSON/simple output
- ✅ Config system
- ✅ Three core plugins (git-status, file-age, grouping)

### Phase 2 (Performance & Extensions)
- Performance: --max-depth default, smart caching by (path, mtime, size)
- WASM plugin support with wasmtime runtime
- Query engine: `name:, ext:, newest:` syntax
- Additional plugins: version-detection, OCR-status, AI-detection, duplicate-finder, secret-scanner

### Future Vision
- Shadow navigation mode for symlinks
- Virtual filesystem across disks/cloud
- Mailbox integration for email attachments
- Focus mode filters
- Shell integration (Windows Explorer, Finder, Nautilus)

## Important Notes & Gotchas

### Git Plugin
- Uses libgit2 which automatically walks up to find `.git/`
- Must strip path prefix gracefully for files outside repo
- Shows branch in separate table column

### Performance
- Add `--max-depth` default to prevent deep recursion
- Implement caching by (path, mtime, size) tuple
- Lazy plugin execution (only enabled plugins)

### Safety Features
- Warn: "3 visible, 1000 hidden" before destructive operations
- Require `--include-hidden` flag for hidden files
- No accidental operations on system directories

### Build
- `target/` must be in `.gitignore`
- Use `cargo build --release` for performance testing
- Profile with `cargo build --release && time ./target/release/fslint scan .`

## Plugin Development

### Creating a New Plugin

1. Create directory in `plugins/my-plugin/`
2. Add `Cargo.toml`:
```toml
[package]
name = "fslint-plugin-my-plugin"
version = "0.1.0"

[dependencies]
fslint-plugin-api = { path = "../../crates/fslint-plugin-api" }
fslint-plugin-sdk = { path = "../../crates/fslint-plugin-sdk" }
```

3. Implement trait in `src/lib.rs`:
```rust
use fslint_plugin_api::{Plugin, PluginContext, PluginResult, PluginMetadata};

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn metadata() -> PluginMetadata {
        PluginMetadata {
            name: "my-plugin".into(),
            version: "0.1.0".into(),
            description: "Does something useful".into(),
        }
    }

    fn check(context: &PluginContext) -> PluginResult {
        // Implementation
    }
}
```

4. Register in `fslint-core/src/plugin_loader.rs`

## Distribution Targets

- **Cargo**: `cargo install fslint`
- **Homebrew**: Formula for macOS/Linux
- **winget**: Windows package manager
- **Future**: Shell extension integration

## Troubleshooting

### Permission Errors
- Ensure read permissions on scanned directories
- Use `--include-hidden` for hidden files

### Performance Issues
- Use `--max-depth N` to limit recursion
- Enable only needed plugins
- Check cache effectiveness

### Plugin Not Loading
- Verify plugin is in `config.toml` enabled list
- Check plugin compilation: `cargo build -p fslint-plugin-NAME`
- Review logs for plugin errors

## Resources

- Repository: file-soup (FSLint)
- Documentation: FSLINT_README.md
- Issue tracking: GitHub Issues
- Previous work: `claude/git-status-shell-extension-01L7Ppr72PPKbg6yb4Ko4baG`

## Development Philosophy

**Tiny plugins that can be added incrementally, not monolithic features.**

Each plugin should:
- Do one thing well
- Be independently enable/disable-able
- Have minimal performance impact when disabled
- Fail gracefully
- Provide clear, actionable information

---

**Last Updated**: 2025-11-22
