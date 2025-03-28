// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;

use tauri::{
  webview,
  AppHandle,
  Manager,
  WebviewUrl,
  WebviewWindow,
  WebviewWindowBuilder,
  menu::{Menu, MenuItem},
  tray::TrayIconBuilder,
  App
};

fn create_window_livechat(app: &tauri::AppHandle, url: &str) -> Result<WebviewWindow, String> {
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
fn get_url_from_arg() -> String {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];
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

#[tauri::command]
async fn close_config_window(app: AppHandle) {
    if let Some(window) = app.get_webview_window("config".into()) {
        window.close().expect("Error closing the config window");
    } else {
        println!("The config window was not found!");
    }
}

#[tauri::command]
async fn build_livechat_window_from_config(app: tauri::AppHandle, url: &str) -> Result<(), String> {
    match create_window_livechat(&app, url) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!(
            "Cannot create livechat window with url {} : {}",
            url, e
        )),
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
            .theme(Some(tauri::Theme::Dark))
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
                setup_tray(app);
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                url_is_parsable,
                build_livechat_window_from_config,
                close_config_window
            ])
            .run(tauri::generate_context!())
            .expect("Error launching config window");
    } else {
        tauri::Builder::default()
            .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
            .plugin(tauri_plugin_shell::init())
            .setup(move |app| {
                let handle = app.handle();
                create_window_livechat(handle, &get_url_from_arg())?
                    .maximize()
                    .unwrap();
                setup_tray(app);
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![get_url_from_arg])
            .run(tauri::generate_context!())
            .expect("Error launching livechat window");
    }
}

fn setup_tray(app: &App) {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&quit_i]).unwrap();

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0x0);
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .build(app)
        .expect("Failed to build tray");
}
