use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[tracing::instrument(
    name="Adding a new subscriber",
    skip(form,connection),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>, connection: web::Data<PgPool>) -> HttpResponse {
    let name = match SubscriberName::parse(form.0.name) {
        Ok(n) => n,
        Err(e) => return HttpResponse::BadRequest().body(e),
    };

    let email = match SubscriberEmail::parse(form.0.email) {
        Ok(e) => e,
        Err(e) => return HttpResponse::BadRequest().body(e),
    };

    let new_subscriber = NewSubscriber { name, email };

    match insert_subscriber(&connection, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name="Saving new subscriber details in the database",
    skip(new_subscriber,pool),
    fields(
        subscriber_email = %new_subscriber.email.as_ref(),
        subscriber_name = %new_subscriber.name.as_ref()
    )
)]

pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::error::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
