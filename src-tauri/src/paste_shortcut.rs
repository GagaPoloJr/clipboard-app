use tauri::{App, Emitter, Manager};
use tauri_plugin_global_shortcut::{Shortcut, ShortcutState};

pub fn init_paste_shortcut(app: &App, paste_shortcut: Shortcut) -> tauri::Result<()> {
    let main_window = app.get_webview_window("main").unwrap();
    // register the global shortcut plugin with a handler
    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler({
                let app = app.handle().clone();
                move |_app, shortcut, event| {
                    println!("{:?}", shortcut);
                    if shortcut == &paste_shortcut {
                        match event.state() {
                            ShortcutState::Pressed => {
                                println!("CmdOrCtrl + Shift + V pressed!");
                                // use emit for inject data into frontend
                                let _ = app.emit("open-clipboard-history", ());
                                if main_window.is_visible().unwrap() {
                                    main_window.hide().unwrap();
                                } else {
                                    main_window.show().unwrap();
                                    main_window.set_focus().unwrap();
                                }
                            }
                            ShortcutState::Released => {
                                println!("CmdOrCtrl + Shift + V Released!");
                            }
                        }
                    }
                }
            })
            .build(),
    )?;

    println!("Paste shortcut registered.");
    Ok(())
}
