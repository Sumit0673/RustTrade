use crate::cli::LifecycleState;
use super::AppConfig;

pub struct RuntimeConfig {
    config: AppConfig,
    frozen: bool,
}

impl RuntimeConfig {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            frozen: false,
        }
    }

    pub fn freeze(&mut self) {
        self.frozen = true;
    }

    pub fn get(&self) -> &AppConfig {
        &self.config
    }

    pub fn update(
        &mut self,
        new_config: AppConfig,
        lifecycle: &LifecycleState,
    ) -> Result<(), String> {
        if self.frozen || *lifecycle != LifecycleState::Stopped {
            return Err("Config is immutable while system is running".into());
        }

        self.config = new_config;
        Ok(())
    }
}
