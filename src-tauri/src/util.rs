use crate::{
    process::{
        compare::ReferenceResult,
        deserialize::{deserialize_caterease_excel, deserialize_intuit_excel, Order, TimeActivity},
        get_references,
        validate::{validate_order_input, validate_time_input},
        write_excel,
    },
    AppState,
};
use anyhow::{anyhow, Result};
use serde::Serialize;
use std::{path::Path, sync::MutexGuard};
use tauri_plugin_opener::reveal_item_in_dir;

pub const OUTPUT_PATH: &str = "formatted_payroll.xlsx";

/**
* # Util
*
* This file is for the reusable services underneath route handlers
*
*/

#[derive(Serialize)]
pub struct ProcessResult {
    pub expanded: usize,
    pub matched: u32,
    pub skipped: u32,
    pub total: usize,
}

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

pub fn handle_review(
    precision: usize,
    state: &mut MutexGuard<'_, AppState>,
) -> Result<ReferenceResult> {
    if state.caterease.is_empty() || state.intuit.is_empty() {
        return Err(anyhow!("Both documents must be linked"));
    }

    let mut cloned_timesheet = state.intuit.clone();

    let referenced = get_references(
        &mut state.caterease,
        &mut cloned_timesheet,
        precision as i64,
    )?;

    Ok(referenced)
}

pub fn handle_submit(
    precision: usize,
    state: &mut MutexGuard<'_, AppState>,
) -> Result<ProcessResult> {
    if state.caterease.is_empty() || state.intuit.is_empty() {
        return Err(anyhow!("Both documents must be linked"));
    }

    let mut cloned_timesheet = state.intuit.clone();

    let referenced = get_references(
        &mut state.caterease,
        &mut cloned_timesheet,
        precision as i64,
    )?;

    let total = referenced.rows.len();

    write_excel(&referenced, &state.intuit).map_err(|e| e.to_string());

    let result = ProcessResult {
        expanded: total - state.caterease.len(),
        matched: referenced.matched.clone(),
        skipped: referenced.skipped,
        total,
    };

    reveal_item_in_dir(OUTPUT_PATH).unwrap();

    Ok(result)
}
