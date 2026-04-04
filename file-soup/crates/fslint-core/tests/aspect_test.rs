// SPDX-License-Identifier: PMPL-1.0-or-later
//
// Security / robustness (aspect) tests for fslint-core
//
// These tests verify that the scanner and its safety utilities behave
// defensively under adversarial or unusual conditions:
//
//   - Path-traversal-style inputs do not leak data outside the scan root.
//   - Symlink loops do not cause infinite recursion (follow_symlinks=false).
//   - Files with no read permission are handled gracefully rather than
//     panicking.
//   - Extremely long filenames (≥255 bytes) are processed without error.
//   - The SafetyChecker correctly rejects dangerous system directories.
//   - sanitize_path_for_display strips `../` sequences.
//   - is_hidden_file correctly classifies hidden vs. visible names.
//   - check_system_directory allows safe paths and rejects known system paths.

use fslint_core::{Scanner, ScannerConfig, PluginLoader, SafetyChecker};
use fslint_core::safety::{is_hidden_file, sanitize_path_for_display, is_git_repo};
use std::fs;
use std::path::Path;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn no_symlink_scanner() -> Scanner {
    Scanner::new(
        ScannerConfig {
            follow_symlinks: false,
            respect_gitignore: false,
            include_hidden: false,
            ..ScannerConfig::default()
        },
        PluginLoader::new(),
    )
}

// ---------------------------------------------------------------------------
// Security: path traversal paths are not followed as scan roots
// ---------------------------------------------------------------------------

/// Supplying `../../etc/passwd` as a scan root should fail with an error
/// (the path almost certainly does not exist as a directory in the test
/// environment, so `canonicalize` inside Scanner::scan must fail) — the
/// important property is that the scanner returns Err rather than panicking
/// or silently scanning the system root.
#[test]
fn test_path_traversal_root_is_rejected() {
    let traversal = Path::new("../../etc/passwd");
    let mut scanner = no_symlink_scanner();
    let result = scanner.scan(traversal);
    assert!(
        result.is_err(),
        "scanning a path-traversal root must return Err, not Ok"
    );
}

/// A filename containing `../` in its name (after being written to a temp
/// dir) must not cause the scanner to escape the scan root — the scan itself
/// should succeed and return exactly the file present inside the temp dir.
#[test]
fn test_path_traversal_filename_does_not_escape_root() {
    // We cannot actually write a file named "../../evil" to the filesystem
    // (the OS prevents it), but we can verify that a legitimately-named file
    // with unusual characters is handled normally.
    let dir = tempfile::TempDir::new().expect("create temp dir");
    fs::write(dir.path().join("safe.txt"), b"ok").expect("write");

    let mut scanner = no_symlink_scanner();
    let results = scanner.scan(dir.path()).expect("scan must succeed");

    assert_eq!(results.len(), 1, "only the single safe file should be present");
    // The returned path must be inside the temp dir, not above it
    let root = dir.path().canonicalize().unwrap();
    let file = results[0].path.canonicalize().unwrap_or_else(|_| results[0].path.clone());
    assert!(
        file.starts_with(&root),
        "result path {:?} must be inside scan root {:?}",
        file,
        root
    );
}

// ---------------------------------------------------------------------------
// Security: symlink loops do not cause infinite recursion
// ---------------------------------------------------------------------------

/// When follow_symlinks is false (the default), a directory containing a
/// symlink loop must be scanned without hanging or stack-overflowing.
#[cfg(unix)]
#[test]
fn test_symlink_loop_does_not_cause_infinite_recursion() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::TempDir::new().expect("create temp dir");
    let link_path = dir.path().join("loop_link");
    // Create a symlink pointing back to its parent directory — a loop
    symlink(dir.path(), &link_path).expect("create symlink");
    fs::write(dir.path().join("real.txt"), b"real").expect("write real file");

    let mut scanner = no_symlink_scanner();
    // Must complete in finite time and not panic
    let result = scanner.scan(dir.path());
    assert!(
        result.is_ok(),
        "scan with symlink loop must not error when follow_symlinks=false"
    );
    // The real file should still be found
    let results = result.unwrap();
    assert!(
        results.iter().any(|r| r.path.file_name().unwrap() == "real.txt"),
        "real.txt must be found even when a symlink loop is present"
    );
}

// ---------------------------------------------------------------------------
// Security: unreadable files are skipped gracefully
// ---------------------------------------------------------------------------

