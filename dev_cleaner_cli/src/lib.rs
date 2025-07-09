use clap::{Parser, Subcommand};
use log::{LevelFilter, debug};
use owo_colors::OwoColorize;
use std::path::PathBuf;

use dev_cleaner_core::{
    config::{self, Config},
    folderscan::{remove_folders, scan_folders},
};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None,
    args_conflicts_with_subcommands = true,
    subcommand_precedence_over_arg = true
)]
pub struct DevCleanerCli {
    /// Optional path argument
    #[arg(value_hint = clap::ValueHint::DirPath)]
    #[arg(index = 1)]
    pub path: Option<PathBuf>,

    /// Set log level (e.g., DEBUG, INFO, WARN, ERROR)
    #[arg(short, long, value_enum, default_value = "info")]
    pub log_level: LevelFilter,

    /// Run in GUI mode
    #[arg(long, default_value = "false")]
    #[arg(conflicts_with = "path")]
    pub gui: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Manage configuration
    #[command(visible_aliases = ["c", "cfg"])]
    Config {
        #[command(subcommand)]
        command: Option<ConfigCommands>,
    },
}
#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    /// Folder patterns
    #[command(visible_alias = "f")]
    Filters {
        #[command(subcommand)]
        command: Option<FilterCommands>,
    },
    /// List all folder patterns
    #[command(visible_alias = "ls")]
    List,
    /// Reset all folder patterns
    Reset,
}

#[derive(Debug, Subcommand)]
pub enum FilterCommands {
    /// Add a folder pattern
    #[command(visible_alias = "+")]
    Add { pattern: String },
    /// Remove a folder pattern
    #[command(visible_aliases = ["rm", "-"])]
    Remove { pattern: String },
    /// List all folder patterns
    #[command(visible_alias = "ls")]
    List,
    /// Reset all folder patterns
    Reset,
}

impl DevCleanerCli {
    /// Parse command line arguments
    pub fn parse_args() -> DevCleanerCli {
        DevCleanerCli::parse()
    }
    /// Process the command
    pub fn process(&self, config: &mut config::Config) -> Result<(), Box<dyn std::error::Error>> {
        if self.path.is_none() {
            Self::show_about()?;
            return Ok(());
        }

        if let Some(path_buf) = &self.path {
            // FIXME: target dir passing bodge
            let found_folders = scan_folders(
                path_buf,
                config
                    .filters
                    .iter()
                    .map(|f| f.as_str())
                    .collect::<Vec<_>>()
                    .as_slice(),
            );

            let items = found_folders
                .iter()
                .map(|f| format!("{}", f.display()))
                .collect::<Vec<_>>();

            let selected =
                dialoguer::MultiSelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
                    .with_prompt(format!("Found {} folders to clean", found_folders.len()))
                    .items(items.as_slice())
                    .report(true)
                    .interact()?;

            let selected_folders = selected
                .iter()
                .map(|i| found_folders[*i].clone())
                .collect::<Vec<_>>();

            let (removed, errored) = remove_folders(selected_folders);

            println!("Removed {} folders", removed.len());
            for f in removed {
                println!("\t- {}", f.display().green());
            }
            println!("Errored {} folders", errored.len());
            for f in errored {
                println!("\t- {}", f.display().red());
            }
        }

        let _ = match self.command.as_ref() {
            Some(cmd) => cmd.process(config),
            None => Ok(()),
        };
        Ok(())
    }
    fn show_about() -> Result<(), Box<dyn std::error::Error>> {
        let font =
            figlet_rs::FIGfont::from_file("dev_cleaner_cli/resources/fonts/Tubes-Smushed.flf")?;

        println!("\n");

        match font.convert(dev_cleaner_core::APP_NAME) {
            Some(figure) => {
                let fig = format!("{figure}");
                println!("{}", fig.cyan());
            }
            None => {
                println!("{}", dev_cleaner_core::APP_NAME);
            }
        };

        let version = env!("CARGO_PKG_VERSION");
        let author = env!("CARGO_PKG_AUTHORS");
        let repo = env!("CARGO_PKG_REPOSITORY");
        let issues = format!("{repo}/issues");

        println!("Version {version}");
        println!("© 2025 {author}");
        println!("Github: {url}", url = repo.blue().underline());
        println!("\n\n");
        println!("✨ Thank you for using {}!", dev_cleaner_core::APP_NAME);
        println!("✨ Hope you find this app useful!");
        println!("\n\n");
        println!(
            "→ To get started, run the {} command",
            "help".bright_black()
        );
        println!(
            "→ Alternatively, you can run the tool in GUI mode with the {} flag",
            "--gui".bright_black()
        );
        println!("\n\n");
        println!("If you have any questions, suggestions, or feedback,");
        println!("Please file any issues here: ");
        println!("{}", issues.green().underline());
        println!("\n");
        println!("Press [Enter] to continue...");

        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;

        Ok(())
    }
}

impl Commands {
    pub fn process(&self, config: &mut config::Config) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Commands::Config { command } => {
                if let Some(cmd) = command {
                    cmd.process(config)?;
                } else {
                    ConfigCommands::List.process(config)?;
                }
            }
        }
        Ok(())
    }
}

impl ConfigCommands {
    pub fn process(&self, config: &mut config::Config) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            ConfigCommands::Filters { command } => {
                if let Some(cmd) = command {
                    cmd.process(config)?;
                } else {
                    println!("No filter command provided.");
                }
            }
            ConfigCommands::List => {
                println!("{}", &config);
            }
            ConfigCommands::Reset => {
                println!("Do you want to reset the configuration? [y/N]");
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf)?;
                if buf.trim().to_lowercase() == "y" {
                    *config = Config::default();
                    config.store()?;
                }
            }
        }
        Ok(())
    }
}

impl FilterCommands {
    pub fn process(&self, config: &mut config::Config) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            FilterCommands::Add { pattern } => {
                debug!("Adding pattern: {pattern}");
                config.filters.push(pattern.clone());
                config.store()?;
            }
            FilterCommands::Remove { pattern } => {
                debug!("Removing pattern: {pattern}");
                config.filters.retain(|p| p != pattern);
                config.store()?;
            }
            FilterCommands::List => {
                debug!("Listing patterns");
                println!("{}", &config);
                return Ok(());
            }
            FilterCommands::Reset => {
                debug!("Resetting patterns");
                println!("Do you want to reset the filter configuration? [y/N]");
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf)?;
                if buf.trim().to_lowercase() == "y" {
                    config.filters = Config::default().filters;
                    config.store()?;
                }
            }
        }
        Ok(())
    }
}
