use std::env;

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    webview, App, AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder,
};

use crate::warning::create_warning_window;

pub fn create_window_livechat(app: &tauri::AppHandle, url: &str) -> Result<WebviewWindow, String> {
    let url: webview::Url = match url.trim().parse() {
        Ok(parsed_url) => parsed_url,
        Err(_) => {
            create_warning_window();
            return Err("Error parsing URL".to_string());
        }
    };

    let window: Result<WebviewWindow, tauri::Error> =
        WebviewWindowBuilder::new(app, "livechat", WebviewUrl::External(url.clone()))
            .title("Transparent Overlay")
            .transparent(true)
            .decorations(false)
            .always_on_top(true)
            .build();
    match window {
        Ok(w) => {
            w.maximize().unwrap();
            w.set_skip_taskbar(true).unwrap();
            let hwnd = w.hwnd().unwrap().0;
            let _pre_val;
            let hwnd = windows::Win32::Foundation::HWND(hwnd as isize);
            unsafe {
                use windows::Win32::UI::WindowsAndMessaging::*;
                let nindex = GWL_EXSTYLE;
                let style = WS_EX_APPWINDOW
                    | WS_EX_COMPOSITED
                    | WS_EX_LAYERED
                    | WS_EX_TRANSPARENT
                    | WS_EX_TOPMOST;
                _pre_val = SetWindowLongA(hwnd, nindex, style.0 as i32);
            };
            Ok(w)
        }
        Err(e) => Err(format!(
            "Failed to create window with the url {} : {}",
            url, e
        )),
    }
}

#[tauri::command]
pub async fn build_livechat_window_from_config(
    app: tauri::AppHandle,
    url: &str,
) -> Result<(), String> {
    match create_window_livechat(&app, url) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!(
            "Cannot create livechat window with url {} : {}",
            url, e
        )),
    }
}
