use logger::debug;

#[derive(Debug)]
pub enum RuntimeOptions {}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(options: &Vec<RuntimeOptions>) {
    debug!("Running Tauri application with options: {:?}", options);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
