use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;
use thiserror::Error;

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    pub enabled_by_default: bool,
}

/// Context provided to plugins for each file
#[derive(Debug, Clone)]
pub struct PluginContext {
    /// Absolute path to the file
    pub path: PathBuf,
    /// File metadata (size, modified time, etc.)
    pub metadata: std::fs::Metadata,
    /// Working directory for relative path calculations
    pub working_dir: PathBuf,
    /// Additional context that plugins can share
    pub shared_context: HashMap<String, String>,
}

/// Result returned by a plugin check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginResult {
    /// Plugin name
    pub plugin_name: String,
    /// Status of the check
    pub status: PluginStatus,
    /// Optional message or detail
    pub message: Option<String>,
    /// Color for display (ANSI color name)
    pub color: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Plugin check status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginStatus {
    /// Plugin check passed/active
    Active,
    /// Plugin check inactive/not applicable
    Inactive,
    /// Plugin check found something noteworthy
    Alert,
    /// Plugin check found a warning
    Warning,
    /// Plugin check found an error
    Error,
    /// Plugin check skipped (not enabled or not applicable)
    Skipped,
}

/// Plugin trait that all plugins must implement
pub trait Plugin: Send + Sync {
    /// Returns plugin metadata
    fn metadata() -> PluginMetadata
    where
        Self: Sized;

    /// Checks a file and returns results
    fn check(&self, context: &PluginContext) -> Result<PluginResult, PluginError>;

    /// Optional: Initialize plugin with configuration
    fn initialize(&mut self, _config: &HashMap<String, String>) -> Result<(), PluginError> {
        Ok(())
    }

    /// Optional: Cleanup when plugin is disabled
    fn cleanup(&mut self) -> Result<(), PluginError> {
        Ok(())
    }
}

/// Plugin errors
#[derive(Error, Debug)]
pub enum PluginError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Plugin configuration error: {0}")]
    Config(String),

    #[error("Plugin execution error: {0}")]
    Execution(String),

    #[error("Plugin not applicable: {0}")]
    NotApplicable(String),

    #[error("External dependency error: {0}")]
    ExternalDependency(String),
}

impl PluginResult {
    /// Create a new active result
    pub fn active(plugin_name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            plugin_name: plugin_name.into(),
            status: PluginStatus::Active,
            message: Some(message.into()),
            color: None,
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Create a new inactive result
    pub fn inactive(plugin_name: impl Into<String>) -> Self {
        Self {
            plugin_name: plugin_name.into(),
            status: PluginStatus::Inactive,
            message: None,
            color: None,
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Create a new alert result
    pub fn alert(plugin_name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            plugin_name: plugin_name.into(),
            status: PluginStatus::Alert,
            message: Some(message.into()),
            color: Some("yellow".into()),
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Create a new warning result
    pub fn warning(plugin_name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            plugin_name: plugin_name.into(),
            status: PluginStatus::Warning,
            message: Some(message.into()),
            color: Some("red".into()),
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Create a skipped result
    pub fn skipped(plugin_name: impl Into<String>) -> Self {
        Self {
            plugin_name: plugin_name.into(),
            status: PluginStatus::Skipped,
            message: None,
            color: None,
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a color to the result
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Add tags to the result
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Add metadata to the result
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_result_builders() {
        let result = PluginResult::active("test", "message");
        assert_eq!(result.status, PluginStatus::Active);
        assert_eq!(result.message, Some("message".into()));

        let result = PluginResult::inactive("test");
        assert_eq!(result.status, PluginStatus::Inactive);
        assert_eq!(result.message, None);
    }

    #[test]
    fn test_plugin_result_with_color() {
        let result = PluginResult::active("test", "message").with_color("green");
        assert_eq!(result.color, Some("green".into()));
    }

    #[test]
    fn test_plugin_result_with_tags() {
        let result = PluginResult::active("test", "message")
            .with_tags(vec!["tag1".into(), "tag2".into()]);
        assert_eq!(result.tags.len(), 2);
    }
}
