#[derive(Debug, Clone, PartialEq)]
pub enum FailureAction {
    Ignore,     // log & continue
    Retry,      // retry tick
    Pause,      // pause lifecycle
    Lock,       // lock execution
    Kill,       // kill system
}
