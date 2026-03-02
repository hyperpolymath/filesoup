use fslint_plugin_api::{Plugin, PluginContext, PluginError, PluginMetadata, PluginResult, PluginStatus};
use fslint_plugin_sdk::path;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub struct AiDetectionPlugin;

impl AiDetectionPlugin {
    pub fn new() -> Self {
        Self
    }

    fn check_exif_for_ai(&self, path: &std::path::Path) -> Result<Option<String>, String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let mut reader = BufReader::new(file);

        let exif_reader = exif::Reader::new();
        let exif = exif_reader.read_from_container(&mut reader)
            .map_err(|e| e.to_string())?;

        // Check for common AI generation markers
        let ai_markers = vec![
            "Software",
            "ProcessingSoftware",
            "Creator",
            "Model",
        ];

        for tag_name in ai_markers {
            if let Some(field) = exif.fields().find(|f| format!("{}", f.tag) == tag_name) {
                let value = format!("{}", field.display_value());
                let value_lower = value.to_lowercase();

                // Check for common AI tool markers
                if value_lower.contains("stable diffusion") ||
                   value_lower.contains("midjourney") ||
                   value_lower.contains("dall-e") ||
                   value_lower.contains("dalle") ||
                   value_lower.contains("ai generated") ||
                   value_lower.contains("artificial intelligence") ||
                   value_lower.contains("generative") {
                    return Ok(Some(value));
                }
            }
        }

        Ok(None)
    }

    fn check_png_text_chunks(&self, _path: &std::path::Path) -> Result<Option<String>, String> {
        // Placeholder for PNG tEXt chunk checking
        // Would check for Stable Diffusion parameters, etc.
        Ok(None)
    }
}

impl Default for AiDetectionPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for AiDetectionPlugin {
    fn metadata() -> PluginMetadata {
        PluginMetadata {
            name: "ai-detection".to_string(),
            version: "0.1.0".to_string(),
            description: "Detects AI-generated images via EXIF tags and PNG metadata".to_string(),
            author: Some("FSLint Contributors".to_string()),
            enabled_by_default: false,
        }
    }

    fn check(&self, context: &PluginContext) -> Result<PluginResult, PluginError> {
        let ext = path::extension(&context.path);
        let is_image = matches!(
            ext.as_deref(),
            Some("jpg") | Some("jpeg") | Some("png") | Some("webp")
        );

        if !is_image {
            return Ok(PluginResult::skipped("ai-detection"));
        }

        // Check EXIF data
        if let Ok(Some(ai_tool)) = self.check_exif_for_ai(&context.path) {
            let mut result = PluginResult {
                plugin_name: "ai-detection".to_string(),
                status: PluginStatus::Alert,
                message: Some(format!("AI-generated ({})", ai_tool)),
                color: Some("magenta".to_string()),
                tags: vec!["ai".to_string(), "generated".to_string()],
                metadata: HashMap::new(),
            };

            result.metadata.insert("ai_tool".to_string(), ai_tool);
            result.metadata.insert("detection_method".to_string(), "exif".to_string());

            return Ok(result);
        }

        // Check PNG text chunks
        if ext.as_deref() == Some("png") {
            if let Ok(Some(ai_tool)) = self.check_png_text_chunks(&context.path) {
                let mut result = PluginResult {
                    plugin_name: "ai-detection".to_string(),
                    status: PluginStatus::Alert,
                    message: Some(format!("AI-generated ({})", ai_tool)),
                    color: Some("magenta".to_string()),
                    tags: vec!["ai".to_string(), "generated".to_string()],
                    metadata: HashMap::new(),
                };

                result.metadata.insert("ai_tool".to_string(), ai_tool);
                result.metadata.insert("detection_method".to_string(), "png_text".to_string());

                return Ok(result);
            }
        }

        Ok(PluginResult::inactive("ai-detection"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_metadata() {
        let metadata = AiDetectionPlugin::metadata();
        assert_eq!(metadata.name, "ai-detection");
        assert!(!metadata.enabled_by_default);
    }
}
