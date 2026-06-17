use serde::{Deserialize, Serialize};

use super::UiConfig;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Station {
    pub name: String,
    pub description: String,
    pub run: Vec<String>,
    pub default: Option<String>,
    pub override_ui: Option<UiConfig>,
}

impl Station {
    pub fn execute(&self, input: &str) -> anyhow::Result<()> {
        let args: Vec<&str> = self
            .run
            .iter()
            .map(|s| if s == "{}" { input } else { s.as_str() })
            .collect();
        std::process::Command::new(&args[0])
            .args(&args[1..])
            .status()?;
        Ok(())
    }
}
