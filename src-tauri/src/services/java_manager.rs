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
        self.install_arch(version, false).await
    }

    /// Install a Temurin JDK, optionally forcing the x86_64 build.
    ///
    /// `prefer_x64` matters on Apple Silicon: Minecraft ≤1.18.2 ships x86_64-only
    /// lwjgl natives, so the *JVM itself* has to be x86_64 (running under Rosetta)
    /// for those natives to load. Modern versions use arm64 natives and are happy
    /// with the native aarch64 build.
    pub async fn install_arch(&self, version: &str, prefer_x64: bool) -> Result<JavaVersion> {
        // Keep x86_64 and arm64 installs of the same release side by side: an
        // Apple Silicon box needs x64 Java for ≤1.18.2 (x86_64-only natives) and
        // arm64 Java for 1.19+ (arm64 natives).
        let dir_name = if prefer_x64 {
            format!("temurin-{version}-x64")
        } else {
            format!("temurin-{version}")
        };
        let target_dir = self.javas_dir.join(&dir_name);
        if target_dir.exists() && self.dir_holds_release(&target_dir, version) {
            return Ok(self.build_entry(&target_dir, version, true));
        }
        // A previous run may have extracted the tarball but failed to move it.
        // Only adopt such a leftover when it truly holds the requested release —
        // otherwise a Java 8 extraction would get renamed into `temurin-17`.
        if let Ok(entries) = std::fs::read_dir(&self.javas_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
                if p.is_dir()
                    && name.starts_with("jdk")
                    && name != dir_name
                    && self.dir_holds_release(&p, version)
                {
                    if target_dir.exists() {
                        let _ = std::fs::remove_dir_all(&target_dir);
                    }
                    std::fs::rename(&p, &target_dir)?;
                    return Ok(self.build_entry(&target_dir, version, true));
                }
            }
        }
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

        // Adoptium 并非所有 JDK 都有 arm64 原生包(例: Java 8 只有 x64);
        // 首选架构 404 时回退 x64(mac 上经 Rosetta 运行)。
        let http = reqwest::Client::new();
        let candidates: Vec<&str> = if prefer_x64 {
            vec!["x64"]
        } else if arch == "aarch64" {
            vec!["aarch64", "x64"]
        } else {
            vec![arch]
        };
        std::fs::create_dir_all(&self.javas_dir)?;
        // Adoptium answers with a 307 to GitHub's release-assets origin. reqwest
        // occasionally fails that handshake while curl sails through, so every
        // candidate gets a curl second chance before we give up.
        let mut payload: Option<Vec<u8>> = None;
        let mut failures: Vec<String> = Vec::new();
        'candidates: for a in &candidates {
            let url = format!(
                "https://api.adoptium.net/v3/binary/latest/{version}/ga/{os}/{a}/jdk/hotspot/normal/eclipse",
            );
            match http.get(&url).send().await {
                Ok(r) if r.status().is_success() => match r.bytes().await {
                    Ok(b) if !b.is_empty() => {
                        payload = Some(b.to_vec());
                        break 'candidates;
                    }
                    Ok(_) => failures.push(format!("{a}: empty body")),
                    Err(e) => failures.push(format!("{a}: body {e}")),
                },
                Ok(r) => failures.push(format!("{a}: status {}", r.status())),
                Err(e) => failures.push(format!("{a}: {e}")),
            }
            // Fast mirror before the official redirect: GitHub release-assets are
            // often throttled to <100 KB/s while Tsinghua's mirror is quick.
            if let Some(bytes) = Self::download_from_tuna(version, a, os).await {
                payload = Some(bytes);
                break 'candidates;
            }
            // curl fallback against the official endpoint (slow but authoritative)
            let tmp = self
                .javas_dir
                .join(format!("temurin-{version}-{a}.tar.gz.part"));
            let _ = std::fs::remove_file(&tmp);
            let ok = Command::new("curl")
                .args([
                    "-fL",
                    "--retry",
                    "3",
                    "--connect-timeout",
                    "30",
                    "-o",
                ])
                .arg(&tmp)
                .arg(&url)
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
            if ok {
                if let Ok(bytes) = std::fs::read(&tmp) {
                    let _ = std::fs::remove_file(&tmp);
                    if !bytes.is_empty() {
                        payload = Some(bytes);
                        break 'candidates;
                    }
                }
            }
            let _ = std::fs::remove_file(&tmp);
            eprintln!("[java] Adoptium {a} 不可用({}),尝试下一个架构", failures.last().cloned().unwrap_or_default());
        }
        let resp = match payload {
            Some(b) => Some(b),
            None => None,
        };

        // Adoptium never shipped a macOS **aarch64** Java 8 build, and its legacy
        // macOS x64 artifacts are unreliable (404 / throttled). Zulu still
        // publishes Java 8 for macOS x86_64 — and an x86_64 JVM is mandatory for
        // Minecraft ≤1.12 natives on Apple Silicon (they load through Rosetta 2).
        let mut via_zulu = false;
        let mut resp = resp;
        if resp.is_none() && os == "mac" && Self::zulu_eligible(version) {
            if Self::fetch_zulu_archive(version, &self.javas_dir).await {
                via_zulu = true;
                eprintln!("[java] Adoptium 无 macOS {version} 包，已改用 Zulu（Rosetta 2 运行）");
            }
        }
        if resp.is_none() && !via_zulu {
            return Err(anyhow::anyhow!(
                "Temurin {version} 下载失败({os}, 尝试过 {:?}): {}",
                candidates,
                failures.join("; ")
            ));
        }
        let archive = self
            .javas_dir
            .join(format!("{}-{version}.tar.gz", if via_zulu { "zulu" } else { "temurin" }));
        if let Some(bytes) = resp.take() {
            std::fs::write(&archive, &bytes)?;
        }

        // Snapshot existing dirs so the freshly extracted one can be identified
        // reliably (a leftover `jdk8u...` dir must not be mistaken for the new one).
        let before: std::collections::HashSet<String> = std::fs::read_dir(&self.javas_dir)?
            .flatten()
            .filter(|e| e.path().is_dir())
            .filter_map(|e| e.file_name().to_str().map(|s| s.to_string()))
            .collect();

        let status = Command::new("tar")
            .args(["-xzf", archive.to_str().unwrap(), "-C"])
            .arg(&self.javas_dir)
            .status()?;
        if !status.success() {
            return Err(anyhow::anyhow!("failed to extract JDK archive"));
        }
        // adoptium layout: a `jdk-<ver>+<build>` directory (Java 11+) or
        // `jdk8u504-b01` (Java 8). Only accept directories that appeared *after*
        // extraction so we never adopt a stale sibling of a different release.
        let mut extracted: Option<PathBuf> = None;
        for entry in std::fs::read_dir(&self.javas_dir)?.flatten() {
            let p = entry.path();
            if !p.is_dir() {
                continue;
            }
            let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
            // Adoptium extracts as `jdk-<ver>+<build>` / `jdk8u504-b01`; Zulu
            // extracts as `zulu8.98.0.107-ca-jdk8.0.512-macosx_x64`.
            if (name.starts_with("jdk") || name.starts_with("zulu"))
                && name != format!("temurin-{version}")
                && !before.contains(name)
            {
                extracted = Some(p);
                break;
            }
        }
        if let Some(p) = extracted {
            let is_zulu = p
                .file_name()
                .and_then(|s| s.to_str())
                .map(|n| n.starts_with("zulu"))
                .unwrap_or(false);
            // Keep vendor-specific directory names so both an x64 and an arm64 JDK
            // of the same release can coexist (Apple Silicon needs exactly that).
            let final_dir = if is_zulu {
                self.javas_dir.join(format!("zulu-{version}-x64"))
            } else {
                target_dir.clone()
            };
            // Drop a stale target from a previous partial run, then move into place.
            if final_dir.exists() {
                let _ = std::fs::remove_dir_all(&final_dir);
            }
            std::fs::rename(&p, &final_dir)?;
            let _ = std::fs::remove_file(&archive);
            return Ok(self.build_entry(&final_dir, version, true));
        }
        Err(anyhow::anyhow!(
            "JDK extracted but no jdk* directory found in {}",
            self.javas_dir.display()
        ))
    }

    /// Pull a Temurin tarball from Tsinghua's Adoptium mirror.
    ///
    /// GitHub's release-assets origin (where `api.adoptium.net` redirects) is
    /// routinely throttled to well under 100 KB/s from mainland China, so the
    /// official endpoint can look broken while the mirror is fast.
    async fn download_from_tuna(version: &str, arch: &str, os: &str) -> Option<Vec<u8>> {
        let os_dir = match os {
            "mac" => "mac",
            "linux" => "linux",
            _ => return None, // Windows builds are .zip; keep the official path only.
        };
        let listing_url = format!(
            "https://mirrors.tuna.tsinghua.edu.cn/Adoptium/{version}/jdk/{arch}/{os_dir}/"
        );
        let listing = reqwest::get(&listing_url).await.ok()?.text().await.ok()?;
        let needle = format!("OpenJDK{version}U-jdk_{arch}_{os_dir}_hotspot_");
        let mut best: Option<String> = None;
        for token in listing.split('"') {
            if token.starts_with(&needle) && token.ends_with(".tar.gz") {
                if best.as_deref().map(|b| token > b).unwrap_or(true) {
                    best = Some(token.to_string());
                }
            }
        }
        let file = best?;
        let url = format!("{listing_url}{file}");
        eprintln!("[java] Adoptium 官方源不可用，改用清华镜像: {url}");
        let tmp = std::env::temp_dir().join(format!("craftcraft-{file}"));
        let ok = Command::new("curl")
            .args(["-fL", "--retry", "3", "--connect-timeout", "30", "-o"])
            .arg(&tmp)
            .arg(&url)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if !ok {
            let _ = std::fs::remove_file(&tmp);
            return None;
        }
        let bytes = std::fs::read(&tmp).ok();
        let _ = std::fs::remove_file(&tmp);
        bytes.filter(|b| !b.is_empty())
    }

    /// Is this release one that Adoptium cannot serve on macOS but Zulu can?
    /// (Java 8 never had an Apple Silicon build, and legacy x64 mac artifacts are
    /// frequently missing from Adoptium's API.)
    fn zulu_eligible(version: &str) -> bool {
        let v = version.trim().trim_start_matches("1.");
        v == "8" || v == "11"
    }

    /// Download the latest Azul Zulu JDK tarball into `dir`.
    ///
    /// Returns `true` when the archive landed on disk at
    /// `dir/zulu-<version>.tar.gz` and is non-trivial in size.
    async fn fetch_zulu_archive(version: &str, dir: &PathBuf) -> bool {
        let java_version = version.trim().trim_start_matches("1.").to_string();
        // Zulu's metadata API reports the exact asset name for the current build,
        // so we never hard-code a version that can get rotated away.
        let meta = format!(
            "https://api.azul.com/metadata/v1/zulu/packages/?java_version={java_version}\
             &os=macos&arch=x64&archive_type=tar.gz&java_package_type=jdk\
             &release_status=ga&latest=true&page_size=1"
        );
        let client = reqwest::Client::new();
        let mut download_url: Option<String> = None;
        if let Ok(resp) = client.get(&meta).send().await {
            if resp.status().is_success() {
                // `json` feature is not enabled on this reqwest build — decode by hand.
                if let Ok(text) = resp.text().await {
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                        download_url = v
                            .get(0)
                            .and_then(|p| p.get("download_url"))
                            .and_then(|u| u.as_str())
                            .map(|s| s.to_string());
                    }
                }
            }
        }
        // Offline / API-shape fallback: a build we have verified end-to-end.
        let url = download_url
            .unwrap_or_else(|| "https://cdn.azul.com/zulu/bin/zulu8.98.0.107-ca-jdk8.0.512-macosx_x64.tar.gz".to_string());
        if !url.contains("macosx_x64") && !url.contains("macos") {
            return false;
        }
        let archive = dir.join(format!("zulu-{version}.tar.gz"));
        let _ = std::fs::remove_file(&archive);
        let ok = Command::new("curl")
            .args(["-fL", "--retry", "3", "--connect-timeout", "30", "-o"])
            .arg(&archive)
            .arg(&url)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if !ok {
            let _ = std::fs::remove_file(&archive);
            return false;
        }
        let size = std::fs::metadata(&archive).map(|m| m.len()).unwrap_or(0);
        if size < 10 * 1024 * 1024 {
            let _ = std::fs::remove_file(&archive);
            return false;
        }
        eprintln!("[java] Zulu 下载完成: {} ({:.0} MB)", url, size as f64 / 1_048_576.0);
        true
    }

    /// Does the JDK in `dir` actually contain the release we want?
    /// Probes `java -version` so a stale extraction of a *different* Java can't be
    /// adopted (Java 8 extracts as `jdk8u504-b01`, which used to get renamed into
    /// `temurin-17` by the leftover-recovery path).
    fn dir_holds_release(&self, dir: &PathBuf, want: &str) -> bool {
        let java_bin = utils::get_java_binary_path(dir);
        if !java_bin.exists() {
            return false;
        }
        match self.validate(&java_bin) {
            Some((probed, _)) => {
                let probe = crate::models::java::JavaVersion::new(
                    String::new(),
                    String::new(),
                    probed,
                    String::new(),
                    String::new(),
                    false,
                );
                probe.matches_release(want)
            }
            None => false,
        }
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
                // Probe the real version (`1.8.0_504`, `21.0.13`) instead of trusting
                // the directory name — Adoptium extracts Java 8 as `jdk8u504-b01`,
                // which would otherwise never match a "8" recommendation.
                let dir_name = p
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("?")
                    .to_string();
                let (version, probed_vendor) = match self.validate(&java_bin) {
                    Some((v, vendor)) => (v, vendor),
                    None => (dir_name, "Temurin".to_string()),
                };
                out.push(JavaVersion::new(
                    Uuid::new_v4().to_string(),
                    java_bin.to_string_lossy().to_string(),
                    version,
                    if probed_vendor.is_empty() { "Temurin".to_string() } else { probed_vendor },
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
