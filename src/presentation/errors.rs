use serde::{Serialize, ser::SerializeStruct};

use crate::domain::errors::DomainError;

pub enum PresentationError {
    InvalidInput(String),
    NotFound(String),
    Internal(String),
}

impl std::fmt::Display for PresentationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PresentationError::InvalidInput(msg) => {
                write!(f, "{msg}")
            }
            Self::NotFound(msg) => {
                write!(f, "{msg}")
            }
            Self::Internal(msg) => {
                write!(f, "{msg}")
            }
        }
    }
}

impl Serialize for PresentationError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let message = match self {
            Self::InvalidInput(msg) => msg,
            Self::NotFound(msg) => msg,
            Self::Internal(msg) => msg,
        };

        let mut state = serializer.serialize_struct("PresentationError", 1)?;
        state.serialize_field("error", message)?;
        state.end()
    }
}

impl From<DomainError> for PresentationError {
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::Unknown(msg) => Self::Internal(msg),
        }
    }
}
