use crate::models::java::JavaVersion;
use crate::utils;
use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use uuid::Uuid;

pub struct JavaManager {
    pub javas_dir: PathBuf,
}

impl JavaManager {
    pub fn new() -> Self {
        Self {
            javas_dir: utils::get_app_data_dir().join("javas"),
        }
    }

    /// Discover Java on the system (PATH scan + macOS framework + common locations).
    pub fn list(&self) -> Result<Vec<JavaVersion>> {
        let mut out = Vec::new();
        out.extend(self.scan_path()?);
        out.extend(self.scan_macos_frameworks()?);
        out.extend(self.scan_managed()?);
        out.sort_by(|a, b| b.version.cmp(&a.version));
        Ok(out)
    }

    /// Install a Temurin JDK into the managed dir. For now, download the Adoptium tarball.
    pub async fn install(&self, version: &str) -> Result<JavaVersion> {
        let arch = if cfg!(target_arch = "aarch64") {
            "aarch64"
        } else {
            "x64"
        };
        let os = if cfg!(target_os = "macos") {
            "mac"
        } else if cfg!(target_os = "windows") {
            "win"
        } else {
            "linux"
        };

        let url = format!(
            "https://api.adoptium.net/v3/binary/latest/{version}/ga/{os}/{arch}/jdk/hotspot/normal/eclipse",
        );
        let target_dir = self.javas_dir.join(format!("temurin-{version}"));
        if target_dir.exists() {
            return Ok(self.build_entry(&target_dir, version, true));
        }

        let http = reqwest::Client::new();
        let resp = http.get(&url).send().await?.bytes().await?;
        std::fs::create_dir_all(&self.javas_dir)?;
        let archive = self.javas_dir.join(format!("temurin-{version}.tar.gz"));
        std::fs::write(&archive, &resp)?;

        let status = Command::new("tar")
            .args(["-xzf", archive.to_str().unwrap(), "-C"])
            .arg(&self.javas_dir)
            .status()?;
        if !status.success() {
            return Err(anyhow::anyhow!("failed to extract JDK archive"));
        }
        // adoptium layout: jdk-<ver>_... directory
        let entries = std::fs::read_dir(&self.javas_dir)?;
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() && p.to_string_lossy().starts_with("jdk-") {
                std::fs::rename(&p, &target_dir)?;
                return Ok(self.build_entry(&target_dir, version, true));
            }
        }
        Err(anyhow::anyhow!("JDK extracted but could not be located"))
    }

    /// Run a quick `java -version` probe to validate a path.
    pub fn validate(&self, path: &PathBuf) -> Option<(String, String)> {
        let output = Command::new(path)
            .arg("-version")
            .output()
            .ok()?;
        let mut text = String::new();
        text.push_str(&String::from_utf8_lossy(&output.stdout));
        text.push_str(&String::from_utf8_lossy(&output.stderr));
        let version = text
            .lines()
            .next()
            .unwrap_or("?")
            .split('"')
            .nth(1)
            .unwrap_or("?")
            .to_string();
        let vendor = text.lines().next().unwrap_or("unknown").to_string();
        Some((version, vendor))
    }

    fn scan_path(&self) -> Result<Vec<JavaVersion>> {
        let mut out = vec![];
        if let Ok(java) = Command::new("which").arg("java").output() {
            if java.status.success() {
                let path = String::from_utf8_lossy(&java.stdout).trim().to_string();
                let p = PathBuf::from(&path);
                if let Some((version, vendor)) = self.validate(&p) {
                    out.push(JavaVersion::new(
                        Uuid::new_v4().to_string(),
                        path,
                        version,
                        vendor,
                        "unknown".into(),
                        false,
                    ));
                }
            }
        }
        Ok(out)
    }

    fn scan_macos_frameworks(&self) -> Result<Vec<JavaVersion>> {
        if !cfg!(target_os = "macos") {
            return Ok(vec![]);
        }
        let dir = PathBuf::from("/Library/Java/JavaVirtualMachines");
        let mut out = vec![];
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let home = entry
                    .path()
                    .join("Contents/Home")
                    .join("bin/java");
                if home.exists() {
                    if let Some((version, vendor)) = self.validate(&home) {
                        out.push(JavaVersion::new(
                            Uuid::new_v4().to_string(),
                            home.to_string_lossy().to_string(),
                            version,
                            vendor,
                            "x64".into(),
                            false,
                        ));
                    }
                }
            }
        }
        Ok(out)
    }

    fn scan_managed(&self) -> Result<Vec<JavaVersion>> {
        let mut out = vec![];
        if !self.javas_dir.exists() {
            return Ok(out);
        }
        for entry in std::fs::read_dir(&self.javas_dir)?.flatten() {
            let p = entry.path();
            if !p.is_dir() {
                continue;
            }
            let java_bin = utils::get_java_binary_path(&p);
            if java_bin.exists() {
                let version = p
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("?")
                    .to_string();
                out.push(JavaVersion::new(
                    Uuid::new_v4().to_string(),
                    java_bin.to_string_lossy().to_string(),
                    version,
                    "Temurin".into(),
                    "managed".into(),
                    true,
                ));
            }
        }
        Ok(out)
    }

    fn build_entry(&self, path: &PathBuf, version: &str, managed: bool) -> JavaVersion {
        JavaVersion::new(
            Uuid::new_v4().to_string(),
            utils::get_java_binary_path(path).to_string_lossy().to_string(),
            version.to_string(),
            "Temurin".into(),
            "managed".into(),
            managed,
        )
    }
}
