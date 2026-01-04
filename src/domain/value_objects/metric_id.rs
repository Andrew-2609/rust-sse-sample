use std::{fmt, str::FromStr};

use uuid::Uuid;

use crate::domain::errors::DomainError;

#[derive(Clone)]
pub struct MetricID(Uuid);

impl MetricID {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl FromStr for MetricID {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = DomainError::InvalidMetricID(s.into());

        let metric_id = match Uuid::try_parse(s) {
            Err(_) => return Err(err),
            Ok(v) => v,
        };

        match metric_id.get_version() {
            None => return Err(err),
            Some(v) => {
                if !v.eq(&uuid::Version::SortRand) {
                    return Err(err);
                }
            }
        }

        Ok(Self(metric_id))
    }
}

impl fmt::Display for MetricID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
