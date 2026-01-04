use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    entities::metric::MetricEntity, errors::DomainError, repositories::metric::MetricRepository,
};

#[derive(Default)]
pub struct InMemoryMetricRepository {
    store: Arc<RwLock<HashMap<String, MetricEntity>>>,
}

#[async_trait]
impl MetricRepository for InMemoryMetricRepository {
    async fn create_metric(&self, metric: MetricEntity) -> Result<MetricEntity, DomainError> {
        let mut map = self
            .store
            .write()
            .map_err(|e| DomainError::Unknown(format!("metric store poisoned: {e}")))?;

        map.insert(metric.id.to_string(), metric.clone());

        Ok(metric)
    }

    async fn get_metric_by_id(&self, id: Uuid) -> Result<Option<MetricEntity>, DomainError> {
        let map = self
            .store
            .read()
            .map_err(|e| DomainError::Unknown(format!("metric store poisoned: {e}")))?;

        let result = map.get(&id.to_string()).cloned();

        Ok(result)
    }
}
