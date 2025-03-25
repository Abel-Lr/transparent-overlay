// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;

#[tauri::command]
async fn _create_window(app: tauri::AppHandle) -> tauri::WebviewWindow {
    use tauri::{WebviewUrl, WebviewWindowBuilder};

    WebviewWindowBuilder::new(&app, "label", WebviewUrl::App("index.html".into()))
        .build()
        .unwrap()
}

#[tauri::command]
fn get_url() -> String {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    println!("{}", url);
    url.into()
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            use tauri::Manager;
            let window = app.get_webview_window("main").unwrap();
            window.maximize().unwrap();
            let hwnd = window.hwnd().unwrap().0;
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
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
