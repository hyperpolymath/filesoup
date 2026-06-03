use std::fs;
use std::process::Command;
use tempfile::TempDir;

fn get_binary_path() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop(); // Remove test binary name
    path.pop(); // Remove "deps"
    path.push("fslint");
    if cfg!(windows) {
        path.set_extension("exe");
    }
    path.to_string_lossy().to_string()
}

#[test]
fn test_scan_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let binary = get_binary_path();

    let output = Command::new(&binary)
        .args(&["scan", temp_dir.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute fslint");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0 files scanned") || stdout.contains("No files found"));
}

#[test]
fn test_scan_with_files() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("test.txt"), "content").unwrap();
    fs::write(temp_dir.path().join("test.rs"), "fn main() {}").unwrap();

    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(&["scan", temp_dir.path().to_str().unwrap(), "--format", "simple"])
        .output()
        .expect("Failed to execute fslint");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("test.txt") || stdout.contains("test.rs"));
}

#[test]
fn test_scan_json_format() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("test.txt"), "content").unwrap();

    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(&["scan", temp_dir.path().to_str().unwrap(), "--format", "json"])
        .output()
        .expect("Failed to execute fslint");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should be valid JSON
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
    assert!(parsed.is_ok(), "Output should be valid JSON");
}

#[test]
fn test_plugins_command() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(&["plugins"])
        .output()
        .expect("Failed to execute fslint");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("git-status"));
    assert!(stdout.contains("file-age"));
    assert!(stdout.contains("grouping"));
}

#[test]
fn test_enable_disable_plugin() {
    let binary = get_binary_path();

    // Enable a plugin
    let output = Command::new(&binary)
        .args(&["enable", "version-detection"])
        .output()
        .expect("Failed to execute fslint");
    assert!(output.status.success());

    // Disable the plugin
    let output = Command::new(&binary)
        .args(&["disable", "version-detection"])
        .output()
        .expect("Failed to execute fslint");
    assert!(output.status.success());
}

#[test]
fn test_config_command() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(&["config"])
        .output()
        .expect("Failed to execute fslint");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Configuration file") || stdout.contains("Enabled plugins"));
}

#[test]
fn test_query_command() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("test.txt"), "content").unwrap();
    fs::write(temp_dir.path().join("test.rs"), "fn main() {}").unwrap();
    fs::write(temp_dir.path().join("config.toml"), "[config]").unwrap();

    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(&[
            "query",
            "ext:txt",
            temp_dir.path().to_str().unwrap(),
            "--format",
            "simple"
        ])
        .output()
        .expect("Failed to execute fslint");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("test.txt"));
    assert!(!stdout.contains("test.rs"));
}

#[test]
fn test_version_detection_plugin() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("file_v1.txt"), "version 1").unwrap();
    fs::write(temp_dir.path().join("file_v2.txt"), "version 2").unwrap();
    fs::write(temp_dir.path().join("file_final.txt"), "final").unwrap();

    let binary = get_binary_path();

    // Enable version-detection plugin
    Command::new(&binary)
        .args(&["enable", "version-detection"])
        .output()
        .expect("Failed to enable plugin");

    let output = Command::new(&binary)
        .args(&["scan", temp_dir.path().to_str().unwrap(), "--format", "simple"])
        .output()
        .expect("Failed to execute fslint");

    assert!(output.status.success());

    // Cleanup: disable plugin
    Command::new(&binary)
        .args(&["disable", "version-detection"])
        .output()
        .expect("Failed to disable plugin");
}

#[test]
fn test_secret_scanner_plugin() {
    let temp_dir = TempDir::new().unwrap();
    let api_key_file = temp_dir.path().join("config.js");
    fs::write(
        &api_key_file,
        r#"const api_key = "AKIAIOSFODNN7EXAMPLE";"#
    ).unwrap();

    let binary = get_binary_path();

    // Enable secret-scanner plugin
    Command::new(&binary)
        .args(&["enable", "secret-scanner"])
        .output()
        .expect("Failed to enable plugin");

    let output = Command::new(&binary)
        .args(&["scan", temp_dir.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute fslint");

    assert!(output.status.success());

    // Cleanup: disable plugin
    Command::new(&binary)
        .args(&["disable", "secret-scanner"])
        .output()
        .expect("Failed to disable plugin");
}
