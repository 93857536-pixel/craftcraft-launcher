use crate::models::instance::{Instance, LaunchResult};
use crate::services::mojang::MojangClient;
use crate::utils;
use anyhow::Result;
use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use tauri::{AppHandle, Emitter, State};

pub struct LauncherState {
    pub mojang: MojangClient,
    pub running: tokio::sync::Mutex<Vec<u32>>,
}

impl LauncherState {
    pub fn new() -> Self {
        let data = utils::get_app_data_dir();
        Self {
            mojang: MojangClient::new(&data),
            running: tokio::sync::Mutex::new(Vec::new()),
        }
    }
}

/// Render one argument entry from a version JSON `arguments` array into zero or
/// more concrete tokens.
///
/// Entries are either a plain string or a rule-gated object
/// (`{"rules":[...], "value": "..." | [...]}`). Entries whose rules don't apply
/// on this platform yield **no tokens** (important: an empty token would be
/// parsed by the JVM as the main class name).
pub fn render_values(v: &Value, ctx: &std::collections::HashMap<&str, String>) -> Vec<String> {
    fn subst(s: &str, ctx: &std::collections::HashMap<&str, String>) -> String {
        let mut out = s.to_string();
        for (k, val) in ctx {
            out = out.replace(&format!("${{{}}}", k), val);
        }
        out
    }

    /// Does a single rule object apply on this platform?
    fn rule_applies(rule: &serde_json::Value) -> bool {
        if rule.get("action").and_then(|a| a.as_str()) != Some("allow") {
            return false;
        }
        // Feature-gated args (demo, custom resolution, quick play) are never enabled.
        if rule.get("features").is_some() {
            return false;
        }
        if let Some(os) = rule.get("os") {
            if let Some(n) = os.get("name").and_then(|n| n.as_str()) {
                let cur = if cfg!(target_os = "macos") { "osx" } else { std::env::consts::OS };
                if n != cur {
                    return false;
                }
            }
            if let Some(a) = os.get("arch").and_then(|a| a.as_str()) {
                // Mojang uses x86 / aarch64; Rust uses x86 / x86_64 / aarch64.
                let cur = std::env::consts::ARCH;
                let matches = match a {
                    "x86" => cur == "x86" || cur == "i686",
                    "x86_64" => cur == "x86_64",
                    "aarch64" => cur == "aarch64",
                    other => cur == other,
                };
                if !matches {
                    return false;
                }
            }
        }
        true
    }

    fn collect(v: &Value, ctx: &std::collections::HashMap<&str, String>, out: &mut Vec<String>) {
        match v {
            Value::String(s) => out.push(subst(s, ctx)),
            Value::Array(items) => {
                for item in items {
                    collect(item, ctx, out);
                }
            }
            Value::Object(obj) => {
                if let Some(rules) = obj.get("rules").and_then(|r| r.as_array()) {
                    if !rules.iter().any(rule_applies) {
                        return; // rule does not apply: emit nothing
                    }
                }
                if let Some(val) = obj.get("value") {
                    collect(val, ctx, out);
                }
            }
            _ => {}
        }
    }

    let mut parts = Vec::new();
    collect(v, ctx, &mut parts);
    parts
}

/// Backwards-compatible wrapper: joins tokens with a space.
pub fn render_value(v: &Value, ctx: &std::collections::HashMap<&str, String>) -> String {
    render_values(v, ctx).join(" ")
}

/// Build the JVM + game arguments for a vanilla launch.
pub fn build_launch_args(
    instance: &Instance,
    version_json: &crate::services::mojang::VersionJson,
    client_jar: &PathBuf,
    libraries: &[PathBuf],
    assets_dir: &PathBuf,
) -> Result<(Vec<String>, Vec<String>)> {
    build_launch_args_with_natives(
        instance,
        version_json,
        client_jar,
        libraries,
        assets_dir,
        &PathBuf::from(&instance.game_directory).join("natives"),
    )
}

