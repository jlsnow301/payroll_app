use crate::deserialize::{Order, TimeActivity};
use anyhow::{anyhow, Context, Result};
use calamine::{Data, DataType, Range};

pub fn validate_headers(worksheet: &Range<Data>, expected_headers: &[&str]) -> Result<()> {
    for (col, header) in expected_headers.iter().enumerate() {
        let value = worksheet
            .get((0, col))
            .context(format!("Couldn't read header column {}", col))?
            .get_string()
            .context("Couldn't read header")?;

        if *header != value.trim().to_lowercase() {
            return Err(anyhow!(
                "Improper header in file: '{}' (Expected: '{}')",
                value,
                header
            ));
        }
    }

    Ok(())
}

pub fn validate_order_input(orders: &[Order]) -> Result<()> {
    // No orders
    if orders.is_empty() {
        return Err(anyhow!("No orders found in Excel file"));
    }

    // Missing first date
    let current_date = orders[0].date;
    if current_date == 45658.0 {
        return Err(anyhow!("First order date is invalid"));
    }

    // Missing valid dates
    for order in orders.iter() {
        if order.date == 45658.0 {
            return Err(anyhow!("Invalid order date found"));
        }
    }

    Ok(())
}

pub fn validate_time_input(timesheets: &[TimeActivity]) -> Result<()> {
    // No time activities
    if timesheets.is_empty() {
        return Err(anyhow!("No entries found in time sheet"));
    }

    // Can't reference
    for entry in timesheets.iter() {
        if entry.first_name.is_empty() {
            return Err(anyhow!("Unknown entry in timesheet"));
        }
    }

    Ok(())
}
