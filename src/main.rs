use warp::Filter;

mod health;

#[tokio::main]
async fn main() {
    let api = warp::path("api").and(health::filter());

    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;
}
