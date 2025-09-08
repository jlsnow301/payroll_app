use serde::Serialize;
use serde_json::{json, Value};
use std::sync::Mutex;
use tauri::State;

use crate::{
    process::deserialize::{Order, TimeActivity},
    util::{get_filename, get_orders, get_path, get_timesheet, handle_review, handle_submit},
};

/**
* ## Handlers
*
* This file is in charge of routing front end requests to their associated services
* and returning a web-friendly result
*/

#[derive(Clone, Default, Serialize)]
pub struct AppState {
    pub caterease: Vec<Order>,
    pub intuit: Vec<TimeActivity>,
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

    let referenced = handle_review(precision, &mut state).map_err(|e| e.to_string())?;

    Ok(json!(referenced))
}

#[tauri::command]
pub fn submit(precision: usize, state: State<'_, Mutex<AppState>>) -> Result<Value, String> {
    let mut state = state.lock().unwrap();

    let processed = handle_submit(precision, &mut state).map_err(|e| e.to_string())?;

    Ok(json!(processed))
}
