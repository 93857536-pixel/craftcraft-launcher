use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub java_home_override: Option<String>,
    pub default_memory_min_mb: u64,
    pub default_memory_max_mb: u64,
    pub open_log_on_launch: bool,
}

impl AppSettings {
    pub fn default_values() -> Self {
        Self {
            theme: "dark".into(),
            java_home_override: None,
            default_memory_min_mb: 2048,
            default_memory_max_mb: 6144,
            open_log_on_launch: true,
        }
    }
}
