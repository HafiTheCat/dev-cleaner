use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Filters(Vec<String>);

impl Filters {
    pub fn new(values: Vec<String>) -> Filters {
        Self(values)
    }
}

impl Deref for Filters {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Filters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::iter::FromIterator<std::string::String> for Filters {
    fn from_iter<T: IntoIterator<Item = std::string::String>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl std::iter::IntoIterator for Filters {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a Filters {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

use owo_colors::OwoColorize;
impl Display for Filters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.is_empty() {
            true => writeln!(f, "\t{}", "(no filters specified)".to_string().yellow())?,
            false => self
                .0
                .iter()
                .try_for_each(|entry| writeln!(f, "\t- {entry}"))?,
        }

        writeln!(
            f,
            "\t- {}",
            "To edit use: {add | remove} <value>".to_string().cyan()
        )?;
        Ok(())
    }
}
