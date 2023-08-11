use warp::Filter;

mod health;

#[tokio::main]
async fn main() {
    let health_service = health::HealthService::new();
    // TODO: Only set service as healthy if other services have been initialized correctly.
    health_service.clone().set(health::HealthStatus::Healthy(health::HealthyStatus::Running));
    let api = warp::path("api").and(health::filter(health_service.clone()));

    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;
}
