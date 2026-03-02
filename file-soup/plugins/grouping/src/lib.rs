use fslint_plugin_api::{Plugin, PluginContext, PluginError, PluginMetadata, PluginResult, PluginStatus};
use fslint_plugin_sdk::{path, patterns};
use std::collections::HashMap;

pub struct GroupingPlugin;

impl GroupingPlugin {
    pub fn new() -> Self {
        Self
    }

    fn detect_group(&self, context: &PluginContext) -> Option<(String, String, String)> {
        let _path_str = context.path.to_string_lossy();

        // Node modules
        if patterns::matches(&context.path, &patterns::Patterns::node_modules()) {
            return Some((
                "Node Dependencies".to_string(),
                "yellow".to_string(),
                "dependencies".to_string(),
            ));
        }

        // .DS_Store
        if patterns::matches(&context.path, &patterns::Patterns::ds_store()) {
            return Some((
                "macOS System File".to_string(),
                "gray".to_string(),
                "system".to_string(),
            ));
        }

        // Temp files
        if patterns::matches(&context.path, &patterns::Patterns::temp_files()) {
            return Some((
                "Temporary File".to_string(),
                "gray".to_string(),
                "temp".to_string(),
            ));
        }

        // Build artifacts
        if patterns::matches(&context.path, &patterns::Patterns::build_artifacts()) {
            return Some((
                "Build Artifact".to_string(),
                "blue".to_string(),
                "build".to_string(),
            ));
        }

        // Media files - Images
        if patterns::matches(&context.path, &patterns::Patterns::image_files()) {
            return Some((
                "Image File".to_string(),
                "magenta".to_string(),
                "media".to_string(),
            ));
        }

        // Media files - Videos
        if patterns::matches(&context.path, &patterns::Patterns::video_files()) {
            return Some((
                "Video File".to_string(),
                "magenta".to_string(),
                "media".to_string(),
            ));
        }

        // Media files - Audio
        if patterns::matches(&context.path, &patterns::Patterns::audio_files()) {
            return Some((
                "Audio File".to_string(),
                "magenta".to_string(),
                "media".to_string(),
            ));
        }

        // Documents
        if patterns::matches(&context.path, &patterns::Patterns::document_files()) {
            return Some((
                "Document".to_string(),
                "cyan".to_string(),
                "document".to_string(),
            ));
        }

        // Archives
        if patterns::matches(&context.path, &patterns::Patterns::archive_files()) {
            return Some((
                "Archive".to_string(),
                "yellow".to_string(),
                "archive".to_string(),
            ));
        }

        // Hidden files
        if path::is_hidden(&context.path) {
            return Some((
                "Hidden File".to_string(),
                "gray".to_string(),
                "hidden".to_string(),
            ));
        }

        None
    }
}

impl Default for GroupingPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for GroupingPlugin {
    fn metadata() -> PluginMetadata {
        PluginMetadata {
            name: "grouping".to_string(),
            version: "0.1.0".to_string(),
            description: "Identifies file groups: node_modules, .DS_Store, media sets, bundles".to_string(),
            author: Some("FSLint Contributors".to_string()),
            enabled_by_default: true,
        }
    }

    fn check(&self, context: &PluginContext) -> Result<PluginResult, PluginError> {
        match self.detect_group(context) {
            Some((group_name, color, tag)) => {
                let mut result = PluginResult {
                    plugin_name: "grouping".to_string(),
                    status: PluginStatus::Active,
                    message: Some(group_name.clone()),
                    color: Some(color),
                    tags: vec!["group".to_string(), tag.clone()],
                    metadata: HashMap::new(),
                };

                result.metadata.insert("group".to_string(), group_name);
                result.metadata.insert("group_tag".to_string(), tag);

                Ok(result)
            }
            None => Ok(PluginResult::inactive("grouping")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_plugin_metadata() {
        let metadata = GroupingPlugin::metadata();
        assert_eq!(metadata.name, "grouping");
        assert_eq!(metadata.version, "0.1.0");
        assert!(metadata.enabled_by_default);
    }

    #[test]
    fn test_detect_node_modules() {
        let plugin = GroupingPlugin::new();
        let path = PathBuf::from("/project/node_modules/package/index.js");
        let metadata = std::fs::metadata(".").unwrap(); // Dummy metadata

        let context = PluginContext {
            path,
            metadata,
            working_dir: PathBuf::from("/project"),
            shared_context: HashMap::new(),
        };

        let result = plugin.check(&context).unwrap();
        assert_eq!(result.status, PluginStatus::Active);
        assert!(result.message.unwrap().contains("Node Dependencies"));
    }
}
