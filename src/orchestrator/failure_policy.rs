use crate::types::{SystemError, RuntimeState};
use crate::types::failure::FailureAction;

pub struct FailurePolicy;

impl FailurePolicy {
    pub fn decide(
        error: &SystemError,
        runtime: &RuntimeState,
    ) -> FailureAction {
        match error {
            SystemError::Transient { .. } => {
                if runtime.consecutive_failures >= 3 {
                    FailureAction::Pause
                } else {
                    FailureAction::Retry
                }
            }

            SystemError::PolicyViolation { .. } => {
                FailureAction::Pause
            }

            SystemError::Fatal { .. } => {
                FailureAction::Kill
            }
        }
    }
}
