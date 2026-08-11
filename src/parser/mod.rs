pub mod dotenv;
pub mod ini;
pub mod json;
pub mod system;
pub mod toml;
pub mod yaml;

use crate::types::{ConfigSource, EnvEntry, MaskMode};
use regex::Regex;
use std::path::Path;

pub use dotenv::parse_dotenv;
pub use ini::parse_ini;
pub use json::parse_json;
pub use system::read_system_env;
pub use toml::parse_toml;
pub use yaml::parse_yaml;

/// Auto-detect config source type based on filename extension or path
pub fn detect_config_source(path_str: &str) -> ConfigSource {
    let p = Path::new(path_str);
    if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
        match ext.to_lowercase().as_str() {
            "json" => return ConfigSource::Json,
            "yaml" | "yml" => return ConfigSource::Yaml,
            "toml" => return ConfigSource::Toml,
            "ini" | "properties" | "cfg" | "conf" => return ConfigSource::Ini,
            "env" => return ConfigSource::Dotenv,
            _ => {}
        }
    }
    if p.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.starts_with(".env"))
        .unwrap_or(false)
    {
        return ConfigSource::Dotenv;
    }
    ConfigSource::Dotenv
}

/// Load and parse environment configuration from file or system
pub fn load_config(
    path: Option<&str>,
    use_system_env: bool,
    mask_mode: MaskMode,
    custom_pattern: Option<&Regex>,
) -> anyhow::Result<(Vec<EnvEntry>, ConfigSource)> {
    if use_system_env || path.is_none() {
        return Ok((read_system_env(mask_mode, custom_pattern), ConfigSource::System));
    }

    let file_path = path.unwrap();
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| anyhow::anyhow!("Failed to read file '{}': {}", file_path, e))?;

    let source = detect_config_source(file_path);

    let entries = match source {
        ConfigSource::Json => match parse_json(&content, mask_mode, custom_pattern) {
            Ok(e) => e,
            Err(_) => parse_dotenv(&content, mask_mode, custom_pattern),
        },
        ConfigSource::Yaml => match parse_yaml(&content, mask_mode, custom_pattern) {
            Ok(e) => e,
            Err(_) => parse_dotenv(&content, mask_mode, custom_pattern),
        },
        ConfigSource::Toml => match parse_toml(&content, mask_mode, custom_pattern) {
            Ok(e) => e,
            Err(_) => parse_dotenv(&content, mask_mode, custom_pattern),
        },
        ConfigSource::Ini => match parse_ini(&content, mask_mode, custom_pattern) {
            Ok(e) => e,
            Err(_) => parse_dotenv(&content, mask_mode, custom_pattern),
        },
        ConfigSource::Dotenv => parse_dotenv(&content, mask_mode, custom_pattern),
        ConfigSource::System => read_system_env(mask_mode, custom_pattern),
    };

    Ok((entries, source))
}
