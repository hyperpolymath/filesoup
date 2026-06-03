use anyhow::{Context, Result};
use fslint_core::{Config, PluginLoader, Scanner};
use fslint_plugin_api::Plugin;
use std::path::PathBuf;

use crate::output::{OutputFormat, OutputFormatter};
use crate::query::Query;

pub fn scan(path: PathBuf, format: OutputFormat, query_str: Option<String>) -> Result<()> {
    // Load configuration
    let config = Config::load().context("Failed to load configuration")?;

    // Create plugin loader and register all plugins
    let mut plugin_loader = create_plugin_loader();

    // Set enabled plugins from config
    plugin_loader.set_enabled(config.enabled_plugins.clone());

    // Initialize plugins with config
    plugin_loader.initialize_all(&config.plugin_config)?;

    // Create scanner
    let mut scanner = Scanner::new(config.scanner, plugin_loader);

    // Scan directory
    let files = scanner.scan(&path)
        .with_context(|| format!("Failed to scan directory: {:?}", path))?;

    // Apply query if provided
    let filtered_files = if let Some(query_str) = query_str {
        let query = Query::parse(&query_str)
            .map_err(|e| anyhow::anyhow!("Failed to parse query: {}", e))?;
        query.apply(files)
    } else {
        files
    };

    // Format and display output
    OutputFormatter::format(&filtered_files, format, &path);

    // Print cache stats
    let (hits, misses) = scanner.cache_stats();
    if hits + misses > 0 {
        eprintln!("\nCache: {} hits, {} misses ({:.1}% hit rate)",
            hits, misses, (hits as f64 / (hits + misses) as f64) * 100.0);
    }

    Ok(())
}

pub fn list_plugins() -> Result<()> {
    let plugin_loader = create_plugin_loader();
    let config = Config::load().unwrap_or_default();

    println!("{:<25} {:<10} {:<50}", "Plugin", "Status", "Description");
    println!("{}", "-".repeat(90));

    for plugin_name in plugin_loader.list_plugins() {
        let enabled = config.is_plugin_enabled(&plugin_name);
        let status = if enabled { "enabled" } else { "disabled" };

        // Get plugin description (we'll need to pass metadata)
        let description = get_plugin_description(&plugin_name);

        println!("{:<25} {:<10} {:<50}", plugin_name, status, description);
    }

    Ok(())
}

pub fn enable_plugin(name: String) -> Result<()> {
    let mut config = Config::load().unwrap_or_default();
    config.enable_plugin(&name);
    config.save().context("Failed to save configuration")?;
    println!("Enabled plugin: {}", name);
    Ok(())
}

pub fn disable_plugin(name: String) -> Result<()> {
    let mut config = Config::load().unwrap_or_default();
    config.disable_plugin(&name);
    config.save().context("Failed to save configuration")?;
    println!("Disabled plugin: {}", name);
    Ok(())
}

pub fn show_config() -> Result<()> {
    let config = Config::load().unwrap_or_default();
    let config_path = Config::config_path()?;

    println!("Configuration file: {:?}", config_path);
    println!("\nEnabled plugins:");
    for plugin in &config.enabled_plugins {
        println!("  - {}", plugin);
    }

    println!("\nScanner configuration:");
    println!("  Max depth: {:?}", config.scanner.max_depth);
    println!("  Include hidden: {}", config.scanner.include_hidden);
    println!("  Follow symlinks: {}", config.scanner.follow_symlinks);
    println!("  Respect .gitignore: {}", config.scanner.respect_gitignore);

    Ok(())
}

fn create_plugin_loader() -> PluginLoader {
    let mut loader = PluginLoader::new();

    // Register all plugins
    loader.register(
        fslint_plugin_git_status::GitStatusPlugin::new(),
        fslint_plugin_git_status::GitStatusPlugin::metadata()
    );

    loader.register(
        fslint_plugin_file_age::FileAgePlugin::new(),
        fslint_plugin_file_age::FileAgePlugin::metadata()
    );

    loader.register(
        fslint_plugin_grouping::GroupingPlugin::new(),
        fslint_plugin_grouping::GroupingPlugin::metadata()
    );

    loader.register(
        fslint_plugin_version_detection::VersionDetectionPlugin::new(),
        fslint_plugin_version_detection::VersionDetectionPlugin::metadata()
    );

    loader.register(
        fslint_plugin_ocr_status::OcrStatusPlugin::new(),
        fslint_plugin_ocr_status::OcrStatusPlugin::metadata()
    );

    loader.register(
        fslint_plugin_ai_detection::AiDetectionPlugin::new(),
        fslint_plugin_ai_detection::AiDetectionPlugin::metadata()
    );

    loader.register(
        fslint_plugin_duplicate_finder::DuplicateFinderPlugin::new(),
        fslint_plugin_duplicate_finder::DuplicateFinderPlugin::metadata()
    );

    loader.register(
        fslint_plugin_secret_scanner::SecretScannerPlugin::new(),
        fslint_plugin_secret_scanner::SecretScannerPlugin::metadata()
    );

    loader
}

fn get_plugin_description(name: &str) -> String {
    match name {
        "git-status" => fslint_plugin_git_status::GitStatusPlugin::metadata().description,
        "file-age" => fslint_plugin_file_age::FileAgePlugin::metadata().description,
        "grouping" => fslint_plugin_grouping::GroupingPlugin::metadata().description,
        "version-detection" => fslint_plugin_version_detection::VersionDetectionPlugin::metadata().description,
        "ocr-status" => fslint_plugin_ocr_status::OcrStatusPlugin::metadata().description,
        "ai-detection" => fslint_plugin_ai_detection::AiDetectionPlugin::metadata().description,
        "duplicate-finder" => fslint_plugin_duplicate_finder::DuplicateFinderPlugin::metadata().description,
        "secret-scanner" => fslint_plugin_secret_scanner::SecretScannerPlugin::metadata().description,
        _ => "Unknown plugin".to_string(),
    }
}
