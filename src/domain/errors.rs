use std::fmt;

pub enum DomainError {
    Unknown(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::Unknown(msg) => {
                write!(f, "{msg}")
            }
        }
    }
}
