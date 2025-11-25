use std::collections::HashMap;

use crate::stats::DriverAccumulator;

/// Find the driver(s) with the single latest clock-in (up to 3-way tie)
pub fn find_latest_clock_in_driver(acc: &HashMap<String, DriverAccumulator>) -> (String, f64) {
    let max_late_diff = acc
        .iter()
        .map(|(_, v)| v.max_late_diff_seconds)
        .max()
        .unwrap_or(0);
    if max_late_diff == 0 {
        println!("\n=== Latest Single Clock-In ===");
        println!("  No late clock-ins recorded");
        return (String::new(), 0.0);
    }
    let mut winners: Vec<&String> = acc
        .iter()
        .filter(|(_, v)| v.max_late_diff_seconds == max_late_diff)
        .map(|(driver, _)| driver)
        .collect();
    winners.sort();
    winners.truncate(3);
    let minutes = max_late_diff as f64 / 60.0;
    let result = (
        winners
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(", "),
        minutes,
    );
    println!("\n=== Latest Single Clock-In ===");
    println!("  Winner: {} at {:.2} minutes late", result.0, result.1);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_accumulator(max_late_diff_seconds: i64) -> DriverAccumulator {
        DriverAccumulator {
            total_diff_seconds: 0,
            count: 1,
            late_count: if max_late_diff_seconds > 0 { 1 } else { 0 },
            max_late_diff_seconds,
        }
    }

    #[test]
    fn test_empty_accumulator() {
        let acc: HashMap<String, DriverAccumulator> = HashMap::new();
        let (driver, minutes) = find_latest_clock_in_driver(&acc);
        assert!(driver.is_empty());
        assert_eq!(minutes, 0.0);
    }

    #[test]
    fn test_all_zero_late_diff() {
        let mut acc = HashMap::new();
        acc.insert("Driver A".to_string(), make_accumulator(0));
        acc.insert("Driver B".to_string(), make_accumulator(0));
        let (driver, minutes) = find_latest_clock_in_driver(&acc);
        assert!(driver.is_empty());
        assert_eq!(minutes, 0.0);
    }

    #[test]
    fn test_single_driver_late() {
        let mut acc = HashMap::new();
        acc.insert("Driver A".to_string(), make_accumulator(300)); // 5 minutes late
        let (driver, minutes) = find_latest_clock_in_driver(&acc);
        assert_eq!(driver, "Driver A");
        assert!((minutes - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_multiple_drivers_clear_winner() {
        let mut acc = HashMap::new();
        acc.insert("Driver A".to_string(), make_accumulator(120)); // 2 minutes late
        acc.insert("Driver B".to_string(), make_accumulator(600)); // 10 minutes late
        acc.insert("Driver C".to_string(), make_accumulator(300)); // 5 minutes late
        let (driver, minutes) = find_latest_clock_in_driver(&acc);
        assert_eq!(driver, "Driver B");
        assert!((minutes - 10.0).abs() < 0.0001);
    }

    #[test]
    fn test_two_way_tie() {
        let mut acc = HashMap::new();
        acc.insert("Alpha".to_string(), make_accumulator(600)); // 10 minutes late
        acc.insert("Beta".to_string(), make_accumulator(600)); // 10 minutes late
        acc.insert("Gamma".to_string(), make_accumulator(300)); // 5 minutes late
        let (driver, minutes) = find_latest_clock_in_driver(&acc);
        assert_eq!(driver, "Alpha, Beta"); // sorted alphabetically
        assert!((minutes - 10.0).abs() < 0.0001);
    }

    #[test]
    fn test_three_way_tie() {
        let mut acc = HashMap::new();
        acc.insert("Charlie".to_string(), make_accumulator(900)); // 15 minutes late
        acc.insert("Alice".to_string(), make_accumulator(900)); // 15 minutes late
        acc.insert("Bob".to_string(), make_accumulator(900)); // 15 minutes late
        let (driver, minutes) = find_latest_clock_in_driver(&acc);
        assert_eq!(driver, "Alice, Bob, Charlie"); // sorted alphabetically
        assert!((minutes - 15.0).abs() < 0.0001);
    }

    #[test]
    fn test_more_than_three_way_tie_truncates() {
        let mut acc = HashMap::new();
        acc.insert("A".to_string(), make_accumulator(1200)); // 20 minutes late
        acc.insert("B".to_string(), make_accumulator(1200)); // 20 minutes late
        acc.insert("C".to_string(), make_accumulator(1200)); // 20 minutes late
        acc.insert("D".to_string(), make_accumulator(1200)); // 20 minutes late
        let (driver, minutes) = find_latest_clock_in_driver(&acc);
        assert_eq!(driver, "A, B, C"); // truncated to 3, sorted alphabetically
        assert!((minutes - 20.0).abs() < 0.0001);
    }

    #[test]
    fn test_negative_diff_ignored() {
        let mut acc = HashMap::new();
        acc.insert("Early".to_string(), make_accumulator(-300)); // 5 minutes early
        acc.insert("Late".to_string(), make_accumulator(180)); // 3 minutes late
        let (driver, minutes) = find_latest_clock_in_driver(&acc);
        assert_eq!(driver, "Late");
        assert!((minutes - 3.0).abs() < 0.0001);
    }

    #[test]
    fn test_fractional_minutes() {
        let mut acc = HashMap::new();
        acc.insert("Driver A".to_string(), make_accumulator(90)); // 1.5 minutes late
        let (driver, minutes) = find_latest_clock_in_driver(&acc);
        assert_eq!(driver, "Driver A");
        assert!((minutes - 1.5).abs() < 0.0001);
    }
}