pub fn build_launch_args_with_natives(
    instance: &Instance,
    version_json: &crate::services::mojang::VersionJson,
    client_jar: &PathBuf,
    libraries: &[PathBuf],
    assets_dir: &PathBuf,
    natives_dir: &PathBuf,
) -> Result<(Vec<String>, Vec<String>)> {
    let assets = version_json.asset_index_id().unwrap_or_default();
    let sep = if cfg!(windows) {
        ";".to_string()
    } else {
        ":".to_string()
    };
    let classpath: String = {
        let mut cp = vec![client_jar.to_string_lossy().to_string()];
        cp.extend(libraries.iter().map(|p| p.to_string_lossy().to_string()));
        cp.join(&sep)
    };
    // Offline-mode identity. Vanilla accepts any well-formed UUID/token pair when
    // launching without contacting the auth servers.
    let offline_uuid = {
        use md5::{Digest, Md5};
        let mut hasher = Md5::new();
        hasher.update(format!("OfflinePlayer:{}", instance.account_name).as_bytes());
        let d = hasher.finalize();
        let mut b = [0u8; 16];
        b.copy_from_slice(&d[..16]);
        b[6] = (b[6] & 0x0f) | 0x30; // version 3
        b[8] = (b[8] & 0x3f) | 0x80; // variant
        let h: Vec<String> = b.iter().map(|x| format!("{x:02x}")).collect();
        format!(
            "{}-{}-{}-{}-{}",
            h[0..4].join(""),
            h[4..6].join(""),
            h[6..8].join(""),
            h[8..10].join(""),
            h[10..16].join("")
        )
    };
    let natives_dir = natives_dir.to_string_lossy().to_string();

    let ctx_items: Vec<(&str, String)> = vec![
        // classpath / paths
        ("classpath", classpath.clone()),
        ("classpath_separator", sep.clone()),
        ("natives_directory", natives_dir),
        ("game_directory", instance.game_directory.clone()),
        ("game_assets", assets_dir.to_string_lossy().to_string()),
        ("assets_root", assets_dir.to_string_lossy().to_string()),
        ("assets_directory", assets_dir.to_string_lossy().to_string()),
        ("asset_index", assets.clone()),
        ("assets_index_name", assets.clone()),
        // version identity
        ("version", instance.version.clone()),
        ("version_name", instance.version.clone()),
        ("version_type", "release".to_string()),
        // launcher identity
        ("launcher_name", "CraftCraft".to_string()),
        ("launcher_version", env!("CARGO_PKG_VERSION").to_string()),
        // auth (offline)
        ("auth_player_name", instance.account_name.clone()),
        ("auth_uuid", offline_uuid.clone()),
        ("uuid", offline_uuid),
        ("auth_access_token", "0".to_string()),
        ("access_token", "0".to_string()),
        ("clientid", String::new()),
        ("auth_xuid", String::new()),
        ("user_type", "legacy".to_string()),
        ("user_properties", "{}".to_string()),
        // legacy / platform
        ("legacy_user_hash", "0".to_string()),
        ("xplatform", if cfg!(target_os = "macos") { "osx".to_string() } else { std::env::consts::OS.to_string() }),
        // window size defaults (only used when the custom-resolution feature is on)
        ("resolution_width", "1280".to_string()),
        ("resolution_height", "720".to_string()),
        ("quickPlayPath", String::new()),
        ("quickPlaySingleplayer", String::new()),
        ("quickPlayMultiplayer", String::new()),
        ("quickPlayRealms", String::new()),
    ];
    let mut ctx: std::collections::HashMap<&str, String> = std::collections::HashMap::new();
    for (k, v) in ctx_items {
        ctx.insert(k, v);
    }

    let mut jvm_args: Vec<String> = vec![
        format!("-Xmx{}m", instance.memory_max_mb),
        format!("-Xms{}m", instance.memory_min_mb),
        format!("-XX:MaxMetaspaceSize={}m", (instance.memory_max_mb / 4).max(256)),
    ];

    // 1.7.x 老版本可能没有 arguments 块, 但有 minecraftArguments
    let args_block = match version_json.arguments.as_ref() {
        Some(a) => a,
        None => {
            // Fallback: create empty args_block for versions that only have minecraftArguments
            if version_json.minecraft_arguments.is_some() {
                use crate::services::mojang::ArgumentsBlock;
                &ArgumentsBlock { game: None, jvm: None }
            } else {
                anyhow::bail!("version json has no arguments block");
            }
        }
    };

    if let Some(jvm) = &args_block.jvm {
        for v in jvm {
            jvm_args.extend(render_values(v, &ctx));
        }
    }

    let mut game_args: Vec<String> = vec![];
    if let Some(game) = &args_block.game {
        for v in game {
            game_args.extend(render_values(v, &ctx));
        }
    } else if let Some(legacy) = &version_json.minecraft_arguments {
        // versions ≤ 1.12: a single whitespace-separated template string
        for token in legacy.split_whitespace() {
            let rendered = render_value(&Value::String(token.to_string()), &ctx);
            if !rendered.is_empty() {
                game_args.push(rendered);
            }
        }
    }

    // Newer versions require these to be present; ensure --assetIndex/--assetsDir
    if !game_args.iter().any(|a| a.starts_with("--assetsDir")) {
        game_args.push("--assetsDir".into());
        game_args.push(assets_dir.to_string_lossy().to_string());
        game_args.push("--assetIndex".into());
        game_args.push(assets);
    }
    if !game_args.iter().any(|a| a.starts_with("--gameDir")) {
        game_args.push("--gameDir".into());
        game_args.push(instance.game_directory.clone());
    }
    if !game_args.iter().any(|a| a.starts_with("--username")) {
        game_args.push("--username".into());
        game_args.push(instance.account_name.clone());
    }
    // 游戏内语言：`--language <locale>`。1.6+ 才认这个参数（更老的版本会忽略未知参数，
    // 但为稳妥起见只在支持的版本上追加）。
    if let Some(lang) = instance
        .language
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        if language_arg_supported(&instance.version) && !game_args.iter().any(|a| a == "--language") {
            game_args.push("--language".into());
            game_args.push(lang.replace('-', "_").to_ascii_lowercase());
        }
    }
    jvm_args.extend(instance.jvm_extra_args.clone());
    Ok((jvm_args, game_args))
}

