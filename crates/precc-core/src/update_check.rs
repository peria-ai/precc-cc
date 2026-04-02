//! Push update notifications for PRECC.
//!
//! Three-layer architecture:
//! - **Miner** (producer): periodically checks GitHub API, writes `.update_available` marker
//! - **Hook** (fast reader): reads marker file, prints stderr hint (< 0.5ms)
//! - **CLI** (display): shows banner on any `precc` command; clears marker after `precc update`
//!
//! Rate-limited to at most one GitHub API call per 24 hours.
//! Respects `PRECC_NO_TELEMETRY=1` to disable all network checks.

use anyhow::{Context, Result};
use std::path::Path;

/// Rate limit: minimum seconds between update checks.
const CHECK_INTERVAL_SECS: u64 = 86400; // 24 hours

/// Marker file name for the last check timestamp.
const LAST_CHECKED_FILE: &str = ".update_last_checked";

/// Marker file name for an available update.
const UPDATE_AVAILABLE_FILE: &str = ".update_available";

/// GitHub repository for release checks.
const REPO: &str = "yijunyu/precc-cc";

// ─── Rate limiting ──────────────────────────────────────────────────────────

/// Returns `true` if an update check should be performed now.
///
/// Returns `false` if:
/// - `PRECC_NO_TELEMETRY=1` is set (disables all network activity)
/// - The last check was less than 24 hours ago
pub fn should_check(data_dir: &Path) -> bool {
    if std::env::var("PRECC_NO_TELEMETRY").is_ok() {
        return false;
    }

    let marker = data_dir.join(LAST_CHECKED_FILE);
    if marker.exists() {
        if let Ok(meta) = std::fs::metadata(&marker) {
            if let Ok(modified) = meta.modified() {
                let elapsed = std::time::SystemTime::now()
                    .duration_since(modified)
                    .unwrap_or_default()
                    .as_secs();
                if elapsed < CHECK_INTERVAL_SECS {
                    return false;
                }
            }
        }
    }

    true
}

// ─── GitHub check ───────────────────────────────────────────────────────────

