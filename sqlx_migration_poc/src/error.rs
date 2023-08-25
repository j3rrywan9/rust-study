#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
