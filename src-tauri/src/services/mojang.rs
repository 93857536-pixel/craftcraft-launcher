use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use futures_util::StreamExt;

pub const PISTON_META: &str = "https://piston-meta.mojang.com";

/// Progress events are emitted on the `download:progress` Tauri channel:
/// { "label": "client|libraries|assets|index|java", "step": u32, "total": u32, "done": bool }
pub fn emit_progress(_label: &str, _step: u32, _total: u32, _done: bool) {
    // Progress events are not yet wired to the UI (app handle is optional in this path).
    // TODO: wire launch_instance to forward download events.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionEntry {
    pub id: String,
    #[serde(default, rename = "type")]
    pub r#type: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuleOs {
    pub name: Option<String>,
    #[serde(default)]
    pub arch: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DownloadRule {
    pub action: String,
    #[serde(default)]
    pub os: Option<RuleOs>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NativePlatform {
    pub name: Option<String>,
    pub classifier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NativePlatforms {
    #[serde(default)]
    pub linux: Option<NativePlatform>,
    #[serde(default)]
    pub windows: Option<NativePlatform>,
    #[serde(default)]
    pub osx: Option<NativePlatform>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LibraryFile {
    pub url: String,
    #[serde(default)]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LibraryDownloads {
    pub artifact: Option<LibraryFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Library {
    pub name: Option<String>,
    pub url: Option<String>,
    #[serde(default)]
    pub natives: Option<NativePlatforms>,
    #[serde(default)]
    pub downloads: Option<LibraryDownloads>,
    #[serde(default)]
    pub rules: Option<Vec<DownloadRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetsBlock {
    pub id: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientDownload {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DownloadBlock {
    pub client: Option<ClientDownload>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArgumentsBlock {
    pub game: Option<Vec<serde_json::Value>>,
    pub jvm: Option<Vec<serde_json::Value>>,
}

/// Version JSON. Mojang's current format is camelCase (`mainClass`, `assetIndex`,
/// `downloads.client`); very old versions used snake_case. Both are accepted.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VersionJson {
    pub id: String,
    #[serde(
        alias = "main_class",
        rename = "mainClass",
        default
    )]
    pub main_class: Option<String>,
    #[serde(alias = "arguments", default)]
    pub arguments: Option<ArgumentsBlock>,
    /// Legacy string form (versions ≤ 1.12.x).
    #[serde(
        alias = "minecraft_arguments",
        rename = "minecraftArguments",
        default
    )]
    pub minecraft_arguments: Option<String>,
    #[serde(default)]
    pub libraries: Option<Vec<Library>>,
    #[serde(
        alias = "asset_index",
        rename = "assetIndex",
        default
    )]
    pub asset_index: Option<AssetsBlock>,
    #[serde(default)]
    pub assets: Option<String>,
    #[serde(alias = "download", default)]
    pub download: Option<DownloadBlock>,
    #[serde(
        alias = "downloads",
        rename = "downloads",
        default
    )]
    pub download_2: Option<DownloadBlock>,
}

impl VersionJson {
    /// Client jar URL — current camelCase `downloads`, legacy snake_case `download`.
    pub fn client_download_url(&self) -> Option<String> {
        self.download_2
            .as_ref()
            .and_then(|d| d.client.as_ref())
            .or_else(|| self.download.as_ref().and_then(|d| d.client.as_ref()))
            .map(|c| c.url.clone())
    }

    /// Asset index id (current versions put it in `assets`, older ones in `assetIndex.id`).
    pub fn asset_index_id(&self) -> Option<String> {
        if let Some(assets) = &self.assets {
            return Some(assets.clone());
        }
        self.asset_index.as_ref().map(|a| a.id.clone())
    }

    /// Where native libraries should be unpacked for this game directory.
    pub fn natives_dir(&self, game_dir: &Path) -> PathBuf {
        game_dir.join("natives")
    }

    /// The main class to run, with a sane fallback for very old versions.
    pub fn main_class_or_fallback(&self) -> String {
        self.main_class
            .clone()
            .unwrap_or_else(|| "net.minecraft.client.main.Main".to_string())
    }

    /// Parse `<major>` out of a `javaVersion.majorVersion` like "21" (used for logging only).
    pub fn required_java_major(&self) -> Option<String> {
        None
    }
}

pub struct MojangClient {
    pub http: reqwest::Client,
    pub versions_dir: PathBuf,
    pub assets_dir: PathBuf,
    pub libraries_dir: PathBuf,
}

impl MojangClient {
    pub fn new(base: &Path) -> Self {
        Self {
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .expect("http client"),
            versions_dir: base.join("versions"),
            assets_dir: base.join("assets"),
            libraries_dir: base.join("libraries"),
        }
    }

