use std::collections::HashMap;

use crate::stats::DriverAccumulator;

/// Find the driver(s) with the most late clock-ins (up to 3-way tie)
pub fn find_most_late_driver(acc: &HashMap<String, DriverAccumulator>) -> (String, u32) {
    let max_late = acc.iter().map(|(_, v)| v.late_count).max().unwrap_or(0);
    if max_late == 0 {
        return (String::new(), 0);
    }
    let mut winners: Vec<&String> = acc
        .iter()
        .filter(|(_, v)| v.late_count == max_late)
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
        max_late,
    );
    println!("\n=== Most Late Driver ===");
    println!("  Winner: {} with {} late clock-ins", result.0, result.1);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_accumulator(late_count: u32, count: u32) -> DriverAccumulator {
        DriverAccumulator {
            late_count,
            count,
            total_diff_seconds: 0,
            max_late_diff_seconds: 0,
        }
    }

    #[test]
    fn test_empty_accumulator() {
        let acc: HashMap<String, DriverAccumulator> = HashMap::new();
        let (driver, count) = find_most_late_driver(&acc);
        assert_eq!(driver, "");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_all_drivers_zero_late() {
        let mut acc = HashMap::new();
        acc.insert("Alice".to_string(), create_accumulator(0, 5));
        acc.insert("Bob".to_string(), create_accumulator(0, 10));

        let (driver, count) = find_most_late_driver(&acc);
        assert_eq!(driver, "");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_single_driver_with_late() {
        let mut acc = HashMap::new();
        acc.insert("Alice".to_string(), create_accumulator(3, 10));

        let (driver, count) = find_most_late_driver(&acc);
        assert_eq!(driver, "Alice");
        assert_eq!(count, 3);
    }

    #[test]
    fn test_multiple_drivers_clear_winner() {
        let mut acc = HashMap::new();
        acc.insert("Alice".to_string(), create_accumulator(2, 10));
        acc.insert("Bob".to_string(), create_accumulator(5, 10));
        acc.insert("Charlie".to_string(), create_accumulator(1, 10));

        let (driver, count) = find_most_late_driver(&acc);
        assert_eq!(driver, "Bob");
        assert_eq!(count, 5);
    }

    #[test]
    fn test_two_way_tie() {
        let mut acc = HashMap::new();
        acc.insert("Alice".to_string(), create_accumulator(5, 10));
        acc.insert("Bob".to_string(), create_accumulator(5, 10));
        acc.insert("Charlie".to_string(), create_accumulator(2, 10));

        let (driver, count) = find_most_late_driver(&acc);
        assert_eq!(driver, "Alice, Bob");
        assert_eq!(count, 5);
    }

    #[test]
    fn test_three_way_tie() {
        let mut acc = HashMap::new();
        acc.insert("Alice".to_string(), create_accumulator(4, 10));
        acc.insert("Bob".to_string(), create_accumulator(4, 10));
        acc.insert("Charlie".to_string(), create_accumulator(4, 10));

        let (driver, count) = find_most_late_driver(&acc);
        assert_eq!(driver, "Alice, Bob, Charlie");
        assert_eq!(count, 4);
    }

    #[test]
    fn test_more_than_three_way_tie_truncates() {
        let mut acc = HashMap::new();
        acc.insert("Alice".to_string(), create_accumulator(3, 10));
        acc.insert("Bob".to_string(), create_accumulator(3, 10));
        acc.insert("Charlie".to_string(), create_accumulator(3, 10));
        acc.insert("Diana".to_string(), create_accumulator(3, 10));

        let (driver, count) = find_most_late_driver(&acc);
        // Should be truncated to first 3 alphabetically: Alice, Bob, Charlie
        assert_eq!(driver, "Alice, Bob, Charlie");
        assert_eq!(count, 3);
    }

    #[test]
    fn test_drivers_sorted_alphabetically() {
        let mut acc = HashMap::new();
        acc.insert("Zoe".to_string(), create_accumulator(5, 10));
        acc.insert("Aaron".to_string(), create_accumulator(5, 10));
        acc.insert("Mike".to_string(), create_accumulator(5, 10));

        let (driver, count) = find_most_late_driver(&acc);
        assert_eq!(driver, "Aaron, Mike, Zoe");
        assert_eq!(count, 5);
    }
}
