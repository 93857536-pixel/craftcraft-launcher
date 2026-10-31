use crate::models::instance::{Instance, InstanceList, LaunchResult, LoaderType};
use crate::services::{
    instance_store::InstanceUpdates,
    java_manager::JavaManager,
    launcher::LauncherState,
    server_store::{self, ServerStore},
    settings_store::SettingsStore,
    InstanceStore,
};
use crate::models::java::{JavaRecommendation, JavaVersion};
use crate::models::settings::AppSettings;
use crate::models::server::{ServerBookmark, ServerStatus};
use tauri::State;

// ---------- instances ----------

#[tauri::command]
pub fn get_instances(store: State<'_, InstanceStore>) -> Result<InstanceList, String> {
    Ok(store.list_with_meta())
}

#[tauri::command]
pub fn create_instance(
    store: State<'_, InstanceStore>,
    name: String,
    version: String,
    loader: Option<LoaderType>,
    account_name: Option<String>,
) -> Result<Instance, String> {
    let mut instance = Instance::new(name, version, String::new());
    if let Some(loader) = loader {
        instance.loader = loader;
    }
    if let Some(acc) = account_name {
        instance.account_name = acc;
    }
    store.create(instance).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_instance(
    store: State<'_, InstanceStore>,
    id: String,
    changes: InstanceUpdates,
) -> Result<Instance, String> {
    store.update(&id, changes).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_instance(store: State<'_, InstanceStore>, id: String) -> Result<(), String> {
    store.delete(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn launch_instance(
    app: tauri::AppHandle,
    store: State<'_, InstanceStore>,
    launcher: State<'_, LauncherState>,
    id: String,
) -> Result<LaunchResult, String> {
    let instance = store.get(&id).map_err(|e| e.to_string())?;
    match crate::services::launcher::launch(&app, &launcher, &instance).await {
        Ok(result) => {
            let _ = store
                .update(&id, InstanceUpdates::last_played_touch())
                .map_err(|e| e.to_string());
            Ok(result)
        }
        Err(e) => Ok(LaunchResult {
            success: false,
            pid: None,
            message: format!("launch failed: {}", e),
        }),
    }
}

#[tauri::command]
pub fn read_log_tail(instance_id: String, lines: u32) -> Result<Vec<String>, String> {
    crate::services::launcher::read_log_tail(&instance_id, lines).map_err(|e| e.to_string())
}

// ---------- mojang versions ----------

#[tauri::command]
pub async fn list_minecraft_versions(launcher: State<'_, LauncherState>) -> Result<Vec<MojangVersionInfo>, String> {
    let entries = launcher.mojang.fetch_version_list().await.map_err(|e| e.to_string())?;
    Ok(entries
        .into_iter()
        .map(|v| MojangVersionInfo {
            id: v.id,
            r#type: v.r#type,
            url: v.url,
        })
        .collect())
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct MojangVersionInfo {
    pub id: String,
    pub r#type: Option<String>,
    pub url: Option<String>,
}

// ---------- java ----------

#[tauri::command]
pub fn list_java() -> Result<Vec<JavaVersion>, String> {
    JavaManager::new().list().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn install_java(_launcher: State<'_, LauncherState>, version: String) -> Result<JavaVersion, String> {
    let mgr = JavaManager::new();
    mgr.install(&version).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn recommend_java(mc_version: String) -> JavaRecommendation {
    JavaRecommendation::get_for_version(&mc_version)
}

// ---------- servers ----------

#[tauri::command]
pub fn list_servers(store: State<'_, ServerStore>) -> Result<Vec<ServerBookmark>, String> {
    store.list().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_server(
    store: State<'_, ServerStore>,
    address: String,
    name: String,
) -> Result<ServerBookmark, String> {
    let bookmark = ServerBookmark::new(address, name);
    store.add(bookmark).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn remove_server(store: State<'_, ServerStore>, id: String) -> Result<(), String> {
    store.remove(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_server_favorite(
    store: State<'_, ServerStore>,
    id: String,
) -> Result<Option<ServerBookmark>, String> {
    store.toggle_favorite(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn query_server(address: String) -> Result<ServerStatus, String> {
    Ok(server_store::query_status(&address))
}

// ---------- settings ----------

#[tauri::command]
pub fn get_settings(store: State<'_, SettingsStore>) -> Result<AppSettings, String> {
    Ok(store.load())
}

#[tauri::command]
pub fn save_settings(store: State<'_, SettingsStore>, settings: AppSettings) -> Result<(), String> {
    store.save(&settings).map_err(|e| e.to_string())
}
