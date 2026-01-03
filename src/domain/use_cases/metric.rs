use async_trait::async_trait;
use uuid::Uuid;

use crate::presentation::dtos::metric::{
    CreateMetricRequestDTO, CreateMetricResponseDTO, GetMetricResponseDTO,
};

#[async_trait]
pub trait MetricUseCase {
    async fn create_metric(
        &self,
        metric_dto: CreateMetricRequestDTO,
    ) -> Result<CreateMetricResponseDTO, std::io::Error>;

    async fn get_metric_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<GetMetricResponseDTO>, std::io::Error>;
}
