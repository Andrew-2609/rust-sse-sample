use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{
    domain::{
        entities::metric_reading::MetricReadingEntity, errors::DomainError,
        value_objects::metric_id::MetricID,
    },
    presentation::errors::PresentationError,
};

#[derive(Deserialize)]
pub struct CreateMetricReadingRequestDTO {
    pub metric_id: String,
    pub value: f64,
    pub timestamp: Option<String>,
}

impl CreateMetricReadingRequestDTO {
    pub fn validate(&self) -> Result<(), PresentationError> {
        if self.metric_id.trim().is_empty() {
            return Err(PresentationError::Empty("metric_id".into()));
        }

        if self.value < 0.0 {
            return Err(PresentationError::NonNegative("value".into()));
        }

        Ok(())
    }
}

impl TryFrom<CreateMetricReadingRequestDTO> for MetricReadingEntity {
    type Error = DomainError;

    fn try_from(value: CreateMetricReadingRequestDTO) -> Result<Self, Self::Error> {
        let metric_id: MetricID = value.metric_id.parse()?;

        let timestamp: OffsetDateTime = match value.timestamp {
            None => OffsetDateTime::now_utc(),
            Some(input_timestamp) => {
                let Ok(timestamp) = time::OffsetDateTime::parse(
                    &input_timestamp,
                    &time::format_description::well_known::Rfc3339,
                ) else {
                    return Err(DomainError::InvalidTimestamp(input_timestamp));
                };

                timestamp
            }
        };

        let metric_reading = Self::create(metric_id, value.value, timestamp)?;

        Ok(metric_reading)
    }
}

#[derive(Serialize)]
pub struct CreateMetricReadingResponseDTO {
    pub id: String,
    pub metric_id: String,
    pub value: f64,
    pub timestamp: String,
}

impl From<MetricReadingEntity> for CreateMetricReadingResponseDTO {
    fn from(value: MetricReadingEntity) -> Self {
        Self {
            id: value.id.to_string(),
            metric_id: value.metric_id.to_string(),
            value: value.value,
            timestamp: value.timestamp.to_string(),
        }
    }
}
