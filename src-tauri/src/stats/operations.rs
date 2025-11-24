use crate::{
    compare::PreparedRow,
    stats::{
        punctual::find_most_punctual_driver,
        util::{calculate_time_difference, normalize_driver_name, update_driver_stats},
        DriverAccumulator, DriverStats,
    },
};

use std::collections::HashMap;

/// Determine the most punctual driver based on two factors:
/// 1. Fewest late clock-ins (clocking in after kitchen ready time)
/// 2. Lowest average signed difference from kitchen ready time
///
/// Logic:
/// 1. Filter rows that have a suggested_in time.
/// 2. Convert Excel serial `ready` value (f64) to a `DateTime<Utc>`.
/// 3. Compute signed difference in seconds (positive = late, negative = early).
/// 4. Aggregate per driver: count late instances and track average diff.
/// 5. Rank by: (1) fewest late clock-ins, then (2) smallest average difference.
///
/// If no rows have a suggested_in time this returns an empty string.
pub fn get_driver_stats(rows: &[PreparedRow]) -> DriverStats {
    let acc = build_driver_accumulator(rows);

    let (top_used, top_used_count) = find_most_used_driver(&acc);
    let (most_late, most_late_count) = find_most_late_driver(&acc);
    let (highest_late_percent_driver, highest_late_percent) =
        find_highest_late_percent_driver(&acc);
    let (latest_clock_in_driver, latest_clock_in_diff_minutes) = find_latest_clock_in_driver(&acc);
    let (punctual, punctual_avg, punctual_late_count) = find_most_punctual_driver(&acc);

    DriverStats {
        top_used,
        top_used_count,
        most_late,
        most_late_count,
        highest_late_percent_driver,
        highest_late_percent,
        latest_clock_in_driver,
        latest_clock_in_diff_minutes,
        punctual,
        punctual_avg,
        punctual_late_count,
    }
}

/// Process all rows and accumulate statistics for each driver
fn build_driver_accumulator(rows: &[PreparedRow]) -> HashMap<String, DriverAccumulator> {
    let mut acc: HashMap<String, DriverAccumulator> = HashMap::new();

    println!("\n=== Building Driver Accumulator ===");
    println!("Total rows to process: {}", rows.len());

    for row in rows {
        let driver = normalize_driver_name(&row.order.employee);
        if let Some(diff_seconds) = calculate_time_difference(row) {
            let entry = acc
                .entry(driver.clone())
                .or_insert_with(DriverAccumulator::default);
            update_driver_stats(entry, diff_seconds);
        }
    }

    println!("\n=== Driver Summary ===");
    let mut driver_vec: Vec<_> = acc.iter().collect();
    driver_vec.sort_by_key(|(_, v)| std::cmp::Reverse(v.count));
    for (driver, v) in driver_vec {
        if v.count == 0 {
            continue;
        }
        let avg_minutes = (v.total_diff_seconds as f64 / v.count as f64) / 60.0;
        let late_percent = (v.late_count as f64 / v.count as f64) * 100.0;
        let latest_minutes = v.max_late_diff_seconds as f64 / 60.0;
        println!(
            "  {}: {} deliveries, {} late ({:.1}%), avg diff: {:.2} min, latest: {:.2} min late",
            driver, v.count, v.late_count, late_percent, avg_minutes, latest_minutes
        );
    }
    acc
}

/// Find the driver with the most deliveries
fn find_most_used_driver(acc: &HashMap<String, DriverAccumulator>) -> (String, u32) {
    let result = acc
        .iter()
        .max_by_key(|(_, v)| v.count)
        .map(|(driver, v)| (driver.clone(), v.count))
        .unwrap_or_default();
    println!("\n=== Most Used Driver ===");
    println!("  Winner: {} with {} deliveries", result.0, result.1);
    result
}

/// Find the driver with the most late clock-ins
fn find_most_late_driver(acc: &HashMap<String, DriverAccumulator>) -> (String, u32) {
    let result = acc
        .iter()
        .max_by_key(|(_, v)| v.late_count)
        .map(|(driver, v)| (driver.clone(), v.late_count))
        .unwrap_or_default();
    println!("\n=== Most Late Driver ===");
    println!("  Winner: {} with {} late clock-ins", result.0, result.1);
    result
}

fn find_highest_late_percent_driver(acc: &HashMap<String, DriverAccumulator>) -> (String, f64) {
    let mut best: Option<(String, f64)> = None;
    for (driver, v) in acc.iter() {
        if v.count == 0 {
            continue;
        }
        let percent = (v.late_count as f64 / v.count as f64) * 100.0;
        if best.as_ref().map_or(true, |(_, bp)| percent > *bp) {
            best = Some((driver.clone(), percent));
        }
    }
    let result = best.unwrap_or_default();
    println!("\n=== Highest Late Percent Driver ===");
    if !result.0.is_empty() {
        println!("  Winner: {} at {:.2}% late", result.0, result.1);
    } else {
        println!("  No data available");
    }
    result
}

fn find_latest_clock_in_driver(acc: &HashMap<String, DriverAccumulator>) -> (String, f64) {
    let result = acc
        .iter()
        .max_by_key(|(_, v)| v.max_late_diff_seconds)
        .map(|(driver, v)| (driver.clone(), v.max_late_diff_seconds as f64 / 60.0))
        .unwrap_or_default();
    println!("\n=== Latest Single Clock-In ===");
    if !result.0.is_empty() {
        println!("  Winner: {} at {:.2} minutes late", result.0, result.1);
    } else {
        println!("  No late clock-ins recorded");
    }
    result
}
