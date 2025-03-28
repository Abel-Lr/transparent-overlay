use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    App, Manager,
};

pub fn setup_tray(app: &App) {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();
    let reload_i = MenuItem::with_id(app, "reload", "Reload", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&reload_i, &quit_i]).unwrap();

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
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .build(app)
        .expect("Failed to build tray");
}
