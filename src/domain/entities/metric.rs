use std::time::Duration;
use uuid::Uuid;

#[derive(Clone)]
pub struct MetricEntity {
    pub id: Uuid,
    pub name: String,
    pub input_frequency: Duration,
}
