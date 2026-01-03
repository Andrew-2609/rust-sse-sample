use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{entities::metric::MetricEntity, errors::DomainError};

#[async_trait]
pub trait MetricRepository {
    async fn create_metric(&self, metric: MetricEntity) -> Result<MetricEntity, DomainError>;
    async fn get_metric_by_id(&self, id: Uuid) -> Result<Option<MetricEntity>, DomainError>;
}
