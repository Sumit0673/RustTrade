use std::fmt;

#[derive(Debug)]
pub enum SystemError {
    Transient {
        message: String,
    },

    Fatal {
        message: String,
    },

    PolicyViolation {
        message: String,
    },
}

impl SystemError {
    pub fn transient(msg: impl Into<String>) -> Self {
        Self::Transient { message: msg.into() }
    }

    pub fn fatal(msg: impl Into<String>) -> Self {
        Self::Fatal { message: msg.into() }
    }

    pub fn policy(msg: impl Into<String>) -> Self {
        Self::PolicyViolation { message: msg.into() }
    }
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SystemError::Transient { message } => write!(f, "Transient error: {}", message),
            SystemError::Fatal { message } => write!(f, "Fatal error: {}", message),
            SystemError::PolicyViolation { message } => {
                write!(f, "Policy violation: {}", message)
            }
        }
    }
}
