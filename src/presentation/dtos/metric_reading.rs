use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    domain::{entities::metric_reading::MetricReadingEntity, errors::DomainError},
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
        let Ok(metric_id) = Uuid::try_parse(&self.metric_id) else {
            return Err(PresentationError::BadRequest(
                "metric_id is not a valid uuid".into(),
            ));
        };

        let Some(version) = metric_id.get_version() else {
            return Err(PresentationError::BadRequest(
                "metric_id is not a valid v7 uuid".into(),
            ));
        };

        if !version.eq(&uuid::Version::SortRand) {
            return Err(PresentationError::BadRequest(
                "metric_id is not a valid v7 uuid".into(),
            ));
        }

        Ok(())
    }
}

impl TryFrom<CreateMetricReadingRequestDTO> for MetricReadingEntity {
    type Error = DomainError;

    fn try_from(value: CreateMetricReadingRequestDTO) -> Result<Self, Self::Error> {
        let Ok(metric_id) = Uuid::parse_str(&value.metric_id) else {
            // TODO: this awkward domain error will be removed when metric_id becomes a VO
            return Err(DomainError::BusinessRuleViolation(
                "invalid metric_id".into(),
            ));
        };

        let timestamp: OffsetDateTime = match value.timestamp {
            None => OffsetDateTime::now_utc(),
            Some(input_timestamp) => {
                let Ok(timestamp) = time::OffsetDateTime::parse(
                    &input_timestamp,
                    &time::format_description::well_known::Rfc3339,
                ) else {
                    return Err(DomainError::BusinessRuleViolation(
                        "invalid timestamp".into(),
                    ));
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
