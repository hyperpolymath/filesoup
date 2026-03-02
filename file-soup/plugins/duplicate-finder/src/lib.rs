use fslint_plugin_api::{Plugin, PluginContext, PluginError, PluginMetadata, PluginResult, PluginStatus};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref FILE_HASHES: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
}

pub struct DuplicateFinderPlugin {
    min_size: u64,
}

impl DuplicateFinderPlugin {
    pub fn new() -> Self {
        Self {
            min_size: 1024, // Only check files >= 1KB by default
        }
    }

    pub fn with_min_size(min_size: u64) -> Self {
        Self { min_size }
    }

    fn calculate_hash(&self, path: &std::path::Path) -> Result<String, String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let mut reader = BufReader::new(file);
        let mut hasher = Sha256::new();
        let mut buffer = [0; 8192];

        loop {
            let count = reader.read(&mut buffer).map_err(|e| e.to_string())?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }

        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    fn check_duplicate(&self, path: &std::path::Path, hash: &str) -> Option<Vec<String>> {
        let mut hashes = FILE_HASHES.lock().unwrap();

        if let Some(paths) = hashes.get(hash) {
            // Found duplicate
            Some(paths.clone())
        } else {
            // First occurrence
            hashes.insert(hash.to_string(), vec![path.to_string_lossy().to_string()]);
            None
        }
    }
}

impl Default for DuplicateFinderPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for DuplicateFinderPlugin {
    fn metadata() -> PluginMetadata {
        PluginMetadata {
            name: "duplicate-finder".to_string(),
            version: "0.1.0".to_string(),
            description: "Finds duplicate files using SHA-256 hash comparison".to_string(),
            author: Some("FSLint Contributors".to_string()),
            enabled_by_default: false,
        }
    }

    fn check(&self, context: &PluginContext) -> Result<PluginResult, PluginError> {
        let file_size = context.metadata.len();

        // Skip small files
        if file_size < self.min_size {
            return Ok(PluginResult::skipped("duplicate-finder"));
        }

        // Calculate hash
        let hash = self.calculate_hash(&context.path)
            .map_err(|e| PluginError::Execution(format!("Failed to hash file: {}", e)))?;

        // Check for duplicates
        match self.check_duplicate(&context.path, &hash) {
            Some(duplicate_paths) => {
                let count = duplicate_paths.len();
                let mut result = PluginResult {
                    plugin_name: "duplicate-finder".to_string(),
                    status: PluginStatus::Warning,
                    message: Some(format!("Duplicate ({} copies)", count + 1)),
                    color: Some("red".to_string()),
                    tags: vec!["duplicate".to_string()],
                    metadata: HashMap::new(),
                };

                result.metadata.insert("hash".to_string(), hash);
                result.metadata.insert("duplicate_count".to_string(), (count + 1).to_string());
                result.metadata.insert("duplicates".to_string(), duplicate_paths.join(";"));

                Ok(result)
            }
            None => {
                let mut result = PluginResult::inactive("duplicate-finder");
                result.metadata.insert("hash".to_string(), hash);
                Ok(result)
            }
        }
    }

    fn initialize(&mut self, config: &HashMap<String, String>) -> Result<(), PluginError> {
        if let Some(min_size) = config.get("min_size") {
            self.min_size = min_size.parse()
                .map_err(|e| PluginError::Config(format!("Invalid min_size: {}", e)))?;
        }
        Ok(())
    }

    fn cleanup(&mut self) -> Result<(), PluginError> {
        // Clear the hash cache
        FILE_HASHES.lock().unwrap().clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_metadata() {
        let metadata = DuplicateFinderPlugin::metadata();
        assert_eq!(metadata.name, "duplicate-finder");
        assert!(!metadata.enabled_by_default);
    }

    #[test]
    fn test_min_size_config() {
        let mut plugin = DuplicateFinderPlugin::new();
        let mut config = HashMap::new();
        config.insert("min_size".to_string(), "2048".to_string());

        plugin.initialize(&config).unwrap();
        assert_eq!(plugin.min_size, 2048);
    }
}
