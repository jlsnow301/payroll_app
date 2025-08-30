use std::sync::LazyLock;

use chrono::{DateTime, Duration, Utc};

use crate::process::deserialize::{Order, TimeActivity};

static INVALID_NAMES: LazyLock<Vec<String>> =
    LazyLock::new(|| vec!["patio party".to_string(), "pickup".to_string()]);

pub fn is_within_one_hour(time_a: &DateTime<Utc>, time_b: &DateTime<Utc>) -> bool {
    let duration_limit = Duration::hours(1);

    let diff = (*time_a - *time_b).abs();

    diff <= duration_limit
}

pub fn is_name_match(order: &Order, time_activity: &TimeActivity) -> bool {
    let lower_emp = order.employee.to_lowercase();

    if !order.expanded {
        let time_name = time_activity.full_name.to_lowercase();
        if lower_emp != time_name {
            return false;
        }
    } else {
        let time_name = time_activity.first_name.to_lowercase();
        if !lower_emp
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

pub fn is_valid_order(name: String) -> bool {
    if name.is_empty() {
        return false;
    }

    for invalid in &*INVALID_NAMES {
        if name.contains(invalid) {
            return false;
        }
    }

    true
}
