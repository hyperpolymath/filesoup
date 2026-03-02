mod commands;
mod output;
mod query;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::output::OutputFormat;

#[derive(Parser)]
#[command(name = "fslint")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a directory and show file intelligence
    Scan {
        /// Path to scan
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Output format (table, json, simple)
        #[arg(short, long, default_value = "table")]
        format: String,

        /// Query filter (e.g., "name:test ext:txt")
        #[arg(short, long)]
        query: Option<String>,
    },

    /// List all available plugins
    Plugins,

    /// Enable a plugin
    Enable {
        /// Plugin name
        name: String,
    },

    /// Disable a plugin
    Disable {
        /// Plugin name
        name: String,
    },

    /// Show configuration
    Config,

    /// Run a query on files
    Query {
        /// Query string (e.g., "name:myfile ext:tiff newest:true")
        query: String,

        /// Path to search
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Output format
        #[arg(short, long, default_value = "simple")]
        format: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path, format, query } => {
            let output_format = OutputFormat::from_str(&format)
                .map_err(|e| anyhow::anyhow!(e))?;
            commands::scan(path, output_format, query)?;
        }
        Commands::Plugins => {
            commands::list_plugins()?;
        }
        Commands::Enable { name } => {
            commands::enable_plugin(name)?;
        }
        Commands::Disable { name } => {
            commands::disable_plugin(name)?;
        }
        Commands::Config => {
            commands::show_config()?;
        }
        Commands::Query { query, path, format } => {
            let output_format = OutputFormat::from_str(&format)
                .map_err(|e| anyhow::anyhow!(e))?;
            commands::scan(path, output_format, Some(query))?;
        }
    }

    Ok(())
}
