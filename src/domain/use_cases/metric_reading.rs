use async_trait::async_trait;

use crate::{
    domain::errors::DomainError,
    presentation::dtos::metric_reading::{
        CreateMetricReadingRequestDTO, CreateMetricReadingResponseDTO,
    },
};

#[async_trait]
pub trait MetricReadingUseCase {
    async fn create_metric_reading(
        &self,
        metric_reading_dto: CreateMetricReadingRequestDTO,
    ) -> Result<CreateMetricReadingResponseDTO, DomainError>;
}
