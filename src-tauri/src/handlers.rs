use serde::Serialize;
use serde_json::{json, Value};
use std::sync::Mutex;
use tauri::State;
use tauri_plugin_opener::reveal_item_in_dir;

use crate::{
    deserialize::{Order, TimeActivity},
    util::{
        get_filename, get_orders, get_path, get_references, get_timesheet, write_excel, OUTPUT_PATH,
    },
};

#[derive(Clone, Default, Serialize)]
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

#[tauri::command]
pub fn caterease_input(
    file_path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<String, String> {
    let mut state = state.lock().unwrap();

    let path = get_path(&file_path).map_err(|e| e.to_string())?;

    let orders = get_orders(&file_path).map_err(|e| e.to_string())?;

    state.caterease = orders;

    let file_name = get_filename(path);

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

    let file_name = get_filename(path);

    Ok(file_name)
}

#[tauri::command]
pub fn manual_review(precision: usize, state: State<'_, Mutex<AppState>>) -> Result<Value, String> {
    let mut state = state.lock().unwrap();

    let referenced = get_references(precision as i64, &mut state).map_err(|e| e.to_string())?;

    Ok(json!(referenced))
}

#[tauri::command]
pub fn submit(precision: usize, state: State<'_, Mutex<AppState>>) -> Result<Value, String> {
    let mut state = state.lock().unwrap();

    let referenced = get_references(precision as i64, &mut state).map_err(|e| e.to_string())?;

    let total = referenced.rows.len();

    write_excel(&referenced, &state.intuit).map_err(|e| e.to_string())?;

    let result = ProcessResult {
        expanded: total - state.caterease.len(),
        matched: referenced.matched,
        skipped: referenced.skipped,
        total,
    };

    reveal_item_in_dir(OUTPUT_PATH).unwrap();

    Ok(json!(result))
}
