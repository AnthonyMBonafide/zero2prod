use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

use zero2prod::{configuration::DatabaseSettings, get_configuration, run};

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscribe", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let config = get_configuration().expect("Failed to get configuration");
    let mut connection = PgConnection::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to database");

    let r = sqlx::query!("SELECT id, name, email, subscribed_at FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to execute query");

    assert_eq!("le guin", r.name);
    assert_eq!("ursula_le_guin@gmail.com", r.email);
}

#[tokio::test]
async fn subscribe_returns_400_for_missing_form_data() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursla_le_guin%40gmail.com", "missing name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with a 400 when the payload was {}",
            error_message
        )
    }
}
async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();
    let mut config = get_configuration().expect("Failed to get configuration");
    config.database.database_name = Uuid::new_v4().to_string();

    let pool = configure_database(&config.database).await;
    let server = run(listener, pool).expect("Failed to bind address");

    let _server_run = tokio::spawn(server);
    std::mem::drop(_server_run);
    format!("http://127.0.0.1:{}", port)
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to database");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create test database");

    let connections_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to create connection pool");

    sqlx::migrate!("./migrations")
        .run(&connections_pool)
        .await
        .expect("Failed to migrate test database");

    connections_pool
}
