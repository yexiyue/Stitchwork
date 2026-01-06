use std::sync::Arc;
use tokio::sync::Mutex;

mod sse;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Arc::new(Mutex::new(sse::SseState::default())) as sse::SharedSseState)
        .setup(|_app| {
            #[cfg(mobile)]
            {
                _app.handle().plugin(tauri_plugin_barcode_scanner::init())?;
                _app.handle().plugin(tauri_plugin_biometric::init())?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            sse::connect_sse,
            sse::disconnect_sse
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
