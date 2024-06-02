use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, get_subscriber, init_subscriber, run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to get configuration");
    let connection = PgPool::connect_lazy(config.database.connection_string().expose_secret())
        .expect("Failed to connect to database");

    let listener = TcpListener::bind(config.application.address_string())?;
    run(listener, connection)?.await
}
