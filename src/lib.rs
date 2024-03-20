use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
#[derive(serde::Deserialize, serde::Serialize)]
struct FormData {
    name: String,
    email: String,
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(tcp_listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}