/// Check the latest release version from GitHub.
///
/// If a newer version is available, writes the `.update_available` marker and
/// returns `Some(version)`. If already up-to-date or on error, clears the
/// marker and returns `None`.
///
/// Always touches `.update_last_checked` to reset the rate limiter.
pub fn check_latest_version(data_dir: &Path) -> Result<Option<String>> {
    if !should_check(data_dir) {
        return Ok(None);
    }

    // Touch rate-limit marker first (even if the check fails, don't retry immediately)
    let last_checked = data_dir.join(LAST_CHECKED_FILE);
    let _ = std::fs::write(&last_checked, "");

    let api_url = format!("https://api.github.com/repos/{REPO}/releases/latest");

    let output = std::process::Command::new("curl")
        .args(["-fsSL", "--max-time", "5", &api_url])
        .stdin(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .output()
        .context("curl not available")?;

    if !output.status.success() {
        return Ok(None);
    }

    let release: serde_json::Value =
        serde_json::from_slice(&output.stdout).context("parsing GitHub API response")?;

    let tag_name = match release["tag_name"].as_str() {
        Some(t) => t,
        None => return Ok(None),
    };

    let latest = tag_name.trim_start_matches('v');
    let current = env!("CARGO_PKG_VERSION");

    if is_newer(latest, current) {
        // Write marker file
        let marker_path = data_dir.join(UPDATE_AVAILABLE_FILE);
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let marker_content = format!("{{\"version\":\"{}\",\"checked_at\":{}}}", latest, ts);
        let _ = std::fs::write(&marker_path, &marker_content);
        Ok(Some(latest.to_string()))
    } else {
        // Up-to-date — clear any stale marker
        clear_update_marker(data_dir);
        Ok(None)
    }
}

// ─── Marker file operations (fast, filesystem-only) ─────────────────────────

/// Read the update-available marker file. Returns `Some(version)` if a newer
/// version is available, `None` otherwise.
///
/// This is designed for the hot path (hook + CLI): single `stat()` + `read()`,
/// no network, no DB.
pub fn read_update_available(data_dir: &Path) -> Option<String> {
    let marker_path = data_dir.join(UPDATE_AVAILABLE_FILE);
    let content = std::fs::read_to_string(&marker_path).ok()?;
    let parsed: serde_json::Value = serde_json::from_str(&content).ok()?;
    parsed["version"].as_str().map(|s| s.to_string())
}

/// Remove the update-available marker (called after a successful `precc update`).
pub fn clear_update_marker(data_dir: &Path) {
    let marker_path = data_dir.join(UPDATE_AVAILABLE_FILE);
    let _ = std::fs::remove_file(&marker_path);
}

// ─── Auto-update ────────────────────────────────────────────────────────────

/// Returns `true` if auto-update is enabled via env var or config file.
pub fn auto_update_enabled() -> bool {
    // Environment variable takes precedence
    if std::env::var("PRECC_AUTO_UPDATE").is_ok_and(|v| v == "1" || v == "true") {
        return true;
    }

    // Check config file: ~/.config/precc/config.toml
    if let Ok(home) = std::env::var("HOME") {
        let config_path = std::path::Path::new(&home).join(".config/precc/config.toml");
        if let Ok(content) = std::fs::read_to_string(&config_path) {
            if let Ok(parsed) = content.parse::<toml::Table>() {
                if let Some(update) = parsed.get("update").and_then(|v| v.as_table()) {
                    if let Some(auto) = update.get("auto").and_then(|v| v.as_bool()) {
                        return auto;
                    }
                }
            }
        }
    }

    false
}

/// Returns `true` if the user has already been asked about auto-updates
/// (regardless of their answer).  Used to gate the consent prompt.
pub fn has_auto_update_consent() -> bool {
    if let Ok(home) = std::env::var("HOME") {
        let config_path = std::path::Path::new(&home).join(".config/precc/config.toml");
        if let Ok(content) = std::fs::read_to_string(&config_path) {
            if let Ok(parsed) = content.parse::<toml::Table>() {
                if let Some(update) = parsed.get("update").and_then(|v| v.as_table()) {
                    return update.contains_key("auto");
                }
            }
        }
    }
    false
}

/// Interactively ask the user about auto-updates (enabled by default).
///
/// Reads a single line from stdin.  Returns the user's choice and persists
/// it to `config.toml` so the question is only asked once.
pub fn prompt_auto_update_consent() -> Result<bool> {
    use std::io::{BufRead, Write};

    println!("Auto-update: PRECC will check once daily and upgrade silently.");
    print!("Disable automatic updates? [y/N] ");
    std::io::stdout().flush()?;

    let mut answer = String::new();
    std::io::stdin().lock().read_line(&mut answer)?;
    let disable = matches!(answer.trim().to_lowercase().as_str(), "y" | "yes");
    let enabled = !disable;

    set_auto_update(enabled)?;

    if enabled {
        println!(
            "  Auto-update enabled — PRECC will upgrade silently when new versions are released."
        );
    } else {
        println!("  Auto-update disabled — run `precc update` manually to upgrade.");
    }
    println!("  Change later: edit ~/.config/precc/config.toml → [update] auto = true/false");

    Ok(enabled)
}

/// Persist auto-update setting to `~/.config/precc/config.toml`.
pub fn set_auto_update(enabled: bool) -> Result<()> {
    let home = std::env::var("HOME").context("HOME not set")?;
    let config_dir = std::path::Path::new(&home).join(".config/precc");
    std::fs::create_dir_all(&config_dir)?;
    let config_path = config_dir.join("config.toml");

    // Load existing config or start fresh
    let mut table: toml::Table = if config_path.exists() {
        let content = std::fs::read_to_string(&config_path)?;
        content.parse().unwrap_or_default()
    } else {
        toml::Table::new()
    };

    // Set [update] auto = true/false
    let update_section = table
        .entry("update")
        .or_insert_with(|| toml::Value::Table(toml::Table::new()));
    if let Some(t) = update_section.as_table_mut() {
        t.insert("auto".to_string(), toml::Value::Boolean(enabled));
    }

    let toml_str = toml::to_string_pretty(&table).context("serializing config")?;
    std::fs::write(&config_path, toml_str).context("writing config.toml")?;
    Ok(())
}

/// Perform an auto-update: download and install the specified version.
///
/// Reuses the same download-and-replace logic as `precc update`.
/// Logs progress to `data_dir/auto_update.log`.
pub fn perform_auto_update(data_dir: &Path, version: &str) -> Result<()> {
    let log_path = data_dir.join("auto_update.log");

    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let mut log_msg = format!("[{ts}] auto-update to v{version} starting\n");

    // Detect target triple
    let target_triple = match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => "x86_64-unknown-linux-gnu",
        ("linux", "aarch64") => "aarch64-unknown-linux-gnu",
        ("macos", "x86_64") => "x86_64-apple-darwin",
        ("macos", "aarch64") => "aarch64-apple-darwin",
        _ => {
            log_msg.push_str("  unsupported platform\n");
            let _ = append_log(&log_path, &log_msg);
            anyhow::bail!("unsupported platform");
        }
    };

    // Fetch release metadata
    let tag = format!("v{version}");
    let api_url = format!("https://api.github.com/repos/{REPO}/releases/tags/{tag}");

    let api_out = std::process::Command::new("curl")
        .args(["-fsSL", "--max-time", "10", &api_url])
        .stdin(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .output()
        .context("curl not available")?;

    if !api_out.status.success() {
        log_msg.push_str("  GitHub API request failed\n");
        let _ = append_log(&log_path, &log_msg);
        anyhow::bail!("GitHub API request failed");
    }

    let release: serde_json::Value = serde_json::from_slice(&api_out.stdout)?;

    let assets = release["assets"]
        .as_array()
        .context("no assets in release")?;

    // Find matching asset
    let (asset_name, download_url) = assets
        .iter()
        .find_map(|a| {
            let name = a["name"].as_str()?;
            let url = a["browser_download_url"].as_str()?;
            if name.contains(target_triple) && name.ends_with(".tar.gz") {
                Some((name.to_string(), url.to_string()))
            } else {
                None
            }
        })
        .context("no matching asset found")?;

    // Download to temp dir
    let tmp_dir = tempfile::tempdir().context("creating temp dir")?;
    let archive_path = tmp_dir.path().join(&asset_name);

    let status = std::process::Command::new("curl")
        .args([
            "-fsSL",
            "--max-time",
            "60",
            "-o",
            archive_path.to_str().unwrap(),
            &download_url,
        ])
        .stdin(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .context("curl download failed")?;

    if !status.success() {
        log_msg.push_str("  download failed\n");
        let _ = append_log(&log_path, &log_msg);
        anyhow::bail!("download failed");
    }

    // Extract
    let status = std::process::Command::new("tar")
        .args([
            "-xzf",
            archive_path.to_str().unwrap(),
            "-C",
            tmp_dir.path().to_str().unwrap(),
        ])
        .stdin(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .context("tar extraction failed")?;

    if !status.success() {
        log_msg.push_str("  extraction failed\n");
        let _ = append_log(&log_path, &log_msg);
        anyhow::bail!("extraction failed");
    }

    // Find install directory from current exe
    let current_exe = std::env::current_exe().context("cannot determine current binary path")?;
    let bin_dir = current_exe
        .parent()
        .context("binary has no parent directory")?;

    let inner_dir = asset_name.trim_end_matches(".tar.gz");
    let extracted = tmp_dir.path().join(inner_dir);

    // Replace binaries
    for bin in ["precc", "precc-hook", "precc-miner"] {
        let src = extracted.join(bin);
        let dst = bin_dir.join(bin);
        if !src.exists() {
            continue;
        }
        let old = bin_dir.join(format!("{bin}.old"));
        if dst.exists() {
            std::fs::rename(&dst, &old).with_context(|| format!("cannot move {dst:?}"))?;
        }
        std::fs::copy(&src, &dst).with_context(|| format!("cannot write {dst:?}"))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&dst, std::fs::Permissions::from_mode(0o755))?;
        }
        if old.exists() {
            let _ = std::fs::remove_file(&old);
        }
        log_msg.push_str(&format!("  updated {}\n", dst.display()));
    }

    // Clear the update marker
    clear_update_marker(data_dir);

    log_msg.push_str(&format!("[{ts}] auto-update to v{version} complete\n"));
    let _ = append_log(&log_path, &log_msg);

    Ok(())
}

/// Append text to a log file (best-effort).
fn append_log(path: &Path, msg: &str) -> std::io::Result<()> {
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    f.write_all(msg.as_bytes())
}

// ─── Semver comparison ──────────────────────────────────────────────────────

/// Returns `true` if `latest` is strictly newer than `current` (semver comparison).
fn is_newer(latest: &str, current: &str) -> bool {
    let parse = |s: &str| -> (u32, u32, u32) {
        let parts: Vec<u32> = s.split('.').filter_map(|p| p.parse().ok()).collect();
        (
            parts.first().copied().unwrap_or(0),
            parts.get(1).copied().unwrap_or(0),
            parts.get(2).copied().unwrap_or(0),
        )
    };

    let l = parse(latest);
    let c = parse(current);
    l > c
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_check_respects_no_telemetry() {
        let dir = tempfile::tempdir().unwrap();
        std::env::set_var("PRECC_NO_TELEMETRY", "1");
        assert!(!should_check(dir.path()));
        std::env::remove_var("PRECC_NO_TELEMETRY");
    }

    #[test]
    fn should_check_true_when_no_marker() {
        let dir = tempfile::tempdir().unwrap();
        // Ensure PRECC_NO_TELEMETRY is not set for this test
        let had_env = std::env::var("PRECC_NO_TELEMETRY").ok();
        std::env::remove_var("PRECC_NO_TELEMETRY");
        assert!(should_check(dir.path()));
        if let Some(v) = had_env {
            std::env::set_var("PRECC_NO_TELEMETRY", v);
        }
    }

    #[test]
    fn should_check_false_when_recent() {
        let dir = tempfile::tempdir().unwrap();
        let had_env = std::env::var("PRECC_NO_TELEMETRY").ok();
        std::env::remove_var("PRECC_NO_TELEMETRY");
        // Touch the marker file (mtime = now)
        std::fs::write(dir.path().join(LAST_CHECKED_FILE), "").unwrap();
        assert!(!should_check(dir.path()));
        if let Some(v) = had_env {
            std::env::set_var("PRECC_NO_TELEMETRY", v);
        }
    }

    #[test]
    fn read_update_available_missing_file() {
        let dir = tempfile::tempdir().unwrap();
        assert!(read_update_available(dir.path()).is_none());
    }

    #[test]
    fn read_update_available_corrupt_file() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join(UPDATE_AVAILABLE_FILE), "not json").unwrap();
        assert!(read_update_available(dir.path()).is_none());
    }

    #[test]
    fn read_update_available_valid() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join(UPDATE_AVAILABLE_FILE),
            r#"{"version":"0.2.0","checked_at":1741795200}"#,
        )
        .unwrap();
        assert_eq!(read_update_available(dir.path()), Some("0.2.0".to_string()));
    }

    #[test]
    fn clear_update_marker_removes_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(UPDATE_AVAILABLE_FILE);
        std::fs::write(&path, "test").unwrap();
        assert!(path.exists());
        clear_update_marker(dir.path());
        assert!(!path.exists());
    }

    #[test]
    fn clear_update_marker_noop_missing() {
        let dir = tempfile::tempdir().unwrap();
        // Should not panic on missing file
        clear_update_marker(dir.path());
    }

    #[test]
    fn auto_update_disabled_by_default() {
        // With no env var and no config file, should be false
        let had_env = std::env::var("PRECC_AUTO_UPDATE").ok();
        std::env::remove_var("PRECC_AUTO_UPDATE");
        assert!(!auto_update_enabled());
        if let Some(v) = had_env {
            std::env::set_var("PRECC_AUTO_UPDATE", v);
        }
    }

    #[test]
    fn auto_update_from_env() {
        std::env::set_var("PRECC_AUTO_UPDATE", "1");
        assert!(auto_update_enabled());
        std::env::remove_var("PRECC_AUTO_UPDATE");
    }

    #[test]
    fn is_newer_basic() {
        assert!(is_newer("0.2.0", "0.1.9"));
        assert!(is_newer("1.0.0", "0.9.9"));
        assert!(is_newer("0.1.10", "0.1.9"));
    }

    #[test]
    fn is_newer_equal() {
        assert!(!is_newer("0.1.9", "0.1.9"));
    }

    #[test]
    fn is_newer_older() {
        assert!(!is_newer("0.1.8", "0.1.9"));
    }
}
