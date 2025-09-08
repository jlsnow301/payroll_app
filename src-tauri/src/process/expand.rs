use crate::process::deserialize::Order;

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
