use actix_web::{HttpResponse, ResponseError};
use serde::{Serialize, ser::SerializeStruct};

use crate::domain::errors::DomainError;

#[derive(Debug)]
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

impl ResponseError for PresentationError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            PresentationError::BadRequest(_) => StatusCode::BAD_REQUEST,
            PresentationError::NotFound(_) => StatusCode::NOT_FOUND,
            PresentationError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(self)
    }
}
