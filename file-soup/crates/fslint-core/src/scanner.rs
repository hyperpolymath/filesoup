use anyhow::{Context, Result};
use fslint_plugin_api::{PluginContext, PluginResult};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use ignore::WalkBuilder;

use crate::config::ScannerConfig;
use crate::plugin_loader::PluginLoader;
use crate::cache::ResultCache;

/// File scanner that walks directories and applies plugins
pub struct Scanner {
    config: ScannerConfig,
    plugin_loader: PluginLoader,
    cache: ResultCache,
}

/// Simple file entry struct (internal use)
struct FileEntry {
    path: PathBuf,
    metadata: std::fs::Metadata,
}

/// Scanned file with plugin results
#[derive(Debug, Clone)]
pub struct ScannedFile {
    pub path: PathBuf,
    pub metadata: std::fs::Metadata,
    pub results: Vec<PluginResult>,
}

impl Scanner {
    /// Create a new scanner
    pub fn new(config: ScannerConfig, plugin_loader: PluginLoader) -> Self {
        Self {
            config,
            plugin_loader,
            cache: ResultCache::new(),
        }
    }

    /// Scan a directory
    pub fn scan<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<ScannedFile>> {
        let path = path.as_ref();
        let working_dir = path.canonicalize()
            .with_context(|| format!("Failed to canonicalize path: {:?}", path))?;

        let mut scanned_files = Vec::new();
        let entries = self.collect_entries(&working_dir)?;

        for entry in entries {
            match self.scan_entry(&entry, &working_dir) {
                Ok(scanned) => scanned_files.push(scanned),
                Err(e) => {
                    eprintln!("Warning: Failed to scan {:?}: {}", entry.path, e);
                }
            }

            // Check max_files limit
            if let Some(max) = self.config.max_files {
                if scanned_files.len() >= max {
                    eprintln!("Warning: Reached max files limit ({})", max);
                    break;
                }
            }
        }

        Ok(scanned_files)
    }

    /// Collect directory entries based on configuration
    fn collect_entries(&self, path: &Path) -> Result<Vec<FileEntry>> {
        let mut entries = Vec::new();

        if self.config.respect_gitignore {
            // Use ignore crate for .gitignore support
            let mut builder = WalkBuilder::new(path);
            builder
                .hidden(!self.config.include_hidden)
                .follow_links(self.config.follow_symlinks);

            if let Some(depth) = self.config.max_depth {
                builder.max_depth(Some(depth));
            }

            for result in builder.build() {
                match result {
                    Ok(entry) => {
                        if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                            if let Ok(metadata) = entry.metadata() {
                                entries.push(FileEntry {
                                    path: entry.path().to_path_buf(),
                                    metadata,
                                });
                            }
                        }
                    }
                    Err(e) => eprintln!("Warning: {}", e),
                }
            }
        } else {
            // Use walkdir for simple directory traversal
            let mut walker = WalkDir::new(path)
                .follow_links(self.config.follow_symlinks);

            if let Some(depth) = self.config.max_depth {
                walker = walker.max_depth(depth);
            }

            for result in walker {
                match result {
                    Ok(entry) => {
                        if entry.file_type().is_file() {
                            if !self.config.include_hidden && self.is_hidden(entry.file_name()) {
                                continue;
                            }
                            if let Ok(metadata) = entry.metadata() {
                                entries.push(FileEntry {
                                    path: entry.path().to_path_buf(),
                                    metadata,
                                });
                            }
                        }
                    }
                    Err(e) => eprintln!("Warning: {}", e),
                }
            }
        }

        Ok(entries)
    }

    /// Check if filename is hidden
    fn is_hidden(&self, filename: &std::ffi::OsStr) -> bool {
        filename
            .to_str()
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
    }

    /// Scan a single entry
    fn scan_entry(&mut self, entry: &FileEntry, working_dir: &Path) -> Result<ScannedFile> {
        let path = entry.path.clone();
        let metadata = entry.metadata.clone();

        // Check cache
        let cache_key = (path.clone(), metadata.modified().ok(), metadata.len());
        if let Some(cached_results) = self.cache.get(&cache_key) {
            return Ok(ScannedFile {
                path,
                metadata,
                results: cached_results,
            });
        }

        // Create plugin context
        let context = PluginContext {
            path: path.clone(),
            metadata: metadata.clone(),
            working_dir: working_dir.to_path_buf(),
            shared_context: HashMap::new(),
        };

        // Run plugins
        let results = self.plugin_loader.run_plugins(&context)?;

        // Cache results
        self.cache.insert(cache_key, results.clone());

        Ok(ScannedFile {
            path,
            metadata,
            results,
        })
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        self.cache.stats()
    }

    /// Clear cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_scanner_basic() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "test content").unwrap();

        let config = ScannerConfig::default();
        let plugin_loader = PluginLoader::new();
        let mut scanner = Scanner::new(config, plugin_loader);

        let results = scanner.scan(temp_dir.path()).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].path, file_path);
    }
}
