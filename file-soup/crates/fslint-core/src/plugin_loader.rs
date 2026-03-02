use anyhow::Result;
use fslint_plugin_api::{Plugin, PluginContext, PluginResult};
use std::collections::HashMap;

/// Plugin loader that manages all plugins
pub struct PluginLoader {
    plugins: HashMap<String, Box<dyn Plugin>>,
    enabled_plugins: Vec<String>,
}

impl PluginLoader {
    /// Create a new plugin loader
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            enabled_plugins: Vec::new(),
        }
    }

    /// Register a plugin
    pub fn register<P: Plugin + 'static>(&mut self, plugin: P, metadata: fslint_plugin_api::PluginMetadata) {
        let name = metadata.name.clone();
        self.plugins.insert(name.clone(), Box::new(plugin));
        if metadata.enabled_by_default {
            self.enabled_plugins.push(name);
        }
    }

    /// Enable a plugin
    pub fn enable(&mut self, name: impl Into<String>) {
        let name = name.into();
        if !self.enabled_plugins.contains(&name) {
            self.enabled_plugins.push(name);
        }
    }

    /// Disable a plugin
    pub fn disable(&mut self, name: &str) {
        self.enabled_plugins.retain(|p| p != name);
    }

    /// Check if a plugin is enabled
    pub fn is_enabled(&self, name: &str) -> bool {
        self.enabled_plugins.contains(&name.to_string())
    }

    /// Get list of all registered plugins
    pub fn list_plugins(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }

    /// Get list of enabled plugins
    pub fn list_enabled(&self) -> Vec<String> {
        self.enabled_plugins.clone()
    }

    /// Run all enabled plugins on a context
    pub fn run_plugins(&self, context: &PluginContext) -> Result<Vec<PluginResult>> {
        let mut results = Vec::new();

        for plugin_name in &self.enabled_plugins {
            if let Some(plugin) = self.plugins.get(plugin_name) {
                match plugin.check(context) {
                    Ok(result) => results.push(result),
                    Err(e) => {
                        // Log error but don't fail the whole scan
                        eprintln!("Warning: Plugin '{}' error: {}", plugin_name, e);
                    }
                }
            }
        }

        Ok(results)
    }

    /// Initialize all plugins with configuration
    pub fn initialize_all(&mut self, configs: &HashMap<String, HashMap<String, String>>) -> Result<()> {
        for (name, plugin) in self.plugins.iter_mut() {
            if let Some(config) = configs.get(name) {
                plugin.initialize(config)?;
            }
        }
        Ok(())
    }

    /// Set enabled plugins from list
    pub fn set_enabled(&mut self, enabled: Vec<String>) {
        self.enabled_plugins = enabled;
    }
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fslint_plugin_api::{PluginMetadata, PluginError};

    struct TestPlugin;

    impl Plugin for TestPlugin {
        fn metadata() -> PluginMetadata {
            PluginMetadata {
                name: "test".into(),
                version: "0.1.0".into(),
                description: "Test plugin".into(),
                author: None,
                enabled_by_default: true,
            }
        }

        fn check(&self, _context: &PluginContext) -> Result<PluginResult, PluginError> {
            Ok(PluginResult::active("test", "test message"))
        }
    }

    #[test]
    fn test_plugin_loader_basic() {
        let mut loader = PluginLoader::new();
        let metadata = TestPlugin::metadata();
        loader.register(TestPlugin, metadata);

        assert_eq!(loader.list_plugins().len(), 1);
        assert!(loader.is_enabled("test"));
    }

    #[test]
    fn test_plugin_enable_disable() {
        let mut loader = PluginLoader::new();
        let mut metadata = TestPlugin::metadata();
        metadata.enabled_by_default = false;
        loader.register(TestPlugin, metadata);

        assert!(!loader.is_enabled("test"));
        loader.enable("test");
        assert!(loader.is_enabled("test"));
        loader.disable("test");
        assert!(!loader.is_enabled("test"));
    }
}
