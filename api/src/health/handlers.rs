use std::convert::Infallible;
use serde_json::json;
use warp::hyper::StatusCode;

use super::services::HealthService;

pub async fn ready_response(
    health_service: HealthService,
) -> Result<warp::reply::WithStatus<warp::reply::Json>, Infallible> {
    match health_service.ready_status() {
        Ok(status) => Ok(warp::reply::with_status(
            warp::reply::json(&json!({ "status": status })),
            StatusCode::OK,
        )),
        Err(status) => Ok(warp::reply::with_status(
            warp::reply::json(&json!({ "status": status })),
            StatusCode::SERVICE_UNAVAILABLE,
        )),
    }
}

pub async fn live_response(health_service: HealthService) -> Result<impl warp::Reply, Infallible> {
    match health_service.live_status() {
        Ok(status) => Ok(warp::reply::with_status(
            warp::reply::json(&json!({ "status": status })),
            StatusCode::OK,
        )),
        Err(status) => Ok(warp::reply::with_status(
            warp::reply::json(&json!({ "status": status })),
            StatusCode::SERVICE_UNAVAILABLE,
        )),
    }
}
