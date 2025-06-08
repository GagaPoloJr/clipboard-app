use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};
use tauri::{App, AppHandle, Emitter, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardEntry {
    content: String,
    timestamp: String,
}

pub type ClipboardHistory = Arc<Mutex<Vec<ClipboardEntry>>>;

const MAX_HISTORY: usize = 20;

#[tauri::command]
pub fn get_clipboard_history(app: AppHandle) -> Vec<ClipboardEntry> {
    let history = load_history(&app);
    let history = history.lock().unwrap().clone();
    history
}

/// Returns the path to `database/clipboard.json` inside app data dir
fn get_clipboard_file_path(app: &AppHandle) -> PathBuf {
    let base_dir = app
        .path()
        .app_data_dir()
        .expect("❌ Could not resolve app data dir");

    let database_dir = base_dir.join("database");

    if let Err(e) = fs::create_dir_all(&database_dir) {
        eprintln!("❌ Failed to create clipboard history folder: {}", e);
    }

    let file_path = database_dir.join("clipboard.json");
    println!("📁 Clipboard file path: {:?}", file_path);
    file_path
}

fn save_history(app: &AppHandle, history: &ClipboardHistory) {
    let path = get_clipboard_file_path(app);
    match serde_json::to_string_pretty(&*history.lock().unwrap()) {
        Ok(json) => {
            if let Err(e) = fs::write(&path, json) {
                eprintln!("❌ Error writing clipboard history: {}", e);
            } else {
                println!("💾 Clipboard history saved to {:?}", path);
            }
        }
        Err(e) => {
            eprintln!("❌ Error serializing clipboard history: {}", e);
        }
    }
}

fn load_history(app: &AppHandle) -> ClipboardHistory {
    let path = get_clipboard_file_path(app);

    match fs::read_to_string(&path) {
        Ok(content) => match serde_json::from_str::<Vec<ClipboardEntry>>(&content) {
            Ok(data) => {
                println!("✅ Loaded clipboard history from file");
                Arc::new(Mutex::new(data))
            }
            Err(e) => {
                eprintln!("⚠️ Failed to parse clipboard.json: {}", e);
                Arc::new(Mutex::new(Vec::new()))
            }
        },
        Err(_) => {
            println!("📂 clipboard.json not found, starting with empty history");
            Arc::new(Mutex::new(Vec::new()))
        }
    }
}

pub fn init_clipboard_monitor(app: &App) -> tauri::Result<ClipboardHistory> {
    let handle = app.handle().clone();
    let history = load_history(&handle);

    // Force create file & directory at startup
    save_history(&handle, &history);

    let history_clone = history.clone();
    let last_clip = Arc::new(Mutex::new(String::new()));
    let last_clip_clone = last_clip.clone();

    println!("📋 Clipboard monitor initialized.");

    tauri::async_runtime::spawn(async move {
        loop {
            match handle.clipboard().read_text() {
                Ok(content) => {
                    let mut last = last_clip_clone.lock().unwrap();

                    if *last != content {
                        *last = content.clone();
                        let timestamp = Local::now().to_rfc3339();

                        let mut history = history_clone.lock().unwrap();
                        history.push(ClipboardEntry {
                            content: content.clone(),
                            timestamp,
                        });

                        if history.len() > MAX_HISTORY {
                            history.remove(0);
                        }

                        save_history(&handle, &history_clone);
                        println!("📋 Clipboard updated: {}", content);

                        let _ = handle.emit("clipboard-update", content);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to read clipboard: {:?}", e);
                }
            }

            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });

    Ok(history)
}
