use serde::{Serialize, ser::SerializeStruct};

use crate::domain::errors::DomainError;

pub enum PresentationError {
    BadRequest(String),
    NotFound(String),
    UnprocessableEntity(String),
}

impl PresentationError {
    fn message(&self) -> &str {
        match self {
            Self::BadRequest(msg) | Self::NotFound(msg) | Self::UnprocessableEntity(msg) => msg,
        }
    }
}

impl std::fmt::Display for PresentationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Serialize for PresentationError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let message = self.message();
        let mut state = serializer.serialize_struct("PresentationError", 1)?;
        state.serialize_field("error", message)?;
        state.end()
    }
}

impl From<DomainError> for PresentationError {
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::BusinessRuleViolation(msg) => Self::UnprocessableEntity(msg),
        }
    }
}
