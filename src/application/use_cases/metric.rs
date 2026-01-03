use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::{
        errors::DomainError, repositories::metric::MetricRepository,
        use_cases::metric::MetricUseCase,
    },
    presentation::dtos::metric::GetMetricResponseDTO,
};

pub type DynMetricRepository = Arc<dyn MetricRepository + Send + Sync>;

pub struct MetricUseCaseImpl {
    metric_repository: DynMetricRepository,
}

impl MetricUseCaseImpl {
    pub fn new(metric_repository: DynMetricRepository) -> Self {
        Self { metric_repository }
    }
}

#[async_trait]
impl MetricUseCase for MetricUseCaseImpl {
    async fn create_metric(
        &self,
        metric_dto: crate::presentation::dtos::metric::CreateMetricRequestDTO,
    ) -> Result<crate::presentation::dtos::metric::CreateMetricResponseDTO, DomainError> {
        let metric = self
            .metric_repository
            .create_metric(metric_dto.into())
            .await?;

        Ok(metric.into())
    }

    async fn get_metric_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<GetMetricResponseDTO>, DomainError> {
        let metric = self.metric_repository.get_metric_by_id(id).await?;

        if let Some(metric) = metric {
            return Ok(Some(metric.into()));
        }

        Ok(None)
    }
}
