use std::time::Duration;
use uuid::Uuid;

use crate::domain::errors::DomainError;

#[derive(Clone)]
pub struct MetricEntity {
    pub id: Uuid,
    pub name: String,
    pub input_frequency: Duration,
}

impl MetricEntity {
    pub fn create(name: String, input_frequency: Duration) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::BusinessRuleViolation(
                "metric name cannot be empty".to_string(),
            ));
        }

        Ok(Self {
            id: Uuid::now_v7(),
            name,
            input_frequency,
        })
    }
}