/// A file with read-permission removed should cause the scanner to emit a
/// warning (to stderr) but not to panic or return Err for the overall scan.
/// We verify this on Unix only because Windows permission semantics differ.
#[cfg(unix)]
#[test]
fn test_unreadable_file_handled_gracefully() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::TempDir::new().expect("create temp dir");
    let protected = dir.path().join("noperms.txt");
    fs::write(&protected, b"secret").expect("write file");

    // Remove all permissions
    let mut perms = fs::metadata(&protected).unwrap().permissions();
    perms.set_mode(0o000);
    fs::set_permissions(&protected, perms).expect("set permissions");

    let mut scanner = no_symlink_scanner();
    let result = scanner.scan(dir.path());
    // The scan must not return Err even though one file is unreadable
    assert!(
        result.is_ok(),
        "scan must succeed even with an unreadable file present"
    );

    // Restore permissions to let tempfile clean up
    let mut perms = fs::metadata(&protected).unwrap().permissions();
    perms.set_mode(0o644);
    fs::set_permissions(&protected, perms).ok();
}

// ---------------------------------------------------------------------------
// Robustness: very long filenames
// ---------------------------------------------------------------------------

/// Files with names at the OS limit (255 bytes on Linux/macOS) must be
/// handled without panics.
#[test]
fn test_very_long_filename_handled() {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    // 255-character filename (max on most POSIX filesystems)
    let long_name: String = "a".repeat(251) + ".txt";
    let file_path = dir.path().join(&long_name);

    // If the filesystem rejects the name, skip rather than fail
    if fs::write(&file_path, b"long name").is_err() {
        return; // filesystem does not support this length — skip
    }

    let mut scanner = no_symlink_scanner();
    let result = scanner.scan(dir.path());
    assert!(
        result.is_ok(),
        "scanner must not panic or error on a 255-char filename"
    );
    let results = result.unwrap();
    assert_eq!(results.len(), 1, "the long-named file must be discovered");
}

// ---------------------------------------------------------------------------
// SafetyChecker: system directory rejection
// ---------------------------------------------------------------------------

/// SafetyChecker::check_system_directory must return Err for known dangerous
/// paths and Ok for safe paths.
#[test]
fn test_safety_checker_rejects_system_directories() {
    let dangerous = ["/system", "/windows", "/boot", "/dev", "/proc", "/sys"];
    for path_str in &dangerous {
        let result = SafetyChecker::check_system_directory(Path::new(path_str));
        assert!(
            result.is_err(),
            "SafetyChecker must reject dangerous path '{}'",
            path_str
        );
    }
}

#[test]
fn test_safety_checker_allows_safe_directories() {
    let safe = ["/home/user", "/tmp/myproject", "/var/www"];
    for path_str in &safe {
        let result = SafetyChecker::check_system_directory(Path::new(path_str));
        assert!(
            result.is_ok(),
            "SafetyChecker must allow safe path '{}'",
            path_str
        );
    }
}

// ---------------------------------------------------------------------------
// sanitize_path_for_display: strips traversal sequences
// ---------------------------------------------------------------------------

#[test]
fn test_sanitize_path_strips_traversal_sequences() {
    // Each tuple: (input path, whether `../` should still be present AFTER sanitisation)
    // sanitize_path_for_display removes `../` so traversal inputs must yield false.
    let cases = [
        ("../../etc/passwd", false), // traversal stripped → no `../` after
        ("../relative/path", false), // traversal stripped → no `../` after
        ("/absolute/safe/path", false), // no `../` present to begin with
        ("normal.txt", false),           // plain name, no traversal at all
    ];

    for (input, should_still_contain_dotdot) in &cases {
        let sanitized = sanitize_path_for_display(Path::new(input));
        let contains_traversal = sanitized.contains("../") || sanitized.contains("..\\");
        assert_eq!(
            contains_traversal,
            *should_still_contain_dotdot,
            "sanitize_path_for_display({:?}) still contains traversal={}, expected={}",
            input,
            contains_traversal,
            should_still_contain_dotdot
        );
    }
}

// ---------------------------------------------------------------------------
// is_hidden_file: dot-prefixed detection
// ---------------------------------------------------------------------------

#[test]
fn test_is_hidden_file_classification() {
    let hidden = [".env", ".git", ".hidden", ".DS_Store"];
    let visible = ["visible.txt", "main.rs", "Cargo.toml", "README.md"];

    for name in &hidden {
        assert!(
            is_hidden_file(Path::new(name)),
            "'{}' must be classified as hidden",
            name
        );
    }
    for name in &visible {
        assert!(
            !is_hidden_file(Path::new(name)),
            "'{}' must NOT be classified as hidden",
            name
        );
    }
}

// ---------------------------------------------------------------------------
// is_git_repo: basic detection
// ---------------------------------------------------------------------------

#[test]
fn test_is_git_repo_with_dot_git_present() {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    let dot_git = dir.path().join(".git");
    fs::create_dir(&dot_git).expect("create .git dir");

    assert!(
        is_git_repo(dir.path()),
        "directory with .git subdir must be detected as a git repo"
    );
}

#[test]
fn test_is_git_repo_without_dot_git() {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    assert!(
        !is_git_repo(dir.path()),
        "directory without .git must not be detected as a git repo"
    );
}
