use sqlx_migration_poc::{init_logging_from_env, start_server};
use tracing::error;

#[tokio::main]
async fn main() {
    init_logging_from_env();

    if let Err(e) = start_server().await {
        error!("An error occurred when starting server: {}", e);
        std::process::exit(1);
    }
}
