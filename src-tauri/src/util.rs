use crate::{
    compare::{cross_reference_orders, ReferenceResult},
    deserialize::{deserialize_caterease_excel, deserialize_intuit_excel, Order, TimeActivity},
    expand::expand_orders,
    handlers::AppState,
    validate::{validate_order_input, validate_time_input},
};

use anyhow::{anyhow, Result};
use std::{
    path::{Path, PathBuf},
    sync::MutexGuard,
};

pub const OUTPUT_PATH: &str = "formatted_payroll.xlsx";

pub fn get_filename(path: &Path) -> String {
    path.file_name()
        .unwrap()
        .to_string_lossy()
        .to_string()
        .replace(".xlsx", "")
}

pub fn get_path(file_path: &str) -> Result<PathBuf> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(anyhow!("File doesn't exist"));
    }

    Ok(path.to_path_buf())
}

pub fn get_orders(file_path: &str) -> Result<Vec<Order>> {
    let orders: Vec<Order> = match deserialize_caterease_excel(file_path) {
        Ok(res) => res,
        Err(e) => return Err(anyhow!(e)),
    };

    validate_order_input(&orders)?;

    Ok(orders)
}

pub fn get_timesheet(file_path: &str) -> Result<Vec<TimeActivity>> {
    let timesheets: Vec<TimeActivity> = match deserialize_intuit_excel(file_path) {
        Ok(res) => res,
        Err(e) => return Err(anyhow!(e)),
    };

    validate_time_input(&timesheets)?;

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
