use dev_cleaner_cli::DevCleanerCli;
use dev_cleaner_core::config;
use dev_cleaner_gui::DevCleanerGui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = DevCleanerCli::parse_args();

    setup_logger(cli.log_level)?;

    let mut config = config::Config::load().unwrap_or_default();

    
    if cli.gui {
        DevCleanerGui::new(cli.path.clone()).run()?;
        return Ok(());
    }

    cli.process(&mut config)?;
    

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