use std::env;
use std::net::{Ipv4Addr, SocketAddr};
use tracing::info;
use tracing::subscriber::set_global_default;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, registry, EnvFilter};

mod db;
mod endpoints;
mod error;
mod schema;

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

    let app = endpoints::create_routes(pool);
    let port = 3000;
    let addr = SocketAddr::from((Ipv4Addr::new(0, 0, 0, 0), port));

    info!("Listening on port: {}", port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
