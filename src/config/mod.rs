use serde::{Deserialize, Serialize};

mod station;
mod ui_config;
mod utils;

pub use station::Station;
pub use ui_config::UiConfig;
pub use utils::{CONFIG, init, parse_hex_color};

use crate::config::ui_config::CursorStyle;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub stations: Vec<Station>,
    pub ui: Option<UiConfig>,
}

impl Config {
    pub fn get_ui(&self) -> UiConfig {
        match &self.ui {
            Some(ui) => ui.clone().override_with(&UiConfig::default()),
            None => UiConfig::default(),
        }
    }

    pub fn init_config() -> Self {
        let mut conf = Self::default();
        conf.stations.push(Station {
            name: String::from("echo"),
            description: String::from("Echo"),
            run: vec![String::from("echo"), String::from("{}")],
            override_ui: Some(UiConfig {
                max_height: Some(10),
                max_width: Some(70),
                rounded_corners: Some(true),
                border: Some(true),
                border_color: Some(String::from("#0000FF")),
                prefix: Some(String::from(" => ")),
                prefix_color: Some(String::from("#00FF00")),
                multiline: Some(true),
                cursor_style: Some(CursorStyle::Block),
                cursor_color: Some(String::from("#FFFFFF")),
            }),
        });
        conf
    }

    pub fn list_station_by_names(&self) -> Vec<&str> {
        let mut result: Vec<&str> = vec![];
        self.stations.iter().for_each(|s| {
            result.push(&s.name);
        });
        result
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            stations: vec![],
            ui: Some(UiConfig::default()),
        }
    }
}
