use colored::*;
use fslint_core::ScannedFile;
use fslint_plugin_api::{PluginResult, PluginStatus};

pub enum OutputFormat {
    Table,
    Json,
    Simple,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "table" => Ok(Self::Table),
            "json" => Ok(Self::Json),
            "simple" => Ok(Self::Simple),
            _ => Err(format!("Unknown output format: {}", s)),
        }
    }
}

pub struct OutputFormatter;

impl OutputFormatter {
    pub fn format(files: &[ScannedFile], format: OutputFormat, working_dir: &std::path::Path) {
        match format {
            OutputFormat::Table => Self::format_table(files, working_dir),
            OutputFormat::Json => Self::format_json(files),
            OutputFormat::Simple => Self::format_simple(files, working_dir),
        }
    }

    fn format_table(files: &[ScannedFile], working_dir: &std::path::Path) {
        if files.is_empty() {
            println!("{}", "No files found.".dimmed());
            return;
        }

        // Print header
        println!(
            "{:<50} {:<15} {:<15} {:<20} {:<15}",
            "File".bold(),
            "Git".bold(),
            "Age".bold(),
            "Group".bold(),
            "Other".bold()
        );
        println!("{}", "-".repeat(120));

        // Print each file
        for file in files {
            let relative_path = file.path
                .strip_prefix(working_dir)
                .unwrap_or(&file.path)
                .to_string_lossy();

            let git_status = Self::get_result_message(&file.results, "git-status");
            let age_status = Self::get_result_message(&file.results, "file-age");
            let group_status = Self::get_result_message(&file.results, "grouping");
            let other_status = Self::get_other_results(&file.results);

            let git_colored = Self::colorize(&git_status, &file.results, "git-status");
            let age_colored = Self::colorize(&age_status, &file.results, "file-age");
            let group_colored = Self::colorize(&group_status, &file.results, "grouping");

            println!(
                "{:<50} {:<15} {:<15} {:<20} {:<15}",
                relative_path,
                git_colored,
                age_colored,
                group_colored,
                other_status
            );
        }

        println!("\n{} files scanned", files.len());
    }

    fn format_json(files: &[ScannedFile]) {
        let output: Vec<serde_json::Value> = files
            .iter()
            .map(|file| {
                serde_json::json!({
                    "path": file.path,
                    "size": file.metadata.len(),
                    "results": file.results,
                })
            })
            .collect();

        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }

    fn format_simple(files: &[ScannedFile], working_dir: &std::path::Path) {
        for file in files {
            let relative_path = file.path
                .strip_prefix(working_dir)
                .unwrap_or(&file.path)
                .to_string_lossy();

            print!("{}", relative_path);

            // Print active results
            let active_results: Vec<&PluginResult> = file.results
                .iter()
                .filter(|r| r.status != PluginStatus::Inactive && r.status != PluginStatus::Skipped)
                .collect();

            if !active_results.is_empty() {
                print!(" [");
                for (i, result) in active_results.iter().enumerate() {
                    if i > 0 {
                        print!(", ");
                    }
                    if let Some(msg) = &result.message {
                        print!("{}", msg);
                    }
                }
                print!("]");
            }

            println!();
        }
    }

    fn get_result_message(results: &[PluginResult], plugin_name: &str) -> String {
        results
            .iter()
            .find(|r| r.plugin_name == plugin_name)
            .and_then(|r| r.message.clone())
            .unwrap_or_else(|| "-".to_string())
    }

    fn get_other_results(results: &[PluginResult]) -> String {
        let other_plugins = ["version-detection", "ocr-status", "ai-detection", "duplicate-finder", "secret-scanner"];

        let active: Vec<String> = results
            .iter()
            .filter(|r| other_plugins.contains(&r.plugin_name.as_str()))
            .filter(|r| r.status != PluginStatus::Inactive && r.status != PluginStatus::Skipped)
            .filter_map(|r| r.message.clone())
            .collect();

        if active.is_empty() {
            "-".to_string()
        } else {
            active.join(", ")
        }
    }

    fn colorize(text: &str, results: &[PluginResult], plugin_name: &str) -> String {
        if text == "-" {
            return text.dimmed().to_string();
        }

        if let Some(result) = results.iter().find(|r| r.plugin_name == plugin_name) {
            if let Some(color_name) = &result.color {
                return Self::apply_color(text, color_name);
            }
        }

        text.to_string()
    }

    fn apply_color(text: &str, color_name: &str) -> String {
        match color_name {
            "red" => text.red().to_string(),
            "green" | "bright_green" => text.green().to_string(),
            "yellow" => text.yellow().to_string(),
            "blue" => text.blue().to_string(),
            "magenta" => text.magenta().to_string(),
            "cyan" => text.cyan().to_string(),
            "gray" => text.dimmed().to_string(),
            _ => text.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_from_str() {
        assert!(matches!(OutputFormat::from_str("table"), Ok(OutputFormat::Table)));
        assert!(matches!(OutputFormat::from_str("json"), Ok(OutputFormat::Json)));
        assert!(matches!(OutputFormat::from_str("simple"), Ok(OutputFormat::Simple)));
        assert!(OutputFormat::from_str("invalid").is_err());
    }
}
