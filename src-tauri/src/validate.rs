use crate::deserialize::{Order, TimeActivity};
use anyhow::{anyhow, Context, Result};
use calamine::{Data, DataType, Range};

pub fn validate_headers(worksheet: &Range<Data>, expected_headers: &[String]) -> Result<()> {
    // 1. Improper number of headers
    if expected_headers.len() != worksheet.width() {
        return Err(anyhow!(
            "Improper number of columns in file: Found {}, expected {}",
            worksheet.width(),
            expected_headers.len()
        ));
    }

    // 2. Wrong headers (positional check)
    for (col, expected) in expected_headers.iter().enumerate() {
        let actual = worksheet
            .get((0, col))
            .context(format!("Couldn't read header column {}", col + 1))?
            .get_string()
            .context(format!("Header at column {} is not a string", col + 1))?
            .trim();

        if !actual.eq_ignore_ascii_case(expected) {
            return Err(anyhow!(
                "Improper header in file: '{}' (Expected: '{}')",
                actual,
                expected
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
