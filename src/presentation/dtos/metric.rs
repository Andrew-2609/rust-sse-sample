use std::time::Duration;

use uuid::Uuid;

use crate::{domain::entities::metric::MetricEntity, presentation::errors::PresentationError};

#[derive(serde::Deserialize)]
pub struct CreateMetricRequestDTO {
    pub name: String,
    pub input_frequency_in_seconds: Option<u64>,
}

impl CreateMetricRequestDTO {
    pub fn validate(&self) -> Result<(), PresentationError> {
        if self.name.trim().len() == 0 {
            return Err(PresentationError::BadRequest("name must be present".into()));
        }

        if let Some(input_frequency_in_seconds) = self.input_frequency_in_seconds
            && input_frequency_in_seconds <= 0
        {
            return Err(PresentationError::BadRequest(
                "input frequency must be greater than zero".into(),
            ));
        }

        Ok(())
    }
}

impl Into<MetricEntity> for CreateMetricRequestDTO {
    fn into(self) -> MetricEntity {
        let input_frequency = match self.input_frequency_in_seconds {
            Some(value) => Duration::from_secs(value),
            None => Duration::ZERO,
        };

        MetricEntity {
            id: Uuid::now_v7(),
            name: self.name,
            input_frequency,
        }
    }
}

#[derive(serde::Serialize)]
pub struct CreateMetricResponseDTO {
    pub id: String,
    pub name: String,
    pub input_frequency_seconds: u64,
}

impl From<MetricEntity> for CreateMetricResponseDTO {
    fn from(value: MetricEntity) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name,
            input_frequency_seconds: value.input_frequency.as_secs(),
        }
    }
}

#[derive(serde::Serialize)]
pub struct GetMetricResponseDTO {
    pub id: String,
    pub name: String,
    pub input_frequency_seconds: u64,
}

impl From<MetricEntity> for GetMetricResponseDTO {
    fn from(value: MetricEntity) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name,
            input_frequency_seconds: value.input_frequency.as_secs(),
        }
    }
}
