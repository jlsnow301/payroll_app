use crate::{
    process::compare::{
        types::PreparedRow,
        util::{is_name_match, is_valid_order, is_within_one_hour},
    },
    process::deserialize::{Order, TimeActivity},
};

pub fn cross_reference_orders(
    orders: &[Order],
    time_sheets: &mut [TimeActivity],
) -> Vec<PreparedRow> {
    let mut to_write: Vec<PreparedRow> = Vec::new();

    for order in orders.iter() {
        let mut entry = PreparedRow {
            order: order.clone(),
            hours: 0.0,
            miles: 0.0,
        };

        let lower_emp = order.employee.to_lowercase();
        if !is_valid_order(lower_emp) {
            // Patio party or something
            to_write.push(entry);
            continue;
        }

        for time_activity in time_sheets.iter_mut() {
            if time_activity.matched {
                continue;
            }

            if !is_name_match(order, time_activity) {
                continue;
            }

            if is_within_one_hour(&order.datetime, &time_activity.in_time) {
                entry.hours = time_activity.hours;
                entry.miles = time_activity.miles;

                break;
            }
        }

        to_write.push(entry)
    }

    to_write
}
