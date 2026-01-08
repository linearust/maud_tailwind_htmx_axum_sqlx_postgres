use time::{OffsetDateTime, format_description::well_known::Rfc3339};

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

pub fn format_datetime(dt: OffsetDateTime) -> String {
    let formatted = match dt.format(&Rfc3339) {
        Ok(s) => s,
        Err(e) => {
            tracing::error!(error = %e, "Failed to format datetime as RFC3339");
            return "Invalid date".to_string();
        }
    };

    let datetime_parts: Vec<&str> = formatted.split('T').collect();
    let date_part = datetime_parts.first().copied().unwrap_or("");
    let time_part = datetime_parts
        .get(1)
        .and_then(|t| t.split('.').next())
        .unwrap_or("");

    if time_part.is_empty() {
        date_part.to_string()
    } else {
        format!("{} {}", date_part, time_part)
    }
}
