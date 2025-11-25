use std::collections::HashMap;

use crate::stats::DriverAccumulator;

/// Find the driver(s) with the most deliveries (up to 3-way tie)
pub fn find_most_used_driver(acc: &HashMap<String, DriverAccumulator>) -> (String, u32) {
    let max_count = acc.iter().map(|(_, v)| v.count).max().unwrap_or(0);
    if max_count == 0 {
        return (String::new(), 0);
    }
    let mut winners: Vec<&String> = acc
        .iter()
        .filter(|(_, v)| v.count == max_count)
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
        max_count,
    );
    println!("\n=== Most Used Driver ===");
    println!("  Winner: {} with {} deliveries", result.0, result.1);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_accumulator(count: u32, late_count: u32) -> DriverAccumulator {
        DriverAccumulator {
            total_diff_seconds: 0,
            count,
            late_count,
            max_late_diff_seconds: 0,
        }
    }

    #[test]
    fn test_find_most_used_driver_single_winner() {
        let mut acc: HashMap<String, DriverAccumulator> = HashMap::new();
        acc.insert("Alice".to_string(), create_accumulator(10, 0));
        acc.insert("Bob".to_string(), create_accumulator(5, 0));
        acc.insert("Charlie".to_string(), create_accumulator(3, 0));

        let (winner, count) = find_most_used_driver(&acc);
        assert_eq!(winner, "Alice");
        assert_eq!(count, 10);
    }

    #[test]
    fn test_find_most_used_driver_two_way_tie() {
        let mut acc: HashMap<String, DriverAccumulator> = HashMap::new();
        acc.insert("Alice".to_string(), create_accumulator(10, 0));
        acc.insert("Bob".to_string(), create_accumulator(10, 0));
        acc.insert("Charlie".to_string(), create_accumulator(3, 0));

        let (winner, count) = find_most_used_driver(&acc);
        assert_eq!(winner, "Alice, Bob");
        assert_eq!(count, 10);
    }

    #[test]
    fn test_find_most_used_driver_three_way_tie() {
        let mut acc: HashMap<String, DriverAccumulator> = HashMap::new();
        acc.insert("Alice".to_string(), create_accumulator(10, 0));
        acc.insert("Bob".to_string(), create_accumulator(10, 0));
        acc.insert("Charlie".to_string(), create_accumulator(10, 0));

        let (winner, count) = find_most_used_driver(&acc);
        assert_eq!(winner, "Alice, Bob, Charlie");
        assert_eq!(count, 10);
    }

    #[test]
    fn test_find_most_used_driver_more_than_three_way_tie_truncates() {
        let mut acc: HashMap<String, DriverAccumulator> = HashMap::new();
        acc.insert("Alice".to_string(), create_accumulator(10, 0));
        acc.insert("Bob".to_string(), create_accumulator(10, 0));
        acc.insert("Charlie".to_string(), create_accumulator(10, 0));
        acc.insert("Diana".to_string(), create_accumulator(10, 0));

        let (winner, count) = find_most_used_driver(&acc);
        // Should only include first 3 alphabetically
        assert_eq!(winner, "Alice, Bob, Charlie");
        assert_eq!(count, 10);
    }

    #[test]
    fn test_find_most_used_driver_empty() {
        let acc: HashMap<String, DriverAccumulator> = HashMap::new();

        let (winner, count) = find_most_used_driver(&acc);
        assert_eq!(winner, "");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_find_most_used_driver_all_zero_count() {
        let mut acc: HashMap<String, DriverAccumulator> = HashMap::new();
        acc.insert("Alice".to_string(), create_accumulator(0, 0));
        acc.insert("Bob".to_string(), create_accumulator(0, 0));

        let (winner, count) = find_most_used_driver(&acc);
        assert_eq!(winner, "");
        assert_eq!(count, 0);
    }
}
