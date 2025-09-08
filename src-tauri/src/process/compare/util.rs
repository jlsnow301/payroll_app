use std::sync::LazyLock;

use chrono::{DateTime, Duration, Utc};

use crate::process::deserialize::TimeActivity;

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
        if !driver.contains(time_name.as_str()) {
            return false;
        }
    } else {
        let time_name = time_activity.first_name.to_lowercase();
        if !driver
            .split(" ")
            .next()
            .unwrap()
            .contains(time_name.as_str())
        {
            return false;
        }
    }

    true
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
