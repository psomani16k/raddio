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
            default: Some(String::from("Hello World!")),
            override_ui: Some(UiConfig {
                max_height: Some(5),
                max_width: Some(20),
                rounded_corners: Some(false),
                border: Some(true),
                border_color: Some(String::from("#33FF33")),
                prefix: Some(String::from(" => ")),
                prefix_color: Some(String::from("#33FFFF")),
                multiline: Some(true),
            }),
        });
        return conf;
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
