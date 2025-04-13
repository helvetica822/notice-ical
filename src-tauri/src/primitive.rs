use chrono::{DateTime, Duration, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc};

pub fn is_date(value: &str) -> bool {
    NaiveDate::parse_from_str(value, "%Y%m%d").is_ok()
}

pub fn get_current_datetime_jst() -> DateTime<FixedOffset> {
    let utc_now = Utc::now();
    let jst_offset =
        FixedOffset::east_opt(9 * 3600).expect("JST の FixedOffset 作成に失敗しました。");

    let jst_now = utc_now.with_timezone(&jst_offset);

    // ミリ秒を落とす
    let formatted = jst_now.format("%Y-%m-%d %H:%M:%S").to_string();
    let n = NaiveDateTime::parse_from_str(&formatted, "%Y-%m-%d %H:%M:%S").unwrap();
    let remove_millisecond_datetime: DateTime<FixedOffset> =
        jst_offset.from_local_datetime(&n).unwrap();

    remove_millisecond_datetime
}

pub fn calculate_duration(
    value1: DateTime<FixedOffset>,
    value2: DateTime<FixedOffset>,
) -> Duration {
    value2.signed_duration_since(value1)
}

pub fn parse_datetime(value: &str) -> Result<DateTime<FixedOffset>, String> {
    if let Ok(date_time) = NaiveDateTime::parse_from_str(value, "%Y%m%dT%H%M%SZ") {
        let jst_offset = FixedOffset::east_opt(9 * 3600)
            .ok_or_else(|| format!("JST の日時変換に失敗しました。: {}", value))?;

        let jst_datetime = date_time.and_utc().with_timezone(&jst_offset);

        return Ok(jst_datetime);
    }

    if let Ok(date_time) = NaiveDateTime::parse_from_str(value, "%Y%m%dT%H%M%S") {
        let du = date_time.and_utc();

        let d = FixedOffset::east_opt(0 * 3600)
            .map(|offset| Ok(du.with_timezone(&offset)))
            .unwrap_or_else(|| {
                Err(format!(
                    "UTC の FixedOffset 作成に失敗しました。: {}",
                    value
                ))
            })?;

        return Ok(d);
    }

    Err(format!("日時変換に失敗しました。: {}", value))
}

pub fn convert_datetime_2_timeonly(value: &str) -> Result<String, String> {
    parse_datetime(value).map(|d| d.format("%H:%M").to_string())
}

pub fn get_total_millisecond_2_next_minute() -> i64 {
    let jst_offset = FixedOffset::east_opt(9 * 3600).unwrap();

    let jst_now: DateTime<FixedOffset> = Utc::now().with_timezone(&jst_offset);

    let seconds = jst_now.second() as i64;
    let millisecond = (jst_now.nanosecond() / 1_000_000) as i64;

    let total_seconds = 60 - seconds;

    (total_seconds * 1000) - millisecond
}
