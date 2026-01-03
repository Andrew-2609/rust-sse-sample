use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::metric::MetricEntity;

#[async_trait]
pub trait MetricRepository {
    async fn create_metric(&self, metric: MetricEntity) -> Result<MetricEntity, std::io::Error>;
    async fn get_metric_by_id(&self, id: Uuid) -> Result<Option<MetricEntity>, std::io::Error>;
}
