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

pub fn render_value(v: &Value, ctx: &std::collections::HashMap<&str, String>) -> String {
    match v {
        Value::String(s) => {
            // substitute ${var} placeholders
            let mut out = s.clone();
            for (k, val) in ctx {
                out = out.replace(&format!("${{{}}}", k), val);
            }
            out
        }
        Value::Array(items) => {
            // new-format arguments: array of {rules:[...]} object or string
            let mut parts = Vec::new();
            for item in items {
                if let Some(obj) = item.as_object() {
                    let rules = obj.get("rules").and_then(|r| r.as_array());
                    let compatible = match rules {
                        Some(rules) => {
                            let os = std::env::consts::OS;
                            rules.iter().any(|r| {
                                let r = r.as_object().unwrap_or(&obj);
                                if r.get("action").and_then(|a| a.as_str()) == Some("allow") {
                                    let os_rule = r.get("os");
                                    match os_rule {
                                        Some(o) => {
                                            o.get("name")
                                                .and_then(|n| n.as_str())
                                                .map(|n| n == os)
                                                .unwrap_or(true)
                                        }
                                        None => true,
                                    }
                                } else {
                                    false
                                }
                            })
                        }
                        None => true,
                    };
                    if !compatible {
                        continue;
                    }
                    if let Some(v) = obj.get("value") {
                        let s = match v {
                            Value::String(s) => s.clone(),
                            Value::Array(a) => a
                                .iter()
                                .map(|x| x.as_str().unwrap_or("").to_string())
                                .collect(),
                            _ => continue,
                        };
                        parts.push(s);
                    }
                }
            }
            parts.join(" ")
        }
        _ => String::new(),
    }
}

/// Build the JVM + game arguments for a vanilla launch.
pub fn build_launch_args(
    instance: &Instance,
    version_json: &crate::services::mojang::VersionJson,
    client_jar: &PathBuf,
    libraries: &[PathBuf],
    assets_dir: &PathBuf,
) -> Result<(Vec<String>, Vec<String>)> {
    let assets = version_json.asset_index.as_ref().map(|a| a.id.clone()).unwrap_or_default();
    let classpath: String = {
        let mut cp = vec![client_jar.to_string_lossy().to_string()];
        cp.extend(libraries.iter().map(|p| p.to_string_lossy().to_string()));
        cp.join(":")
    };

    let ctx_items: Vec<(&str, String)> = vec![
        ("classpath", classpath.clone()),
        ("classpath_separator", ":".to_string()),
        ("version", instance.version.clone()),
        ("game_directory", instance.game_directory.clone()),
        ("assets_directory", assets_dir.to_string_lossy().to_string()),
        ("asset_index", assets.clone()),
        ("auth_player_name", instance.account_name.clone()),
        ("user_type", "mojang".to_string()),
        ("natives_directory", client_jar.parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default()),
        ("legacy_user_hash", "0".to_string()),
        ("xplatform", if cfg!(target_os = "macos") { "osx".to_string() } else { std::env::consts::OS.to_string() }),
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
            jvm_args.push(render_value(v, &ctx));
        }
    }

    let mut game_args: Vec<String> = vec![];
    if let Some(game) = &args_block.game {
        for v in game {
            game_args.push(render_value(v, &ctx));
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
    let libraries: Vec<PathBuf> = state.mojang.ensure_libraries(&version_json).await?;
    state.mojang.ensure_assets(&version_json).await?;

    // 3. build args
    let assets_dir = data.join("assets");
    let (mut jvm_args, mut game_args) = build_launch_args(
        instance,
        &version_json,
        &client_jar,
        &libraries,
        &assets_dir,
    )?;
    jvm_args.push("-cp".into());
    jvm_args.push(format!(
        "{}:{}",
        client_jar.to_string_lossy(),
        libraries
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(":")
    ));

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