/// `--language` 从 Minecraft 1.6 开始被游戏识别；更早的版本没有内置多语言。
fn language_arg_supported(mc_version: &str) -> bool {
    let clean = mc_version.split('-').next().unwrap_or(mc_version).trim();
    // 新版本命名（21w44a / 26.3 / 24.1）都支持
    if !clean.starts_with("1.") {
        return true;
    }
    let mut it = clean.split('.');
    let major: u32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(1);
    let minor: u32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    major > 1 || minor >= 6
}

pub async fn launch(
    app: &AppHandle,
    state: &State<'_, LauncherState>,
    instance: &Instance,
) -> Result<LaunchResult> {
    let data = utils::get_app_data_dir();
    let game_dir = if PathBuf::from(&instance.game_directory).is_absolute() {
        PathBuf::from(&instance.game_directory)
    } else {
        data.join("instances").join(&instance.id)
    };
    let _ = std::fs::create_dir_all(&game_dir);

    // 1. version metadata first — it declares which Java the game needs.
    let version_json = state.mojang.get_version_json(&instance.version).await?;

    // 2. resolve java (Mojang's javaVersion wins over the UI default)
    let java_path: PathBuf = match &instance.java_path {
        Some(p) => PathBuf::from(p),
        None => {
            let mgr = crate::services::java_manager::JavaManager::new();
            let rec = crate::models::java::JavaRecommendation::from_version_json(
                &instance.version,
                version_json.required_java_major().as_deref(),
            );
            let want = instance
                .java_version
                .clone()
                .unwrap_or_else(|| rec.recommended_java.clone());
            // Apple Silicon + pre-1.19 → the JVM itself must be x86_64 (Rosetta 2),
            // because those releases only ship x86_64 lwjgl natives.
            let force_x64 = crate::models::java::needs_x64_jvm(&instance.version);
            let javas = mgr.list()?;
            let arch_ok = |p: &str| !force_x64 || crate::models::java::binary_arch(p) == "x86_64";
            let major =
                |j: &crate::models::java::JavaVersion| {
                    crate::models::java::major_of(&j.version).unwrap_or(u32::MAX)
                };
            // 兜底也只能用「版本够用」的 JDK：宁可装新的，也绝不退到更低版本，
            // 否则就是 UnsupportedClassVersionError（Java 8 跑 1.18.2）。
            let ok = |j: &crate::models::java::JavaVersion| {
                arch_ok(&j.path) && crate::models::java::satisfies_requirement(&j.version, &want)
            };
            let found = javas
                .iter()
                .find(|j| j.matches_release(&want) && arch_ok(&j.path))
                .or_else(|| {
                    javas
                        .iter()
                        .filter(|j| j.is_managed && ok(j))
                        .min_by_key(|j| major(j))
                })
                .or_else(|| javas.iter().filter(|j| ok(j)).min_by_key(|j| major(j)));
            match found {
                Some(j) => PathBuf::from(&j.path),
                None => {
                    // Nothing suitable installed — fetch the right JVM on demand.
                    let j = mgr.install_arch(&want, force_x64).await?;
                    PathBuf::from(&j.path)
                }
            }
        }
    };

    // 3. download game files
    let client_jar = state.mojang.ensure_client_jar(&instance.version).await?;
    let natives_dir = game_dir.join("natives");
    let libraries: Vec<PathBuf> = state
        .mojang
        .ensure_libraries(&version_json, &natives_dir)
        .await?;
    state.mojang.ensure_assets(&version_json).await?;

    // 3. build args
    let assets_dir = data.join("assets");
    let (mut jvm_args, game_args) =
        build_launch_args_with_natives(
            instance,
            &version_json,
            &client_jar,
            &libraries,
            &assets_dir,
            &natives_dir,
        )?;
    // The version's own jvm block already contributes `-cp ${classpath}`.
    let sep = if cfg!(windows) { ";" } else { ":" };
    if !jvm_args.iter().any(|a| a == "-cp" || a == "-classpath") {
        let mut cp = vec![client_jar.to_string_lossy().to_string()];
        cp.extend(libraries.iter().map(|p| p.to_string_lossy().to_string()));
        jvm_args.push("-cp".into());
        jvm_args.push(cp.join(sep));
    }

    // 4. main class
    let main_class = version_json
        .main_class
        .clone()
        .ok_or_else(|| anyhow::anyhow!("version has no main class"))?;

    // 5. spawn
    let log_dir = game_dir.join("logs");
    let _ = std::fs::create_dir_all(&log_dir);
    let log_file = log_dir.join("latest.log");
    let log = std::fs::File::create(&log_file)?;

    let mut cmd = Command::new(&java_path);
    cmd.args(&jvm_args);
    // 1.13+ versions use the jvmArguments block to set -Djava.library.path.
    // Pre-1.13 (1.7.x–1.12.x) did NOT set it in the version JSON, so we must
    // inject it ourselves here for legacy versions that unpack native jars.
    if version_json.arguments.is_none() && version_json.minecraft_arguments.is_some() {
        // Legacy path: add the flag before the main class.
        cmd.arg(format!("-Djava.library.path={}", natives_dir.display()));
    }
    cmd.arg(&main_class);
    cmd.args(&game_args);
    cmd.current_dir(&game_dir);
    cmd.stdout(Stdio::null());
    cmd.stderr(std::process::Stdio::from(log));

    let child = cmd.spawn()?;
    let pid = child.id();

    {
        let mut running = state.running.lock().await;
        running.push(pid);
    }
    let _ = app.emit(
        "instance:launched",
        serde_json::json!({ "instanceId": instance.id, "pid": pid }),
    );
    Ok(LaunchResult {
        success: true,
        pid: Some(pid),
        message: format!("launched {} (pid {})", instance.name, pid),
    })
}

