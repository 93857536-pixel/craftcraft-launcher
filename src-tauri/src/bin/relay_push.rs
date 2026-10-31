/// Push craftcraft-launcher to GitHub via ECS relay.
/// Usage: cargo run --bin relay-push -- v0.1.0
fn main() -> anyhow::Result<()> {
    let tag = std::env::args().nth(1).expect("usage: relay-push <tag>");
    let remote = std::env::args().nth(2).unwrap_or_else(|| "devbox".to_string());

    let repo = std::env::current_dir()?;
    let bundle_path = repo.join(format!(".tmp-{}.bundle", tag));

    // 1. bundle (incremental: only what's reachable from the tag)
    eprintln!("[relay] bundle -> {}", bundle_path.display());
    let out = std::process::Command::new("git")
        .args([
            "bundle",
            "create",
            bundle_path.to_string_lossy().as_ref(),
            &format!("refs/tags/{}", tag),
        ])
        .output()?;
    if !out.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&out.stderr));
        return Err(anyhow::anyhow!("bundle failed"));
    }

    // 2. scp to server
    eprintln!("[relay] scp -> {remote}:/tmp/craftcraft-{tag}.bundle");
    let out = std::process::Command::new("scp")
        .args([
            "-q",
            bundle_path.to_string_lossy().as_ref(),
            &format!("{remote}:/tmp/craftcraft-{tag}.bundle"),
        ])
        .output()?;
    if !out.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&out.stderr));
        return Err(anyhow::anyhow!("scp failed"));
    }
    std::fs::remove_file(&bundle_path)?;

    // 3. server-side: scp the script, then execute via powershell -File
    // Windows Server has no bash; avoid PowerShell inline -Command quote hell.
    let script_src = std::path::PathBuf::from(
        "/Users/linminhao/.hermes/scripts/craftcraft-relay-push.ps1",
    );
    let script_dst = format!("/tmp/craftcraft-relay-{tag}.ps1");
    let out = std::process::Command::new("scp")
        .args([
            "-q",
            script_src.to_string_lossy().as_ref(),
            &format!("{remote}:{script_dst}"),
        ])
        .output()?;
    if !out.status.success() {
        eprintln!(
            "scp script failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        return Err(anyhow::anyhow!("scp script failed"));
    }

    // 3b. scp the script to server
    let script_src = std::path::PathBuf::from(
        "/Users/linminhao/.hermes/scripts/craftcraft-relay-push.ps1",
    );
    let script_dst = format!("/tmp/craftcraft-relay-{tag}.ps1");
    let out = std::process::Command::new("scp")
        .args([
            "-q",
            script_src.to_string_lossy().as_ref(),
            &format!("{remote}:{script_dst}"),
        ])
        .output()?;
    if !out.status.success() {
        eprintln!("scp script failed: {}", String::from_utf8_lossy(&out.stderr));
        return Err(anyhow::anyhow!("scp script failed"));
    }

    let out = std::process::Command::new("ssh")
        .args([
            remote.as_str(),
            "powershell.exe",
            "-NoProfile",
            "-File",
            script_dst.as_str(),
            "--",
            tag.as_str(),
        ])
        .output()?;
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    eprintln!("[relay] server stderr:\n{stderr}");
    eprintln!("[relay] server stdout:\n{stdout}");
    if !out.status.success() {
        return Err(anyhow::anyhow!("push failed on server: {stderr}"));
    }

    // 4. verify tag exists on GitHub
    eprintln!("[relay] verify tag...");
    let out = std::process::Command::new("gh")
        .args([
            "api",
            "-X",
            "GET",
            &format!("repos/93857536-pixel/craftcraft-launcher/git/ref/tags/{tag}"),
        ])
        .output()?;
    let body = String::from_utf8_lossy(&out.stdout);
    if out.status.success() {
        eprintln!("[relay] tag ok:\n{body}");
    } else {
        eprintln!("[relay] err: {body}");
        return Err(anyhow::anyhow!("tag not found on GitHub"));
    }

    Ok(())
}
