use chrono::{DateTime, Utc};
use fakeit::datetime;

#[warn(deprecated)]
pub fn random_date() -> String {
    let d = datetime::date();
    let dt = DateTime::<Utc>::from_timestamp(d.secs, d.nsecs).unwrap();
    dt.date_naive().to_string()
}
