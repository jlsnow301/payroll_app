use std::sync::Mutex;

use tauri::{Builder, Manager};
use tauri_plugin_updater::UpdaterExt;

use crate::handlers::{
    caterease_input, get_headers, intuit_input, manual_input, manual_review, submit, AppState,
};

mod compare;
mod constants;
mod deserialize;
mod expand;
mod handlers;
mod util;
mod validate;
mod write;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                update(handle).await.unwrap();
            });
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            caterease_input,
            get_headers,
            intuit_input,
            manual_input,
            manual_review,
            submit
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// See the docs at https://tauri.app/plugin/updater/#checking-for-updates
async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;

        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    println!("download finished");
                },
            )
            .await?;

        println!("update installed");
        app.restart();
    }

    Ok(())
}
