use std::sync::LazyLock;

use chrono::{DateTime, Duration, Utc};

use crate::deserialize::TimeActivity;

static INVALID_NAMES: LazyLock<Vec<String>> =
    LazyLock::new(|| vec!["patio party".to_string(), "pickup".to_string()]);

pub fn is_within_time(time_a: &DateTime<Utc>, time_b: &DateTime<Utc>, precision: i64) -> bool {
    let duration_limit = Duration::hours(precision);

    let diff = (*time_a - *time_b).abs();

    diff <= duration_limit
}

pub fn is_name_match(driver: &str, time_activity: &TimeActivity, is_expanded: bool) -> bool {
    if !is_expanded {
        let time_name = time_activity.last_name.to_lowercase();
        driver.contains(time_name.as_str())
    } else {
        let time_name = time_activity.first_name.to_lowercase();
        if let Some(driver_first) = driver.split(" ").next() {
            driver_first.contains(time_name.as_str())
        } else {
            false
        }
    }
}

pub fn is_valid_order(name: &str) -> bool {
    if name.trim().is_empty() {
        return false;
    }

    for invalid in &*INVALID_NAMES {
        if name.contains(invalid) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deserialize::TimeActivity;
    use chrono::{TimeZone, Utc};

    fn make_time_activity(first: &str, last: &str) -> TimeActivity {
        TimeActivity {
            first_name: first.to_string(),
            last_name: last.to_string(),
            in_time: Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap(),
            out_time: Utc.with_ymd_and_hms(2023, 1, 1, 11, 0, 0).unwrap(),
            hours: 1.0,
            miles: 0.0,
            matched: false,
        }
    }

    #[test]
    fn within_time_true_and_false() {
        let a = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let b = Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap();

        assert!(is_within_time(&a, &b, 2));
        assert!(!is_within_time(&a, &b, 1));
    }

    #[test]
    fn name_match_not_expanded_checks_last_name() {
        let activity = make_time_activity("John", "Doe");
        let driver = "john doe";
        assert!(is_name_match(driver, &activity, false));
    }

    #[test]
    fn name_match_expanded_checks_first_name() {
        let activity = make_time_activity("Bob", "Smith");
        let driver = "bob smith"; // The input is always lower cased in ops
        assert!(is_name_match(driver, &activity, true));
    }

    #[test]
    fn invalid_order_detection() {
        assert!(!is_valid_order(""));
        assert!(!is_valid_order("patio party"));
        assert!(is_valid_order("some valid driver"));
    }
}
