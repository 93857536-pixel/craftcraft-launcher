//! End-to-end smoke test for the CraftCraft launch pipeline.
//!
//! Exercises the real network + filesystem path used by `launch_instance`:
//!   1. install a managed Temurin JDK
//!   2. fetch the Mojang version manifest + version JSON
//!   3. download the client jar, all libraries, unpack natives, fetch assets
//!   4. assemble the exact JVM/game arguments
//!   5. boot the JVM against the real classpath and assert the main class resolves
//!
//! Run with:  cargo run --bin smoke -- 1.21
//!
//! It does NOT open a game window: step 5 launches the JVM with a bogus account so
//! the client exits early, which is exactly what we want to verify classpath health.

use craftcraft_lib::models::instance::Instance;
use craftcraft_lib::services::java_manager::JavaManager;
use craftcraft_lib::services::mojang::MojangClient;
use craftcraft_lib::utils;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let version = std::env::args().nth(1).unwrap_or_else(|| "1.21".to_string());
    let data = utils::get_app_data_dir();
    println!("[smoke] data dir: {}", data.display());
    std::fs::create_dir_all(&data)?;

    // ---------- 1. version metadata (needed before Java resolution) ----------
    println!("[smoke] step 1/5: fetching version json for {version}");
    let client = MojangClient::new(&data);
    let vj = client.get_version_json(&version).await?;
    println!(
        "[smoke]   mainClass={:?} assetIndex={:?} libraries={} javaVersion={:?}",
        vj.main_class_or_fallback(),
        vj.asset_index_id(),
        vj.libraries.as_ref().map(|l| l.len()).unwrap_or(0),
        vj.required_java_major()
    );

    // ---------- 2. Java (Mojang's declared major version wins) ----------
    println!("[smoke] step 2/5: resolving Java");
    let jm = JavaManager::new();
    let required = vj.required_java_major();
    let rec = craftcraft_lib::models::java::JavaRecommendation::from_version_json(
        &version,
        required.as_deref(),
    );
    println!(
        "[smoke]   MC {version} needs Java {} (source: {})",
        rec.recommended_java,
        if required.is_some() { "version json" } else { "heuristics" }
    );
    // On Apple Silicon every Minecraft release up to 1.18.2 ships x86_64-only lwjgl
    // natives, so the JVM itself must be x86_64 (Rosetta) for them to load.
    let needs_x64_jvm = craftcraft_lib::models::java::needs_x64_jvm(&version);
    if needs_x64_jvm {
        println!("[smoke]   macOS arm64 + pre-1.19 natives → forcing x86_64 JVM (Rosetta 2)");
    }
    let javas = jm.list()?;
    // Pick a managed JDK of the right release *and* the right CPU architecture.
    let arch_ok = |p: &str| -> bool {
        if !needs_x64_jvm {
            return true;
        }
        java_binary_arch(std::path::Path::new(p)) == "x86_64"
    };
    // 优先找已装的匹配版本, 没有就尝试安装; 安装失败时降级到任意可用 JDK
    let java_bin = match javas
        .iter()
        .find(|j| {
            j.is_managed && j.matches_release(&rec.recommended_java) && arch_ok(&j.path)
        })
    {
        Some(j) => {
            println!(
                "[smoke]   using managed JDK {} ({})",
                j.version,
                java_binary_arch(std::path::Path::new(&j.path))
            );
            PathBuf::from(&j.path)
        }
        None => {
            println!(
                "[smoke]   installing Temurin {} (this takes a while)",
                rec.recommended_java
            );
            match jm.install_arch(&rec.recommended_java, needs_x64_jvm).await {
                Ok(j) => {
                    println!("[smoke]   installed Temurin {}", j.version);
                    PathBuf::from(&j.path)
                }
                Err(e) => {
                    eprintln!(
                        "[smoke]   WARN: could not install Temurin {}: {}\n         falling back to whatever JDK we have",
                        rec.recommended_java, e
                    );
                    match javas
                        .iter()
                        .filter(|j| {
                            j.is_managed
                                && arch_ok(&j.path)
                                && craftcraft_lib::models::java::satisfies_requirement(
                                    &j.version,
                                    &rec.recommended_java,
                                )
                        })
                        .min_by_key(|j| {
                            craftcraft_lib::models::java::major_of(&j.version).unwrap_or(u32::MAX)
                        })
                        .or_else(|| {
                            javas
                                .iter()
                                .filter(|j| {
                                    arch_ok(&j.path)
                                        && craftcraft_lib::models::java::satisfies_requirement(
                                            &j.version,
                                            &rec.recommended_java,
                                        )
                                })
                                .min_by_key(|j| {
                                    craftcraft_lib::models::java::major_of(&j.version)
                                        .unwrap_or(u32::MAX)
                                })
                        }) {
                        Some(j) => {
                            println!(
                                "[smoke]   using fallback JDK {} ({})",
                                j.version,
                                java_binary_arch(std::path::Path::new(&j.path))
                            );
                            PathBuf::from(&j.path)
                        }
                        None => {
                            return Err(anyhow::anyhow!(
                                "no usable Java {} runtime (need x64: {}) on this system",
                                rec.recommended_java,
                                needs_x64_jvm
                            ));
                        }
                    }
                }
            }
        }
    };
    let out = std::process::Command::new(&java_bin).arg("-version").output()?;
    println!(
        "[smoke]   java -version -> {}",
        String::from_utf8_lossy(&out.stderr).lines().next().unwrap_or("?")
    );


    // ---------- 3. files ----------
    println!("[smoke] step 3/5: downloading game files (client jar / libraries / natives / assets)");
    let jar = client.ensure_client_jar(&version).await?;
    println!("[smoke]   client jar {} bytes", std::fs::metadata(&jar)?.len());

    let game_dir = data.join("instances").join("smoke-test");
    std::fs::create_dir_all(&game_dir)?;
    let natives_dir = game_dir.join("natives");
    let libs = client.ensure_libraries(&vj, &natives_dir).await?;
    println!("[smoke]   {} classpath entries", libs.len());

    let native_count = std::fs::read_dir(&natives_dir)
        .map(|d| d.flatten().count())
        .unwrap_or(0);
    println!("[smoke]   {} native files unpacked", native_count);

    client.ensure_assets(&vj).await?;
    println!("[smoke]   assets ok");

    // ---------- 4. args ----------
    println!("[smoke] step 4/5: assembling launch arguments");
    let mut inst = Instance::new("smoke", version.clone(), game_dir.to_string_lossy().to_string());
    inst.java_path = Some(java_bin.to_string_lossy().to_string());
    inst.memory_max_mb = 2048;
    inst.memory_min_mb = 1024;
    let assets_dir = data.join("assets");
    let (jvm_args, game_args) = craftcraft_lib::services::launcher::build_launch_args_with_natives(
        &inst,
        &vj,
        &jar,
        &libs,
        &assets_dir,
        &natives_dir,
    )?;
    println!("[smoke]   jvm args ({} entries):", jvm_args.len());
    for a in &jvm_args {
        let shown = if a.len() > 120 { format!("{}...({} chars)", &a[..120], a.len()) } else { a.clone() };
        println!("[smoke]     | {shown}");
    }
    println!("[smoke]   game args: {}", game_args.join(" "));

    // ---------- 5. classpath health ----------
    // Boot the JVM with a short watchdog: if the client survives N seconds without a
    // class-loading error it means the classpath + natives + main class all resolved.
    println!("[smoke] step 5/5: booting JVM against the real classpath (watchdog 20s)");
    let sep = if cfg!(windows) { ";" } else { ":" };
    let mut cp = vec![jar.to_string_lossy().to_string()];
    cp.extend(libs.iter().map(|p| p.to_string_lossy().to_string()));
    let classpath = cp.join(sep);

    let mut child = std::process::Command::new(&java_bin)
        .args(&jvm_args)
        .arg("-cp")
        .arg(&classpath)
        .arg(format!("-Djava.library.path={}", natives_dir.display()))
        .arg(vj.main_class_or_fallback())
        .args(&game_args)
        .current_dir(&game_dir)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    let mut survived = false;
    for _ in 0..40 {
        std::thread::sleep(std::time::Duration::from_millis(500));
        if let Ok(Some(_)) = child.try_wait() {
            break;
        }
        survived = true;
        if std::time::Instant::now().elapsed().as_secs() > 0 {
            // keep looping until the 20s budget is spent or the process dies
        }
    }
    let exited = child.try_wait()?;
    if survived && exited.is_none() {
        println!("[smoke]   client ran for ~20s without crashing - killing it now");
        let _ = child.kill();
        let _ = child.wait();
        println!("[smoke] PASS - the real {version} client booted from our assembled command line.");
        return Ok(());
    }

    let mut err = String::new();
    if let Some(mut e) = child.stderr.take() {
        use std::io::Read;
        let _ = e.read_to_string(&mut err);
    }
    println!("[smoke] --- stderr ---");
    for line in err.lines().take(30) {
        println!("[smoke] | {line}");
    }
    let fatal = [
        "ClassNotFoundException",
        "NoClassDefFoundError",
        "Could not find or load main class",
        "UnsatisfiedLinkError",
        "ExceptionInInitializerError",
    ];
    let hit: Vec<&str> = fatal.iter().copied().filter(|f| err.contains(f)).collect();
    if !hit.is_empty() {
        // Auto-fallback: on Apple Silicon, lwjgl 2.x natives (MC ≤1.12) are x86_64 only.
        // If Java 8 is unavailable, retry via Rosetta 2 (arch -x86_64).
        if hit.contains(&"UnsatisfiedLinkError")
            && std::env::consts::OS == "macos"
            && std::env::consts::ARCH == "aarch64"
            && rec.recommended_java == "8"
        {
            eprintln!(
                "[smoke]   WARN: lwjgl 2.x natives are x86_64-only on Apple Silicon.\n\
                 [smoke]   Retrying with `arch -x86_64 {}` (Rosetta 2) as fallback...",
                java_bin.display()
            );
            let mut cmd = std::process::Command::new("arch");
            cmd.arg("-x86_64")
                .arg(&java_bin)
                .args(&jvm_args)
                .arg("-cp")
                .arg(&classpath)
                .arg(format!("-Djava.library.path={}", natives_dir.display()))
                .arg(vj.main_class_or_fallback())
                .args(&game_args)
                .current_dir(&game_dir)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped());
            let mut child = cmd.spawn()?;
            let mut survived = false;
            for _ in 0..40 {
                std::thread::sleep(std::time::Duration::from_millis(500));
                if let Ok(Some(_)) = child.try_wait() {
                    break;
                }
                survived = true;
            }
            let exited = child.try_wait()?;
            if survived && exited.is_none() {
                println!("[smoke]   client ran for ~20s without crashing - killing it now");
                let _ = child.kill();
                let _ = child.wait();
                println!("[smoke] PASS - the real {version} client booted via Rosetta 2 fallback.");
                return Ok(());
            }
            // Read stderr from rosetta attempt too
            let mut rosetta_err = String::new();
            if let Some(mut e) = child.stderr.take() {
                use std::io::Read;
                let _ = e.read_to_string(&mut rosetta_err);
            }
            eprintln!("[smoke]   Rosetta attempt stderr: {}", &rosetta_err[..rosetta_err.len().min(500)]);
        }
        anyhow::bail!("classpath unhealthy - found: {:?}", hit);
    }
    println!("[smoke] WARN - client exited early but no class-loading errors were found.");
    Ok(())
}

/// Thin wrapper so call sites stay short.
fn java_binary_arch(path: &std::path::Path) -> String {
    craftcraft_lib::models::java::binary_arch(&path.to_string_lossy())
}
