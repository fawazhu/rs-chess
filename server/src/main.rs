use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing;
use tracing_subscriber;
use warp::Filter;

mod health;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().json().init();

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080);
    let health_service = health::HealthService::new(address);
    // TODO: Only set service as healthy if other services have been initialized correctly.
    health_service.clone().set(health::HealthStatus::Healthy(
        health::HealthyStatus::Running,
    ));
    let api = warp::path("api").and(health::filter(health_service.clone()).with(warp::trace(
        |info| {
            tracing::info_span!(
                "api::request",
                method = %info.method(),
                path = %info.path(),
            )
        },
    )));

    warp::serve(api).run(address).await;
}
