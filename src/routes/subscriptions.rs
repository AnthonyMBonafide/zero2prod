use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
#[derive(serde::Deserialize, serde::Serialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}


