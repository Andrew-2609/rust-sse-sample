use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc};
use tokio::sync;

use crate::domain::{
    entities::metric_reading::MetricReadingEntity, errors::DomainError,
    repositories::metric_reading::MetricReadingRepository,
};

#[derive(Default)]
pub struct InMemoryMetricReadingRepository {
    store: Arc<sync::RwLock<HashMap<String, MetricReadingEntity>>>,
}

#[async_trait]
impl MetricReadingRepository for InMemoryMetricReadingRepository {
    async fn create_metric_reading(
        &self,
        metric_reading: MetricReadingEntity,
    ) -> Result<MetricReadingEntity, DomainError> {
        let mut map = self.store.write().await;
        map.insert(metric_reading.id.to_string(), metric_reading.clone());
        Ok(metric_reading)
    }
}
