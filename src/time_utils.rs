use chrono::{DateTime, Utc};

pub fn format_utc(dt: DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S %z").to_string()
}

pub fn parse_site_datetime(s: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    let dt = DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S %z")?;
    Ok(dt.with_timezone(&Utc))
}