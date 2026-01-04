use uuid::Uuid;

use crate::domain::{errors::DomainError, value_objects::metric_id::MetricID};

#[derive(Clone)]
pub struct MetricReadingEntity {
    pub id: Uuid,
    pub metric_id: MetricID,
    pub value: f64,
    pub timestamp: time::UtcDateTime,
}

impl MetricReadingEntity {
    pub fn create(
        metric_id: MetricID,
        value: f64,
        timestamp: time::OffsetDateTime,
    ) -> Result<MetricReadingEntity, DomainError> {
        if value < 0.0 {
            return Err(DomainError::BusinessRuleViolation(
                "metric reading value cannot be below zero".into(),
            ));
        }

        Ok(Self {
            id: Uuid::now_v7(),
            metric_id,
            value,
            timestamp: timestamp.to_utc(),
        })
    }
}
