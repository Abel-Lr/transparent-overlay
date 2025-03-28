use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

#[tauri::command]
pub async fn close_config_window(app: AppHandle) {
    app.get_webview_window("config")
        .map(|window| window.close().expect("Error Closing the config window"))
        .expect("Can't find config window");
}

pub fn create_config_window(app: &tauri::AppHandle) -> WebviewWindow {
    WebviewWindowBuilder::new(app, "config", WebviewUrl::App("config.html".into()))
        .title("Transparent Overlay - Config")
        .resizable(false)
        .center()
        .inner_size(350.0, 250.0)
        .maximizable(false)
        .theme(Some(tauri::Theme::Dark))
        .build()
        .unwrap()
}
