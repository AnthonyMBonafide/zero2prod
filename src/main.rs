use std::net::TcpListener;

use sqlx::{Connection, PgPool};
use zero2prod::{configuration::get_configuration, run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", config.application_port);
    let config = get_configuration().expect("Failed to get configuration");
    let connection = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to database");

    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
