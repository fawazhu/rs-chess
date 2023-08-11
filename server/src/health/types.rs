use serde::Serialize;
use std::fmt;

#[derive(PartialEq, Debug, Clone, Copy, Serialize)]
pub enum UnhealthyStatus {
    Starting,
    Degraded,
    Down,
}

impl fmt::Display for UnhealthyStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnhealthyStatus::Starting => write!(f, "{}", "Starting"),
            UnhealthyStatus::Degraded => write!(f, "{}", "Degraded"),
            UnhealthyStatus::Down => write!(f, "{}", "Down"),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize)]
pub enum HealthyStatus { 
    Running,
    Recovered,
}

impl fmt::Display for HealthyStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HealthyStatus::Running => write!(f, "{}", "Running"),
            HealthyStatus::Recovered => write!(f, "{}", "Recovered"),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize)]
pub enum HealthStatus {
    Unhealthy(UnhealthyStatus),
    Healthy(HealthyStatus),
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HealthStatus::Unhealthy(status) => write!(f, "{}", status.to_string()),
            HealthStatus::Healthy(status) => write!(f, "{}", status.to_string()),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize)]
pub struct Health {
    pub status: HealthStatus,
    pub ready: bool,
}
