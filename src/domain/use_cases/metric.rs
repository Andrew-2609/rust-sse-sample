use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::errors::DomainError,
    presentation::dtos::metric::{
        CreateMetricRequestDTO, CreateMetricResponseDTO, GetMetricResponseDTO,
    },
};

#[async_trait]
pub trait MetricUseCase {
    async fn create_metric(
        &self,
        metric_dto: CreateMetricRequestDTO,
    ) -> Result<CreateMetricResponseDTO, DomainError>;

    async fn get_metric_by_id(&self, id: Uuid)
    -> Result<Option<GetMetricResponseDTO>, DomainError>;

    async fn get_all_metrics(&self) -> Result<Vec<GetMetricResponseDTO>, DomainError>;
}
