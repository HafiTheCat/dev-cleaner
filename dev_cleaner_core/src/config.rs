use crate::filters::Filters;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub filters: Filters,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            filters: vec!["target", "node_modules", "dist", "out", "__pycache__"]
                .into_iter()
                .map(String::from)
                .collect(),
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::display_field!(f, "Filters", self.filters);
        Ok(())
    }
}

impl Config {
    const CONFIG_PATH: &'static str = "config";
    pub fn new() -> Config {
        Config::default()
    }

    pub fn load() -> Result<Config, std::io::Error> {
        confy::load("dev_cleaner", Self::CONFIG_PATH)
            .map_err(|e| std::io::Error::other(e.to_string()))
    }

    pub fn store(&self) -> Result<(), std::io::Error> {
        confy::store::<Config>("dev_cleaner", Self::CONFIG_PATH, self.clone())
            .map_err(|e| std::io::Error::other(e.to_string()))
    }
}
