use crate::{
    compare::{cross_reference_orders, ReferenceResult},
    deserialize::{deserialize_caterease_excel, deserialize_intuit_excel, Order, TimeActivity},
    expand::expand_orders,
    handlers::AppState,
    validate::{validate_order_input, validate_time_input},
    write::{build_themes, write_prepared_rows, write_unmatched_rows},
};
use anyhow::{anyhow, Context, Result};
use rust_xlsxwriter::workbook::Workbook;
use std::{path::Path, sync::MutexGuard};

pub const OUTPUT_PATH: &str = "formatted_payroll.xlsx";

pub fn get_filename(path: &Path) -> String {
    path.file_name()
        .unwrap()
        .to_string_lossy()
        .to_string()
        .replace(".xlsx", "")
}

pub fn get_path(file_path: &str) -> Result<&Path> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(anyhow!("File doesn't exist"));
    }

    Ok(path)
}

pub fn get_orders(file_path: &str) -> Result<Vec<Order>> {
    let orders: Vec<Order> = match deserialize_caterease_excel(file_path) {
        Ok(res) => res,
        Err(e) => return Err(anyhow!(e)),
    };

    if let Err(e) = validate_order_input(&orders) {
        return Err(anyhow!(e));
    }

    Ok(orders)
}

pub fn get_timesheet(file_path: &str) -> Result<Vec<TimeActivity>> {
    let timesheets: Vec<TimeActivity> = match deserialize_intuit_excel(file_path) {
        Ok(res) => res,
        Err(e) => return Err(anyhow!(e)),
    };

    if let Err(e) = validate_time_input(&timesheets) {
        return Err(anyhow!(e));
    }

    Ok(timesheets)
}

pub fn get_references(
    precision: i64,
    state: &mut MutexGuard<'_, AppState>,
) -> Result<ReferenceResult> {
    if state.caterease.is_empty() || state.intuit.is_empty() {
        return Err(anyhow!("Both documents must be linked"));
    }
    let mut expanded = expand_orders(&state.caterease);

    let reference_result = cross_reference_orders(&mut expanded, &mut state.intuit, precision);

    Ok(reference_result)
}

pub fn write_excel(referenced: &ReferenceResult, intuit: &[TimeActivity]) -> Result<()> {
    let mut workbook = Workbook::new();
    let orders_sheet = workbook
        .add_worksheet()
        .set_name("Orders")
        .context("Couldn't add orders worksheet")?;

    let themes = build_themes();

    write_prepared_rows(orders_sheet, &referenced.rows, &themes)?;

    let unmatched_sheet = workbook
        .add_worksheet()
        .set_name("Unmatched")
        .context("Couldn't add unmatched sheet")?;

    write_unmatched_rows(unmatched_sheet, intuit, &themes)?;

    workbook
        .save(OUTPUT_PATH)
        .context("Couldn't save workbook")?;

    Ok(())
}
