use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub name: String,
    pub buttons: Vec<String>, // e.g. ["site1", "site2", ...] — target stack page names
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub profiles: Vec<ProfileConfig>,
    #[serde(default)]
    pub submenu_selections: std::collections::HashMap<String, usize>,
}

impl AppConfig {
    pub fn load(path: &str) -> Self {
        let data = fs::read_to_string(path)
            .expect("Could not read config.json — does the file exist?");
        serde_json::from_str(&data)
            .expect("Could not parse config.json — check its structure")
    }
    pub fn save(&self, path: &str) {
        let json = serde_json::to_string_pretty(self)
            .expect("Could not serialize AppConfig");
        std::fs::write(path, json)
            .expect("Could not write config.json")
    }
}
