use std::fmt;

pub enum DomainError {
    InvalidMetricID(String),
    InvalidMetricReadingID(String),
    InvalidTimestamp(String),
    BusinessRuleViolation(String),
}

impl DomainError {
    pub fn message(&self) -> String {
        match self {
            Self::InvalidMetricID(id) => format!("metric_id {id} is invalid"),
            Self::InvalidMetricReadingID(id) => format!("metric_reading_id {id} is invalid"),
            Self::InvalidTimestamp(timestamp) => {
                format!("{timestamp} is not a valid timestamp according to RFC3339")
            }
            Self::BusinessRuleViolation(msg) => msg.clone(),
        }
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}
