use crate::error::Error;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions, PgSslMode};
use std::env;
use std::str::FromStr;
use tokio::sync::OnceCell;
use tracing::{debug, error, info};

static POOL: OnceCell<PgPool> = OnceCell::const_new();

fn get_env_var(var_name: &str) -> Result<String, Error> {
    let env_var = env::var(var_name);
    if env_var.is_err() {
        return Err(Error::MissingEnvVar(var_name.to_string()));
    }
    Ok(env_var.unwrap())
}

fn parse_db_port(s: String) -> u16 {
    s.parse()
        .unwrap_or_else(|_| panic!("Invalid port number set via environment variable '{}'", s,))
}

fn parse_ssl_mode(s: String) -> PgSslMode {
    match PgSslMode::from_str(&s) {
        Ok(mode) => mode,
        Err(_) => {
            error!(mode = %s, "SSL mode from env is not a valid mode");
            panic!("Invalid SSL mode set via environment variable '{}'", s)
        }
    }
}

async fn init_pool(
    host: &str,
    port: Option<u16>,
    username: &str,
    password: &str,
    db_name: &str,
    ssl_mode: Option<PgSslMode>,
) -> Result<PgPool, Error> {
    info!("Initializing DB connection pool");
    let mut opts = PgConnectOptions::new()
        .host(host)
        .username(username)
        .password(password)
        .database(db_name);

    match port {
        None => debug!("No DB port found in env, using default"),
        Some(port) => {
            debug!(port, "Setting DB port");
            opts = opts.port(port);
        }
    }

    match ssl_mode {
        None => debug!("No SSL mode found in env, using default"),
        Some(mode) => {
            debug!(?mode, "Setting SSL mode");
            opts = opts.ssl_mode(mode);
        }
    }

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect_with(opts)
        .await?;

    Ok(pool)
}

pub async fn init_pool_from_env() -> Result<PgPool, Error> {
    info!("Fetching DB variables from environment");
    let host = get_env_var("DB_HOST")?;
    let port = get_env_var("DB_PORT").ok().map(parse_db_port);
    let username = get_env_var("DB_USERNAME")?;
    let password = get_env_var("DB_PASSWORD")?;
    let database_name = get_env_var("DB_NAME")?;
    let ssl_mode = get_env_var("DB_SSL_MODE").ok().map(parse_ssl_mode);
    init_pool(&host, port, &username, &password, &database_name, ssl_mode).await
}

pub async fn init_pool_from_env_and_migrate() -> Result<PgPool, Error> {
    let pool = init_pool_from_env().await?;
    info!("Running DB migrations");
    sqlx::migrate!("db/migrations").run(&pool).await?;

    Ok(pool)
}

pub async fn get_pool() -> Result<&'static PgPool, Error> {
    POOL.get_or_try_init(init_pool_from_env_and_migrate).await
}
