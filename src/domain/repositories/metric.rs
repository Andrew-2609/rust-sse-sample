use async_trait::async_trait;

use crate::domain::{
    entities::metric::MetricEntity, errors::DomainError, value_objects::metric_id::MetricID,
};

#[async_trait]
pub trait MetricRepository {
    async fn create_metric(&self, metric: MetricEntity) -> Result<MetricEntity, DomainError>;
    async fn get_metric_by_id(&self, id: MetricID) -> Result<Option<MetricEntity>, DomainError>;
    async fn get_all_metrics(&self) -> Result<Vec<MetricEntity>, DomainError>;
}
