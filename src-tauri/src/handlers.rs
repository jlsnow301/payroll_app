use serde::Serialize;
use serde_json::{json, Value};
use std::{env, path::PathBuf, sync::Mutex};
use tauri::State;
use tauri_plugin_opener::reveal_item_in_dir;

use crate::{
    constants::{CATEREASE_HEADERS, INTUIT_HEADERS},
    deserialize::{Order, TimeActivity},
    util::{get_driver_stats, get_filename, get_orders, get_path, get_references, get_timesheet},
    write::WorkbookWriter,
};

#[derive(Clone, Default)]
pub struct AppState {
    pub caterease: Vec<Order>,
    pub intuit: Vec<TimeActivity>,
}

#[derive(Serialize)]
struct ProcessResult {
    /// Orders with multiple drivers
    expanded: usize,
    /// Total matched times to orders
    matched: u32,
    /// Invalid orders
    skipped: u32,
    /// Total processed rows
    total: usize,
    /// Driver with most assignments
    top_used: String,
    /// The count
    top_used_count: u32,
    /// Most punctual driver by average time difference of clock in to ready time
    punctual: String,
    /// The diff
    punctual_avg: f64,
}

#[derive(Serialize)]
pub struct HeaderResult {
    caterease: Vec<String>,
    intuit: Vec<String>,
}

#[tauri::command]
pub fn get_headers() -> HeaderResult {
    HeaderResult {
        caterease: CATEREASE_HEADERS.clone(),
        intuit: INTUIT_HEADERS.clone(),
    }
}

#[tauri::command]
pub fn caterease_input(
    file_path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<String, String> {
    let mut state = state.lock().unwrap();

    let path = get_path(&file_path).map_err(|e| e.to_string())?;

    let orders = get_orders(&file_path).map_err(|e| e.to_string())?;

    state.caterease = orders;

    let file_name = get_filename(&path);

    Ok(file_name)
}

#[tauri::command]
pub fn intuit_input(
    file_path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<String, String> {
    let mut state = state.lock().unwrap();

    let path = get_path(&file_path).map_err(|e| e.to_string())?;

    let timesheets = get_timesheet(&file_path).map_err(|e| e.to_string())?;

    state.intuit = timesheets;

    let file_name = get_filename(&path);

    Ok(file_name)
}

#[tauri::command]
pub fn submit(precision: usize, state: State<'_, Mutex<AppState>>) -> Result<Value, String> {
    let mut state = state.lock().unwrap();

    let referenced = get_references(precision as i64, &mut state).map_err(|e| e.to_string())?;

    let total = referenced.rows.len();

    let mut excel_writer = WorkbookWriter::new();
    excel_writer
        .write_prepared(&referenced.rows)
        .map_err(|e| e.to_string())?;
    excel_writer
        .write_unmatched(&state.intuit)
        .map_err(|e| e.to_string())?;

    let mut path = PathBuf::from(env::var("USERPROFILE").unwrap());
    path.push("Documents");
    path.push("formatted_payroll.xlsx");

    excel_writer.save(&path).map_err(|e| e.to_string())?;

    let stats = get_driver_stats(&referenced.rows);

    let result = ProcessResult {
        expanded: total - state.caterease.len(),
        matched: referenced.matched,
        skipped: referenced.skipped,
        top_used: stats.top_used,
        top_used_count: stats.top_used_count,
        punctual: stats.punctual,
        punctual_avg: stats.punctual_avg,
        total,
    };

    reveal_item_in_dir(path).unwrap();

    Ok(json!(result))
}
