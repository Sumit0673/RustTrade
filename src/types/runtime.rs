use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct RuntimeState {
    pub last_tick: Option<SystemTime>,
    pub last_success: Option<SystemTime>,
    pub consecutive_failures: u32,
    pub last_error: Option<String>,
}

impl RuntimeState {
    pub fn new() -> Self {
        Self {
            last_tick: None,
            last_success: None,
            consecutive_failures: 0,
            last_error: None,
        }
    }
}

impl RuntimeState {
    pub fn record_success(&mut self) {
        let now = SystemTime::now();
        self.last_tick = Some(now);
        self.last_success = Some(now);
        self.consecutive_failures = 0;
        self.last_error = None;
    }

    pub fn record_failure(&mut self, error: &str) {
        self.last_tick = Some(SystemTime::now());
        self.consecutive_failures += 1;
        self.last_error = Some(error.to_string());
    }
}
