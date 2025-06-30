use dev_cleaner_cli::{Commands, DevCleanerCli};
use dev_cleaner_gui::DevCleanerGui;
use log::info;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = DevCleanerCli::parse_args();

    setup_logger(cli.log_level)?;

    cli.process()?;

    if cli.gui {
        DevCleanerGui::new(cli.path).run()?
    }

    Ok(())
}

/// Initialize the logger
fn setup_logger(log_level: log::LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
        .filter_module(module_path!(), log_level)
        .format_timestamp(None)
        .init();

    log::debug!("Logger initialized with level: {log_level}");
    Ok(())
}
