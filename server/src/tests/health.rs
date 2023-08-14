use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use super::test_logger::TestLogger;
use crate::health::{HealthService, UnhealthyStatus, HealthStatus};

const ADDRESS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080); 

#[test]
fn health_service_logs_address_only_when_ready() {
    let logger = TestLogger::new();
    let logger = Box::new(logger);
    log::set_boxed_logger(logger.clone()).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
    
    let service = HealthService::new(ADDRESS);
    service
        .clone()
        .set(HealthStatus::Unhealthy(UnhealthyStatus::Starting));
    let logs = logger.logs.lock().unwrap();
    assert_eq!(
        logs.iter()
            .filter(|log| log.level == log::Level::Info
                && log.message == "Ready to accept requests on 0.0.0.0:8080")
            .count(),
        0
    );
    drop(logs);
    service.clone().set_ready();
    let logs = logger.logs.lock().unwrap();
    assert_eq!(
        logs.iter()
            .filter(|log| log.level == log::Level::Info
                && log.message == "Ready to accept requests on 0.0.0.0:8080")
            .count(),
        1
    );
}
