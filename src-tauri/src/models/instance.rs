use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LoaderType {
    Vanilla,
    Forge,
    Fabric,
    Quilt,
    NeoForge,
}

impl Default for LoaderType {
    fn default() -> Self {
        Self::Vanilla
    }
}

impl LoaderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            LoaderType::Vanilla => "vanilla",
            LoaderType::Forge => "forge",
            LoaderType::Fabric => "fabric",
            LoaderType::Quilt => "quilt",
            LoaderType::NeoForge => "neoforge",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Instance {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub loader: LoaderType,
    pub loader_version: Option<String>,
    pub java_version: Option<String>,
    pub java_path: Option<String>,
    pub game_directory: String,
    pub memory_min_mb: u64,
    pub memory_max_mb: u64,
    pub jvm_extra_args: Vec<String>,
    pub account_name: String,
    pub last_played_at: Option<u64>,
    pub play_count: u64,
    pub created_at: u64,
    pub tags: Vec<String>,
}

impl Instance {
    pub fn new(name: impl Into<String>, version: impl Into<String>, game_dir: String) -> Self {
        let now = chrono::Utc::now().timestamp() as u64;
        let name = name.into();
        let version = version.into();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description: None,
            version,
            loader: LoaderType::default(),
            loader_version: None,
            java_version: None,
            java_path: None,
            game_directory: game_dir,
            memory_min_mb: 2048,
            memory_max_mb: 6144,
            jvm_extra_args: vec![],
            account_name: "Player".into(),
            last_played_at: None,
            play_count: 0,
            created_at: now,
            tags: vec![],
        }
    }

    pub fn touch_play(&mut self) {
        let now = chrono::Utc::now().timestamp() as u64;
        self.last_played_at = Some(now);
        self.play_count += 1;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstanceList {
    pub instances: Vec<Instance>,
    pub total: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchResult {
    pub success: bool,
    pub pid: Option<u32>,
    pub message: String,
}
