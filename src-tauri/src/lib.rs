// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod clipboard_monitor;
mod paste_shortcut;
mod system_tray;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            // init the called tray icon
            system_tray::init_tray(app)?;
            clipboard_monitor::init_clipboard_monitor(app)?;

            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

                let cmd_shit_v_shortcut =
                    Shortcut::new(Some(Modifiers::SHIFT.union(Modifiers::SUPER)), Code::KeyV);

                paste_shortcut::init_paste_shortcut(app, cmd_shit_v_shortcut)?;

                app.global_shortcut()
                    .register(cmd_shit_v_shortcut.clone())
                    .expect("failed to register global shortcut");
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
