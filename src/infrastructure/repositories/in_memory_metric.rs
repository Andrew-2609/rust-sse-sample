use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    sync::{Arc, RwLock},
};

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{entities::metric::MetricEntity, repositories::metric::MetricRepository};

pub struct InMemoryMetricRepository {
    store: Arc<RwLock<HashMap<String, MetricEntity>>>,
}

impl InMemoryMetricRepository {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl MetricRepository for InMemoryMetricRepository {
    async fn create_metric(&self, metric: MetricEntity) -> Result<MetricEntity, Error> {
        let mut map = self
            .store
            .write()
            .map_err(|e| Error::new(ErrorKind::Other, format!("metric store poisoned: {e}")))?;

        map.insert(metric.id.to_string(), metric.clone());

        Ok(metric)
    }

    async fn get_metric_by_id(&self, id: Uuid) -> Result<Option<MetricEntity>, Error> {
        let map = self
            .store
            .read()
            .map_err(|e| Error::new(ErrorKind::Other, format!("metric store poisoned: {e}")))?;

        let result = map.get(&id.to_string()).cloned();

        Ok(result)
    }
}
