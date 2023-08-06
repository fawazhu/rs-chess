use serde::{ser::SerializeStruct, Serialize};
use std::fmt;

pub enum UnhealthyStatus {
    Degraded,
    Down,
}

impl fmt::Display for UnhealthyStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnhealthyStatus::Degraded => write!(f, "{}", "Degraded"),
            UnhealthyStatus::Down => write!(f, "{}", "Down"),
        }
    }
}

impl Serialize for UnhealthyStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Clone for UnhealthyStatus {
    fn clone(&self) -> Self {
        match *self {
            UnhealthyStatus::Degraded => UnhealthyStatus::Degraded,
            UnhealthyStatus::Down => UnhealthyStatus::Down,
        }
    }
}
impl Copy for UnhealthyStatus {}

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

impl Serialize for HealthyStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Clone for HealthyStatus {
    fn clone(&self) -> Self {
        match *self {
            HealthyStatus::Running => HealthyStatus::Running,
            HealthyStatus::Recovered => HealthyStatus::Recovered,
        }
    }
}
impl Copy for HealthyStatus {}

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

impl Serialize for HealthStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Clone for HealthStatus {
    fn clone(&self) -> Self {
        match self {
            HealthStatus::Unhealthy(status) => HealthStatus::Unhealthy(status.clone()),
            HealthStatus::Healthy(status) => HealthStatus::Healthy(status.clone()),
        }
    }
}
impl Copy for HealthStatus {}

pub struct Health {
    pub status: HealthStatus,
    pub started: bool,
}

impl Serialize for Health {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut health = serializer.serialize_struct("Health", 2)?;
        health.serialize_field("status", &self.status).unwrap();
        health.serialize_field("started", &self.started).unwrap();
        return health.end();
    }
}

impl Clone for Health {
    fn clone(&self) -> Self {
        Health {
            status: self.status.clone(),
            started: self.started.clone(),
        }
    }
}

impl Copy for Health {}

