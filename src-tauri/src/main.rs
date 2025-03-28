// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;

use config::{close_config_window, create_config_window};
use livechat::{build_livechat_window_from_config, create_window_livechat};
use tray::setup_tray;
use url::{get_url_from_arg, url_is_parsable};

mod config;
mod livechat;
mod tray;
mod url;
mod warning;

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
