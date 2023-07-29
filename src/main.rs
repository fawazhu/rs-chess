use actix_web::{get, HttpServer, App, Responder, HttpResponse};

#[get("/")]
async fn hello() -> impl Responder {
    return HttpResponse::Ok().body("Hello, World!");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

