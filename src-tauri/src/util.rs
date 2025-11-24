use crate::{
    compare::{cross_reference_orders, PreparedRow, ReferenceResult},
    deserialize::{deserialize_caterease_excel, deserialize_intuit_excel, Order, TimeActivity},
    expand::expand_orders,
    handlers::AppState,
    validate::{validate_order_input, validate_time_input},
};

use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, NaiveDate, TimeZone, Utc};
use std::{
    path::{Path, PathBuf},
    sync::MutexGuard,
};

pub fn get_filename(path: &Path) -> String {
    path.file_name()
        .unwrap()
        .to_string_lossy()
        .to_string()
        .replace(".xlsx", "")
}

pub fn get_path(file_path: &str) -> Result<PathBuf> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(anyhow!("File doesn't exist"));
    }

    Ok(path.to_path_buf())
}

pub fn get_orders(file_path: &str) -> Result<Vec<Order>> {
    let orders: Vec<Order> = match deserialize_caterease_excel(file_path) {
        Ok(res) => res,
        Err(e) => return Err(anyhow!(e)),
    };

    validate_order_input(&orders)?;

    Ok(orders)
}

pub fn get_timesheet(file_path: &str) -> Result<Vec<TimeActivity>> {
    let timesheets: Vec<TimeActivity> = match deserialize_intuit_excel(file_path) {
        Ok(res) => res,
        Err(e) => return Err(anyhow!(e)),
    };

    validate_time_input(&timesheets)?;

    Ok(timesheets)
}

pub fn get_references(
    precision: i64,
    state: &mut MutexGuard<'_, AppState>,
) -> Result<ReferenceResult> {
    if state.caterease.is_empty() || state.intuit.is_empty() {
        return Err(anyhow!("Both documents must be linked"));
    }
    let mut expanded = expand_orders(&state.caterease);

    let reference_result = cross_reference_orders(&mut expanded, &mut state.intuit, precision);

    Ok(reference_result)
}

pub struct DriverStats {
    pub top_used: String,
    pub top_used_count: u32,
    pub punctual: String,
    // Average absolute clock-in difference in minutes
    pub punctual_avg: f64,
}

/// Determine the most punctual driver based on average absolute difference
/// between the kitchen "ready" time and the matched (suggested) clock in time.
///
/// Logic:
/// 1. Filter rows that have a suggested_in time.
/// 2. Convert Excel serial `ready` value (f64) to a `DateTime<Utc>`.
/// 3. Compute absolute difference in seconds between ready and suggested_in.
/// 4. Aggregate per driver and compute average difference.
/// 5. Return the driver with the smallest average difference (most punctual).
///
/// If no rows have a suggested_in time this returns an empty string.
#[allow(dead_code)]
pub fn get_driver_stats(rows: &[PreparedRow]) -> DriverStats {
    use std::collections::HashMap;

    // Excel epoch: 1899-12-30 (to align with serial where 1 = 1900-01-01)
    let excel_epoch = NaiveDate::from_ymd_opt(1899, 12, 30).unwrap();

    // Accumulators: driver -> (total_diff_seconds, count)
    let mut acc: HashMap<String, (i64, u32)> = HashMap::new();

    for row in rows {
        // Normalize driver name to first word only
        let driver = row
            .order
            .employee
            .split_whitespace()
            .next()
            .unwrap_or(&row.order.employee)
            .to_string();

        let entry = acc.entry(driver.clone()).or_insert((0, 0));

        // Only accumulate and increment count if we have suggested_in and a valid ready serial
        if let Some(suggested_in) = row.suggested_in {
            let serial = row.order.ready;
            if serial > 0.0 {
                let whole_days = serial.floor() as i64;
                let fractional = serial - serial.floor();
                let seconds = (fractional * 86_400.0).round() as i64;
                let naive_ready = excel_epoch.and_hms_opt(0, 0, 0).unwrap()
                    + Duration::days(whole_days)
                    + Duration::seconds(seconds);

                let ready_utc: DateTime<Utc> = Utc.from_utc_datetime(&naive_ready);
                let diff_seconds = suggested_in
                    .signed_duration_since(ready_utc)
                    .num_seconds()
                    .abs();

                // Skip unrealistic diffs (0 or greater than 24h) as likely data errors
                if diff_seconds == 0 || diff_seconds > 86_400 {
                    continue;
                }

                entry.0 += diff_seconds;
                entry.1 += 1; // Count only deliveries with a valid diff
            }
        }
    }

    // Find most used driver
    let (top_used, top_used_count) = acc
        .iter()
        .max_by_key(|(_, (_, count))| count)
        .map(|(driver, (_, count))| (driver.clone(), *count))
        .unwrap_or_default();

    // Find most punctual driver (min 3 valid diffs)
    let mut best: Option<(String, f64)> = None;
    for (driver, (total_diff, count)) in acc.into_iter() {
        // Skip if fewer than 3 valid diffs or no accumulated diff seconds
        if count < 3 || total_diff == 0 {
            continue;
        }
        // Convert average from seconds to minutes for display
        let avg = (total_diff as f64 / count as f64) / 60.0;
        if best.as_ref().map_or(true, |(_, best_avg)| avg < *best_avg) {
            best = Some((driver, avg));
        }
    }

    let (punctual, punctual_avg) = best.unwrap_or_default();

    DriverStats {
        punctual,
        punctual_avg,
        top_used,
        top_used_count,
    }
}
