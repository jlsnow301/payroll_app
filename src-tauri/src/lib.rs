use std::{path::Path, sync::Mutex};

use tauri::{Builder, Manager, State};

use crate::{
    process::{
        deserialize::{deserialize_caterease_excel, deserialize_intuit_excel, Order, TimeActivity},
        process,
        validate::{validate_order_input, validate_time_input},
    },
    responses::ApiResponse,
};

mod process;
mod responses;

#[derive(Default)]
struct AppState {
    caterease: Vec<Order>,
    intuit: Vec<TimeActivity>,
}

#[tauri::command]
fn caterease_input(file_path: String, state: State<'_, Mutex<AppState>>) -> ApiResponse<String> {
    let path = Path::new(file_path.as_str());

    if !path.exists() {
        return ApiResponse::error("FILE_NOT_FOUND", "File doesn't exist".into());
    }

    let orders: Vec<Order> = match deserialize_caterease_excel(&file_path) {
        Ok(res) => res,
        Err(e) => return ApiResponse::error("INVALID_FORMAT", e.to_string()),
    };

    if let Err(e) = validate_order_input(&orders) {
        return ApiResponse::error("VALIDATION_ERROR", e.to_string());
    }

    let mut state = state.lock().unwrap();

    state.caterease = orders;

    ApiResponse::success("Success".to_string())
}

#[tauri::command]
fn intuit_input(file_path: String, state: State<'_, Mutex<AppState>>) -> ApiResponse<String> {
    let path = Path::new(file_path.as_str());

    if !path.exists() {
        return ApiResponse::error("FILE_NOT_FOUND", "File doesn't exist".into());
    }

    let timesheets: Vec<TimeActivity> = match deserialize_intuit_excel(&file_path) {
        Ok(res) => res,
        Err(e) => return ApiResponse::error("INVALID_FORMAT", e.to_string()),
    };

    if let Err(e) = validate_time_input(&timesheets) {
        return ApiResponse::error("VALIDATION_ERROR", e.to_string());
    }

    let mut state = state.lock().unwrap();

    state.intuit = timesheets;

    ApiResponse::success("Success".to_string())
}

#[tauri::command]
fn submit(state: State<'_, Mutex<AppState>>) -> ApiResponse<String> {
    let state = state.lock().unwrap();

    if state.caterease.is_empty() || state.intuit.is_empty() {
        return ApiResponse::error(
            "REQUIRED_FIELD_MISSING",
            "Both documents must be linked".into(),
        );
    }

    let mut cloned_timesheet = state.intuit.clone();

    match process(&state.caterease, &mut cloned_timesheet) {
        Err(error) => ApiResponse::error("INTERNAL_ERROR", error.to_string()),
        Ok(rows) => ApiResponse::success(format!("Wrote {} rows", rows)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            caterease_input,
            intuit_input,
            submit
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
