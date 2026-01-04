use actix_web::{HttpResponse, ResponseError};
use serde::{Serialize, ser::SerializeStruct};

use crate::domain::errors::DomainError;

#[derive(Debug)]
pub enum PresentationError {
    NotFound(String),
    UnprocessableEntity(String),
    DomainValidation(String),
    Empty(String),
    NonNegative(String),
}

impl PresentationError {
    fn message(&self) -> String {
        match self {
            Self::DomainValidation(msg) | Self::NotFound(msg) | Self::UnprocessableEntity(msg) => {
                msg.into()
            }
            Self::Empty(field) => format!("field '{field}' cannot be empty"),
            Self::NonNegative(field) => {
                format!("field '{field}' must be greater than or equal to 0")
            }
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
        state.serialize_field("error", message.as_str())?;
        state.end()
    }
}

impl From<DomainError> for PresentationError {
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::InvalidMetricID(_) | DomainError::InvalidTimestamp(_) => {
                Self::DomainValidation(value.message())
            }
            DomainError::BusinessRuleViolation(_) => Self::UnprocessableEntity(value.message()),
        }
    }
}

impl ResponseError for PresentationError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            PresentationError::DomainValidation(_)
            | PresentationError::Empty(_)
            | PresentationError::NonNegative(_) => StatusCode::BAD_REQUEST,
            PresentationError::NotFound(_) => StatusCode::NOT_FOUND,
            PresentationError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(self)
    }
}
