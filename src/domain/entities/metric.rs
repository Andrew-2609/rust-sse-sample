use std::time::Duration;

use crate::domain::{errors::DomainError, value_objects::metric_id::MetricID};

#[derive(Clone)]
pub struct MetricEntity {
    pub id: MetricID,
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
            id: MetricID::new(),
            name,
            input_frequency,
        })
    }
}
