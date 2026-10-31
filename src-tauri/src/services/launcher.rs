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

    let Some(args_block) = version_json.arguments.as_ref() else {
        anyhow::bail!("version json has no arguments block");
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
    jvm_args.extend(instance.jvm_extra_args.clone());
    Ok((jvm_args, game_args))
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

    // 1. resolve java
    let java_path: PathBuf = match &instance.java_path {
        Some(p) => PathBuf::from(p),
        None => {
            let mgr = crate::services::java_manager::JavaManager::new();
            let javas = mgr.list()?;
            let want = instance
                .java_version
                .as_deref()
                .unwrap_or("21");
            let found = javas
                .iter()
                .find(|j| j.version.starts_with(want))
                .or_else(|| javas.iter().find(|j| j.is_managed))
                .ok_or_else(|| anyhow::anyhow!("no Java found on this system - use the Java tab to install one"))?;
            PathBuf::from(&found.path)
        }
    };

    // 2. download game files
    let version_json = state.mojang.get_version_json(&instance.version).await?;
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
