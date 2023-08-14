use log;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::json::JsonEncoder,
    Config,
};
use std::{
    borrow::BorrowMut,
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use warp::Filter;

mod health;

#[cfg(test)]
mod tests;

fn level_filter_from_str(value: &mut String) -> log::LevelFilter {
    match value.to_lowercase().as_str() {
        "fatal" | "off" => log::LevelFilter::Off,
        "error" => log::LevelFilter::Error,
        "warn" | "warning" => log::LevelFilter::Warn,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        _ => log::LevelFilter::Info,
    }
}

fn init_logs() {
    let log_level = match env::var("LOG_LEVEL") {
        Ok(level) => level_filter_from_str(level.clone().borrow_mut()),
        Err(_) => log::LevelFilter::Info,
    };
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build();
    let log_config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().build(log_level))
        .unwrap();
    log4rs::init_config(log_config).unwrap();
}

#[tokio::main]
async fn main() {
    init_logs();

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080);
    let health_service = health::HealthService::new(address);
    // TODO: Only set service as healthy if other services have been initialized correctly.
    health_service.clone().set(health::HealthStatus::Healthy(
        health::HealthyStatus::Running,
    ));
    let log = warp::log("rs-chess::server");
    let api = warp::path("api")
        .and(health::filter(health_service.clone()))
        .with(log);

    warp::serve(api).run(address).await;
}
