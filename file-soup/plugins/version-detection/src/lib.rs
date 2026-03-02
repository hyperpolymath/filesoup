use fslint_plugin_api::{Plugin, PluginContext, PluginError, PluginMetadata, PluginResult, PluginStatus};
use fslint_plugin_sdk::path;
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref VERSION_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"_v(\d+)(\.\w+)?$").unwrap(),
        Regex::new(r"_version_?(\d+)(\.\w+)?$").unwrap(),
        Regex::new(r"_(final|last|newest|latest)(\.\w+)?$").unwrap(),
        Regex::new(r"_(old|backup|bak|copy|original)(\.\w+)?$").unwrap(),
        Regex::new(r"\((\d+)\)(\.\w+)?$").unwrap(), // file (1), file (2)
    ];
}

pub struct VersionDetectionPlugin;

impl VersionDetectionPlugin {
    pub fn new() -> Self {
        Self
    }

    fn detect_version(&self, filename: &str) -> Option<(String, i32)> {
        for pattern in VERSION_PATTERNS.iter() {
            if let Some(caps) = pattern.captures(filename) {
                let version_str = caps.get(1)?.as_str();

                // Parse version number or categorize by keyword
                let priority = match version_str {
                    "final" | "last" | "newest" | "latest" => 1000,
                    "old" | "backup" | "bak" | "copy" | "original" => -1000,
                    num => num.parse::<i32>().unwrap_or(0),
                };

                return Some((version_str.to_string(), priority));
            }
        }

        None
    }

    fn categorize_version(&self, priority: i32) -> (&str, &str, PluginStatus) {
        if priority >= 1000 {
            ("Latest Version", "green", PluginStatus::Active)
        } else if priority < 0 {
            ("Old Version", "red", PluginStatus::Warning)
        } else if priority > 5 {
            ("Recent Version", "yellow", PluginStatus::Active)
        } else {
            ("Versioned File", "gray", PluginStatus::Inactive)
        }
    }
}

impl Default for VersionDetectionPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for VersionDetectionPlugin {
    fn metadata() -> PluginMetadata {
        PluginMetadata {
            name: "version-detection".to_string(),
            version: "0.1.0".to_string(),
            description: "Detects versioned files (file_v1, file_v2, file_final)".to_string(),
            author: Some("FSLint Contributors".to_string()),
            enabled_by_default: false,
        }
    }

    fn check(&self, context: &PluginContext) -> Result<PluginResult, PluginError> {
        let filename = path::filename(&context.path)
            .ok_or_else(|| PluginError::NotApplicable("No filename".to_string()))?;

        match self.detect_version(&filename) {
            Some((version, priority)) => {
                let (category, color, status) = self.categorize_version(priority);

                let mut result = PluginResult {
                    plugin_name: "version-detection".to_string(),
                    status,
                    message: Some(format!("{} ({})", category, version)),
                    color: Some(color.to_string()),
                    tags: vec!["version".to_string()],
                    metadata: HashMap::new(),
                };

                result.metadata.insert("version".to_string(), version);
                result.metadata.insert("priority".to_string(), priority.to_string());
                result.metadata.insert("category".to_string(), category.to_string());

                Ok(result)
            }
            None => Ok(PluginResult::inactive("version-detection")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_metadata() {
        let metadata = VersionDetectionPlugin::metadata();
        assert_eq!(metadata.name, "version-detection");
        assert!(!metadata.enabled_by_default);
    }

    #[test]
    fn test_detect_version() {
        let plugin = VersionDetectionPlugin::new();

        assert!(plugin.detect_version("file_v1.txt").is_some());
        assert!(plugin.detect_version("file_v2.txt").is_some());
        assert!(plugin.detect_version("file_final.txt").is_some());
        assert!(plugin.detect_version("file_old.txt").is_some());
        assert!(plugin.detect_version("file (1).txt").is_some());
        assert!(plugin.detect_version("regular.txt").is_none());
    }

    #[test]
    fn test_version_priority() {
        let plugin = VersionDetectionPlugin::new();

        let (_, priority) = plugin.detect_version("file_final.txt").unwrap();
        assert_eq!(priority, 1000);

        let (_, priority) = plugin.detect_version("file_old.txt").unwrap();
        assert_eq!(priority, -1000);

        let (_, priority) = plugin.detect_version("file_v5.txt").unwrap();
        assert_eq!(priority, 5);
    }
}
