use std::convert::Infallible;

use hyper::StatusCode;
use serde_json::json;

use super::{
    errors::ServiceUnhealthyError,
    services::{HealthProxy, HealthService},
    types::{HealthStatus, UnhealthyStatus},
};

pub async fn ready_respond(health_service: HealthService) -> Result<impl warp::Reply, Infallible> {
    let health = health_service.get();
    match health.started {
        true => Ok(warp::reply::with_status(
            warp::reply::json(&health),
            StatusCode::OK,
        )),
        false => match health.status {
            HealthStatus::Unhealthy(status) => return Ok(when_unhealthy(status)),
            HealthStatus::Healthy(_) => {
                return Ok(warp::reply::with_status(
                    warp::reply::json(&json!({ "message": "Unexpected health status" })),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        },
    }
}

pub async fn live_respond(health_service: HealthService) -> Result<impl warp::Reply, Infallible> {
    let health = health_service.get();
    match health.status {
        HealthStatus::Unhealthy(status) => return Ok(when_unhealthy(status)),
        HealthStatus::Healthy(_) => Ok(warp::reply::with_status(
            warp::reply::json(&health),
            StatusCode::OK,
        )),
    }
}

pub fn when_unhealthy(status: UnhealthyStatus) -> warp::reply::WithStatus<warp::reply::Json> {
    warp::reply::with_status(
        warp::reply::json(&ServiceUnhealthyError { status }),
        StatusCode::SERVICE_UNAVAILABLE,
    )
}
