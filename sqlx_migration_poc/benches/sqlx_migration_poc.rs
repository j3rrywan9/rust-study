use criterion::{criterion_group, criterion_main, Criterion};
use sql_builder::SqlBuilder;
use sqlx_migration_poc::schema::test_run::TestRun;
use tokio::runtime::Runtime;

async fn query_all_with_pool() -> Result<Vec<TestRun>, sqlx::Error> {
    let pool = &sqlx_migration_poc::db::init_pool_from_env().await.unwrap();
    let mut query = SqlBuilder::select_from("test_run");
    query.field("*");

    let query_string = query.sql().unwrap();

    Ok(sqlx::query_as(query_string.as_str())
        .fetch_all(pool)
        .await?)
}

async fn query_all_with_connection() -> Result<Vec<TestRun>, sqlx::Error> {
    let connection = &mut *(sqlx_migration_poc::db::init_pool_from_env()
        .await
        .unwrap()
        .acquire()
        .await
        .unwrap());
    let mut query = SqlBuilder::select_from("test_run");
    query.field("*");

    let query_string = query.sql().unwrap();

    Ok(sqlx::query_as(query_string.as_str())
        .fetch_all(connection)
        .await?)
}

fn test_sqlx_pool(c: &mut Criterion) {
    let mut group = c.benchmark_group("SQLx PgPool vs PgConnection");
    group.bench_function("PgPool", |b| {
        b.to_async(Runtime::new().unwrap())
            .iter(|| query_all_with_pool())
    });
    group.bench_function("PgConnection", |b| {
        b.to_async(Runtime::new().unwrap())
            .iter(|| query_all_with_connection())
    });

    group.finish();
}

criterion_group!(benches, test_sqlx_pool);
criterion_main!(benches);
