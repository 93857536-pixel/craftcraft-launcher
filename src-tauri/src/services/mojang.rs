use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub const PISTON_META: &str = "https://piston-meta.mojang.com";
pub const MINECART_DOWNLOAD: &str = "https://launchermeta.mojang.com";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionEntry {
    pub id: String,
    pub r#type: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub name: Option<String>,
    pub url: Option<String>,
    pub natives: Option<serde_json::Value>,
    pub downloads: Option<LibraryDownloads>,
    pub rules: Option<Vec<DownloadRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryDownloads {
    pub artifact: Option<LibraryFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryFile {
    pub url: String,
    pub sha1: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRule {
    pub action: String,
    pub os: Option<RuleOs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleOs {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetIndexEntry {
    pub url: String,
    pub hash: Option<String>,
    pub size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetsBlock {
    pub id: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetObject {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetIndex {
    pub objects: Option<std::collections::HashMap<String, AssetObject>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionJson {
    pub id: String,
    pub main_class: Option<String>,
    pub arguments: Option<ArgumentsBlock>,
    pub libraries: Option<Vec<Library>>,
    pub asset_index: Option<AssetsBlock>,
    pub assets: Option<String>,
    pub download: Option<DownloadBlock>,
    pub download_2: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadBlock {
    pub client: Option<DownloadFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadFile {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArgumentsBlock {
    pub game: Option<Vec<serde_json::Value>>,
    pub jvm: Option<Vec<serde_json::Value>>,
}

pub struct MojangClient {
    pub http: reqwest::Client,
    pub versions_dir: PathBuf,
    pub assets_dir: PathBuf,
    pub libraries_dir: PathBuf,
}

impl MojangClient {
    pub fn new(base: &std::path::Path) -> Self {
        Self {
            http: reqwest::Client::new(),
            versions_dir: base.join("versions"),
            assets_dir: base.join("assets"),
            libraries_dir: base.join("libraries"),
        }
    }

    pub async fn fetch_version_list(&self) -> Result<Vec<VersionEntry>> {
        let url = format!("{}/mc/game/version_manifest_v2.json", PISTON_META);
        let text: String = self
            .http
            .get(&url)
            .send()
            .await
            .context("fetch version manifest")?
            .text()
            .await
            .context("read manifest")?;
        let manifest: serde_json::Value = serde_json::from_str(&text)?;
        let versions: Vec<VersionEntry> = serde_json::from_value(manifest["versions"].clone())?;
        Ok(versions)
    }

    /// Download + cache a version JSON; returns it.
    pub async fn get_version_json(&self, version: &str) -> Result<VersionJson> {
        let cache = self.versions_dir.join(version).join(format!("{}.json", version));
        if cache.exists() {
            let text = fs::read_to_string(&cache)?;
            return Ok(serde_json::from_str(&text)?);
        }
        let url = format!(
            "{}/mc/game/download/{}.json",
            MINECART_DOWNLOAD, version
        );
        let text: String = self.http.get(&url).send().await?.text().await?;
        let json: VersionJson = serde_json::from_str(&text)?;
        if let Some(parent) = cache.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&cache, &text)?;
        Ok(json)
    }

    /// Ensure client jar is present. Returns path.
    pub async fn ensure_client_jar(&self, version: &str) -> Result<PathBuf> {
        let json = self.get_version_json(version).await?;
        let dir = self.versions_dir.join(version);
        let target = dir.join(format!("minecraft_{}.jar", version));
        if target.exists() {
            return Ok(target);
        }
        let dl = json
            .download
            .as_ref()
            .and_then(|d| d.client.as_ref())
            .context("version has no client download")?;
        fs::create_dir_all(&dir)?;
        let bytes = self.http.get(&dl.url).send().await?.bytes().await?;
        fs::write(&target, &bytes)?;
        Ok(target)
    }

    /// Ensure all libraries present. Returns classpath entries (in order).
    pub async fn ensure_libraries(&self, json: &VersionJson) -> Result<Vec<PathBuf>> {
        let mut classpath = vec![];
        for lib in json.libraries.iter().flatten() {
            let name = match &lib.name {
                Some(n) => n.clone(),
                None => continue,
            };
            // Skip macos-only javassist/awt etc. unless relevant; keep all for simplicity.
            if let Some(rules) = &lib.rules {
                for rule in rules {
                    if rule.action == "allow" {
                        if let Some(os) = &rule.os {
                            if let Some(name) = &os.name {
                                let current = std::env::consts::OS.to_string();
                                if name != &current {
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
            // Skip natives on macOS (sun.j2d pipeline variants)
            if name.contains("natives-macos") && std::env::consts::OS != "macos" {
                continue;
            }
            if name.contains("natives-windows") && std::env::consts::OS != "windows" {
                continue;
            }
            if name.contains("natives-linux") && std::env::consts::OS != "linux" {
                continue;
            }

            let jar_path = self.libraries_dir.join(&name);
            let needs_download = !jar_path.exists();
            if needs_download {
                let url = lib.downloads.as_ref().and_then(|d| d.artifact.as_ref()).map(|a| a.url.clone())
                    .or_else(|| lib.url.clone())
                    .ok_or_else(|| anyhow::anyhow!("no download url for library {}", name))?;
                if let Some(parent) = jar_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                let bytes = self.http.get(&url).send().await?.bytes().await?;
                fs::write(&jar_path, &bytes)?;
            }
            classpath.push(jar_path);
        }
        Ok(classpath)
    }

    /// Ensure assets present. Returns asset index (for old games) or nothing (new games use classpath assets dir).
    pub async fn ensure_assets(&self, json: &VersionJson) -> Result<()> {
        let Some(idx) = &json.asset_index else {
            return Ok(());
        };
        let idx_path = self.assets_dir.join("indexes").join(&idx.id);
        let contents = self.http.get(idx.url.as_ref().unwrap()).send().await?.text().await?;
        let asset_index: AssetIndex = serde_json::from_str(&contents)?;
        if let Some(objects) = asset_index.objects {
            for (_name, entry) in objects {
                let target = self
                    .assets_dir
                    .join("objects")
                    .join(&entry.url[..2])
                    .join(&entry.url[2..]);
                if !target.exists() {
                    if let Some(parent) = target.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    let url = format!("https://resources.mojang.com/{}/{}", &entry.url[..2], &entry.url[2..]);
                    let bytes = self.http.get(&url).send().await?.bytes().await?;
                    fs::write(&target, &bytes)?;
                }
            }
        }
        Ok(())
    }
}
