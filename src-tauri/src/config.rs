use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

#[tauri::command]
pub async fn close_config_window(app: AppHandle) {
    if let Some(window) = app.get_webview_window("config".into()) {
        window.close().expect("Error closing the config window");
    } else {
        println!("The config window was not found!");
    }
}

pub fn create_config_window(app: &tauri::AppHandle) -> WebviewWindow {
    let window: WebviewWindow =
        WebviewWindowBuilder::new(app, "config", WebviewUrl::App("config.html".into()))
            .title("Transparent Overlay - Config")
            .resizable(false)
            .center()
            .inner_size(350.0, 250.0)
            .maximizable(false)
            .theme(Some(tauri::Theme::Dark))
            .build()
            .unwrap();
    window
}
