use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// FSLint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Enabled plugins
    pub enabled_plugins: Vec<String>,
    /// Plugin-specific configuration
    pub plugin_config: HashMap<String, HashMap<String, String>>,
    /// Scanner configuration
    pub scanner: ScannerConfig,
}

/// Scanner configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerConfig {
    /// Maximum depth for directory traversal
    pub max_depth: Option<usize>,
    /// Include hidden files
    pub include_hidden: bool,
    /// Follow symbolic links
    pub follow_symlinks: bool,
    /// Respect .gitignore files
    pub respect_gitignore: bool,
    /// Maximum number of files to scan
    pub max_files: Option<usize>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled_plugins: vec![
                "git-status".to_string(),
                "file-age".to_string(),
                "grouping".to_string(),
            ],
            plugin_config: HashMap::new(),
            scanner: ScannerConfig::default(),
        }
    }
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            max_depth: Some(10),
            include_hidden: false,
            follow_symlinks: false,
            respect_gitignore: true,
            max_files: None,
        }
    }
}

impl Config {
    /// Get config file path
    pub fn config_path() -> Result<PathBuf> {
        let dirs = directories::ProjectDirs::from("", "", "fslint")
            .context("Failed to determine config directory")?;
        Ok(dirs.config_dir().join("config.toml"))
    }

    /// Load configuration from file
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config from {:?}", path))?;
        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config from {:?}", path))?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory {:?}", parent))?;
        }

        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        fs::write(&path, content)
            .with_context(|| format!("Failed to write config to {:?}", path))?;
        Ok(())
    }

    /// Enable a plugin
    pub fn enable_plugin(&mut self, name: impl Into<String>) {
        let name = name.into();
        if !self.enabled_plugins.contains(&name) {
            self.enabled_plugins.push(name);
        }
    }

    /// Disable a plugin
    pub fn disable_plugin(&mut self, name: &str) {
        self.enabled_plugins.retain(|p| p != name);
    }

    /// Check if a plugin is enabled
    pub fn is_plugin_enabled(&self, name: &str) -> bool {
        self.enabled_plugins.contains(&name.to_string())
    }

    /// Get plugin configuration
    pub fn get_plugin_config(&self, name: &str) -> Option<&HashMap<String, String>> {
        self.plugin_config.get(name)
    }

    /// Set plugin configuration
    pub fn set_plugin_config(&mut self, name: impl Into<String>, config: HashMap<String, String>) {
        self.plugin_config.insert(name.into(), config);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.is_plugin_enabled("git-status"));
        assert!(config.is_plugin_enabled("file-age"));
        assert!(config.is_plugin_enabled("grouping"));
    }

    #[test]
    fn test_enable_disable_plugin() {
        let mut config = Config::default();
        config.enable_plugin("test-plugin");
        assert!(config.is_plugin_enabled("test-plugin"));

        config.disable_plugin("test-plugin");
        assert!(!config.is_plugin_enabled("test-plugin"));
    }

    #[test]
    fn test_scanner_config_defaults() {
        let config = ScannerConfig::default();
        assert_eq!(config.max_depth, Some(10));
        assert!(!config.include_hidden);
        assert!(!config.follow_symlinks);
        assert!(config.respect_gitignore);
    }
}
