use fslint_plugin_api::PluginContext;
use chrono::{DateTime, Utc};
use regex::Regex;
use std::path::Path;

/// Helper functions for working with file paths
pub mod path {
    use std::path::{Path, PathBuf};

    /// Get file extension as lowercase string
    pub fn extension(path: &Path) -> Option<String> {
        path.extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
    }

    /// Get file stem (name without extension)
    pub fn stem(path: &Path) -> Option<String> {
        path.file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
    }

    /// Get filename
    pub fn filename(path: &Path) -> Option<String> {
        path.file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
    }

    /// Check if path is hidden (starts with .)
    pub fn is_hidden(path: &Path) -> bool {
        path.file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
    }

    /// Get relative path from base
    pub fn relative_path(path: &Path, base: &Path) -> Option<PathBuf> {
        path.strip_prefix(base).ok().map(|p| p.to_path_buf())
    }
}

/// Helper functions for working with file metadata
pub mod metadata {
    use super::*;
    use std::time::SystemTime;

    /// Get file age in days
    pub fn age_in_days(modified: SystemTime) -> Option<i64> {
        let modified: DateTime<Utc> = modified.into();
        let now = Utc::now();
        let duration = now.signed_duration_since(modified);
        Some(duration.num_days())
    }

    /// Get file age in hours
    pub fn age_in_hours(modified: SystemTime) -> Option<i64> {
        let modified: DateTime<Utc> = modified.into();
        let now = Utc::now();
        let duration = now.signed_duration_since(modified);
        Some(duration.num_hours())
    }

    /// Check if file was modified recently (within N days)
    pub fn is_recent(modified: SystemTime, days: i64) -> bool {
        age_in_days(modified).map(|age| age <= days).unwrap_or(false)
    }

    /// Format file size to human-readable string
    pub fn format_size(size: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        const TB: u64 = GB * 1024;

        if size >= TB {
            format!("{:.2} TB", size as f64 / TB as f64)
        } else if size >= GB {
            format!("{:.2} GB", size as f64 / GB as f64)
        } else if size >= MB {
            format!("{:.2} MB", size as f64 / MB as f64)
        } else if size >= KB {
            format!("{:.2} KB", size as f64 / KB as f64)
        } else {
            format!("{} B", size)
        }
    }
}

/// Helper functions for pattern matching
pub mod patterns {
    use super::*;

    /// Common file patterns
    pub struct Patterns;

    impl Patterns {
        /// Node.js dependency directory
        pub fn node_modules() -> Regex {
            Regex::new(r"node_modules").unwrap()
        }

        /// macOS system file
        pub fn ds_store() -> Regex {
            Regex::new(r"\.DS_Store$").unwrap()
        }

        /// Temporary files
        pub fn temp_files() -> Regex {
            Regex::new(r"\.(tmp|temp|swp|swo|bak)$").unwrap()
        }

        /// Build artifacts
        pub fn build_artifacts() -> Regex {
            Regex::new(r"(target|build|dist|out|bin)/").unwrap()
        }

        /// Version suffixes (v1, v2, final, etc.)
        pub fn version_suffix() -> Regex {
            Regex::new(r"[_-](v\d+|final|old|new|backup|copy)(\.\w+)?$").unwrap()
        }

        /// Image files
        pub fn image_files() -> Regex {
            Regex::new(r"\.(jpg|jpeg|png|gif|bmp|tiff|webp|svg)$").unwrap()
        }

        /// Video files
        pub fn video_files() -> Regex {
            Regex::new(r"\.(mp4|avi|mov|wmv|flv|mkv|webm)$").unwrap()
        }

        /// Audio files
        pub fn audio_files() -> Regex {
            Regex::new(r"\.(mp3|wav|flac|aac|ogg|m4a)$").unwrap()
        }

        /// Document files
        pub fn document_files() -> Regex {
            Regex::new(r"\.(pdf|doc|docx|txt|md|rtf|odt)$").unwrap()
        }

        /// Archive files
        pub fn archive_files() -> Regex {
            Regex::new(r"\.(zip|tar|gz|bz2|xz|7z|rar)$").unwrap()
        }
    }

    /// Check if path matches pattern
    pub fn matches(path: &Path, pattern: &Regex) -> bool {
        path.to_str().map(|s| pattern.is_match(s)).unwrap_or(false)
    }
}

/// Helper functions for context
pub mod context {
    use super::*;

    /// Get relative path from context
    pub fn relative_path(ctx: &PluginContext) -> Option<String> {
        ctx.path
            .strip_prefix(&ctx.working_dir)
            .ok()
            .and_then(|p| p.to_str())
            .map(|s| s.to_string())
    }

    /// Get file size from context
    pub fn file_size(ctx: &PluginContext) -> u64 {
        ctx.metadata.len()
    }

    /// Get modified time from context
    pub fn modified_time(ctx: &PluginContext) -> Option<std::time::SystemTime> {
        ctx.metadata.modified().ok()
    }

    /// Check if context path is a directory
    pub fn is_directory(ctx: &PluginContext) -> bool {
        ctx.metadata.is_dir()
    }

    /// Check if context path is a file
    pub fn is_file(ctx: &PluginContext) -> bool {
        ctx.metadata.is_file()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension() {
        let path = Path::new("test.txt");
        assert_eq!(path::extension(path), Some("txt".to_string()));

        let path = Path::new("test.TXT");
        assert_eq!(path::extension(path), Some("txt".to_string()));
    }

    #[test]
    fn test_is_hidden() {
        assert!(path::is_hidden(Path::new(".hidden")));
        assert!(!path::is_hidden(Path::new("visible")));
    }

    #[test]
    fn test_format_size() {
        assert_eq!(metadata::format_size(500), "500 B");
        assert_eq!(metadata::format_size(1024), "1.00 KB");
        assert_eq!(metadata::format_size(1024 * 1024), "1.00 MB");
    }

    #[test]
    fn test_patterns() {
        assert!(patterns::matches(
            Path::new("node_modules/package"),
            &patterns::Patterns::node_modules()
        ));
        assert!(patterns::matches(
            Path::new(".DS_Store"),
            &patterns::Patterns::ds_store()
        ));
    }
}
