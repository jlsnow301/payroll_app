use chrono::{DateTime, TimeZone, Utc};

use crate::{compare::PreparedRow, stats::DriverAccumulator};

/// Normalize driver name to first word only
pub fn normalize_driver_name(employee: &str) -> String {
    employee
        .split_whitespace()
        .next()
        .unwrap_or(employee)
        .to_string()
}

/// Calculate the signed difference in seconds between suggested clock-in and kitchen ready time
/// Returns None if the row doesn't have valid data
pub fn calculate_time_difference(row: &PreparedRow) -> Option<i64> {
    let suggested_in = row.suggested_in?;
    let serial = row.order.ready;

    if serial <= 0.0 {
        println!(
            "  [SKIP] {}: Invalid ready serial ({}) - no suggested_in",
            normalize_driver_name(&row.order.employee),
            serial
        );
        return None;
    }

    let ready_utc = convert_excel_serial_to_utc(serial, suggested_in)?;
    let diff_seconds = suggested_in.signed_duration_since(ready_utc).num_seconds();

    // Skip unrealistic diffs (greater than 24h) as likely data errors
    if diff_seconds.abs() > 86_400 {
        println!(
            "  [SKIP] {}: Unrealistic time diff ({} hours)",
            normalize_driver_name(&row.order.employee),
            diff_seconds / 3600
        );
        return None;
    }

    Some(diff_seconds)
}

/// Convert Excel serial time to UTC DateTime, using the clock-in date
pub fn convert_excel_serial_to_utc(
    serial: f64,
    suggested_in: DateTime<Utc>,
) -> Option<DateTime<Utc>> {
    // Extract time-of-day from Excel serial (fractional part only, no date)
    let fractional = serial - serial.floor();
    let seconds_in_day = (fractional * 86_400.0).round() as u32;
    let hours = seconds_in_day / 3600;
    let minutes = (seconds_in_day % 3600) / 60;
    let seconds = seconds_in_day % 60;

    // Use the clock-in date and construct ready time in LA timezone
    let clock_in_date = suggested_in.date_naive();
    let ready_naive = clock_in_date
        .and_hms_opt(hours, minutes, seconds)
        .unwrap_or_else(|| clock_in_date.and_hms_opt(0, 0, 0).unwrap());

    // Convert LA time to UTC
    chrono_tz::America::Los_Angeles
        .from_local_datetime(&ready_naive)
        .single()
        .map(|dt| dt.with_timezone(&Utc))
        .or_else(|| Some(Utc.from_utc_datetime(&ready_naive)))
}

/// Update driver statistics with a new time difference
pub fn update_driver_stats(entry: &mut DriverAccumulator, diff_seconds: i64) {
    entry.total_diff_seconds += diff_seconds;
    entry.count += 1;
    if diff_seconds > 0 {
        entry.late_count += 1;
        if diff_seconds > entry.max_late_diff_seconds {
            entry.max_late_diff_seconds = diff_seconds;
        }
    }
}
