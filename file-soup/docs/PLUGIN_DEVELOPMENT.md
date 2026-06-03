# Plugin Development Guide

This guide walks you through creating custom plugins for FSLint.

## Table of Contents

- [Plugin Architecture](#plugin-architecture)
- [Creating Your First Plugin](#creating-your-first-plugin)
- [Plugin API Reference](#plugin-api-reference)
- [Advanced Features](#advanced-features)
- [Testing Plugins](#testing-plugins)
- [Best Practices](#best-practices)
- [Examples](#examples)

## Plugin Architecture

FSLint uses a trait-based plugin system where each plugin implements the `Plugin` trait from `fslint-plugin-api`.

### Plugin Lifecycle

1. **Registration**: Plugin registered with `PluginLoader`
2. **Initialization**: `initialize()` called with configuration
3. **Execution**: `check()` called for each file
4. **Cleanup**: `cleanup()` called when disabled

### Core Components

- **PluginContext**: Input data (file path, metadata, working directory)
- **PluginResult**: Output data (status, message, color, tags)
- **PluginMetadata**: Plugin information (name, version, description)
- **PluginError**: Error types for plugin failures

## Creating Your First Plugin

### Step 1: Create Plugin Structure

```bash
mkdir -p plugins/hello-world/src
cd plugins/hello-world
```

### Step 2: Create Cargo.toml

```toml
[package]
name = "fslint-plugin-hello-world"
version = "0.1.0"
edition = "2021"

[dependencies]
fslint-plugin-api = { path = "../../crates/fslint-plugin-api" }
fslint-plugin-sdk = { path = "../../crates/fslint-plugin-sdk" }

[lib]
crate-type = ["cdylib", "rlib"]
```

### Step 3: Implement Plugin

Create `src/lib.rs`:

```rust
use fslint_plugin_api::{Plugin, PluginContext, PluginError, PluginMetadata, PluginResult};
use fslint_plugin_sdk::path;
use std::collections::HashMap;

pub struct HelloWorldPlugin;

impl HelloWorldPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Default for HelloWorldPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for HelloWorldPlugin {
    fn metadata() -> PluginMetadata {
        PluginMetadata {
            name: "hello-world".to_string(),
            version: "0.1.0".to_string(),
            description: "A simple hello world plugin".to_string(),
            author: Some("Your Name".to_string()),
            enabled_by_default: false,
        }
    }

    fn check(&self, context: &PluginContext) -> Result<PluginResult, PluginError> {
        // Get filename
        let filename = path::filename(&context.path)
            .ok_or_else(|| PluginError::NotApplicable("No filename".to_string()))?;

        // Check if filename contains "hello"
        if filename.to_lowercase().contains("hello") {
            Ok(PluginResult::active("hello-world", "Hello World!"))
        } else {
            Ok(PluginResult::inactive("hello-world"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_metadata() {
        let metadata = HelloWorldPlugin::metadata();
        assert_eq!(metadata.name, "hello-world");
    }

    #[test]
    fn test_check_hello_file() {
        let plugin = HelloWorldPlugin::new();
        let path = PathBuf::from("hello.txt");
        let metadata = std::fs::metadata(".").unwrap();

        let context = PluginContext {
            path,
            metadata,
            working_dir: PathBuf::from("."),
            shared_context: HashMap::new(),
        };

        let result = plugin.check(&context).unwrap();
        assert_eq!(result.status, fslint_plugin_api::PluginStatus::Active);
    }
}
```

### Step 4: Register Plugin

Add to root `Cargo.toml`:

```toml
members = [
    # ... existing members
    "plugins/hello-world",
]
```

Add dependency to `crates/fslint-cli/Cargo.toml`:

```toml
[dependencies]
fslint-plugin-hello-world = { path = "../../plugins/hello-world" }
```

Register in `crates/fslint-cli/src/commands.rs`:

```rust
loader.register(
    fslint_plugin_hello_world::HelloWorldPlugin::new(),
    fslint_plugin_hello_world::HelloWorldPlugin::metadata()
);
```

### Step 5: Build and Test

```bash
# Build the plugin
cargo build -p fslint-plugin-hello-world

# Run tests
cargo test -p fslint-plugin-hello-world

# Try it out
cargo run -- enable hello-world
cargo run -- scan .
```

## Plugin API Reference

### PluginContext

```rust
pub struct PluginContext {
    pub path: PathBuf,              // File path
    pub metadata: std::fs::Metadata, // File metadata
    pub working_dir: PathBuf,        // Working directory
    pub shared_context: HashMap<String, String>, // Shared data
}
```

### PluginResult

```rust
pub struct PluginResult {
    pub plugin_name: String,
    pub status: PluginStatus,
    pub message: Option<String>,
    pub color: Option<String>,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}
```

#### PluginStatus Variants

- `Active`: Plugin found something noteworthy
- `Inactive`: Plugin not applicable
- `Alert`: Warning level finding
- `Warning`: Important issue found
- `Error`: Critical issue found
- `Skipped`: Plugin skipped execution

#### Builder Methods

```rust
// Create active result
PluginResult::active("plugin-name", "message")

// Create inactive result
PluginResult::inactive("plugin-name")

// Create alert
PluginResult::alert("plugin-name", "message")

// Create warning
PluginResult::warning("plugin-name", "message")

// Add color
result.with_color("green")

// Add tags
result.with_tags(vec!["tag1".into(), "tag2".into()])

// Add metadata
result.with_metadata("key", "value")
```

### PluginError

```rust
pub enum PluginError {
    Io(std::io::Error),
    Config(String),
    Execution(String),
    NotApplicable(String),
    ExternalDependency(String),
}
```

## Advanced Features

### Configuration Support

```rust
impl Plugin for MyPlugin {
    fn initialize(&mut self, config: &HashMap<String, String>) -> Result<(), PluginError> {
        if let Some(threshold) = config.get("threshold") {
            self.threshold = threshold.parse()
                .map_err(|e| PluginError::Config(format!("Invalid threshold: {}", e)))?;
        }
        Ok(())
    }
}
```

Config file:
```toml
[plugin_config.my-plugin]
threshold = "100"
```

### Cleanup Support

```rust
impl Plugin for MyPlugin {
    fn cleanup(&mut self) -> Result<(), PluginError> {
        // Clean up resources
        self.cache.clear();
        Ok(())
    }
}
```

### Using SDK Helpers

```rust
use fslint_plugin_sdk::{path, metadata, patterns};

// Get file extension
let ext = path::extension(&context.path);

// Check file age
let days = metadata::age_in_days(context.metadata.modified()?);

// Pattern matching
if patterns::matches(&context.path, &patterns::Patterns::image_files()) {
    // Handle image file
}
```

### Shared Context

```rust
// Set shared context
context.shared_context.insert("key".to_string(), "value".to_string());

// Read from other plugins
if let Some(value) = context.shared_context.get("other-plugin-key") {
    // Use value
}
```

## Testing Plugins

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_plugin_on_real_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "content").unwrap();

        let plugin = MyPlugin::new();
        let metadata = fs::metadata(&file_path).unwrap();

        let context = PluginContext {
            path: file_path,
            metadata,
            working_dir: temp_dir.path().to_path_buf(),
            shared_context: HashMap::new(),
        };

        let result = plugin.check(&context).unwrap();
        assert_eq!(result.status, PluginStatus::Active);
    }
}
```

### Integration Tests

Create `tests/integration_test.rs`:

```rust
use fslint_core::{PluginLoader, Scanner, ScannerConfig};
use fslint_plugin_my_plugin::MyPlugin;

#[test]
fn test_plugin_integration() {
    let mut loader = PluginLoader::new();
    loader.register(MyPlugin::new(), MyPlugin::metadata());
    loader.enable("my-plugin");

    let config = ScannerConfig::default();
    let mut scanner = Scanner::new(config, loader);

    let results = scanner.scan(".").unwrap();
    assert!(!results.is_empty());
}
```

## Best Practices

### Performance

1. **Early Returns**: Skip files that don't match
   ```rust
   if !should_process(&context.path) {
       return Ok(PluginResult::skipped("plugin"));
   }
   ```

2. **Caching**: Cache expensive computations
   ```rust
   lazy_static! {
       static ref CACHE: Mutex<HashMap<PathBuf, Result>> = Mutex::new(HashMap::new());
   }
   ```

3. **Limit File Size**: Don't process huge files
   ```rust
   const MAX_SIZE: u64 = 10 * 1024 * 1024; // 10MB
   if context.metadata.len() > MAX_SIZE {
       return Ok(PluginResult::skipped("plugin"));
   }
   ```

### Error Handling

```rust
// Good: Provide context
.map_err(|e| PluginError::Execution(
    format!("Failed to read {}: {}", path.display(), e)
))?

// Bad: Generic errors
.map_err(|e| PluginError::Execution(e.to_string()))?
```

### Resource Management

```rust
impl Plugin for MyPlugin {
    fn cleanup(&mut self) -> Result<(), PluginError> {
        // Always cleanup resources
        self.cache.clear();
        self.file_handles.clear();
        Ok(())
    }
}
```

## Examples

See existing plugins for complete examples:

- **Simple**: `plugins/file-age/` - Basic plugin with SDK helpers
- **Pattern Matching**: `plugins/grouping/` - Regex patterns
- **External Deps**: `plugins/git-status/` - Using git2
- **Complex Logic**: `plugins/secret-scanner/` - Multiple patterns
- **Caching**: `plugins/duplicate-finder/` - Global state

## Troubleshooting

### Plugin Not Loading

- Check plugin is in workspace `Cargo.toml`
- Verify registration in `commands.rs`
- Ensure plugin compiles: `cargo build -p plugin-name`

### Plugin Not Executing

- Check if enabled: `fslint plugins`
- Enable: `fslint enable plugin-name`
- Check filter logic in `check()`

### Performance Issues

- Profile with `cargo flamegraph`
- Add early returns
- Implement caching
- Reduce file operations

---

Happy plugin development! 🎉
