// CraftCraft Launcher
pub mod commands;
pub mod models;
pub mod services;
pub mod utils;

use services::{launcher::LauncherState, InstanceStore, ServerStore, SettingsStore};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .manage(InstanceStore::new())
        .manage(LauncherState::new())
        .manage(SettingsStore::new())
        .manage(ServerStore::new())
        .invoke_handler(tauri::generate_handler![
            // instances
            commands::get_instances,
            commands::create_instance,
            commands::update_instance,
            commands::delete_instance,
            commands::launch_instance,
            commands::read_log_tail,
            // mojang
            commands::list_minecraft_versions,
            // i18n / game language
            commands::game_languages,
            commands::match_game_language,
            // java
            commands::list_java,
            commands::install_java,
            commands::recommend_java,
            // servers
            commands::list_servers,
            commands::add_server,
            commands::remove_server,
            commands::toggle_server_favorite,
            commands::query_server,
            // settings
            commands::get_settings,
            commands::save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
