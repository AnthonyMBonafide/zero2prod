use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(tcp_listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}
