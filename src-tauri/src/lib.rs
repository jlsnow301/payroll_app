use std::{path::Path, sync::Mutex};

use tauri::{Builder, Manager, State};

use crate::{process::process, responses::ApiResponse};

mod process;
mod responses;

#[derive(Default)]
struct AppState {
    caterease: String,
    intuit: String,
}

#[tauri::command]
fn caterease_input(file_path: String, state: State<'_, Mutex<AppState>>) -> ApiResponse<String> {
    let path = Path::new(file_path.as_str());

    if !path.exists() {
        return ApiResponse::error("FILE_NOT_FOUND", "File doesn't exist".into());
    }

    let mut state = state.lock().unwrap();

    state.caterease = file_path;

    ApiResponse::success("Success".to_string())
}

#[tauri::command]
fn intuit_input(file_path: String, state: State<'_, Mutex<AppState>>) -> ApiResponse<String> {
    let path = Path::new(file_path.as_str());

    if !path.exists() {
        return ApiResponse::error("FILE_NOT_FOUND", "File doesn't exist".into());
    }

    let mut state = state.lock().unwrap();

    state.intuit = file_path;

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

    match process(&state.caterease, &state.intuit) {
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
