use std::convert::Infallible;
use warp::Filter;

use super::{
    handlers::{live_response, ready_response},
    services::HealthService,
};

pub fn health(
    health_service: HealthService,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("health").and(ready(health_service.clone()).or(live(health_service.clone())))
}

pub fn ready(
    health_service: HealthService,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("ready")
        .and(warp::get())
        .and(with_health(health_service))
        .and_then(ready_response)
}

pub fn live(
    health_service: HealthService,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("live")
        .and(warp::get())
        .and(with_health(health_service))
        .and_then(live_response)
}

pub fn with_health(
    health_service: HealthService,
) -> impl Filter<Extract = (HealthService,), Error = Infallible> + Clone {
    warp::any().map(move || health_service.clone())
}
