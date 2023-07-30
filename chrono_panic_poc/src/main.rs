use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{TimeZone, Utc};

fn main() {
    #[allow(deprecated)]
    let formatted_date: DelayedFormat<StrftimeItems> =
        Utc.ymd(2022, 12, 01).format("%Y-%m-%dT%H:%M:%S.%3f%:z");
    println!("{}", formatted_date.to_string());
}