    async fn stream_download(&self, url: &str, target: &Path, label: &str, step: u32) -> Result<()> {
        let resp = self
            .http
            .get(url)
            .send()
            .await
            .with_context(|| format!("requesting {url}"))?;
        if !resp.status().is_success() {
            anyhow::bail!(
                "download {} failed: {} ({url})",
                label,
                resp.status()
            );
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("creating {}", target.display()))?;
        }
        let tmp = target.with_extension("part");
        let file = fs::File::create(&tmp)
            .with_context(|| format!("creating {}", target.display()))?;
        let mut bytes_total = 0u64;
        let mut resp_body = resp.bytes_stream();
        while let Some(chunk) = resp_body.next().await {
            let chunk = chunk.context("stream chunk")?;
            bytes_total += chunk.len() as u64;
            std::io::Write::write_all(&mut std::io::BufWriter::new(&file), &chunk)
                .with_context(|| format!("writing {}", target.display()))?;
        }
        drop(file);
        fs::rename(&tmp, target).with_context(|| format!("finalizing {}", target.display()))?;
        let _ = (step, label, bytes_total);
        Ok(())
    }

    /// Fetch the version manifest; keeps `url` for direct per-version JSON retrieval.
    pub async fn fetch_version_list(&self) -> Result<Vec<VersionEntry>> {
        let url = format!("{PISTON_META}/mc/game/version_manifest_v2.json");
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
        let versions: Vec<VersionEntry> =
            serde_json::from_value(manifest["versions"].clone())?;
        Ok(versions)
    }

    /// Download + cache a version JSON. The manifest carries the authoritative URL.
    pub async fn get_version_json(&self, version: &str) -> Result<VersionJson> {
        let cache = self.versions_dir.join(version).join(format!("{version}.json"));
        if cache.exists() {
            let text = fs::read_to_string(&cache)?;
            return Ok(serde_json::from_str(&text)?);
        }
        let url = self
            .fetch_version_list()
            .await?
            .into_iter()
            .find(|v| v.id == version)
            .and_then(|v| v.url)
            .ok_or_else(|| anyhow::anyhow!("version {version} not found in manifest"))?;
        let text: String = self
            .http
            .get(&url)
            .send()
            .await
            .with_context(|| format!("fetching version json {url}"))?
            .text()
            .await?;
        let json: VersionJson = serde_json::from_str(&text).with_context(|| {
            format!(
                "parsing version json (len {}); first bytes: {}",
                text.len(),
                &text[..text.len().min(80)]
            )
        })?;
        if let Some(parent) = cache.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&cache, &text)?;
        Ok(json)
    }

    /// Ensure the client jar is present. Returns its path.
    pub async fn ensure_client_jar(
        &self,
        version: &str,
    ) -> Result<PathBuf> {
        let json = self.get_version_json(version).await?;
        let url = json
            .client_download_url()
            .context("version json has no client download url")?;
        let target = self
            .versions_dir
            .join(version)
            .join(format!("minecraft_{version}.jar"));
        if target.exists() {
            return Ok(target);
        }
        // TODO: emit progress when app handle is available
        self.stream_download(&url, &target, "client", 0).await?;
        Ok(target)
    }

    /// Ensure all classpath libraries are present (downloads missing ones), then unpack
    /// native jars into `natives_dir`. Returns classpath entries in version-json order.
    pub async fn ensure_libraries(
        &self,
        json: &VersionJson,
        natives_dir: &Path,
    ) -> Result<Vec<PathBuf>> {
        let mut classpath = Vec::new();
        let mut native_jars = Vec::new();
        let libs = json.libraries.as_deref().unwrap_or_default();
        let _total = libs.len() as u32;
        let current_os = Self::os_name();
        let current_arch = Self::os_arch();
        let mut step: u32 = 0;
        for lib in libs {
            step += 1;
            if let Some(rules) = &lib.rules {
                if !Self::rules_allow(rules, current_os, current_arch) {
                    continue;
                }
            }
            let name = match &lib.name {
                Some(n) => n.clone(),
                None => continue,
            };
            let is_natives = name.contains("natives-") || lib.natives.is_some();
            // maven coordinates: group:artifact:version[:classifier][@ext]
            let coord = name.split('@').next().unwrap();
            let parts: Vec<&str> = coord.split(':').collect();
            if parts.len() < 3 {
                continue;
            }
            let mut file_path = parts[1].to_string(); // fall back to artifact name
            let mut url: Option<String> = None;
            if let Some(artifact) = lib.downloads.as_ref().and_then(|d| d.artifact.as_ref()) {
                if let Some(p) = &artifact.path {
                    file_path = p
                        .rsplit_once('/')
                        .map(|(_, rest)| rest)
                        .unwrap_or(p.as_str())
                        .to_string();
                }
                url = Some(artifact.url.clone());
            }
            if url.is_none() {
                if let Some(u) = &lib.url {
                    url = Some(u.clone());
                } else {
                    // jdk-bundled / no-artifact library
                    if !is_natives {
                        classpath.push(self.libraries_dir.join(file_path.clone()));
                    }
                    continue;
                }
            }
            let url = url.unwrap();
            let jar = self.libraries_dir.join(&file_path);
            if !jar.exists() {
                if let Some(parent) = jar.parent() {
                    fs::create_dir_all(parent).ok();
                }
                self.stream_download(&url, &jar, "library", step)
                    .await
                    .with_context(|| format!("library {name} from {url}"))?;
            }
            if is_natives {
                native_jars.push(jar);
            } else {
                classpath.push(jar);
            }
        }
        Self::unpack_natives(&native_jars, natives_dir);
        Ok(classpath)
    }

    /// Ensure the asset index (used by old versions) + all referenced asset objects.
    pub async fn ensure_assets(&self, json: &VersionJson) -> Result<()> {
        let index_id = match json.asset_index_id() {
            Some(id) => id,
            None => return Ok(()), // modern versions ship assets inside the jar
        };
        let index_path = self.assets_dir.join("indexes").join(&index_id);
        let index_url = json
            .asset_index
            .as_ref()
            .and_then(|a| a.url.clone())
            .unwrap_or_else(|| {
                format!("https://launchermeta.mojang.com/assets/index/{index_id}.json")
            });
        if !index_path.exists() {
            if let Some(parent) = index_path.parent() {
                fs::create_dir_all(parent).ok();
            }
            let text = self.http.get(&index_url).send().await?.text().await?;
            fs::write(&index_path, &text)?;
        }
        let index: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&index_path)?)
                .map_err(|e| anyhow::anyhow!("bad asset index: {e}"))?;
        let objects = match index.get("objects") {
            Some(o) if o.is_object() => o,
            _ => return Ok(()),
        };
        let _total = objects.as_object().map(|m| m.len() as u32).unwrap_or(0);
        let mut step = 0u32;
        for (name, entry) in objects.as_object().unwrap() {
            step += 1;
            let hash = entry.get("url").and_then(|u| u.as_str()).unwrap_or(name);
            let target = self.assets_dir.join("objects").join(&hash[..2]).join(&hash[2..]);
            if !target.exists() {
                let url = format!("https://resources.mojang.com/{}/{}", &hash[..2], &hash[2..]);
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent).ok();
                }
                self.stream_download(&url, &target, "asset", step)
                    .await
                    .ok(); // individual asset failure is non-fatal
            }
        }
        Ok(())
    }

    // ---------- helpers ----------

    fn os_name() -> &'static str {
        std::env::consts::OS
    }

    fn os_arch() -> &'static str {
        std::env::consts::ARCH
    }

    /// Does the current OS/arch pass the rules? (last matching action wins)
    fn rules_allow(rules: &[DownloadRule], os: &str, arch: &str) -> bool {
        let mut allow = true;
        for rule in rules {
            let os_match = match &rule.os {
                Some(r) => {
                    if let Some(n) = &r.name {
                        if n != os {
                            break;
                        }
                    }
                    if let Some(a) = &r.arch {
                        if a != arch {
                            break;
                        }
                    }
                    true
                }
                None => true,
            };
            if !os_match {
                continue;
            }
            allow = rule.action == "allow";
        }
        allow
    }

    /// Unpack native (lwjgl) jars into the per-instance natives dir.
    /// `natives_dir` is per-game-directory so re-unpacking is always correct.
    fn unpack_natives(native_jars: &[PathBuf], natives_dir: &Path) {
        if native_jars.is_empty() {
            return;
        }
        let _ = fs::create_dir_all(natives_dir);
        let arch = Self::os_arch();
        // LWJGL classifier arch: "aarch64" (our host arch) is spelled "arm64" in maven.
        let natives_arch = if arch == "aarch64" { "arm64" } else { arch };
        let os = Self::os_name();
        for jar in native_jars {
            let Some(file_name) = jar.file_name().and_then(|s| s.to_str()).map(|s| s.to_string()) else {
                continue;
            };
            // arch-specific jar when we have one; otherwise the plain per-os jar.
            let _ = (os, natives_arch, file_name); // (kept for clarity)
            let Ok(mut zip) = std::fs::File::open(jar)
                .map_err(|_| ())
                .and_then(|f| zip::ZipArchive::new(std::io::BufReader::new(f)).map_err(|_| ()))
            else {
                continue;
            };
            for i in 0..zip.len() {
                let mut file = match zip.by_index(i) {
                    Ok(f) => f,
                    Err(_) => continue,
                };
                let Some(zname) = file.enclosed_name() else { continue };
                let target = natives_dir.join(zname);
                if file.is_dir() {
                    let _ = fs::create_dir_all(&target);
                    continue;
                }
                if let Some(parent) = target.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                let Ok(mut out) = fs::File::create(&target) else { continue };
                let _ = std::io::copy(&mut file, &mut out);
            }
        }
    }
}
