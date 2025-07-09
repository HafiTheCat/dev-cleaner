pub mod config;
pub mod filters;
pub mod utils;

#[macro_use]
pub mod macros;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &str =  "DevCleaner";