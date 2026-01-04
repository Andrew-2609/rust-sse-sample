use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::use_cases::metric::DynMetricRepository,
    domain::{
        entities::metric_reading::MetricReadingEntity, errors::DomainError,
        repositories::metric_reading::MetricReadingRepository,
        use_cases::metric_reading::MetricReadingUseCase,
    },
    presentation::dtos::metric_reading::{
        CreateMetricReadingRequestDTO, CreateMetricReadingResponseDTO,
    },
};

pub type DynMetricReadingRepository = Arc<dyn MetricReadingRepository + Send + Sync>;

pub struct MetricReadingUseCaseImpl {
    metric_repository: DynMetricRepository,
    metric_reading_repository: DynMetricReadingRepository,
}

impl MetricReadingUseCaseImpl {
    pub fn new(
        metric_repository: DynMetricRepository,
        metric_reading_repository: DynMetricReadingRepository,
    ) -> Self {
        Self {
            metric_repository,
            metric_reading_repository,
        }
    }
}

#[async_trait]
impl MetricReadingUseCase for MetricReadingUseCaseImpl {
    async fn create_metric_reading(
        &self,
        metric_reading_dto: CreateMetricReadingRequestDTO,
    ) -> Result<CreateMetricReadingResponseDTO, DomainError> {
        let metric_reading: MetricReadingEntity = metric_reading_dto.try_into()?;

        if let None = self
            .metric_repository
            .get_metric_by_id(metric_reading.metric_id)
            .await?
        {
            return Err(DomainError::BusinessRuleViolation(format!(
                "metric {} not found",
                metric_reading.metric_id
            )));
        }

        let result = self
            .metric_reading_repository
            .create_metric_reading(metric_reading)
            .await?;

        Ok(result.into())
    }
}
