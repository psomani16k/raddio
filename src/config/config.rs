use serde::{Deserialize, Serialize};

use super::{Station, UiConfig};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub stations: Vec<Station>,
    pub ui: Option<UiConfig>,
}

impl Config {
    pub fn get_ui(&self) -> UiConfig {
        match &self.ui {
            Some(ui) => ui.clone(),
            None => UiConfig::default(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        return Self {
            stations: vec![],
            ui: Some(UiConfig::default()),
        };
    }
}
