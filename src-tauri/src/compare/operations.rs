use crate::{
    compare::{
        util::{is_name_match, is_valid_order, is_within_time},
        PreparedRow, ReferenceResult,
    },
    deserialize::{Order, TimeActivity},
};

pub fn cross_reference_orders(
    orders: &mut [Order],
    time_sheets: &mut [TimeActivity],
    precision: i64,
) -> ReferenceResult {
    let mut rows: Vec<PreparedRow> = Vec::new();
    let mut matched = 0;
    let mut skipped = 0;

    for order in orders.iter() {
        let mut entry = PreparedRow {
            order: order.clone(),
            hours: 0.0,
            miles: 0.0,
            nearest: None,
        };

        let lower_emp = order.employee.to_lowercase();
        if !is_valid_order(&lower_emp) {
            // Patio party or something
            rows.push(entry);
            skipped += 1;
            continue;
        }

        for time_activity in time_sheets.iter_mut() {
            if time_activity.matched {
                continue;
            }

            if !is_name_match(&lower_emp, time_activity, order.expanded) {
                continue;
            }

            if is_within_time(&order.datetime, &time_activity.in_time, precision) {
                entry.hours = time_activity.hours;
                entry.miles = time_activity.miles;
                entry.nearest = Some(time_activity.in_time);
                matched += 1;
                time_activity.matched = true;

                break;
            }
        }

        rows.push(entry)
    }

    ReferenceResult {
        rows,
        matched,
        skipped,
    }
}
