use std::fs;
use crate::types::SystemError;
use super::AppConfig;

pub fn load_config(path: &str) -> Result<AppConfig, SystemError> {
    let raw = fs::read_to_string(path)
        .map_err(|e| SystemError::fatal(e.to_string()))?;

    let config: AppConfig = toml::from_str(&raw)
        .map_err(|e| SystemError::fatal(e.to_string()))?;

    validate(&config)?;

    Ok(config)
}

fn validate(cfg: &AppConfig) -> Result<(), SystemError> {
    if cfg.symbols.is_empty() {
        return Err(SystemError::fatal("No symbols configured"));
    }

    if cfg.tick_interval_secs == 0 {
        return Err(SystemError::fatal("tick_interval_secs must be > 0"));
    }

    Ok(())
}
