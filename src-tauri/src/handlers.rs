use serde::Serialize;
use serde_json::{json, Value};
use std::sync::Mutex;
use tauri::State;
use tauri_plugin_opener::reveal_item_in_dir;

use crate::{
    compare::PreparedRow,
    constants::{CATEREASE_HEADERS, INTUIT_HEADERS},
    deserialize::{Order, TimeActivity},
    util::{get_filename, get_orders, get_path, get_references, get_timesheet, OUTPUT_PATH},
    write::WorkbookWriter,
};

#[derive(Clone, Default)]
pub struct AppState {
    pub caterease: Vec<Order>,
    pub intuit: Vec<TimeActivity>,
}

#[derive(Serialize)]
struct ProcessResult {
    expanded: usize,
    matched: u32,
    skipped: u32,
    total: usize,
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
pub fn manual_review(precision: usize, state: State<'_, Mutex<AppState>>) -> Result<Value, String> {
    let mut state = state.lock().unwrap();

    let referenced = get_references(precision as i64, &mut state).map_err(|e| e.to_string())?;

    Ok(json!(referenced))
}

#[tauri::command]
pub fn manual_input(rows: Vec<PreparedRow>) -> Result<String, String> {
    let mut excel_writer = WorkbookWriter::new();
    excel_writer
        .write_prepared(&rows)
        .map_err(|e| e.to_string())?;

    excel_writer.save().map_err(|e| e.to_string())?;

    reveal_item_in_dir(OUTPUT_PATH).unwrap();

    Ok("Success".to_string())
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

    excel_writer.save().map_err(|e| e.to_string())?;

    let result = ProcessResult {
        expanded: total - state.caterease.len(),
        matched: referenced.matched,
        skipped: referenced.skipped,
        total,
    };

    reveal_item_in_dir(OUTPUT_PATH).unwrap();

    Ok(json!(result))
}
