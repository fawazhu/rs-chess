use std::sync::{Arc, Mutex};

use super::types::{Health, HealthStatus, UnhealthyStatus};

pub struct HealthService {
    health: Arc<Mutex<Health>>,
}

pub trait HealthProxy {
    fn new() -> Self;
    fn get(self) -> Health;
    fn put(self, status: HealthStatus);
}

impl HealthProxy for HealthService {
    fn new() -> Self {
        HealthService {
            health: Arc::new(Mutex::new(Health {
                status: HealthStatus::Unhealthy(UnhealthyStatus::Down),
                started: false,
            })),
        }
    }

    fn get(self) -> Health {
        self.health.lock().unwrap().clone()
    }

    fn put(self, status: HealthStatus) {
        let mut health = self.health.lock().unwrap();
        health.started = true;
        health.status = status;
    }
}

impl Clone for HealthService {
    fn clone(&self) -> Self {
        HealthService {
            health: self.health.clone(),
        }
    }
}

