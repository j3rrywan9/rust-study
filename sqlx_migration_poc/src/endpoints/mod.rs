use axum::http::StatusCode;
use axum::routing::get;
use axum::{Extension, Router};
use chrono::{DateTime, Duration, Utc};
use sqlx::{Error, PgPool};

pub(crate) mod test_run;

pub fn default_until() -> DateTime<Utc> {
    Utc::now()
}

pub fn default_since() -> DateTime<Utc> {
    default_until() - Duration::days(712)
}

pub fn sqlx_err_to_status_code(value: Error) -> StatusCode {
    match value {
        Error::RowNotFound
        | Error::TypeNotFound { .. }
        | Error::ColumnIndexOutOfBounds { .. }
        | Error::ColumnNotFound(_)
        | Error::ColumnDecode { .. }
        | Error::Decode(_)
        | Error::Database(..) => StatusCode::BAD_REQUEST,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub fn create_routes(pool: &'static PgPool) -> Router {
    Router::new()
        .route("/test-runs", get(test_run::handler))
        .layer(Extension(pool))
}
