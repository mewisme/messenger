use tauri::Manager;
use tauri_plugin_positioner::{Position, WindowExt};

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.move_window(Position::Center).unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
