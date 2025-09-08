use std::sync::Mutex;

use tauri::{Builder, Manager};

use crate::handlers::{caterease_input, intuit_input, manual_review, submit, AppState};

mod handlers;
mod process;
mod util;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            caterease_input,
            intuit_input,
            manual_review,
            submit
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
