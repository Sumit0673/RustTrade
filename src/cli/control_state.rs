use std::time::SystemTime;

#[derive(Debug, Clone)]
pub enum LifecycleState {
    Stopped,
    Running,
    Paused,
}
impl LifecycleState {
    pub fn is_running(&self) -> bool {
        matches!(self, LifecycleState::Running)
    }
}

#[derive(Debug, Clone)]
pub enum Mode {
    Info,
    DryRun,
    Live,
}

#[derive(Debug, Clone)]
pub enum ExecutionLock {
    Locked,
    Unlocked,
}
impl ExecutionLock {
    pub fn is_locked(&self) -> bool {
        matches!(self, ExecutionLock::Locked)
    }
    pub fn lock(&mut self) {
        *self = ExecutionLock::Locked;
    }
}


#[derive(Debug, Clone)]
pub enum DecisionOutcome {
    Accepted,
    Rejected,
    Failed,
}

#[derive(Debug, Clone)]
pub struct DecisionLog {
    pub symbol: String,
    pub action: String,
    pub confidence: f32,
    pub outcome: DecisionOutcome,
    pub reason: String,
    pub timestamp: std::time::SystemTime,
}

#[derive(Debug, Clone)]
pub struct ControlState {
    pub lifecycle: LifecycleState,
    pub mode: Mode,
    pub execution_lock: ExecutionLock,
    pub killed: bool,
    pub last_transition: SystemTime,
    pub last_decisions: Vec<DecisionLog>,
}
