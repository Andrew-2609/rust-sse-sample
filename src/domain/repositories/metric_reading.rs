use async_trait::async_trait;

use crate::domain::{entities::metric_reading::MetricReadingEntity, errors::DomainError};

#[async_trait]
pub trait MetricReadingRepository {
    async fn create_metric_reading(
        &self,
        metric_reading: MetricReadingEntity,
    ) -> Result<MetricReadingEntity, DomainError>;
}
