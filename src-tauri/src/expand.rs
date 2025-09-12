use crate::deserialize::Order;

fn get_drivers(order: &Order) -> Vec<String> {
    let mut drivers: Vec<String> = Vec::new();

    let mut driver = order.employee.to_string();

    if driver.to_lowercase().contains("patio party") {
        drivers.push(driver);
        return drivers;
    }

    if let Some(start) = driver.find("(") {
        if let Some(end) = driver.find(")") {
            {
                let extra = driver[start + 1..end].to_string();
                driver = driver[..start].trim().to_string();

                drivers.push(extra);
            }
        }
    }

    if driver.contains(",") {
        let split = driver.split(",");

        split.for_each(|d| drivers.push(d.trim().to_string()));
        return drivers;
    } else if driver.contains("and") {
        let split = driver.split("and");

        split.for_each(|d| drivers.push(d.trim().to_string()));
        return drivers;
    }

    drivers.push(driver);

    drivers
}

pub fn expand_orders(orders: &[Order]) -> Vec<Order> {
    let mut expanded: Vec<Order> = Vec::new();

    for order in orders.iter() {
        let mut drivers = get_drivers(order);

        let was_expanded = drivers.len() > 1;

        if was_expanded {
            drivers.reverse(); // Put helpers last
        }

        for driver in drivers.iter() {
            let mut new_order = order.clone();

            if was_expanded {
                new_order.expanded = true;
            }

            new_order.employee = driver.to_string();

            expanded.push(new_order);
        }
    }

    expanded
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    fn base_order_with_employee(emp: &str) -> Order {
        Order {
            date: 1.0,
            employee: emp.to_string(),
            client: "".to_string(),
            description: "".to_string(),
            count: 0,
            grat: 0.0,
            origin: "".to_string(),
            event: "".to_string(),
            ready: 0.0,
            total: 0.0,
            datetime: Utc.with_ymd_and_hms(2025, 1, 1, 12, 0, 0).unwrap(),
            expanded: false,
        }
    }

    #[test]
    fn get_drivers_single() {
        let order = base_order_with_employee("John Doe");
        let drivers = get_drivers(&order);
        assert_eq!(drivers, vec!["John Doe"]);
    }

    #[test]
    fn get_drivers_comma_and_and() {
        let order = base_order_with_employee("Alice, Bob");
        let drivers = get_drivers(&order);
        assert_eq!(drivers, vec!["Alice", "Bob"]);

        let order2 = base_order_with_employee("Alice and Bob");
        let drivers2 = get_drivers(&order2);
        assert_eq!(drivers2, vec!["Alice", "Bob"]);
    }

    #[test]
    fn get_drivers_parentheses() {
        let order = base_order_with_employee("John (Helper)");
        let drivers = get_drivers(&order);
        // helper first, then the main driver
        assert_eq!(drivers, vec!["Helper", "John"]);
    }

    #[test]
    fn expand_orders_expands_and_marks() {
        let order = base_order_with_employee("Alice and Bob");
        let expanded = expand_orders(&[order]);
        // "Alice and Bob" is split into two; expand_orders reverses drivers before producing rows
        assert_eq!(expanded.len(), 2);
        // expanded flag set on both produced rows
        assert!(expanded[0].expanded && expanded[1].expanded);
        // order of produced employees should be reversed (helpers last in original get_drivers)
        let employees: Vec<_> = expanded.iter().map(|o| o.employee.clone()).collect();
        assert_eq!(employees, vec!["Bob".to_string(), "Alice".to_string()]);
    }
}
