use std::collections::BTreeMap;
use std::process::{Command, Stdio};
use std::{env, fs};

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    let mut map = BTreeMap::new();
    map.insert("apiVersion".to_string(), "batch/v1");
    map.insert("kind".to_string(), "CronJob");

    let yaml = serde_yaml::to_string(&map).unwrap();

    let mut output_path = env::current_dir().unwrap();
    output_path.push(PACKAGE_NAME);
    output_path.push("output");

    let mut absolute_path = fs::canonicalize(output_path).unwrap();
    absolute_path.push("example.yaml");
    fs::write(&absolute_path, yaml).unwrap();

    // stdout must be configured with `Stdio::piped` in order to use `cat_child.stdout`
    let cat_child = Command::new("cat")
        .arg(absolute_path.as_path())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start cat process");

    let yq_child = Command::new("yq")
        .arg(".kind")
        .stdin(Stdio::from(
            cat_child.stdout.expect("Failed to open cat stdout"),
        ))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start yq process");

    let output = yq_child.wait_with_output().expect("Failed to wait on yq");

    assert_eq!(
        "CronJob",
        String::from_utf8(output.stdout)
            .unwrap()
            .strip_suffix("\n")
            .unwrap()
    );
}
