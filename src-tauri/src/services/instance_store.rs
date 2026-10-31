use crate::models::instance::{Instance, InstanceList};
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct InstanceStore {
    pub file: PathBuf,
    pub cache: Mutex<Option<Vec<Instance>>>,
}

impl InstanceStore {
    pub fn new() -> Self {
        let dir = crate::utils::get_app_data_dir();
        let _ = fs::create_dir_all(&dir);
        Self {
            file: dir.join("instances.json"),
            cache: Mutex::new(None),
        }
    }

    pub fn list(&self) -> Result<Vec<Instance>> {
        if self.file.exists() {
            let raw = fs::read_to_string(&self.file)?;
            let v: Vec<Instance> = serde_json::from_str(&raw).unwrap_or_default();
            Ok(v)
        } else {
            Ok(vec![])
        }
    }

    fn persist(&self, instances: &Vec<Instance>) -> Result<()> {
        let raw = serde_json::to_string_pretty(instances)?;
        fs::write(&self.file, raw)?;
        Ok(())
    }

    pub fn create(&self, mut instance: Instance) -> Result<Instance> {
        let mut instances = self.list()?;
        // give it a default game directory under data dir
        let dir = crate::utils::get_app_data_dir().join("instances").join(&instance.id);
        let _ = fs::create_dir_all(&dir);
        instance.game_directory = dir.to_string_lossy().to_string();
        instances.push(instance.clone());
        self.persist(&instances)?;
        Ok(instance)
    }

    pub fn update(&self, id: &str, changes: InstanceUpdates) -> Result<Instance> {
        let mut instances = self.list()?;
        let idx = instances
            .iter()
            .position(|i| i.id == id)
            .ok_or_else(|| anyhow::anyhow!("instance not found"))?;
        let it = &mut instances[idx];
        if let Some(name) = changes.name {
            it.name = name;
        }
        if let Some(version) = changes.version {
            it.version = version;
        }
        if let Some(loader) = changes.loader {
            it.loader = loader;
        }
        if let Some(java_path) = changes.java_path {
            it.java_path = Some(java_path);
        }
        if let Some(java_version) = changes.java_version {
            it.java_version = Some(java_version);
        }
        if let Some(mem_min) = changes.memory_min_mb {
            it.memory_min_mb = mem_min;
        }
        if let Some(mem_max) = changes.memory_max_mb {
            it.memory_max_mb = mem_max;
        }
        if let Some(account) = changes.account_name {
            it.account_name = account;
        }
        if let Some(args) = changes.jvm_extra_args {
            it.jvm_extra_args = args;
        }
        // 空字符串 = 清空为「跟随游戏自身设置」
        if let Some(lang) = changes.language {
            let trimmed = lang.trim();
            it.language = if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.replace('-', "_").to_ascii_lowercase())
            };
        }
        if changes.last_played_touch {
            it.touch_play();
        }
        if let Some(tags) = changes.tags {
            it.tags = tags;
        }
        self.persist(&instances)?;
        Ok(instances[idx].clone())
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        let mut instances = self.list()?;
        let before = instances.len();
        instances.retain(|i| i.id != id);
        if instances.len() != before {
            self.persist(&instances)?;
        }
        Ok(())
    }

    pub fn get(&self, id: &str) -> Result<Instance> {
        let instances = self.list()?;
        instances
            .into_iter()
            .find(|i| i.id == id)
            .ok_or_else(|| anyhow::anyhow!("instance not found"))
    }

    pub fn set_tags(&self, id: &str, tags: &[String]) -> Result<()> {
        let mut instances = self.list()?;
        let idx = instances
            .iter()
            .position(|i| i.id == id)
            .ok_or_else(|| anyhow::anyhow!("instance not found"))?;
        instances[idx].tags = tags.to_vec();
        self.persist(&instances)?;
        Ok(())
    }

    pub fn list_with_meta(&self) -> InstanceList {
        let instances = self.list().unwrap_or_default();
        let total = instances.len() as u32;
        InstanceList { instances, total }
    }
}

#[derive(Default, serde::Deserialize)]
pub struct InstanceUpdates {
    pub name: Option<String>,
    pub version: Option<String>,
    pub loader: Option<crate::models::instance::LoaderType>,
    pub java_path: Option<String>,
    pub java_version: Option<String>,
    pub memory_min_mb: Option<u64>,
    pub memory_max_mb: Option<u64>,
    pub account_name: Option<String>,
    pub jvm_extra_args: Option<Vec<String>>,
    /// 游戏内语言。空字符串表示「跟随游戏自身设置」（清空为 None）。
    #[serde(default)]
    pub language: Option<String>,
    pub tags: Option<Vec<String>>,
    #[serde(skip)]
    pub last_played_touch: bool,
}

impl InstanceUpdates {
    pub fn last_played_touch() -> Self {
        Self {
            last_played_touch: true,
            ..Default::default()
        }
    }
}
