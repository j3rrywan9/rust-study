use axum::{routing::get, Router};
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::US::Pacific;
use std::time::{Duration, UNIX_EPOCH};
use zookeeper_client as zk;

static PERSISTENT_OPEN: &zk::CreateOptions<'static> =
    &zk::CreateMode::Persistent.with_acls(zk::Acls::anyone_all());

#[tokio::main]
async fn main() {
    const ADDRESSES: &str = "localhost:2181";

    println!("Connecting to ZooKeeper at: {}", ADDRESSES);
    let client = zk::Client::builder()
        .assume_server_version(3, 4, u32::MAX)
        .connect(ADDRESSES)
        .await
        .unwrap();

    let options = zk::LockOptions::new(zk::Acls::anyone_all())
        .with_ancestor_options(PERSISTENT_OPEN.clone())
        .unwrap();

    let lock_prefix = zk::LockPrefix::new_shared("/locks/shared/n-").unwrap();
    println!("Acquiring lock at: {:?}", &lock_prefix);
    let lock = client
        .lock(lock_prefix, b"", options.clone())
        .await
        .unwrap();

    let lock_path = lock.lock_path();
    println!("Acquired lock: {:?} at: {}", lock, lock.lock_path());

    let stat = client.check_stat(lock_path).await.unwrap().unwrap();
    let d = UNIX_EPOCH + Duration::from_millis(stat.ctime as u64);
    let datetime = Pacific.from_utc_datetime(&DateTime::<Utc>::from(d).naive_utc());
    let timestamp = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    println!("Child znode was created at: {}", timestamp);

    if let Ok(children) = client.list_children("/locks/shared").await {
        println!("Lock znode children: {:?}", children);
    } else {
        println!("Error!")
    }

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
