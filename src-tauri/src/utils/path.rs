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

/// Resolve the `java` executable, handling both the flat JDK layout
/// (`<home>/bin/java`) and the macOS bundle layout (`<home>/Contents/Home/bin/java`).
pub fn get_java_binary_path(java_home: &PathBuf) -> PathBuf {
    let exe = if cfg!(target_os = "windows") { "java.exe" } else { "java" };
    let bundled = java_home.join("Contents").join("Home").join("bin").join(exe);
    if bundled.exists() {
        return bundled;
    }
    java_home.join("bin").join(exe)
}

pub fn home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"))
}
