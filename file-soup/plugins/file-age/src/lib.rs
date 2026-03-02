use fslint_plugin_api::{Plugin, PluginContext, PluginError, PluginMetadata, PluginResult, PluginStatus};
use fslint_plugin_sdk::metadata;
use std::collections::HashMap;

pub struct FileAgePlugin {
    threshold_days: i64,
}

impl FileAgePlugin {
    pub fn new() -> Self {
        Self {
            threshold_days: 7,
        }
    }

    pub fn with_threshold(threshold_days: i64) -> Self {
        Self { threshold_days }
    }

    fn categorize_age(&self, days: i64) -> (&str, &str, PluginStatus) {
        if days <= 1 {
            ("Today", "bright_green", PluginStatus::Alert)
        } else if days <= 3 {
            ("Recent (1-3 days)", "green", PluginStatus::Active)
        } else if days <= 7 {
            ("This week", "yellow", PluginStatus::Active)
        } else if days <= 30 {
            ("This month", "gray", PluginStatus::Inactive)
        } else if days <= 90 {
            ("Last 3 months", "gray", PluginStatus::Inactive)
        } else if days <= 365 {
            ("This year", "gray", PluginStatus::Inactive)
        } else {
            ("Old", "gray", PluginStatus::Inactive)
        }
    }
}

impl Default for FileAgePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for FileAgePlugin {
    fn metadata() -> PluginMetadata {
        PluginMetadata {
            name: "file-age".to_string(),
            version: "0.1.0".to_string(),
            description: "Highlights recently modified files (< 7 days by default)".to_string(),
            author: Some("FSLint Contributors".to_string()),
            enabled_by_default: true,
        }
    }

    fn check(&self, context: &PluginContext) -> Result<PluginResult, PluginError> {
        let modified = context.metadata.modified()
            .map_err(|e| PluginError::Execution(format!("Failed to get modified time: {}", e)))?;

        let age_days = metadata::age_in_days(modified)
            .ok_or_else(|| PluginError::Execution("Failed to calculate file age".to_string()))?;

        let (category, color, status) = self.categorize_age(age_days);

        let message = if age_days == 0 {
            "Modified today".to_string()
        } else if age_days == 1 {
            "Modified yesterday".to_string()
        } else {
            format!("Modified {} days ago", age_days)
        };

        let mut result = PluginResult {
            plugin_name: "file-age".to_string(),
            status,
            message: Some(message),
            color: Some(color.to_string()),
            tags: vec!["age".to_string()],
            metadata: HashMap::new(),
        };

        result.metadata.insert("age_days".to_string(), age_days.to_string());
        result.metadata.insert("category".to_string(), category.to_string());

        Ok(result)
    }

    fn initialize(&mut self, config: &HashMap<String, String>) -> Result<(), PluginError> {
        if let Some(threshold) = config.get("threshold_days") {
            self.threshold_days = threshold.parse()
                .map_err(|e| PluginError::Config(format!("Invalid threshold_days: {}", e)))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_metadata() {
        let metadata = FileAgePlugin::metadata();
        assert_eq!(metadata.name, "file-age");
        assert_eq!(metadata.version, "0.1.0");
        assert!(metadata.enabled_by_default);
    }

    #[test]
    fn test_categorize_age() {
        let plugin = FileAgePlugin::new();

        let (cat, _, _) = plugin.categorize_age(0);
        assert_eq!(cat, "Today");

        let (cat, _, _) = plugin.categorize_age(2);
        assert_eq!(cat, "Recent (1-3 days)");

        let (cat, _, _) = plugin.categorize_age(5);
        assert_eq!(cat, "This week");
    }

    #[test]
    fn test_initialize_with_config() {
        let mut plugin = FileAgePlugin::new();
        let mut config = HashMap::new();
        config.insert("threshold_days".to_string(), "14".to_string());

        plugin.initialize(&config).unwrap();
        assert_eq!(plugin.threshold_days, 14);
    }
}
