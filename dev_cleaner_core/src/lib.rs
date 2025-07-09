pub mod config;
pub mod filters;
pub mod utils;
pub mod folderscan;

#[macro_use]
pub mod macros;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &str =  "DevCleaner";