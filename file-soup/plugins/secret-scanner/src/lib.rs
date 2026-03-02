use fslint_plugin_api::{Plugin, PluginContext, PluginError, PluginMetadata, PluginResult, PluginStatus};
use fslint_plugin_sdk::path;
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref SECRET_PATTERNS: Vec<(String, Regex)> = vec![
        ("AWS Access Key".to_string(), Regex::new(r"AKIA[0-9A-Z]{16}").unwrap()),
        ("Generic API Key".to_string(), Regex::new(r#"api[_-]?key["']?\s*[:=]\s*["']?([a-zA-Z0-9]{32,})"#).unwrap()),
        ("Generic Secret".to_string(), Regex::new(r#"secret["']?\s*[:=]\s*["']?([a-zA-Z0-9]{32,})"#).unwrap()),
        ("GitHub Token".to_string(), Regex::new(r"ghp_[a-zA-Z0-9]{36}").unwrap()),
        ("GitHub OAuth".to_string(), Regex::new(r"gho_[a-zA-Z0-9]{36}").unwrap()),
        ("Slack Token".to_string(), Regex::new(r"xox[baprs]-[0-9]{10,13}-[0-9]{10,13}-[a-zA-Z0-9]{24,}").unwrap()),
        ("Slack Webhook".to_string(), Regex::new(r"https://hooks\.slack\.com/services/T[a-zA-Z0-9_]{8}/B[a-zA-Z0-9_]{8}/[a-zA-Z0-9_]{24}").unwrap()),
        ("Private Key".to_string(), Regex::new(r"-----BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY-----").unwrap()),
        ("JWT Token".to_string(), Regex::new(r"eyJ[a-zA-Z0-9_-]*\.eyJ[a-zA-Z0-9_-]*\.[a-zA-Z0-9_-]*").unwrap()),
        ("Password in code".to_string(), Regex::new(r#"password[\"']?\s*[:=]\s*[\"']([^\s\"']{8,})"#).unwrap()),
    ];
}

pub struct SecretScannerPlugin {
    max_file_size: u64,
}

impl SecretScannerPlugin {
    pub fn new() -> Self {
        Self {
            max_file_size: 1024 * 1024, // 1 MB max by default
        }
    }

    pub fn with_max_file_size(max_file_size: u64) -> Self {
        Self { max_file_size }
    }

    fn should_scan_file(&self, path: &std::path::Path, size: u64) -> bool {
        // Skip binary files and large files
        if size > self.max_file_size {
            return false;
        }

        // Only scan text-like files
        let ext = path::extension(path);
        matches!(
            ext.as_deref(),
            Some("js") | Some("ts") | Some("py") | Some("rb") | Some("go") |
            Some("java") | Some("c") | Some("cpp") | Some("h") | Some("hpp") |
            Some("rs") | Some("sh") | Some("bash") | Some("zsh") |
            Some("yml") | Some("yaml") | Some("json") | Some("toml") |
            Some("env") | Some("txt") | Some("md") | Some("config") |
            Some("cfg") | Some("conf") | Some("properties")
        )
    }

    fn scan_content(&self, content: &str) -> Vec<(String, usize)> {
        let mut findings = Vec::new();

        for (secret_type, pattern) in SECRET_PATTERNS.iter() {
            for (line_num, line) in content.lines().enumerate() {
                if pattern.is_match(line) {
                    findings.push((secret_type.clone(), line_num + 1));
                }
            }
        }

        findings
    }
}

impl Default for SecretScannerPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for SecretScannerPlugin {
    fn metadata() -> PluginMetadata {
        PluginMetadata {
            name: "secret-scanner".to_string(),
            version: "0.1.0".to_string(),
            description: "Scans for exposed secrets and API keys in source files".to_string(),
            author: Some("FSLint Contributors".to_string()),
            enabled_by_default: false,
        }
    }

    fn check(&self, context: &PluginContext) -> Result<PluginResult, PluginError> {
        let file_size = context.metadata.len();

        if !self.should_scan_file(&context.path, file_size) {
            return Ok(PluginResult::skipped("secret-scanner"));
        }

        // Read file content
        let content = fs::read_to_string(&context.path)
            .map_err(|e| PluginError::Execution(format!("Failed to read file: {}", e)))?;

        // Scan for secrets
        let findings = self.scan_content(&content);

        if findings.is_empty() {
            Ok(PluginResult::inactive("secret-scanner"))
        } else {
            let count = findings.len();
            let types: Vec<String> = findings.iter()
                .map(|(t, _)| t.clone())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();

            let mut result = PluginResult {
                plugin_name: "secret-scanner".to_string(),
                status: PluginStatus::Error,
                message: Some(format!("⚠️  {} secret(s) found: {}", count, types.join(", "))),
                color: Some("red".to_string()),
                tags: vec!["security".to_string(), "secrets".to_string()],
                metadata: HashMap::new(),
            };

            result.metadata.insert("secret_count".to_string(), count.to_string());
            result.metadata.insert("secret_types".to_string(), types.join(", "));

            let lines: Vec<String> = findings.iter().map(|(_, line)| line.to_string()).collect();
            result.metadata.insert("lines".to_string(), lines.join(", "));

            Ok(result)
        }
    }

    fn initialize(&mut self, config: &HashMap<String, String>) -> Result<(), PluginError> {
        if let Some(max_size) = config.get("max_file_size") {
            self.max_file_size = max_size.parse()
                .map_err(|e| PluginError::Config(format!("Invalid max_file_size: {}", e)))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_metadata() {
        let metadata = SecretScannerPlugin::metadata();
        assert_eq!(metadata.name, "secret-scanner");
        assert!(!metadata.enabled_by_default);
    }

    #[test]
    fn test_scan_content() {
        let plugin = SecretScannerPlugin::new();
        let content = r#"
const api_key = "AKIAIOSFODNN7EXAMPLE";
const password = "super_secret_password_12345";
        "#;

        let findings = plugin.scan_content(content);
        assert!(!findings.is_empty());
    }

    #[test]
    fn test_should_scan_file() {
        let plugin = SecretScannerPlugin::new();
        assert!(plugin.should_scan_file(std::path::Path::new("test.js"), 1000));
        assert!(plugin.should_scan_file(std::path::Path::new("config.yml"), 1000));
        assert!(!plugin.should_scan_file(std::path::Path::new("image.png"), 1000));
        assert!(!plugin.should_scan_file(std::path::Path::new("test.js"), 2 * 1024 * 1024));
    }
}
