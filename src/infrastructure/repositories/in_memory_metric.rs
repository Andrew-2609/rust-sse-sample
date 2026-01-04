use crate::domain::{
    entities::metric::MetricEntity, errors::DomainError, repositories::metric::MetricRepository,
    value_objects::metric_id::MetricID,
};

use async_trait::async_trait;

use std::{collections::HashMap, sync::Arc};

use tokio::sync;

#[derive(Default)]
pub struct InMemoryMetricRepository {
    store: Arc<sync::RwLock<HashMap<String, MetricEntity>>>,
}

#[async_trait]
impl MetricRepository for InMemoryMetricRepository {
    async fn create_metric(&self, metric: MetricEntity) -> Result<MetricEntity, DomainError> {
        let mut map = self.store.write().await;
        map.insert(metric.id.to_string(), metric.clone());
        Ok(metric)
    }

    async fn get_metric_by_id(&self, id: MetricID) -> Result<Option<MetricEntity>, DomainError> {
        let map = self.store.read().await;
        let result = map.get(&id.to_string()).cloned();
        Ok(result)
    }

    async fn get_all_metrics(&self) -> Result<Vec<MetricEntity>, DomainError> {
        let map = self.store.read().await;
        let result: Vec<MetricEntity> = map.values().cloned().collect();
        Ok(result)
    }
}
