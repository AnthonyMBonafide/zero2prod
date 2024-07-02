use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;
use zero2prod::{configuration::get_configuration, get_subscriber, init_subscriber, run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to get configuration");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(config.database.with_db());

    let listener = TcpListener::bind(config.application.address_string())?;
    run(listener, connection_pool)?.await
}
