use std::path::PathBuf;

#[cfg(target_os = "macos")]
pub fn get_app_data_dir() -> PathBuf {
    home_dir()
        .join("Library/Application Support/CraftCraft")
}

#[cfg(target_os = "windows")]
pub fn get_app_data_dir() -> PathBuf {
    std::env::var("APPDATA")
        .map(|p| PathBuf::from(p).join("CraftCraft"))
        .unwrap_or_else(|_| home_dir().join("CraftCraft"))
}

#[cfg(target_os = "linux")]
pub fn get_app_data_dir() -> PathBuf {
    home_dir().join(".config/CraftCraft")
}

#[cfg(target_os = "macos")]
pub fn get_java_binary_path(java_home: &PathBuf) -> PathBuf {
    java_home.join("bin").join("java")
}

#[cfg(target_os = "windows")]
pub fn get_java_binary_path(java_home: &PathBuf) -> PathBuf {
    java_home.join("bin").join("java.exe")
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn get_java_binary_path(java_home: &PathBuf) -> PathBuf {
    java_home.join("bin").join("java")
}

pub fn home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"))
}
