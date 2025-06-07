// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use tauri::AppHandle;
use tauri::Emitter;
use tauri_plugin_clipboard_manager::ClipboardExt;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let handle: AppHandle = app.handle().clone();
            let last_clip = Arc::new(Mutex::new(String::new()));
            let last_clip_clone = last_clip.clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    match handle.clipboard().read_text() {
                        Ok(content) => {
                            let mut last = last_clip_clone.lock().unwrap();
                            if *last != content {
                                *last = content.clone();
                                let _ = handle.emit("clipboard-update", content);
                            }
                        }
                        Err(e) => {
                            eprintln!("Clipboard read error: {:?}", e);
                        }
                    }
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
