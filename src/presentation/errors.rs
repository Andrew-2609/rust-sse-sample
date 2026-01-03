use serde::{Serialize, ser::SerializeStruct};

pub enum PresentationError {
    InvalidInput(String),
    NotFound(String),
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
        }
    }
}

impl Serialize for PresentationError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let message = match self {
            PresentationError::InvalidInput(msg) => msg,
            Self::NotFound(msg) => msg,
        };

        let mut state = serializer.serialize_struct("PresentationError", 1)?;
        state.serialize_field("error", message)?;
        state.end()
    }
}