pub fn read_log_tail(instance_id: &str, lines: u32) -> Result<Vec<String>> {
    let data = utils::get_app_data_dir();
    let path = data
        .join("instances")
        .join(instance_id)
        .join("logs")
        .join("latest.log");
    if !path.exists() {
        return Ok(vec![]);
    }
    let content = std::fs::read_to_string(&path)?;
    let all: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    let start = all.len().saturating_sub(lines as usize);
    Ok(all[start..].to_vec())
}

/// Build a human-readable preview of the launch command (for debugging/transparency).
pub fn build_launch_args_debug(instance: &Instance) -> Result<String, String> {
    let java_path = instance
        .java_path
        .as_ref()
        .ok_or("No Java path configured")?;
    let sep = if cfg!(windows) { ";" } else { ":" };
    let cp = format!("{}/client.jar{}", instance.game_directory, sep);
    let natives = format!("{}/natives", instance.game_directory);
    let assets = format!("{}/assets", instance.game_directory);
    let args: Vec<String> = vec![
        format!("-Xmx{}M", instance.memory_max_mb),
        format!("-Xms{}M", instance.memory_min_mb),
        format!("-Djava.library.path={}", natives),
        format!("-Dminecraft.assets.root={}", assets),
        "-Dfml.ignoreInvalidMinecraftCertificates=true".to_string(),
        "-Dfml.ignorePatchDiscrepancies=true".to_string(),
        format!(
            "-Dminecraft.launcher.version=CraftCraft Launcher {}",
            env!("CARGO_PKG_VERSION")
        ),
        format!("-Dminecraft.launcher.brand=CraftCraft Launcher"),
        format!("-Dminecraft.client.name={}", instance.account_name),
        format!("-Dminecraft.client.id={}", "offline"),
    ];
    let jvm = instance.jvm_extra_args.clone();
    let all_args: Vec<String> = args.iter().chain(jvm.iter()).cloned().collect();
    Ok(format!(
        "{} {} {}",
        java_path,
        all_args.join(" "),
        format!("{}{}{}{}", sep, cp, sep, "net.minecraft.client.main.Main"),
    ))
}

