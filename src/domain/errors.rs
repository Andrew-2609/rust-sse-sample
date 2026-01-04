use std::{fmt, sync::PoisonError};

pub enum DomainError {
    BusinessRuleViolation(String),
    Unknown(String),
}

impl DomainError {
    fn message(&self) -> &str {
        match self {
            Self::BusinessRuleViolation(msg) | Self::Unknown(msg) => msg,
        }
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl<T> From<PoisonError<T>> for DomainError {
    fn from(value: PoisonError<T>) -> Self {
        Self::Unknown(format!("store is poisoned: {}", value.to_string()))
    }
}
