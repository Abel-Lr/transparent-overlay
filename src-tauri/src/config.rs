use std::fmt::Display;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri::Monitor;
use tauri_plugin_store::StoreExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub url: String,
    pub monitor: MonitorPos,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonitorPos {
    pub name: String,
    pub pos_x: i32,
}

impl From<Monitor> for MonitorPos {
    fn from(monitor: Monitor) -> Self {
        MonitorPos {
            name: monitor.name().unwrap().to_string(),
            pos_x: monitor.position().x,
        }
    }
}

impl Config {
    pub fn load(app: &AppHandle) -> Option<Self> {
        let store = app.store("store.json").unwrap();
        store
            .get("config")
            .map(|val| serde_json::from_value(val).unwrap())
            .map(|config| get_existing_monitor(app, config))
    }

    pub fn save(&self, app: &AppHandle) {
        let store = app.store("store.json").unwrap();
        let value = serde_json::to_value(self).expect("failed to serialize config");
        store.set("config", value);
    }

    pub fn empty() -> Self {
        Config {
            url: "".into(),
            monitor: MonitorPos {
                name: "".into(),
                pos_x: 0,
            },
        }
    }
}

fn get_existing_monitor(app: &AppHandle, mut config: Config) -> Config {
    let monitor: MonitorPos = app
        .available_monitors()
        .unwrap()
        .iter()
        .find(|&monitor| monitor.name().unwrap().eq(&config.monitor.name))
        .map(|m| m.clone())
        .unwrap_or_else(|| app.primary_monitor().unwrap().unwrap())
        .into();
    config.monitor = monitor;
    config
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for MonitorPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
