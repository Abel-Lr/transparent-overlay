// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;
use tauri::{webview, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

#[tauri::command]
fn create_window_livechat(app: &tauri::AppHandle, url: String) -> WebviewWindow {
    let url: webview::Url = match url.trim().parse() {
        Ok(parsed_url) => parsed_url,
        Err(_) => {
            create_warning_window();
            panic!("Error parsing URL");
        }
    };

    let window: WebviewWindow =
        WebviewWindowBuilder::new(app, "livechat", WebviewUrl::External(url))
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
fn get_url_from_arg() -> String {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    println!("{}", url);
    url.into()
}

#[tauri::command]
fn create_warning_window() {
    use std::ptr::null_mut as NULL;
    use winapi::um::winuser;

    let l_msg: Vec<u16> = "Invalid URL to parse\0".encode_utf16().collect();
    let l_title: Vec<u16> = "Error parsing URL\0".encode_utf16().collect();

    unsafe {
        winuser::MessageBoxW(
            NULL(),
            l_msg.as_ptr(),
            l_title.as_ptr(),
            winuser::MB_OK | winuser::MB_ICONEXCLAMATION,
        );
    }
}

#[tauri::command]
fn url_is_parsable(url: String) -> bool {
    match url.trim().parse::<webview::Url>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn create_config_window(app: &tauri::AppHandle) -> WebviewWindow {
    let window: WebviewWindow =
        WebviewWindowBuilder::new(app, "config", WebviewUrl::App("config.html".into()))
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
        tauri::Builder::default()
            .setup(|app| {
                let handle = app.handle();
                create_config_window(handle);
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![url_is_parsable, create_warning_window])
            .run(tauri::generate_context!())
            .expect("Error launching config window");
    } else {
        tauri::Builder::default()
            .setup(|app| {
                let handle = app.handle();
                create_window_livechat(handle, get_url_from_arg())
                    .maximize()
                    .unwrap();
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![get_url_from_arg])
            .run(tauri::generate_context!())
            .expect("Error launching livechat window");
    }
}
