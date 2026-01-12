use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub symbols: Vec<String>,
    pub tick_interval_secs: u64,
    pub max_consecutive_failures: u32,
}
