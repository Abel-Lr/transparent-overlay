use std::fmt::Display;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub url: String,
}

impl Config {
    pub fn load(app: &AppHandle) -> Option<Self> {
        let store = app.store("store.json").unwrap();
        store
            .get("config")
            .map(|val| serde_json::from_value(val).unwrap())
    }

    pub fn save(&self, app: &AppHandle) {
        let store = app.store("store.json").unwrap();
        let value = serde_json::to_value(self).expect("failed to serialize config");
        store.set("config", value);
    }

    pub fn empty() -> Self {
        Config { url: "".into() }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
