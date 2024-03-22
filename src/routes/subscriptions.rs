use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
