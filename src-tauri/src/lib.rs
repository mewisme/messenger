use tauri::{
    menu::{Menu, MenuItemKind, Submenu},
    Manager,
};
use tauri_plugin_positioner::{Position, WindowExt};

mod update;

use update::{check_for_updates, open_update_window_command};

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.move_window(Position::Center).unwrap();

            // Native menu bar sẽ tự động theo system theme (dark/light mode)
            // Không cần set theme cho menu bar vì nó được render bởi OS

            let check_update = tauri::menu::MenuItem::with_id(
                app,
                "check-update",
                "Check for Updates",
                true,
                None::<&str>,
            );

            if let Ok(check_update_item) = check_update {
                let help_submenu = Submenu::with_items(
                    app,
                    "Help",
                    true,
                    &[&MenuItemKind::MenuItem(check_update_item)],
                );

                if let Ok(help_menu) = help_submenu {
                    let main_menu = Menu::with_items(app, &[&MenuItemKind::Submenu(help_menu)]);

                    if let Ok(menu) = main_menu {
                        // Set menu cho toàn bộ ứng dụng - sẽ hiển thị như native menu bar
                        app.set_menu(menu).expect("Failed to set menu");

                        // Xử lý sự kiện menu
                        app.on_menu_event(|app, event| {
                            if event.id.as_ref() == "check-update" {
                                let app_handle = app.clone();
                                tauri::async_runtime::spawn(async move {
                                    let _ = update::open_update_window(app_handle).await;
                                });
                            }
                        });
                    }
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            check_for_updates,
            open_update_window_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
