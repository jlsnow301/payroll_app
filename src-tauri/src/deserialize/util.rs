use anyhow::Result;
use calamine::{Data, DataType, ExcelDateTime, ExcelDateTimeType};
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};

pub fn deserialize_string_cell(cell: Option<&Data>, default: &str) -> String {
    cell.and_then(|cell| cell.get_string())
        .unwrap_or(default)
        .to_string()
}

pub fn deserialize_date_cell(cell: Option<&Data>, default: f64) -> f64 {
    let excel_dt = cell
        .and_then(|cell| {
            if let Data::DateTime(excel_datetime) = cell {
                Some(*excel_datetime)
            } else {
                None
            }
        })
        .unwrap_or(ExcelDateTime::new(
            default,
            ExcelDateTimeType::DateTime,
            false,
        ));

    excel_dt.as_f64()
}

pub fn deserialize_int_cell(cell: Option<&Data>, default: i64) -> i64 {
    cell.map(|x| match x {
        Data::Int(x) => *x,
        Data::String(x) => x.parse::<i64>().unwrap_or(default),
        Data::Float(x) => *x as i64,
        _ => 0,
    })
    .unwrap()
}

pub fn deserialize_float_cell(cell: Option<&Data>, default: f64) -> f64 {
    cell.map(|x| match x {
        Data::Float(x) => *x,
        Data::String(x) => x.parse::<f64>().unwrap_or(default),
        Data::Int(x) => *x as f64,
        _ => 0.0,
    })
    .unwrap()
}

pub fn deserialize_string_date(cell: Option<&Data>) -> Result<DateTime<Utc>> {
    let string_date = cell.and_then(|c| c.get_string()).unwrap_or("");

    let parsed = NaiveDateTime::parse_from_str(string_date, "%Y-%m-%d %H:%M:%S")?;

    Ok(DateTime::from_naive_utc_and_offset(parsed, Utc))
}

pub fn join_date_and_time(
    date_cell: Option<&Data>,
    time_cell: Option<&Data>,
) -> Option<DateTime<Utc>> {
    // Initial values
    let excel_epoch = NaiveDate::from_ymd_opt(1899, 12, 30)?;
    let date_val = deserialize_date_cell(date_cell, 45658.0);
    let time_val = deserialize_date_cell(time_cell, 0.0);

    // Date
    let days = date_val.floor() as i64;
    let date = excel_epoch.checked_add_signed(Duration::days(days))?;

    // Time
    let total_seconds = (time_val * 24.0 * 60.0 * 60.0).round() as u32;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let time = NaiveTime::from_hms_opt(hours, minutes, seconds)?;

    let naive_datetime = date.and_time(time);

    let dt: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);

    Some(dt)
}
