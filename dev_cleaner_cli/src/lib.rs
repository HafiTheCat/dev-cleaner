use clap::{Parser, Subcommand};
use log::{LevelFilter, info};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct DevCleanerCli {
    /// Optional path argument
    #[arg(value_hint = clap::ValueHint::DirPath)]
    #[arg(short, long, default_value = None)]
    pub path: Option<PathBuf>,

    /// Set log level (e.g., DEBUG, INFO, WARN, ERROR)
    #[arg(short, long, value_enum, default_value = "info")]
    pub log_level: LevelFilter,

    /// Run in GUI mode
    #[arg(long, default_value = "false")]
    pub gui: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add a folder pattern (e.g., node_modules)
    Add {
        /// Folder pattern to add
        pattern: String,
    },
    /// Remove a folder pattern
    Remove {
        /// Folder pattern to remove
        pattern: String,
    },
    /// List all folder patterns
    List,
}

impl DevCleanerCli {
    /// Parse command line arguments
    pub fn parse_args() -> DevCleanerCli {
        DevCleanerCli::parse()
    }
    /// Process the command
    pub fn process(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.command {
            Some(Commands::Add { pattern }) => {
                info!("Adding pattern: {pattern}");
            }
            Some(Commands::Remove { pattern }) => {
                info!("Removing pattern: {pattern}");
            }
            Some(Commands::List) => {
                info!("Listing patterns");
            }
            None => {
                if let Some(path) = &self.path {
                    info!("Running with path: {}", path.display());
                } else {
                    info!("No subcommand and no path provided.");
                }
            }
        }
        Ok(())
    }
}
