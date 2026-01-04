use std::time::Duration;

use crate::{
    domain::{entities::metric::MetricEntity, errors::DomainError},
    presentation::errors::PresentationError,
};

#[derive(serde::Deserialize)]
pub struct CreateMetricRequestDTO {
    pub name: String,
    pub input_frequency_in_seconds: Option<u64>,
}

impl CreateMetricRequestDTO {
    pub fn validate(&self) -> Result<(), PresentationError> {
        if self.name.trim().is_empty() {
            return Err(PresentationError::Empty("name".into()));
        }

        Ok(())
    }
}

impl TryFrom<CreateMetricRequestDTO> for MetricEntity {
    type Error = DomainError;

    fn try_from(value: CreateMetricRequestDTO) -> Result<Self, Self::Error> {
        let input_frequency = Duration::from_secs(value.input_frequency_in_seconds.unwrap_or(0));
        let metric = MetricEntity::create(value.name, input_frequency)?;
        Ok(metric)
    }
}

#[derive(serde::Serialize)]
pub struct CreateMetricResponseDTO {
    pub id: String,
    pub name: String,
    pub input_frequency_in_seconds: u64,
}

impl From<MetricEntity> for CreateMetricResponseDTO {
    fn from(value: MetricEntity) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name,
            input_frequency_in_seconds: value.input_frequency.as_secs(),
        }
    }
}

#[derive(serde::Serialize)]
pub struct GetMetricResponseDTO {
    pub id: String,
    pub name: String,
    pub input_frequency_in_seconds: u64,
}

impl From<MetricEntity> for GetMetricResponseDTO {
    fn from(value: MetricEntity) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name,
            input_frequency_in_seconds: value.input_frequency.as_secs(),
        }
    }
}
