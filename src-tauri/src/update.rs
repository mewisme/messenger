use tauri::{Emitter, Manager, WebviewWindowBuilder};

pub async fn open_update_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(existing_window) = app.get_webview_window("updater") {
        existing_window
            .set_focus()
            .map_err(|e| format!("Failed to focus window: {}", e))?;
        return Ok(());
    }

    let url = if cfg!(debug_assertions) {
        tauri::WebviewUrl::External(
            format!("http://localhost:1420/update.html")
                .parse()
                .map_err(|_| "Failed to parse localhost URL".to_string())?,
        )
    } else {
        tauri::WebviewUrl::App("/update.html".into())
    };

    let _window = WebviewWindowBuilder::new(&app, "updater", url)
        .title("Check for Updates")
        .inner_size(500.0, 400.0)
        .resizable(false)
        .build()
        .map_err(|e| format!("Failed to create window: {}", e))?;

    if let Some(window) = app.get_webview_window("updater") {
        let _ = window.center();
    }

    Ok(())
}

#[tauri::command]
pub async fn open_update_window_command(app: tauri::AppHandle) -> Result<(), String> {
    open_update_window(app).await
}

#[tauri::command]
pub async fn check_for_updates(app: tauri::AppHandle) -> Result<String, String> {
    use tauri_plugin_updater::UpdaterExt;

    if let Some(window) = app.get_webview_window("updater") {
        let _ = window.emit(
            "update-status",
            serde_json::json!({
                "status": "checking",
                "message": "Checking for updates..."
            }),
        );
    }

    let updater = app.updater().map_err(|e| {
        if let Some(window) = app.get_webview_window("updater") {
            let _ = window.emit(
                "update-status",
                serde_json::json!({
                    "status": "error",
                    "message": format!("Failed to initialize updater: {}", e)
                }),
            );
        }
        format!("Failed to get updater: {}", e)
    })?;

    match updater.check().await {
        Ok(update) => {
            if let Some(update) = update {
                if let Some(window) = app.get_webview_window("updater") {
                    let _ = window.emit(
                        "update-status",
                        serde_json::json!({
                            "status": "available",
                            "message": format!("Update available: v{}", update.version),
                            "version": update.version,
                            "body": update.body
                        }),
                    );
                }

                let app_clone = app.clone();
                match update
                    .download_and_install(
                        |chunk_length: usize, content_length: Option<u64>| {
                            if let Some(window) = app_clone.get_webview_window("updater") {
                                let downloaded = chunk_length as u64;
                                let total = content_length.unwrap_or(0);
                                let progress = if total > 0 {
                                    (downloaded as f64 / total as f64) * 100.0
                                } else {
                                    0.0
                                };
                                let _ = window.emit(
                                    "update-progress",
                                    serde_json::json!({
                                        "progress": progress,
                                        "downloaded": downloaded,
                                        "total": total
                                    }),
                                );
                            }
                        },
                        || {
                            if let Some(window) = app_clone.get_webview_window("updater") {
                                let _ = window.emit(
                                    "update-status",
                                    serde_json::json!({
                                        "status": "installing",
                                        "message": "Installing update..."
                                    }),
                                );
                            }
                        },
                    )
                    .await
                {
                    Ok(_) => {
                        if let Some(window) = app.get_webview_window("updater") {
                            let _ = window.emit(
                                "update-status",
                                serde_json::json!({
                                    "status": "complete",
                                    "message": "Update installed. Restarting application..."
                                }),
                            );
                        }
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                        app.restart();
                    }
                    Err(e) => {
                        if let Some(window) = app.get_webview_window("updater") {
                            let _ = window.emit(
                                "update-status",
                                serde_json::json!({
                                    "status": "error",
                                    "message": format!("Failed to install update: {}", e)
                                }),
                            );
                        }
                        Err(format!("Failed to download update: {}", e))
                    }
                }
            } else {
                if let Some(window) = app.get_webview_window("updater") {
                    let _ = window.emit(
                        "update-status",
                        serde_json::json!({
                            "status": "none",
                            "message": "You are using the latest version."
                        }),
                    );
                }
                Ok("No updates available".to_string())
            }
        }
        Err(e) => {
            if let Some(window) = app.get_webview_window("updater") {
                let _ = window.emit(
                    "update-status",
                    serde_json::json!({
                        "status": "error",
                        "message": format!("Failed to check for updates: {}", e)
                    }),
                );
            }
            Err(format!("Failed to check for updates: {}", e))
        }
    }
}
