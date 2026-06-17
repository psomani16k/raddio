mod config;
mod station;
mod ui_config;

pub use config::Config;
pub use station::Station;
pub use ui_config::UiConfig;

use std::{path::PathBuf, sync::LazyLock};

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let path = config_path();
    if !path.exists() {
        let default = Config::default();
        // Written as pretty JSON, which is valid JSON5; users can then edit it
        // with comments, trailing commas, and unquoted keys.
        let content = serde_json::to_string_pretty(&default)
            .unwrap_or_else(|e| panic!("failed to serialize default config: {}", e));
        std::fs::create_dir_all(path.parent().unwrap())
            .unwrap_or_else(|e| panic!("failed to create config dir: {}", e));
        std::fs::write(&path, content)
            .unwrap_or_else(|e| panic!("failed to write default config to {}: {}", path.display(), e));
        return default;
    }
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read config at {}: {}", path.display(), e));
    json5::from_str(&content).unwrap_or_else(|e| panic!("failed to parse config: {}", e))
});

fn config_path() -> PathBuf {
    let config_dir = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").expect("HOME not set");
            PathBuf::from(home).join(".config")
        });
    config_dir.join("raddio").join("config.json")
}
