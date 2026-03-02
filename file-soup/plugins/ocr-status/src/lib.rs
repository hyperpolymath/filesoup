use fslint_plugin_api::{Plugin, PluginContext, PluginError, PluginMetadata, PluginResult, PluginStatus};
use fslint_plugin_sdk::path;
use std::collections::HashMap;

pub struct OcrStatusPlugin;

impl OcrStatusPlugin {
    pub fn new() -> Self {
        Self
    }

    fn check_pdf_ocr(&self, _path: &std::path::Path) -> Result<bool, String> {
        // Placeholder implementation
        // In a real implementation, we would:
        // 1. Open the PDF
        // 2. Check if it contains extractable text
        // 3. Compare text content vs image content ratio

        // For now, we'll return a placeholder
        Err("PDF OCR detection not yet implemented".to_string())
    }
}

impl Default for OcrStatusPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for OcrStatusPlugin {
    fn metadata() -> PluginMetadata {
        PluginMetadata {
            name: "ocr-status".to_string(),
            version: "0.1.0".to_string(),
            description: "Detects OCR status in PDFs (text layer present/absent)".to_string(),
            author: Some("FSLint Contributors".to_string()),
            enabled_by_default: false,
        }
    }

    fn check(&self, context: &PluginContext) -> Result<PluginResult, PluginError> {
        // Only check PDF files
        let ext = path::extension(&context.path);
        if ext.as_deref() != Some("pdf") {
            return Ok(PluginResult::skipped("ocr-status"));
        }

        match self.check_pdf_ocr(&context.path) {
            Ok(has_ocr) => {
                let (message, color, status) = if has_ocr {
                    ("PDF with text layer", "green", PluginStatus::Active)
                } else {
                    ("PDF without text (needs OCR)", "yellow", PluginStatus::Alert)
                };

                let mut result = PluginResult {
                    plugin_name: "ocr-status".to_string(),
                    status,
                    message: Some(message.to_string()),
                    color: Some(color.to_string()),
                    tags: vec!["pdf".to_string(), "ocr".to_string()],
                    metadata: HashMap::new(),
                };

                result.metadata.insert("has_ocr".to_string(), has_ocr.to_string());

                Ok(result)
            }
            Err(_e) => {
                // OCR detection not available or failed
                Ok(PluginResult::skipped("ocr-status"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_metadata() {
        let metadata = OcrStatusPlugin::metadata();
        assert_eq!(metadata.name, "ocr-status");
        assert!(!metadata.enabled_by_default);
    }
}
