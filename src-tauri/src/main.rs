// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;

#[tauri::command]
fn create_window_livechat(app: &tauri::AppHandle) -> tauri::WebviewWindow {
    use tauri::{WebviewUrl, WebviewWindowBuilder};

    // TODO: Fix window size (force fullscreen)
    let window =
        WebviewWindowBuilder::new(app, "livechat", WebviewUrl::App("livechat.html".into()))
            .title("Transparent Overlay")
            .transparent(true)
            .decorations(false)
            .always_on_top(true)
            .build()
            .unwrap();
    window.maximize().unwrap();
    let hwnd = window.hwnd().unwrap().0;
    let _pre_val;
    let hwnd = windows::Win32::Foundation::HWND(hwnd as isize);
    unsafe {
        use windows::Win32::UI::WindowsAndMessaging::*;
        let nindex = GWL_EXSTYLE;
        let style =
            WS_EX_APPWINDOW | WS_EX_COMPOSITED | WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST;
        _pre_val = SetWindowLongA(hwnd, nindex, style.0 as i32);
    };
    window
}

#[tauri::command]
fn get_url() -> String {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    println!("{}", url);
    url.into()
}

fn create_config_window(app: &tauri::AppHandle) -> tauri::WebviewWindow {
    use tauri::{WebviewUrl, WebviewWindowBuilder};

    let window = WebviewWindowBuilder::new(app, "config", WebviewUrl::App("config.html".into()))
        .title("Transparent Overlay - Config")
        .resizable(false)
        .center()
        .inner_size(350.0, 250.0)
        .maximizable(false)
        .build()
        .unwrap();
    window
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Error : [No URL Provided]");
        println!("Example usage : {} http://example.com", &args[0]);
        tauri::Builder::default()
            .setup(|app| {
                let handle = app.handle();
                create_config_window(handle);
                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("Error launching config window");
    } else {
        tauri::Builder::default()
            .setup(|app| {
                let handle = app.handle();
                create_window_livechat(handle);
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![get_url])
            .run(tauri::generate_context!())
            .expect("Error launching livechat window");
    }
}
