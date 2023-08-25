use crate::endpoints::{default_since, default_until, sqlx_err_to_status_code};
use crate::schema::test_run::TestRun;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::{Extension, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, error, info};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestRunQueryParams {
    pub(crate) test_run_id: Option<Uuid>,
    pub(crate) page_num: Option<u32>,
    pub(crate) per_page: Option<u32>,
    #[serde(default = "default_since")]
    pub(crate) since: DateTime<Utc>,
    #[serde(default = "default_until")]
    pub(crate) until: DateTime<Utc>,
}

pub async fn handler(
    pool: Extension<&PgPool>,
    Query(query_params): Query<TestRunQueryParams>,
) -> Result<Json<Vec<TestRun>>, StatusCode> {
    info!("Received an HTTP 'GET' request at the '/test-runs' endpoint");
    debug!("with query params: {:?}", query_params);
    let test_run = TestRun::get_by_query_params(query_params, &pool)
        .await
        .map_err(|e| {
            error!("Error fetching test runs: {e}");
            sqlx_err_to_status_code(e) // TODO: Return a detailed error
        })?;

    Ok(Json(test_run))
}
