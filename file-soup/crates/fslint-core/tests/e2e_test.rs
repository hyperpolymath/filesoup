// SPDX-License-Identifier: PMPL-1.0-or-later
//
// E2E tests for fslint-core Scanner
//
// These tests exercise the full scan pipeline end-to-end: create a temporary
// directory with known files, run Scanner::scan(), and verify the results
// match expectations.  No external crates are used — only `std::fs` for
// temp-file setup, plus the `tempfile` dev-dependency already declared in
// Cargo.toml.

use fslint_core::{Scanner, ScannerConfig, PluginLoader};
use std::fs;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build a Scanner with default config and no plugins (plugins are covered by
/// unit tests in plugin_loader; here we focus on the scan pipeline itself).
fn make_scanner() -> Scanner {
    let config = ScannerConfig::default();
    let loader = PluginLoader::new();
    Scanner::new(config, loader)
}

/// Build a Scanner limited to `depth` levels of directory nesting.
fn make_scanner_with_depth(depth: usize) -> Scanner {
    let config = ScannerConfig {
        max_depth: Some(depth),
        ..ScannerConfig::default()
    };
    Scanner::new(config, PluginLoader::new())
}

/// Build a Scanner that also returns hidden (dot-prefixed) files.
fn make_scanner_include_hidden() -> Scanner {
    let config = ScannerConfig {
        include_hidden: true,
        respect_gitignore: false,
        ..ScannerConfig::default()
    };
    Scanner::new(config, PluginLoader::new())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Scanning an empty temporary directory must succeed and return zero results.
#[test]
fn test_scan_empty_directory_returns_no_files() {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    let mut scanner = make_scanner();
    let results = scanner.scan(dir.path()).expect("scan should succeed");
    assert_eq!(results.len(), 0, "empty directory must yield zero scanned files");
}

/// Scanning a directory with a single file must return exactly one result with
/// the correct path.
#[test]
fn test_scan_single_file() {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    let file_path = dir.path().join("hello.txt");
    fs::write(&file_path, b"hello world").expect("write file");

    let mut scanner = make_scanner();
    let results = scanner.scan(dir.path()).expect("scan should succeed");

    assert_eq!(results.len(), 1, "one file should produce one result");
    assert_eq!(results[0].path, file_path, "result path must match the written file");
}

/// Scanning a directory with multiple files must return all of them.
#[test]
fn test_scan_multiple_files() {
    let dir = tempfile::TempDir::new().expect("create temp dir");

    let filenames = ["alpha.txt", "beta.rs", "gamma.toml", "delta.md"];
    for name in &filenames {
        fs::write(dir.path().join(name), b"content").expect("write file");
    }

    let mut scanner = make_scanner();
    let results = scanner.scan(dir.path()).expect("scan should succeed");

    assert_eq!(
        results.len(),
        filenames.len(),
        "all {} files should be discovered",
        filenames.len()
    );
}

/// Files inside nested subdirectories must be discovered (up to max_depth).
#[test]
fn test_scan_nested_subdirectory() {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    let sub = dir.path().join("sub");
    fs::create_dir(&sub).expect("create subdir");
    fs::write(sub.join("nested.txt"), b"nested").expect("write nested file");
    fs::write(dir.path().join("root.txt"), b"root").expect("write root file");

    let mut scanner = make_scanner();
    let results = scanner.scan(dir.path()).expect("scan should succeed");

    assert_eq!(results.len(), 2, "both root and nested files must be found");
}

/// max_depth=1 must restrict discovery to the top level only.
#[test]
fn test_scan_max_depth_limits_traversal() {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    let sub = dir.path().join("deep");
    fs::create_dir(&sub).expect("create subdir");
    fs::write(sub.join("buried.txt"), b"buried").expect("write buried file");
    fs::write(dir.path().join("surface.txt"), b"surface").expect("write surface file");

    let mut scanner = make_scanner_with_depth(1);
    let results = scanner.scan(dir.path()).expect("scan should succeed");

    // Only the surface file should be found; max_depth=1 means files directly
    // inside the scanned root (depth 0 = root dir, depth 1 = its direct
    // children).
    let paths: Vec<_> = results.iter().map(|r| &r.path).collect();
    assert!(
        paths.iter().any(|p| p.ends_with("surface.txt")),
        "surface file must be present"
    );
    assert!(
        !paths.iter().any(|p| p.ends_with("buried.txt")),
        "buried file must not be present when max_depth=1"
    );
}

/// max_files cap must stop scanning once the limit is reached.
#[test]
fn test_scan_max_files_cap() {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    for i in 0..20usize {
        fs::write(dir.path().join(format!("file{i}.txt")), b"x").expect("write file");
    }

    let config = ScannerConfig {
        max_files: Some(5),
        ..ScannerConfig::default()
    };
    let mut scanner = Scanner::new(config, PluginLoader::new());
    let results = scanner.scan(dir.path()).expect("scan should succeed");

    assert_eq!(results.len(), 5, "scanner must stop at max_files=5");
}

/// Hidden files (dot-prefixed) must not appear by default, but must appear
/// when `include_hidden` is set.
#[test]
fn test_scan_hidden_files_excluded_by_default() {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    fs::write(dir.path().join("visible.txt"), b"v").expect("write visible");
    fs::write(dir.path().join(".hidden"), b"h").expect("write hidden");

    let mut scanner = Scanner::new(
        ScannerConfig {
            respect_gitignore: false,
            include_hidden: false,
            ..ScannerConfig::default()
        },
        PluginLoader::new(),
    );
    let results = scanner.scan(dir.path()).expect("scan should succeed");

    let paths: Vec<_> = results.iter().map(|r| r.path.file_name().unwrap().to_string_lossy().into_owned()).collect();
    assert!(paths.contains(&"visible.txt".to_string()), "visible file must appear");
    assert!(!paths.contains(&".hidden".to_string()), "hidden file must be excluded by default");
}

/// When `include_hidden` is true, hidden files must be returned.
#[test]
fn test_scan_hidden_files_included_when_flag_set() {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    fs::write(dir.path().join("visible.txt"), b"v").expect("write visible");
    fs::write(dir.path().join(".hidden"), b"h").expect("write hidden");

    let mut scanner = make_scanner_include_hidden();
    let results = scanner.scan(dir.path()).expect("scan should succeed");

    let names: Vec<_> = results
        .iter()
        .map(|r| r.path.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    assert!(names.contains(&".hidden".to_string()), "hidden file must be present when include_hidden=true");
}

/// Scanned file entries must expose accessible file-system metadata (size,
/// modified time).
#[test]
fn test_scanned_file_metadata_is_accessible() {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    let content = b"metadata check";
    fs::write(dir.path().join("meta.txt"), content).expect("write file");

    let mut scanner = make_scanner();
    let results = scanner.scan(dir.path()).expect("scan should succeed");

    assert_eq!(results.len(), 1);
    let entry = &results[0];
    assert_eq!(
        entry.metadata.len(),
        content.len() as u64,
        "reported file size must match written content length"
    );
    // modified() should not error on normal files
    assert!(
        entry.metadata.modified().is_ok(),
        "modified time must be accessible"
    );
}

/// Running the scanner twice on the same directory should produce the same
/// paths (cache should not alter the set of discovered files).
#[test]
fn test_scan_results_stable_across_two_runs() {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    for name in &["one.txt", "two.rs", "three.toml"] {
        fs::write(dir.path().join(name), b"data").expect("write file");
    }

    let mut scanner = make_scanner();
    let first = scanner.scan(dir.path()).expect("first scan");
    let second = scanner.scan(dir.path()).expect("second scan");

    assert_eq!(
        first.len(),
        second.len(),
        "file count must be identical on repeated scans"
    );

    let mut first_paths: Vec<_> = first.iter().map(|r| r.path.clone()).collect();
    let mut second_paths: Vec<_> = second.iter().map(|r| r.path.clone()).collect();
    first_paths.sort();
    second_paths.sort();
    assert_eq!(first_paths, second_paths, "paths must be identical on repeated scans");
}

/// Cache stats must show hits after a second scan of the same files.
#[test]
fn test_cache_populated_after_second_scan() {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    fs::write(dir.path().join("cacheable.txt"), b"cache me").expect("write file");

    let mut scanner = make_scanner();
    scanner.scan(dir.path()).expect("first scan");
    scanner.scan(dir.path()).expect("second scan");

    let (hits, _misses) = scanner.cache_stats();
    assert!(hits >= 1, "at least one cache hit expected after second scan of same file");
}
