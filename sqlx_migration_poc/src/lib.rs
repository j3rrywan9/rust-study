use std::env;
use tracing::info;
use tracing::subscriber::set_global_default;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, registry, EnvFilter};

pub mod db;
mod endpoints;
mod error;
pub mod schema;

pub fn init_logging_from_env() {
    let json_layer = fmt::layer()
        .json()
        .flatten_event(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true);

    let compact_layer = fmt::layer()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true);

    let error_message = "Failed to initialize logging";

    if let Ok(logger) = env::var("LOGGER") {
        if logger == "Default" {
            set_global_default(
                registry()
                    .with(EnvFilter::from_default_env())
                    .with(compact_layer),
            )
            .expect(error_message);
        }
    } else {
        set_global_default(
            registry()
                .with(EnvFilter::from_default_env())
                .with(json_layer),
        )
        .expect(error_message);
    }
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let pool = db::get_pool().await?;

    let router = endpoints::create_routes(pool);
    let port = 3000;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    info!("Listening on port: {}", port);
    axum::serve(listener, router).await.unwrap();

    Ok(())
}
