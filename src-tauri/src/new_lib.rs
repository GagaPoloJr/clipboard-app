// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::Duration,
};

use tauri::tray::TrayIconBuilder;
use tauri::AppHandle;
use tauri::Emitter;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_store::StoreBuilder;

const MAX_HISTORY: usize = 20; //limit the size for the data
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let tray = TrayIconBuilder::new().build(app)?;

            let handle: AppHandle = app.handle();

            let history_store_path = "clipboard.json"; //store the history copied

            let store = Arc::new(Mutex::new(
                StoreBuilder::new(app, history_store_path).build(),
            ));

            // load saved history
            let clipboard_history: Arc<Mutex<VecDeque<String>>> =
                Arc::new(Mutex::new(VecDeque::new()));
            {
                let mut s = store.lock().unwrap();
                s.load().unwrap_or_default();
                if let Some(history) = s.get("history").and_then(|v| v.as_array()) {
                    let mut queue = clipboard_history.lock().unwrap();
                    for entry in history {
                        if let Some(text) = entry.as_str() {
                            queue.push_back(text.to_string());
                        }
                    }
                }
            }

            // polling clipboard
            let last_clip = Arc::new(Mutex::new(String::new()));
            let last_clip_clone = last_clip.clone();

            let history_clone = clipboard_history.clone();
            let store_clone: Arc<
                Mutex<Result<Arc<tauri_plugin_store::Store<_>>, tauri_plugin_store::Error>>,
            > = store.clone();
            let handle_clone = handle.clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    match handle_clone.clipboard().read_text() {
                        Ok(content) => {
                            let mut last = last_clip.lock().unwrap();
                            if *last != content {
                                *last = content.clone();

                                let mut history = history_clone.lock().unwrap();
                                if !history.contains(&content) {
                                    if history.len() >= MAX_HISTORY {
                                        history.pop_front();
                                    }
                                    history.push_back(content.clone());

                                    // save the content
                                    let mut store = store_clone.lock().unwrap();
                                    store.insert("history", history.iter().collect());
                                    let _ = store.save();

                                    let _ = handle_clone.emit("clipboard-update", content);
                                }
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
