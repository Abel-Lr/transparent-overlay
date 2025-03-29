use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    App, Manager,
};

use crate::{config_window::create_config_window, livechat::close_livechat};

pub fn setup_tray(app: &App) {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();
    let separator_i = PredefinedMenuItem::separator(app).unwrap();
    let reload_i = MenuItem::with_id(app, "reload", "Reload", true, None::<&str>).unwrap();
    let config_i = MenuItem::with_id(app, "config", "Open Config", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&config_i, &reload_i, &separator_i, &quit_i]).unwrap();

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0x0);
            }
            "reload" => app
                .get_webview_window("livechat")
                .map(|window| {
                    window
                        .reload()
                        .expect("Error reloading the livechat window")
                })
                .expect("Can't find the livechat window"),
            "config" => {
                create_config_window(app.app_handle());
                close_livechat(app);
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .build(app)
        .expect("Failed to build tray");
}
