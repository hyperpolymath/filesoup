<!--
SPDX-License-Identifier: MPL-2.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# Contributing to FSLint

Thank you for your interest in contributing to FSLint! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Plugin Development](#plugin-development)
- [Coding Standards](#coding-standards)

## Code of Conduct

Be respectful, inclusive, and professional. We're all here to make FSLint better.

## Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/file-soup.git
   cd file-soup
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/Hyperpolymath/file-soup.git
   ```

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git 2.0 or later
- Cargo

### Building

```bash
# Build all crates
cargo build --workspace

# Build release
cargo build --release

# Run tests
cargo test --workspace

# Run clippy
cargo clippy --workspace -- -D warnings

# Format code
cargo fmt --all
```

### Running Locally

```bash
# Run from source
cargo run -- scan .

# Or build and use binary
cargo build --release
./target/release/fslint scan .
```

## Making Changes

### Branch Naming

- Features: `feat/description`
- Bugs: `fix/description`
- Documentation: `docs/description`
- Refactoring: `refactor/description`

Example:
```bash
git checkout -b feat/add-malware-scanner
```

### Commit Messages

Follow conventional commits:

- `feat: Add new feature`
- `fix: Fix bug`
- `docs: Update documentation`
- `test: Add tests`
- `refactor: Refactor code`
- `perf: Performance improvement`
- `chore: Maintenance task`

Example:
```bash
git commit -m "feat: Add malware scanner plugin"
```

## Testing

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p fslint-core

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test '*'
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Arrange
        let input = "test";

        // Act
        let result = function(input);

        // Assert
        assert_eq!(result, expected);
    }
}
```

## Pull Request Process

1. **Update from upstream**:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Make your changes** following coding standards

3. **Add tests** for new functionality

4. **Run checks**:
   ```bash
   cargo fmt --all
   cargo clippy --workspace -- -D warnings
   cargo test --workspace
   ```

5. **Push to your fork**:
   ```bash
   git push origin feat/your-feature
   ```

6. **Create Pull Request**:
   - Provide clear description
   - Reference related issues
   - Include screenshots if UI changes
   - Ensure CI passes

7. **Code Review**:
   - Address feedback
   - Update PR as needed

## Plugin Development

### Creating a New Plugin

1. **Create plugin directory**:
   ```bash
   mkdir -p plugins/my-plugin/src
   ```

2. **Add Cargo.toml**:
   ```toml
   [package]
   name = "fslint-plugin-my-plugin"
   version.workspace = true
   edition.workspace = true

   [dependencies]
   fslint-plugin-api = { path = "../../crates/fslint-plugin-api" }
   fslint-plugin-sdk = { path = "../../crates/fslint-plugin-sdk" }

   [lib]
   crate-type = ["cdylib", "rlib"]
   ```

3. **Implement Plugin trait** in `src/lib.rs`:
   ```rust
   use fslint_plugin_api::*;

   pub struct MyPlugin;

   impl Plugin for MyPlugin {
       fn metadata() -> PluginMetadata {
           PluginMetadata {
               name: "my-plugin".into(),
               version: "0.1.0".into(),
               description: "My awesome plugin".into(),
               author: Some("Your Name".into()),
               enabled_by_default: false,
           }
       }

       fn check(&self, context: &PluginContext) -> Result<PluginResult, PluginError> {
           // Your logic here
           Ok(PluginResult::active("my-plugin", "Found something!"))
       }
   }
   ```

4. **Register plugin** in `crates/fslint-cli/src/commands.rs`:
   ```rust
   loader.register(
       my_plugin::MyPlugin::new(),
       my_plugin::MyPlugin::metadata()
   );
   ```

5. **Add to workspace** in root `Cargo.toml`:
   ```toml
   members = [
       # ... existing members
       "plugins/my-plugin",
   ]
   ```

6. **Add tests**:
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_plugin_metadata() {
           let metadata = MyPlugin::metadata();
           assert_eq!(metadata.name, "my-plugin");
       }
   }
   ```

### Plugin Best Practices

- **Single Responsibility**: One plugin = one feature
- **Performance**: Minimize expensive operations
- **Error Handling**: Use `PluginError` appropriately
- **Documentation**: Add clear docstrings
- **Configuration**: Support plugin-specific config
- **Testing**: Add comprehensive tests

## Coding Standards

### Rust Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Write documentation comments for public APIs

### Code Quality

- **No panics**: Use `Result` and `Option`
- **Error handling**: Provide context with `anyhow::Context`
- **Type safety**: Leverage Rust's type system
- **Documentation**: Document public APIs
- **Tests**: Cover edge cases

### Performance

- Avoid unnecessary allocations
- Use efficient data structures
- Profile before optimizing
- Consider caching for expensive operations

### Security

- Validate all inputs
- Avoid command injection
- Handle sensitive data carefully
- Follow security best practices

## Documentation

### Code Comments

```rust
/// Calculates the hash of a file
///
/// # Arguments
/// * `path` - Path to the file
///
/// # Returns
/// * `Ok(String)` - SHA-256 hash as hex string
/// * `Err(String)` - Error message
pub fn calculate_hash(path: &Path) -> Result<String, String> {
    // Implementation
}
```

### README Updates

Update README.md when:
- Adding new features
- Changing CLI interface
- Adding new plugins
- Updating configuration format

## Release Process

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Create git tag: `git tag -a v0.2.0 -m "Release v0.2.0"`
4. Push tag: `git push --tags`
5. CI will build and create release

## Questions?

- Open an issue for bug reports
- Start a discussion for feature requests
- Join our community chat (coming soon)

## License

By contributing, you agree that your contributions will be dual-licensed under MIT and Apache-2.0.

---

Thank you for contributing to FSLint! 🎉
