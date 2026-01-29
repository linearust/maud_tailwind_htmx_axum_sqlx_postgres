use chrono::{DateTime, Utc};

pub fn format_price(amount: i32) -> String {
    amount
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .expect("Price formatting should always produce valid UTF-8")
        .join(",")
}

pub fn format_datetime(datetime: DateTime<Utc>) -> String {
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
