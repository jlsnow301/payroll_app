use std::collections::HashMap;

use crate::stats::DriverAccumulator;

/// Find the most punctual driver (minimum 3 deliveries required)
/// Ranked by: (1) fewest late clock-ins, then (2) avg diff closest to zero (but not positive)
///
/// For the "closest to zero" comparison:
/// - Negative avg diff means clocking in early (good)
/// - We want the driver whose avg is closest to 0 without being late
/// - e.g., -3.11 min is better than -21.38 min (closer to on-time)
pub fn find_most_punctual_driver(acc: &HashMap<String, DriverAccumulator>) -> (String, f64, u32) {
    let mut best: Option<(String, f64, u32)> = None;
    for (driver, v) in acc.iter() {
        if v.count < 3 {
            continue;
        }
        let avg_minutes = (v.total_diff_seconds as f64 / v.count as f64) / 60.0;
        let is_better = best.as_ref().map_or(true, |(_, best_avg, best_late)| {
            if v.late_count < *best_late {
                // Fewer late clock-ins is always better
                true
            } else if v.late_count == *best_late {
                // Same late count: prefer avg closest to zero
                // For early arrivals (negative), closer to zero is better
                // e.g., -3 is better than -21 because abs(-3) < abs(-21)
                avg_minutes.abs() < best_avg.abs()
            } else {
                false
            }
        });
        if is_better {
            best = Some((driver.clone(), avg_minutes, v.late_count));
        }
    }
    let result = best.unwrap_or_default();
    println!("\n=== Most Punctual Driver ===");
    if !result.0.is_empty() {
        println!(
            "  Winner: {} with {} late clock-ins, avg diff: {:.2} min",
            result.0, result.2, result.1
        );
    } else {
        println!("  No driver qualified (need minimum 3 deliveries)");
    }
    println!("===================================\n");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that the most punctual driver is the one with avg diff closest to zero
    /// Given:
    ///   DriverA: 6 deliveries, 0 late, avg diff: -21.38 min
    ///   DriverB: 6 deliveries, 0 late, avg diff: -3.11 min
    /// Expected: DriverB wins because -3.11 is closer to 0 than -21.38
    #[test]
    fn test_most_punctual_prefers_closer_to_zero() {
        let mut acc: HashMap<String, DriverAccumulator> = HashMap::new();

        // DriverA: 6 deliveries, 0 late, avg diff -21.38 min = -1282.8 seconds total / 6
        // total_diff_seconds = -1282.8 * 6 = -7696.8 => round to -7697
        acc.insert(
            "DriverA".to_string(),
            DriverAccumulator {
                total_diff_seconds: (-21.38 * 60.0 * 6.0) as i64, // -7696
                count: 6,
                late_count: 0,
                max_late_diff_seconds: 0,
            },
        );

        // DriverB: 6 deliveries, 0 late, avg diff -3.11 min
        // total_diff_seconds = -3.11 * 60 * 6 = -1119.6 => round to -1120
        acc.insert(
            "DriverB".to_string(),
            DriverAccumulator {
                total_diff_seconds: (-3.11 * 60.0 * 6.0) as i64, // -1119
                count: 6,
                late_count: 0,
                max_late_diff_seconds: 0,
            },
        );

        let (winner, avg, late_count) = find_most_punctual_driver(&acc);

        assert_eq!(
            winner, "DriverB",
            "DriverB should be most punctual (avg diff closer to 0)"
        );
        assert_eq!(late_count, 0);
        // avg should be around -3.11
        assert!(
            avg > -4.0 && avg < -2.0,
            "Expected avg around -3.11, got {}",
            avg
        );
    }

    /// Test that fewer late clock-ins still takes priority over avg diff
    #[test]
    fn test_most_punctual_fewer_late_wins() {
        let mut acc: HashMap<String, DriverAccumulator> = HashMap::new();

        // Driver A: 6 deliveries, 1 late, avg diff: -1 min (very close to zero)
        acc.insert(
            "DriverA".to_string(),
            DriverAccumulator {
                total_diff_seconds: (-1.0 * 60.0 * 6.0) as i64,
                count: 6,
                late_count: 1,
                max_late_diff_seconds: 60,
            },
        );

        // Driver B: 6 deliveries, 0 late, avg diff: -20 min (far from zero)
        acc.insert(
            "DriverB".to_string(),
            DriverAccumulator {
                total_diff_seconds: (-20.0 * 60.0 * 6.0) as i64,
                count: 6,
                late_count: 0,
                max_late_diff_seconds: 0,
            },
        );

        let (winner, _, late_count) = find_most_punctual_driver(&acc);

        assert_eq!(
            winner, "DriverB",
            "DriverB should win with 0 late clock-ins"
        );
        assert_eq!(late_count, 0);
    }

    /// Test that drivers with fewer than 3 deliveries are excluded
    #[test]
    fn test_most_punctual_minimum_deliveries() {
        let mut acc: HashMap<String, DriverAccumulator> = HashMap::new();

        // Driver with only 2 deliveries (should be excluded)
        acc.insert(
            "TooFew".to_string(),
            DriverAccumulator {
                total_diff_seconds: 0,
                count: 2,
                late_count: 0,
                max_late_diff_seconds: 0,
            },
        );

        // Driver with 3 deliveries (should qualify)
        acc.insert(
            "JustEnough".to_string(),
            DriverAccumulator {
                total_diff_seconds: (-5.0 * 60.0 * 3.0) as i64,
                count: 3,
                late_count: 0,
                max_late_diff_seconds: 0,
            },
        );

        let (winner, _, _) = find_most_punctual_driver(&acc);

        assert_eq!(
            winner, "JustEnough",
            "Only driver with 3+ deliveries should win"
        );
    }
}
