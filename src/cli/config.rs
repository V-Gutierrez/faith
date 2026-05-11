//! Configuration management for Faith CLI.
//!
//! Provides persistent user preferences via `~/.faith/config.toml`.

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_config_version")]
    pub config_version: String,
    #[serde(default)]
    pub preferences: Preferences,
    #[serde(default)]
    pub search: SearchPrefs,
}

fn default_config_version() -> String {
    "1.0".to_string()
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Preferences {
    pub lang: Option<String>,
    pub translation: Option<String>,
    pub format: Option<String>,
    pub seed: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchPrefs {
    pub limit: Option<usize>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config_version: "1.0".to_string(),
            preferences: Preferences::default(),
            search: SearchPrefs::default(),
        }
    }
}

/// Load config from `~/.faith/config.toml`, returning defaults if file doesn't exist.
pub fn load_config() -> Result<Config> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(Config::default());
    }
    let contents = std::fs::read_to_string(&path)
        .map_err(|e| crate::error::FaithError::Io(format!("Failed to read config: {}", e)))?;

    match toml::from_str::<Config>(&contents) {
        Ok(cfg) => Ok(cfg),
        Err(e) => {
            eprintln!("⚠️  Config parse error: {}, using defaults", e);
            Ok(Config::default())
        }
    }
}

/// Save config to `~/.faith/config.toml`.
pub fn save_config(cfg: &Config) -> Result<()> {
    let path = config_path()?;
    let toml_str = toml::to_string_pretty(cfg)
        .map_err(|e| crate::error::FaithError::Io(format!("Failed to serialize config: {}", e)))?;
    std::fs::write(&path, toml_str)
        .map_err(|e| crate::error::FaithError::Io(format!("Failed to write config: {}", e)))?;
    Ok(())
}

/// Returns the path to the config file: `~/.faith/config.toml`.
pub fn config_path() -> Result<PathBuf> {
    let dir = crate::store::data_dir()?;
    Ok(dir.join("config.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_version() {
        let cfg = Config::default();
        assert_eq!(cfg.config_version, "1.0");
    }

    #[test]
    fn can_serialize_and_deserialize() {
        let cfg = Config {
            config_version: "1.0".to_string(),
            preferences: Preferences {
                lang: Some("pt".to_string()),
                translation: Some("ONBV".to_string()),
                format: None,
                seed: Some(42),
            },
            search: SearchPrefs { limit: Some(20) },
        };

        let toml_str = toml::to_string_pretty(&cfg).unwrap();
        let deserialized: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(deserialized.config_version, "1.0");
        assert_eq!(deserialized.preferences.lang, Some("pt".to_string()));
        assert_eq!(deserialized.preferences.translation, Some("ONBV".to_string()));
        assert_eq!(deserialized.preferences.seed, Some(42));
        assert_eq!(deserialized.search.limit, Some(20));
    }
}