#[cfg(test)]
mod language_tests {
    use super::*;
    use crate::models::instance::Instance;
    use crate::services::mojang::{AssetsBlock, VersionJson};

    fn minimal_version(mc: &str) -> VersionJson {
        VersionJson {
            id: mc.to_string(),
            main_class: Some("net.minecraft.client.main.Main".into()),
            minecraft_arguments: Some(
                "--username ${auth_player_name} --version ${version_name}".into(),
            ),
            asset_index: Some(AssetsBlock {
                id: "test".into(),
                url: None,
            }),
            ..Default::default()
        }
    }

    fn args_for(mc: &str, language: Option<&str>) -> Vec<String> {
        let mut inst = Instance::new("t", mc, "/tmp/t".into());
        inst.language = language.map(|s| s.to_string());
        let vj = minimal_version(mc);
        let (_jvm, game) = build_launch_args_with_natives(
            &inst,
            &vj,
            &PathBuf::from("/tmp/client.jar"),
            &[],
            &PathBuf::from("/tmp/assets"),
            &PathBuf::from("/tmp/natives"),
        )
        .expect("args");
        game
    }

    #[test]
    fn debug_preview_embeds_package_version() {
        use crate::models::instance::Instance;
        let mut inst = Instance::new("t", "1.21", "/tmp/g".into());
        inst.java_path = Some("/usr/bin/java".into());
        let cmd = build_launch_args_debug(&inst).expect("preview");
        assert!(
            cmd.contains(env!("CARGO_PKG_VERSION")),
            "preview must carry the real package version, got: {cmd}"
        );
        assert!(
            !cmd.contains("v0.1.0"),
            "stale hardcoded version leaked into preview: {cmd}"
        );
    }

    #[test]
    fn injects_language_flag_when_set() {
        let args = args_for("1.21", Some("tlh_aa"));
        let pos = args.iter().position(|a| a == "--language");
        assert!(pos.is_some(), "expected --language in {:?}", args);
        assert_eq!(args[pos.unwrap() + 1], "tlh_aa");
    }

    #[test]
    fn normalises_hyphenated_locale() {
        let args = args_for("1.21", Some("zh-CN"));
        let pos = args.iter().position(|a| a == "--language").expect("flag");
        assert_eq!(args[pos + 1], "zh_cn");
    }

    #[test]
    fn omits_flag_when_unset_or_empty() {
        for lang in [None, Some(""), Some("   ")] {
            let args = args_for("1.21", lang);
            assert!(!args.iter().any(|a| a == "--language"), "lang={:?} -> {:?}", lang, args);
        }
    }

    #[test]
    fn skips_flag_for_pre_1_6() {
        assert!(!language_arg_supported("1.5.2"));
        assert!(language_arg_supported("1.6.4"));
        assert!(language_arg_supported("1.7.10"));
        assert!(language_arg_supported("1.21"));
        assert!(language_arg_supported("26.3")); // 新版本命名
        let args = args_for("1.5.2", Some("zh_cn"));
        assert!(!args.iter().any(|a| a == "--language"));
    }
}
