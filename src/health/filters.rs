use std::convert::Infallible;
use warp::Filter;

use super::{
    handlers::{live_respond, ready_respond},
    services::{HealthProxy, HealthService},
    types::{HealthStatus, HealthyStatus},
};

pub fn health() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    let health = HealthService::new();
    health
        .clone()
        .put(HealthStatus::Healthy(HealthyStatus::Running));
    warp::path("health").and(ready(health.clone()).or(live(health.clone())))
}

pub fn ready(
    health_service: HealthService,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("ready")
        .and(warp::get())
        .and(with_health(health_service))
        .and_then(ready_respond)
}

pub fn live(
    health_service: HealthService,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("live")
        .and(warp::get())
        .and(with_health(health_service))
        .and_then(live_respond)
}

pub fn with_health(
    health_service: HealthService,
) -> impl Filter<Extract = (HealthService,), Error = Infallible> + Clone {
    warp::any().map(move || health_service.clone())
}
