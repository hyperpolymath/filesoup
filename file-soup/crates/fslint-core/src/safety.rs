use anyhow::{bail, Result};
use std::path::Path;

/// Safety checker for file operations
pub struct SafetyChecker {
    visible_count: usize,
    hidden_count: usize,
    warning_threshold: usize,
}

impl SafetyChecker {
    pub fn new() -> Self {
        Self {
            visible_count: 0,
            hidden_count: 0,
            warning_threshold: 1000,
        }
    }

    pub fn with_threshold(threshold: usize) -> Self {
        Self {
            visible_count: 0,
            hidden_count: 0,
            warning_threshold: threshold,
        }
    }

    /// Track a file and check if it's hidden
    pub fn track_file(&mut self, path: &Path) -> bool {
        let is_hidden = is_hidden_file(path);

        if is_hidden {
            self.hidden_count += 1;
        } else {
            self.visible_count += 1;
        }

        is_hidden
    }

    /// Check if we should warn about hidden file ratio
    pub fn should_warn(&self) -> Option<String> {
        if self.hidden_count > self.warning_threshold &&
           self.visible_count < 10 &&
           self.hidden_count > self.visible_count * 100 {
            Some(format!(
                "⚠️  Warning: {} visible files, {} hidden files detected. \
                 Use --include-hidden to scan hidden files.",
                self.visible_count, self.hidden_count
            ))
        } else {
            None
        }
    }

    /// Get statistics
    pub fn stats(&self) -> (usize, usize) {
        (self.visible_count, self.hidden_count)
    }

    /// Check if path is a system directory that should warn before scanning
    pub fn check_system_directory(path: &Path) -> Result<()> {
        let path_str = path.to_string_lossy().to_lowercase();

        // Dangerous system directories
        let dangerous_paths = [
            "/system",
            "/windows",
            "/boot",
            "/dev",
            "/proc",
            "/sys",
            "c:\\windows",
            "c:\\program files",
            "/library/system",
        ];

        for dangerous in &dangerous_paths {
            if path_str.starts_with(dangerous) || path_str.contains(dangerous) {
                bail!(
                    "⚠️  Refusing to scan system directory: {:?}\n\
                     This could be dangerous or very slow. If you're sure, use --allow-system flag.",
                    path
                );
            }
        }

        Ok(())
    }
}

impl Default for SafetyChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if a path is hidden
pub fn is_hidden_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

/// Check if a path is a git repository
pub fn is_git_repo(path: &Path) -> bool {
    path.join(".git").exists()
}

/// Sanitize path for display (prevent path traversal attacks in output)
pub fn sanitize_path_for_display(path: &Path) -> String {
    path.to_string_lossy()
        .replace("../", "")
        .replace("..\\", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_hidden_file() {
        assert!(is_hidden_file(Path::new(".hidden")));
        assert!(is_hidden_file(Path::new(".git")));
        assert!(!is_hidden_file(Path::new("visible.txt")));
    }

    #[test]
    fn test_safety_checker_tracking() {
        let mut checker = SafetyChecker::new();

        checker.track_file(Path::new("visible.txt"));
        checker.track_file(Path::new(".hidden"));
        checker.track_file(Path::new("another.txt"));

        let (visible, hidden) = checker.stats();
        assert_eq!(visible, 2);
        assert_eq!(hidden, 1);
    }

    #[test]
    fn test_safety_checker_warning() {
        let mut checker = SafetyChecker::with_threshold(10);

        // Add many hidden files
        for i in 0..1000 {
            checker.track_file(Path::new(&format!(".hidden{}", i)));
        }

        // Add few visible files
        checker.track_file(Path::new("visible.txt"));

        assert!(checker.should_warn().is_some());
    }

    #[test]
    fn test_system_directory_check() {
        assert!(SafetyChecker::check_system_directory(Path::new("/home/user")).is_ok());
        assert!(SafetyChecker::check_system_directory(Path::new("/system")).is_err());
        assert!(SafetyChecker::check_system_directory(Path::new("/boot")).is_err());
    }

    #[test]
    fn test_sanitize_path() {
        let malicious = Path::new("../../../etc/passwd");
        let sanitized = sanitize_path_for_display(malicious);
        assert!(!sanitized.contains("../"));
    }
}
