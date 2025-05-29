use std::sync::Mutex;

use tauri::{generate_context, Builder, Manager};

mod app_data;
use app_data::AppData;

#[derive(Debug)]
pub enum RuntimeOptions {
    OpenInFullscreen,
    HideAppbar,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(options: Vec<RuntimeOptions>) {
    Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppData::from(options)));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .run(generate_context!())
        .expect("error while running tauri application");
}
