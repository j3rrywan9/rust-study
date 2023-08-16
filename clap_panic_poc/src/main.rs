//! A POC to reproduce the <https://github.com/pact-foundation/pact-reference/issues/217>

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("cli")
        .version("1.0")
        .arg_required_else_help(true)
        .arg(
            Arg::new("filter-consumer")
                .short('c')
                .long("filter-consumer")
                .multiple(true)
                .takes_value(true)
                .forbid_empty_values(true)
                .help("Consumer name to filter the pacts to be verified (can be repeated)"),
        )
        .get_matches_from(vec!["cli", "--filter-consumer", "consumer1", "consumer2"]);

    let values = matches
        .values_of_lossy("filter-consumer")
        .unwrap_or_default();
    // Uncomment the following line to fix
    // let values = matches.get_many::<String>("filter-consumer").unwrap_or_default().map(|v| v.to_string()).collect::<Vec<_>>();
    assert_eq!(values, vec!["consumer1", "consumer2"]);
}
