use std::sync::{Arc, Mutex};

use super::{
    types::{Health, HealthStatus},
    HealthyStatus, UnhealthyStatus,
};

#[derive(Clone)]
pub struct HealthService {
    health: Arc<Mutex<Health>>,
}


impl HealthService {
    pub fn new() -> Self {
        HealthService {
            health: Arc::new(Mutex::new(Health {
                status: HealthStatus::Unhealthy(UnhealthyStatus::Down),
                ready: false,
            })),
        }
    }

    pub fn get(self) -> Health {
        self.health.lock().unwrap().clone()
    }

    pub fn set(self, status: HealthStatus) {
        let mut health = self.health.lock().unwrap();
        match status {
            HealthStatus::Healthy(_) => health.ready = true,
            _ => {}
        }
        health.status = status;
    }

    pub fn set_ready(self) {
        let mut health = self.health.lock().unwrap();
        health.ready = true;
    }

    pub fn ready_status(self) -> Result<HealthStatus, UnhealthyStatus> {
        let health = self.get();
        match health.ready {
            false => match health.status {
                HealthStatus::Healthy(_) => panic!("Service is healthy, but not started yet."),
                HealthStatus::Unhealthy(status) => Err(status),
            },
            true => Ok(health.status)
        }
    }

    pub fn live_status(self) -> Result<HealthyStatus, UnhealthyStatus> {
        let health = self.get();
        match health.status {
            HealthStatus::Healthy(status) => Ok(status),
            HealthStatus::Unhealthy(status) => Err(status),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: This test should be changes as the app requires time to setup.
    #[test]
    fn health_service_starts_as_down() {
        let service = HealthService::new();
        assert_eq!(
            service.get().status,
            HealthStatus::Unhealthy(UnhealthyStatus::Down)
        );
    }

    #[test]
    fn health_service_set_updates_status() {
        let service = HealthService::new();
        service.clone().set(HealthStatus::Unhealthy(UnhealthyStatus::Degraded));
        assert_eq!(
            service.clone().get().status,
            HealthStatus::Unhealthy(UnhealthyStatus::Degraded)
        );
        service.clone().set(HealthStatus::Healthy(HealthyStatus::Recovered));
        assert_eq!(
            service.clone().get().status,
            HealthStatus::Healthy(HealthyStatus::Recovered)
        );
    }

    #[test]
    fn health_service_ready_status_returns_ok_status_when_healthy() {
        let service = HealthService::new();
        service.clone().set(HealthStatus::Healthy(HealthyStatus::Running));
        let status = service.clone().ready_status();
        assert_eq!(status.is_ok(), true);
        assert_eq!(service.clone().ready_status().unwrap(), HealthStatus::Healthy(HealthyStatus::Running));
    }

    #[test]
    fn health_service_ready_status_returns_error_status_when_unhealthy_and_not_ready() {
        let service = HealthService::new();
        service.clone().set(HealthStatus::Unhealthy(UnhealthyStatus::Down));
        let status = service.clone().ready_status();
        assert_eq!(status.is_err(), true);
        assert_eq!(service.clone().ready_status().unwrap_err(), UnhealthyStatus::Down);
    }

    #[test]
    fn health_service_ready_status_returns_ok_status_when_unhealthy_and_ready() {
        let service = HealthService::new();
        service.clone().set(HealthStatus::Unhealthy(UnhealthyStatus::Starting));
        service.clone().set_ready();
        let status = service.clone().ready_status();
        assert_eq!(status.is_ok(), true);
        assert_eq!(service.clone().ready_status().unwrap(), HealthStatus::Unhealthy(UnhealthyStatus::Starting));
    }

    #[test]
    fn health_service_live_status_returns_ok_status_when_healthy() {
        let service = HealthService::new();
        service.clone().set(HealthStatus::Healthy(HealthyStatus::Running));
        let status = service.clone().live_status();
        assert_eq!(status.is_ok(), true);
        assert_eq!(service.clone().live_status().unwrap(), HealthyStatus::Running);
    }

    #[test]
    fn health_service_live_status_returns_error_status_when_unhealthy() {
        let service = HealthService::new();
        service.clone().set(HealthStatus::Unhealthy(UnhealthyStatus::Down));
        let status = service.clone().live_status();
        assert_eq!(status.is_err(), true);
        assert_eq!(service.clone().live_status().unwrap_err(), UnhealthyStatus::Down);
    }
}
