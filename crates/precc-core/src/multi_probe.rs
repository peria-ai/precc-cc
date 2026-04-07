//! Multi-mode probe: re-runs an original command through all available
//! compression modes (other than the one that already fired) and records
//! per-mode token measurements.
//!
//! Gated behind `PRECC_MULTI_MODE_PROBE=1`. Runs in a background thread to
//! avoid blocking PostToolUse latency. Concurrency is capped to avoid
//! overwhelming the user's machine.

use crate::mode::CompressionMode;
use crate::post_observe;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

const MAX_CONCURRENT_PROBES: usize = 2;
const PROBE_TIMEOUT_SECS: u64 = 30;
const PROBE_MAX_OUTPUT_BYTES: usize = 256 * 1024;

static IN_FLIGHT: AtomicUsize = AtomicUsize::new(0);

/// Spawn a background probe (if PRECC_MULTI_MODE_PROBE=1) that runs the
/// original command through every compression mode except `live_mode` and
/// records per-mode measurements.
pub fn maybe_probe_all_modes(
    data_dir: &Path,
    cmd_class: &str,
    original_cmd: &str,
    cwd: &str,
    session_id: &str,
    live_mode: CompressionMode,
) {
    if std::env::var("PRECC_MULTI_MODE_PROBE").as_deref() != Ok("1") {
        return;
    }
    if !post_observe::is_safe_to_rerun(original_cmd) {
        return;
    }
    if IN_FLIGHT.load(Ordering::Relaxed) >= MAX_CONCURRENT_PROBES {
        return;
    }

    // Capture owned values for the thread
    let data_dir = data_dir.to_path_buf();
    let cmd_class = cmd_class.to_string();
    let original_cmd = original_cmd.to_string();
    let cwd = cwd.to_string();
    let session_id = session_id.to_string();

    IN_FLIGHT.fetch_add(1, Ordering::Relaxed);
    std::thread::spawn(move || {
        let _guard = ProbeGuard;
        run_all_modes(&data_dir, &cmd_class, &original_cmd, &cwd, &session_id, live_mode);
    });
}

struct ProbeGuard;
impl Drop for ProbeGuard {
    fn drop(&mut self) {
        IN_FLIGHT.fetch_sub(1, Ordering::Relaxed);
    }
}

fn run_all_modes(
    data_dir: &PathBuf,
    cmd_class: &str,
    original_cmd: &str,
    cwd: &str,
    session_id: &str,
    live_mode: CompressionMode,
) {
    use CompressionMode::*;

    // First measure the basic (uncompressed) baseline if we don't already have it.
    let basic_tokens = if live_mode == Basic {
        return; // Already recorded as live
    } else {
        run_and_count(original_cmd, cwd)
    };
    if basic_tokens == 0 {
        return; // Couldn't measure basic — skip
    }

    // Always record the basic measurement (it's the denominator)
    if live_mode != Basic {
        post_observe::append_savings_measurement(
            data_dir,
            cmd_class,
            "probe-basic",
            "basic",
            "probe",
            session_id,
            basic_tokens,
            basic_tokens,
            "multi_mode_probe",
            0.0,
        );
    }

    // For each non-live mode, apply its rewrite and measure
    for mode in [Diet, Nushell, LeanCtx, Rtk] {
        if mode == live_mode {
            continue;
        }
        let rewritten = match mode {
            Diet => crate::diet::apply(original_cmd).map(|(cmd, _)| cmd),
            Nushell => {
                if !crate::nushell::nu_available() {
                    None
                } else {
                    crate::nushell::wrap(original_cmd)
                }
            }
            LeanCtx => {
                if !crate::lean_ctx::lean_ctx_available() {
                    None
                } else {
                    crate::lean_ctx::wrap(original_cmd)
                }
            }
            Rtk => {
                if !crate::rtk::rtk_available() {
                    None
                } else {
                    crate::rtk::rewrite(original_cmd)
                }
            }
            Basic | AdaptiveExpand => continue,
        };

        let Some(rewritten_cmd) = rewritten else {
            continue;
        };

        let actual_tokens = run_and_count(&rewritten_cmd, cwd);
        if actual_tokens == 0 {
            continue;
        }

        post_observe::append_savings_measurement(
            data_dir,
            cmd_class,
            &format!("probe-{}", mode.as_str()),
            mode.as_str(),
            "probe",
            session_id,
            basic_tokens,
            actual_tokens,
            "multi_mode_probe",
            0.0,
        );
    }
}

/// Run a shell command with a timeout and output cap, return token count
/// (bytes / 4) of stdout+stderr. Returns 0 on failure.
fn run_and_count(cmd: &str, cwd: &str) -> u64 {
    use std::process::{Command, Stdio};
    use std::time::{Duration, Instant};

    let mut child = match Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .current_dir(if cwd.is_empty() { "." } else { cwd })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return 0,
    };

    let deadline = Instant::now() + Duration::from_secs(PROBE_TIMEOUT_SECS);
    loop {
        match child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    return 0;
                }
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(_) => return 0,
        }
    }

    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(_) => return 0,
    };

    let total_bytes = output.stdout.len() + output.stderr.len();
    let capped = total_bytes.min(PROBE_MAX_OUTPUT_BYTES);
    (capped / 4) as u64
}
