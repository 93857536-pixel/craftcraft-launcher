use crate::models::settings::AppSettings;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub struct SettingsStore {
    pub file: PathBuf,
}

impl SettingsStore {
    pub fn new() -> Self {
        let dir = crate::utils::get_app_data_dir();
        let _ = fs::create_dir_all(&dir);
        Self {
            file: dir.join("settings.json"),
        }
    }

    pub fn load(&self) -> AppSettings {
        if self.file.exists() {
            let raw = fs::read_to_string(&self.file).ok();
            if let Some(raw) = raw {
                serde_json::from_str(&raw).ok().unwrap_or_default()
            } else {
                AppSettings::default()
            }
        } else {
            AppSettings::default_values()
        }
    }

    pub fn save(&self, s: &AppSettings) -> Result<()> {
        let raw = serde_json::to_string_pretty(s)?;
        fs::write(&self.file, raw)?;
        Ok(())
    }
}
