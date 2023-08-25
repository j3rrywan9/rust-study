use crate::endpoints::test_run::TestRunQueryParams;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sql_builder::{quote, SqlBuilder};
use sqlx::PgPool;
use tracing::{debug, info};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TestRun {
    pub(crate) test_run_id: Uuid,
    pub(crate) build_number: String,
    pub(crate) build_url: Option<String>,
    pub(crate) build_timestamp: DateTime<Utc>,
}

impl TestRun {
    pub(crate) async fn get_by_query_params(
        query_params: TestRunQueryParams,
        pool: &PgPool,
    ) -> Result<Vec<TestRun>, sqlx::Error> {
        let mut query = SqlBuilder::select_from("test_run");
        query.field("*");

        if let Some(id) = query_params.test_run_id {
            query.and_where_eq("test_run_id", quote(id));
        }

        let query_string = query.sql().unwrap();

        info!("Querying DB for test runs");
        debug!("using SQL command: {}", query_string);
        Ok(sqlx::query_as(query_string.as_str())
            .fetch_all(pool)
            .await?)
    }
}
