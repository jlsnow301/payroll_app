use anyhow::{Context, Result};
use calamine::{open_workbook, Reader, Xlsx};

use crate::process::{
    deserialize::{
        types::{Order, TimeActivity},
        util::{
            deserialize_date_cell, deserialize_float_cell, deserialize_int_cell,
            deserialize_string_cell, deserialize_string_date, join_date_and_time,
        },
    },
    validate::validate_headers,
};

pub fn deserialize_caterease_excel(file_path: &str) -> Result<Vec<Order>> {
    let mut workbook: Xlsx<_> = open_workbook(file_path)
        .with_context(|| format!("Failed to open Excel file: {}", file_path))?;

    let worksheet = workbook
        .worksheet_range_at(0)
        .context("Couldn't find first worksheet")?
        .context("Error reading worksheet data")?;

    validate_headers(
        &worksheet,
        &[
            "Date",
            "Delivery Person",
            "Client/Organization",
            "Description",
            "Actual",
            "Grat",
            "Delivery Category",
            "Sub-Event #",
            "Kitchen Ready by",
            "Subtotal",
        ],
    )?;

    let mut orders = Vec::new();

    let last = worksheet.height() - 2;
    for (index, row) in worksheet.rows().enumerate() {
        match index {
            0 => continue,
            i if i == last => break,
            _ => {
                let datetime = match join_date_and_time(row.first(), row.get(8)) {
                    Some(dt) => dt,
                    None => continue,
                };

                let order = Order {
                    date: deserialize_date_cell(row.first(), 45658.0),
                    employee: deserialize_string_cell(row.get(1), ""),
                    client: deserialize_string_cell(row.get(2), ""),
                    description: deserialize_string_cell(row.get(3), ""),
                    count: deserialize_int_cell(row.get(4), 0),
                    grat: deserialize_float_cell(row.get(5), 0.0),
                    origin: deserialize_string_cell(row.get(6), ""),
                    event: deserialize_string_cell(row.get(7), ""),
                    ready: deserialize_date_cell(row.get(8), 0.0),
                    total: deserialize_float_cell(row.get(9), 0.0),
                    datetime,
                    expanded: false,
                };

                orders.push(order);
            }
        }
    }
    Ok(orders)
}

pub fn deserialize_intuit_excel(file_path: &str) -> Result<Vec<TimeActivity>> {
    let mut workbook: Xlsx<_> = open_workbook(file_path)
        .with_context(|| format!("Failed to open Excel file: {}", file_path))?;

    let worksheet = workbook
        .worksheet_range("Timesheets")
        .context("Cannot find sheet named 'Timesheets'")?;

    validate_headers(
        &worksheet,
        &[
            "First name",
            "Last name",
            "Username",
            "Start time",
            "End time",
            "Customer",
            "Hours",
            "Miles",
        ],
    )?;

    let mut time_sheets: Vec<TimeActivity> = Vec::new();

    for row in worksheet.rows().skip(1) {
        let activity_type = deserialize_string_cell(row.get(5), "");
        if activity_type != "Shift Total" {
            continue;
        }

        let in_result = deserialize_string_date(row.get(3));
        let in_time = match in_result {
            Ok(dt) => dt,
            Err(_) => continue,
        };

        let out_result = deserialize_string_date(row.get(4));
        let out_time = match out_result {
            Ok(dt) => dt,
            Err(_) => continue,
        };

        let first_name = deserialize_string_cell(row.first(), "");
        let last_name = deserialize_string_cell(row.get(1), "");
        // let full_name = format!("{} {}", first_name, last_name);

        let activity = TimeActivity {
            first_name,
            last_name,
            // full_name,
            in_time,
            out_time,
            hours: deserialize_float_cell(row.get(6), 0.0),
            miles: deserialize_float_cell(row.get(7), 0.0),
            matched: false,
        };

        time_sheets.push(activity);
    }

    Ok(time_sheets)
}
