// SPDX-License-Identifier: PMPL-1.0-or-later
//
// Property tests for fslint-core Scanner
//
// Instead of a property-based test library (which would add a dependency), we
// use a fixed table of known inputs and verify the invariants that a true
// property test would check:
//
//   1. The scanner never panics on any of the listed inputs.
//   2. Results are deterministic — scanning the same directory twice yields the
//      same file paths.
//   3. Edge cases (empty dirs, single-file dirs, dirs with no matches) all
//      return without error.
//
// All temp-file work uses `std::fs`; `tempfile` is the only dev-dependency
// beyond the workspace.

use fslint_core::{Scanner, ScannerConfig, PluginLoader};
use std::fs;
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn no_gitignore_scanner() -> Scanner {
    Scanner::new(
        ScannerConfig {
            respect_gitignore: false,
            include_hidden: false,
            ..ScannerConfig::default()
        },
        PluginLoader::new(),
    )
}

// ---------------------------------------------------------------------------
// Fixed test-input table
//
// Each entry describes what to write into the temp directory before scanning.
// `paths` is a list of (relative-path, content) pairs.  An empty slice means
// the directory is left completely empty.
// ---------------------------------------------------------------------------

struct InputCase {
    label: &'static str,
    paths: &'static [(&'static str, &'static [u8])],
}

/// The full table of input cases exercised by the property tests below.
const CASES: &[InputCase] = &[
    InputCase { label: "empty_dir",          paths: &[] },
    InputCase { label: "single_file",        paths: &[("only.txt", b"hello")] },
    InputCase { label: "two_files",          paths: &[("a.txt", b"aaa"), ("b.txt", b"bbb")] },
    InputCase { label: "binary_file",        paths: &[("data.bin", &[0x00, 0xFF, 0xFE, 0x01])] },
    InputCase { label: "empty_file",         paths: &[("empty.txt", b"")] },
    InputCase { label: "rust_source",        paths: &[("main.rs", b"fn main() {}")] },
    InputCase { label: "toml_config",        paths: &[("Cargo.toml", b"[package]\nname = \"x\"")] },
    InputCase { label: "dotfile_only",       paths: &[(".env", b"SECRET=hunter2")] },
    InputCase { label: "multiple_types",     paths: &[
        ("readme.md", b"# readme"),
        ("config.toml", b"[x]"),
        ("script.sh", b"#!/bin/sh"),
    ]},
    InputCase { label: "subdir_with_files",  paths: &[
        ("top.txt", b"top"),
        ("sub/nested.txt", b"nested"),
    ]},
];

// ---------------------------------------------------------------------------
// Helper: populate a temp dir from an InputCase
// ---------------------------------------------------------------------------

fn populate(dir: &std::path::Path, case: &InputCase) {
    for (rel_path, content) in case.paths {
        let full = dir.join(rel_path);
        if let Some(parent) = full.parent() {
            if parent != dir {
                fs::create_dir_all(parent).expect("create parent dirs");
            }
        }
        fs::write(&full, content).expect("write test file");
    }
}

// ---------------------------------------------------------------------------
// Property 1: Scanner never panics and always returns Ok for all input cases
// ---------------------------------------------------------------------------

#[test]
fn property_scan_never_errors_on_fixed_inputs() {
    for case in CASES {
        let dir = tempfile::TempDir::new().expect("create temp dir");
        populate(dir.path(), case);
        let mut scanner = no_gitignore_scanner();
        let result = scanner.scan(dir.path());
        assert!(
            result.is_ok(),
            "scan must succeed for case '{}': {:?}",
            case.label,
            result.err()
        );
    }
    // 10 cases × 1 assertion each = 10 assertions
}

// ---------------------------------------------------------------------------
// Property 2: File count matches the number of visible files written
// ---------------------------------------------------------------------------

#[test]
fn property_file_count_matches_visible_inputs() {
    // For each case, count non-dotfile paths — those are the ones the scanner
    // should return with default config (include_hidden=false).
    for case in CASES {
        let expected_visible: usize = case
            .paths
            .iter()
            .filter(|(p, _)| {
                // Count only paths whose final component is not hidden
                let last = PathBuf::from(p);
                let name = last
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                !name.starts_with('.')
            })
            .count();

        let dir = tempfile::TempDir::new().expect("create temp dir");
        populate(dir.path(), case);
        let mut scanner = no_gitignore_scanner();
        let results = scanner.scan(dir.path()).expect("scan must succeed");

        assert_eq!(
            results.len(),
            expected_visible,
            "case '{}': expected {} visible files, got {}",
            case.label,
            expected_visible,
            results.len()
        );
    }
    // 10 assertions
}

// ---------------------------------------------------------------------------
// Property 3: Results are deterministic — same input ⇒ same paths both runs
// ---------------------------------------------------------------------------

#[test]
fn property_scan_results_are_deterministic() {
    for case in CASES {
        let dir = tempfile::TempDir::new().expect("create temp dir");
        populate(dir.path(), case);

        let mut scanner = no_gitignore_scanner();
        let mut run1: Vec<_> = scanner
            .scan(dir.path())
            .expect("first scan")
            .into_iter()
            .map(|r| r.path)
            .collect();
        let mut run2: Vec<_> = scanner
            .scan(dir.path())
            .expect("second scan")
            .into_iter()
            .map(|r| r.path)
            .collect();

        run1.sort();
        run2.sort();

        assert_eq!(
            run1, run2,
            "case '{}': paths must be identical on second scan",
            case.label
        );
    }
    // 10 assertions
}

// ---------------------------------------------------------------------------
// Property 4: Scanner with max_depth=1 never returns paths deeper than depth 1
// ---------------------------------------------------------------------------

#[test]
fn property_max_depth_respected_across_inputs() {
    for case in CASES {
        let dir = tempfile::TempDir::new().expect("create temp dir");
        populate(dir.path(), case);

        let config = ScannerConfig {
            max_depth: Some(1),
            respect_gitignore: false,
            include_hidden: false,
            ..ScannerConfig::default()
        };
        let mut scanner = Scanner::new(config, PluginLoader::new());
        let results = scanner.scan(dir.path()).expect("scan must succeed");

        for scanned in &results {
            // A result path must be at most one directory deeper than the root
            let root = dir.path().canonicalize().unwrap();
            let file = scanned.path.canonicalize().unwrap_or_else(|_| scanned.path.clone());
            let relative = file.strip_prefix(&root).unwrap_or(&file);
            let depth = relative.components().count();
            assert!(
                depth <= 1,
                "case '{}': path {:?} exceeds max_depth=1 (depth={})",
                case.label,
                relative,
                depth
            );
        }
    }
    // 10 assertions (one per case, each checks all returned paths)
}

// ---------------------------------------------------------------------------
// Property 5: max_files cap is never exceeded
// ---------------------------------------------------------------------------

#[test]
fn property_max_files_cap_never_exceeded() {
    // Write 15 files but cap at 5
    let dir = tempfile::TempDir::new().expect("create temp dir");
    for i in 0..15usize {
        fs::write(dir.path().join(format!("file{i}.txt")), b"x").expect("write");
    }
    let config = ScannerConfig {
        max_files: Some(5),
        respect_gitignore: false,
        ..ScannerConfig::default()
    };
    let mut scanner = Scanner::new(config, PluginLoader::new());
    let results = scanner.scan(dir.path()).expect("scan must succeed");

    assert!(
        results.len() <= 5,
        "max_files=5 must not be exceeded; got {}",
        results.len()
    );
}
