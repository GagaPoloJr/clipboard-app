use std::sync::{Arc, Mutex};
use std::time::Duration;

use tauri::{App, AppHandle};
use tauri::Emitter;
use tauri_plugin_clipboard_manager::ClipboardExt;

pub fn init_clipboard_monitor(app: &App) -> tauri::Result<()> {
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

    println!("clipboard monitor initialized");
    Ok(())
}
