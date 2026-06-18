use ratatui::style::Color;

use crate::config::Config;
use std::{path::PathBuf, sync::LazyLock};

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let path = config_path();
    if !path.exists() {
        let default = Config::default();
        write_config(&path, &default)
            .unwrap_or_else(|e| panic!("failed to write default config: {}", e));
        return default;
    }
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read config at {}: {}", path.display(), e));
    json5::from_str(&content).unwrap_or_else(|e| panic!("failed to parse config: {}", e))
});

/// Writes the given config to `path`, creating parent directories as needed.
/// Serialized as pretty JSON (valid JSON5), so it can then be hand-edited with
/// comments, trailing commas, and unquoted keys.
fn write_config(path: &PathBuf, config: &Config) -> anyhow::Result<()> {
    let content = serde_json::to_string_pretty(config)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content)?;
    Ok(())
}

/// Handles `raddio init`: writes a default config, refusing to overwrite an
/// existing one. Does not read or parse `CONFIG`, so it works before any valid
/// config exists.
pub fn init() -> anyhow::Result<()> {
    let path = config_path();
    if path.exists() {
        println!("config already exists at {}", path.display());
        return Ok(());
    }
    write_config(&path, &Config::init_config())?;
    println!("created default config at {}", path.display());
    Ok(())
}

pub fn parse_hex_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
    Color::Rgb(r, g, b)
}

fn config_path() -> PathBuf {
    let config_dir = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").expect("HOME not set");
            PathBuf::from(home).join(".config")
        });
    config_dir.join("raddio").join("config.json5")
}

