use std::fmt;

pub enum DomainError {
    InvalidMetricID(String),
    BusinessRuleViolation(String),
}

impl DomainError {
    pub fn message(&self) -> String {
        match self {
            Self::InvalidMetricID(id) => format!("metric_id {id} is invalid"),
            Self::BusinessRuleViolation(msg) => msg.clone(),
        }
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}
