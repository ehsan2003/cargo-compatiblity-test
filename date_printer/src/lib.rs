use chrono::{DateTime, Utc};

pub fn print_date(d: &DateTime<Utc>) {
    println!("{}", d.format("%Y-%m-%d %H:%M:%S"));
}
