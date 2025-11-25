use std::collections::HashMap;

use crate::stats::DriverAccumulator;

/// Find the driver(s) with the highest late percentage (up to 3-way tie)
pub fn find_highest_late_percent_driver(acc: &HashMap<String, DriverAccumulator>) -> (String, f64) {
    // First pass: find the maximum percentage
    let mut max_percent: f64 = 0.0;
    for (_, v) in acc.iter() {
        if v.count == 0 {
            continue;
        }
        let percent = (v.late_count as f64 / v.count as f64) * 100.0;
        if percent > max_percent {
            max_percent = percent;
        }
    }
    if max_percent == 0.0 {
        println!("\n=== Highest Late Percent Driver ===");
        println!("  No data available");
        return (String::new(), 0.0);
    }
    // Second pass: collect all drivers with that percentage (up to 3)
    let mut winners: Vec<&String> = acc
        .iter()
        .filter(|(_, v)| {
            if v.count == 0 {
                return false;
            }
            let percent = (v.late_count as f64 / v.count as f64) * 100.0;
            (percent - max_percent).abs() < 0.0001 // float equality check
        })
        .map(|(driver, _)| driver)
        .collect();
    winners.sort();
    winners.truncate(3);
    let result = (
        winners
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(", "),
        max_percent,
    );
    println!("\n=== Highest Late Percent Driver ===");
    println!("  Winner: {} at {:.2}% late", result.0, result.1);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_accumulator(count: u32, late_count: u32) -> DriverAccumulator {
        DriverAccumulator {
            total_diff_seconds: 0,
            count,
            late_count,
            max_late_diff_seconds: 0,
        }
    }

    #[test]
    fn test_empty_accumulator() {
        let acc: HashMap<String, DriverAccumulator> = HashMap::new();
        let (driver, percent) = find_highest_late_percent_driver(&acc);
        assert!(driver.is_empty());
        assert_eq!(percent, 0.0);
    }

    #[test]
    fn test_all_zero_counts() {
        let mut acc = HashMap::new();
        acc.insert("Driver A".to_string(), make_accumulator(0, 0));
        acc.insert("Driver B".to_string(), make_accumulator(0, 0));
        let (driver, percent) = find_highest_late_percent_driver(&acc);
        assert!(driver.is_empty());
        assert_eq!(percent, 0.0);
    }

    #[test]
    fn test_no_late_entries() {
        let mut acc = HashMap::new();
        acc.insert("Driver A".to_string(), make_accumulator(10, 0));
        acc.insert("Driver B".to_string(), make_accumulator(5, 0));
        let (driver, percent) = find_highest_late_percent_driver(&acc);
        assert!(driver.is_empty());
        assert_eq!(percent, 0.0);
    }

    #[test]
    fn test_single_driver_with_late() {
        let mut acc = HashMap::new();
        acc.insert("Driver A".to_string(), make_accumulator(10, 5)); // 50% late
        let (driver, percent) = find_highest_late_percent_driver(&acc);
        assert_eq!(driver, "Driver A");
        assert!((percent - 50.0).abs() < 0.0001);
    }

    #[test]
    fn test_multiple_drivers_clear_winner() {
        let mut acc = HashMap::new();
        acc.insert("Driver A".to_string(), make_accumulator(10, 2)); // 20% late
        acc.insert("Driver B".to_string(), make_accumulator(10, 8)); // 80% late
        acc.insert("Driver C".to_string(), make_accumulator(10, 5)); // 50% late
        let (driver, percent) = find_highest_late_percent_driver(&acc);
        assert_eq!(driver, "Driver B");
        assert!((percent - 80.0).abs() < 0.0001);
    }

    #[test]
    fn test_two_way_tie() {
        let mut acc = HashMap::new();
        acc.insert("Alpha".to_string(), make_accumulator(10, 5)); // 50% late
        acc.insert("Beta".to_string(), make_accumulator(20, 10)); // 50% late
        acc.insert("Gamma".to_string(), make_accumulator(10, 3)); // 30% late
        let (driver, percent) = find_highest_late_percent_driver(&acc);
        assert_eq!(driver, "Alpha, Beta"); // sorted alphabetically
        assert!((percent - 50.0).abs() < 0.0001);
    }

    #[test]
    fn test_three_way_tie() {
        let mut acc = HashMap::new();
        acc.insert("Charlie".to_string(), make_accumulator(10, 5)); // 50% late
        acc.insert("Alice".to_string(), make_accumulator(20, 10)); // 50% late
        acc.insert("Bob".to_string(), make_accumulator(4, 2)); // 50% late
        let (driver, percent) = find_highest_late_percent_driver(&acc);
        assert_eq!(driver, "Alice, Bob, Charlie"); // sorted alphabetically
        assert!((percent - 50.0).abs() < 0.0001);
    }

    #[test]
    fn test_more_than_three_way_tie_truncates() {
        let mut acc = HashMap::new();
        acc.insert("A".to_string(), make_accumulator(10, 5)); // 50% late
        acc.insert("B".to_string(), make_accumulator(20, 10)); // 50% late
        acc.insert("C".to_string(), make_accumulator(4, 2)); // 50% late
        acc.insert("D".to_string(), make_accumulator(8, 4)); // 50% late
        let (driver, percent) = find_highest_late_percent_driver(&acc);
        assert_eq!(driver, "A, B, C"); // truncated to 3, sorted alphabetically
        assert!((percent - 50.0).abs() < 0.0001);
    }

    #[test]
    fn test_hundred_percent_late() {
        let mut acc = HashMap::new();
        acc.insert("Always Late".to_string(), make_accumulator(10, 10)); // 100% late
        acc.insert("Sometimes Late".to_string(), make_accumulator(10, 5)); // 50% late
        let (driver, percent) = find_highest_late_percent_driver(&acc);
        assert_eq!(driver, "Always Late");
        assert!((percent - 100.0).abs() < 0.0001);
    }
}
