use tauri::WebviewUrl;
use tauri_plugin_positioner::{Position, WindowExt};
use url::Url;

const MESSENGER_URL: &str = "https://www.messenger.com";
const OPEN_IN_BROWSER_SCHEME: &str = "tauri";
const OPEN_IN_BROWSER_HOST: &str = "localhost";
const OPEN_IN_BROWSER_PATH: &str = "/open-in-browser";

const CONTEXT_MENU_SCRIPT: &str = r#"
document.addEventListener('contextmenu', function(e) {
  var a = e.target.closest('a');
  if (a && a.href && a.href.startsWith('http')) {
    e.preventDefault();
    e.stopPropagation();
    window.location.href = 'tauri://localhost/open-in-browser?url=' + encodeURIComponent(a.href);
  }
}, true);
"#;

fn is_internal(url: &Url) -> bool {
    let host = match url.host_str() {
        Some(h) => h,
        None => return false,
    };
    host == "messenger.com"
        || host.ends_with(".messenger.com")
        || host == "facebook.com"
        || host.ends_with(".facebook.com")
        || host == "fb.com"
        || host.ends_with(".fb.com")
        || host == "fbcdn.net"
        || host.ends_with(".fbcdn.net")
        || host == "facebook.net"
        || host.ends_with(".facebook.net")
}

fn open_in_browser_request(url: &Url) -> Option<String> {
    if url.scheme() != OPEN_IN_BROWSER_SCHEME {
        return None;
    }
    if url.host_str() != Some(OPEN_IN_BROWSER_HOST) {
        return None;
    }
    if url.path() != OPEN_IN_BROWSER_PATH {
        return None;
    }
    url.query_pairs()
        .find(|(k, _)| k == "url")
        .map(|(_, v)| v.to_string())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let url = Url::parse(MESSENGER_URL).expect("valid initial URL");
            let window = tauri::WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url))
                .title("Messenger")
                .inner_size(800.0, 600.0)
                .min_inner_size(800.0, 600.0)
                .initialization_script(CONTEXT_MENU_SCRIPT)
                .on_navigation(move |url| {
                    if let Some(target) = open_in_browser_request(url) {
                        let _ = tauri_plugin_opener::open_url(&target, None::<&str>);
                        return false;
                    }
                    if is_internal(url) {
                        true
                    } else {
                        let _ = tauri_plugin_opener::open_url(url.as_str(), None::<&str>);
                        false
                    }
                })
                .build()?;
            window.move_window(Position::Center).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
