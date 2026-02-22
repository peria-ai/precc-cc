//! precc-miner: Background daemon for mining Claude Code session logs.
//!
//! Watches ~/.claude/projects/ for new JSONL session files,
//! mines failure-fix pairs into history.db, and promotes
//! recurring patterns into skills in heuristics.db.
//!
//! Modes:
//! - `precc-miner` — run daemon (default: poll every 60s)
//! - `precc-miner --once` — single pass: mine + promote, then exit
//! - `precc-miner --interval 30` — poll every 30 seconds
//! - `precc-miner --foreground` — don't write PID file, log to stderr

use anyhow::{Context, Result};
use precc_core::{db, mining, promote};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Default poll interval in seconds.
const DEFAULT_INTERVAL_SECS: u64 = 60;

/// Minimum occurrence count before promoting a pattern to a skill.
const PROMOTION_THRESHOLD: i64 = 3;

fn main() {
    let args = parse_args();

    if args.once {
        // Single pass mode
        if let Err(e) = run_once() {
            log(&format!("error: {e:#}"));
            std::process::exit(1);
        }
    } else {
        // Daemon mode
        if let Err(e) = run_daemon(args) {
            log(&format!("fatal: {e:#}"));
            std::process::exit(1);
        }
    }
}

/// CLI arguments (hand-parsed to avoid pulling in clap for this small binary).
struct Args {
    once: bool,
    foreground: bool,
    interval: u64,
}

fn parse_args() -> Args {
    let mut args = Args {
        once: false,
        foreground: false,
        interval: DEFAULT_INTERVAL_SECS,
    };

    let mut iter = std::env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--once" | "-1" => args.once = true,
            "--foreground" | "-f" => args.foreground = true,
            "--interval" | "-i" => {
                if let Some(val) = iter.next() {
                    args.interval = val.parse().unwrap_or(DEFAULT_INTERVAL_SECS);
                }
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => {
                eprintln!("precc-miner: unknown argument: {other}");
                eprintln!("Run `precc-miner --help` for usage.");
                std::process::exit(1);
            }
        }
    }

    args
}

fn print_help() {
    eprintln!("precc-miner — background daemon for mining Claude Code sessions");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("  precc-miner [OPTIONS]");
    eprintln!();
    eprintln!("OPTIONS:");
    eprintln!("  --once, -1         Single pass: mine + promote, then exit");
    eprintln!("  --foreground, -f   Run in foreground (no PID file)");
    eprintln!("  --interval, -i N   Poll interval in seconds (default: 60)");
    eprintln!("  --help, -h         Show this help");
}

// =============================================================================
// Single-pass mode
// =============================================================================

fn run_once() -> Result<()> {
    log("precc-miner: single pass starting");

    let data_dir = db::data_dir()?;
    let history_conn = db::open_history(&data_dir)?;
    let heuristics_conn = db::open_heuristics(&data_dir)?;
    let metrics_conn = db::open_metrics(&data_dir)?;

    // Import activation log (append-log bridge from hook)
    let imported = import_activation_log(&heuristics_conn, &data_dir)?;
    if imported > 0 {
        log(&format!("activations: {} imported from log", imported));
    }

    // Mine
    let mine_summary = mining::mine_all(&history_conn)?;
    log(&format!(
        "mining: {} processed, {} skipped, {} pairs",
        mine_summary.sessions_processed, mine_summary.sessions_skipped, mine_summary.pairs_found,
    ));

    // Extract PRECC events retroactively from session logs
    let precc_count =
        mining::extract_all_precc_events(&history_conn, &heuristics_conn, &metrics_conn)?;
    if precc_count > 0 {
        log(&format!(
            "precc-events: {} extracted from sessions",
            precc_count
        ));
    }

    // Promote
    let promo_summary =
        promote::promote_patterns(&history_conn, &heuristics_conn, Some(PROMOTION_THRESHOLD))?;
    log(&format!(
        "promote: {} candidates, {} created, {} existing, {} skipped",
        promo_summary.candidates_found,
        promo_summary.skills_created,
        promo_summary.already_exists,
        promo_summary.skipped,
    ));

    log("precc-miner: single pass complete");
    Ok(())
}

// =============================================================================
// Daemon mode
// =============================================================================

fn run_daemon(args: Args) -> Result<()> {
    let data_dir = db::data_dir()?;

    // PID file management
    let pid_path = if !args.foreground {
        let path = data_dir.join("miner.pid");
        check_existing_pid(&path)?;
        write_pid_file(&path)?;
        Some(path)
    } else {
        None
    };

    // Install signal handler for graceful shutdown
    let running = Arc::new(AtomicBool::new(true));
    {
        let running = running.clone();
        let pid_path = pid_path.clone();
        ctrlc_handler(move || {
            log("precc-miner: shutting down");
            running.store(false, Ordering::SeqCst);
            if let Some(ref p) = pid_path {
                let _ = std::fs::remove_file(p);
            }
        });
    }

    log(&format!(
        "precc-miner: daemon started (interval={}s, pid={})",
        args.interval,
        std::process::id()
    ));

    // Main loop
    let interval = std::time::Duration::from_secs(args.interval);

    while running.load(Ordering::SeqCst) {
        if let Err(e) = tick(&data_dir) {
            log(&format!("tick error: {e:#}"));
        }

        // Sleep in small increments so we can check the shutdown flag
        let sleep_end = std::time::Instant::now() + interval;
        while std::time::Instant::now() < sleep_end && running.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }

    // Cleanup
    if let Some(ref p) = pid_path {
        let _ = std::fs::remove_file(p);
    }

    log("precc-miner: stopped");
    Ok(())
}

/// One tick of the daemon: mine new sessions, then promote patterns.
fn tick(data_dir: &std::path::Path) -> Result<()> {
    let history_conn = db::open_history(data_dir)?;
    let heuristics_conn = db::open_heuristics(data_dir)?;
    let metrics_conn = db::open_metrics(data_dir)?;

    // Import activation log (append-log bridge from hook)
    let imported = import_activation_log(&heuristics_conn, data_dir)?;
    if imported > 0 {
        log(&format!("activations: {} imported from log", imported));
    }

    // Mine new sessions
    let mine_summary = mining::mine_all(&history_conn)?;

    if mine_summary.sessions_processed > 0 || mine_summary.pairs_found > 0 {
        log(&format!(
            "mined: {} sessions, {} pairs",
            mine_summary.sessions_processed, mine_summary.pairs_found,
        ));
    }

    // Extract PRECC events retroactively from session logs
    let precc_count =
        mining::extract_all_precc_events(&history_conn, &heuristics_conn, &metrics_conn)?;
    if precc_count > 0 {
        log(&format!(
            "precc-events: {} extracted from sessions",
            precc_count
        ));
    }

    // Promote patterns to skills
    let promo_summary =
        promote::promote_patterns(&history_conn, &heuristics_conn, Some(PROMOTION_THRESHOLD))?;

    if promo_summary.skills_created > 0 {
        log(&format!(
            "promoted: {} new skill(s)",
            promo_summary.skills_created,
        ));
    }

    // Record miner tick metric (best-effort)
    let _ = precc_core::metrics::record(
        &metrics_conn,
        precc_core::metrics::MetricType::MinerTick,
        mine_summary.pairs_found as f64,
        None,
    );

    Ok(())
}

// =============================================================================
// Activation log import
// =============================================================================

/// Import all pending skill activations from the append-log written by precc-hook.
///
/// Reads all JSONL lines from `activations.log`, calls `record_activation()` for each,
/// then atomically renames/removes the log to prevent double-counting.
///
/// Returns the number of activations imported.
fn import_activation_log(
    heuristics_conn: &rusqlite::Connection,
    data_dir: &std::path::Path,
) -> Result<usize> {
    let log_path = data_dir.join("activations.log");

    if !log_path.exists() {
        return Ok(0);
    }

    // Atomically rename the log so the hook can write a new one concurrently
    let processing_path = data_dir.join("activations.log.processing");
    if let Err(e) = std::fs::rename(&log_path, &processing_path) {
        // Another miner tick may have already renamed it — not an error
        log(&format!("activations: rename skipped: {e}"));
        return Ok(0);
    }

    let content = match std::fs::read_to_string(&processing_path) {
        Ok(c) => c,
        Err(_) => {
            let _ = std::fs::remove_file(&processing_path);
            return Ok(0);
        }
    };

    let mut count = 0;
    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let parsed: serde_json::Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let skill_id = match parsed.get("skill_id").and_then(|v| v.as_i64()) {
            Some(id) => id,
            None => continue,
        };
        let _ = precc_core::skills::record_activation(heuristics_conn, skill_id);
        count += 1;
    }

    let _ = std::fs::remove_file(&processing_path);
    Ok(count)
}

// =============================================================================
// PID file management
// =============================================================================

fn write_pid_file(path: &PathBuf) -> Result<()> {
    let pid = std::process::id();
    std::fs::write(path, pid.to_string())
        .with_context(|| format!("failed to write PID file: {}", path.display()))?;
    Ok(())
}

fn check_existing_pid(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    let content = std::fs::read_to_string(path).unwrap_or_default();
    let pid: u32 = match content.trim().parse() {
        Ok(p) => p,
        Err(_) => {
            // Stale/corrupt PID file — remove it
            let _ = std::fs::remove_file(path);
            return Ok(());
        }
    };

    // Check if the process is still running
    if process_alive(pid) {
        anyhow::bail!(
            "another precc-miner is already running (pid={pid}). \
             Remove {} if this is stale.",
            path.display()
        );
    }

    // Stale PID — remove the file
    let _ = std::fs::remove_file(path);
    Ok(())
}

/// Check if a process with the given PID is alive.
fn process_alive(pid: u32) -> bool {
    // On Linux, kill(pid, 0) checks if the process exists without sending a signal
    // We use /proc which is more portable across Rust targets
    std::path::Path::new(&format!("/proc/{pid}")).exists()
}

// =============================================================================
// Signal handling
// =============================================================================

/// Install a handler for SIGINT/SIGTERM.
fn ctrlc_handler<F: Fn() + Send + 'static>(handler: F) {
    // Use a simple approach: spawn a thread that blocks on a signal
    std::thread::spawn(move || {
        // Wait for SIGINT or SIGTERM
        // Since we can't use the `signal-hook` crate without adding it,
        // we use a simpler approach: set up a flag that the main loop checks
        wait_for_signal();
        handler();
    });
}

/// Block until SIGINT or SIGTERM is received.
fn wait_for_signal() {
    unsafe {
        // Register handlers that set a global flag
        libc::signal(libc::SIGINT, signal_flag as *const () as libc::sighandler_t);
        libc::signal(
            libc::SIGTERM,
            signal_flag as *const () as libc::sighandler_t,
        );
    }

    // Now block until the flag is set
    while !SIGNAL_RECEIVED.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

static SIGNAL_RECEIVED: AtomicBool = AtomicBool::new(false);

extern "C" fn signal_flag(_sig: libc::c_int) {
    SIGNAL_RECEIVED.store(true, Ordering::SeqCst);
}

// =============================================================================
// Logging
// =============================================================================

fn log(msg: &str) {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    eprintln!("[{ts}] {msg}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args_defaults() {
        let args = Args {
            once: false,
            foreground: false,
            interval: DEFAULT_INTERVAL_SECS,
        };
        assert!(!args.once);
        assert!(!args.foreground);
        assert_eq!(args.interval, 60);
    }

    #[test]
    fn process_alive_self() {
        assert!(process_alive(std::process::id()));
    }

    #[test]
    fn process_alive_nonexistent() {
        // PID 999999 is almost certainly not running
        assert!(!process_alive(999_999));
    }

    #[test]
    fn tick_creates_dbs() {
        let dir = tempfile::tempdir().unwrap();
        // tick should work even with empty databases
        tick(dir.path()).unwrap();
        assert!(dir.path().join("history.db").exists());
        assert!(dir.path().join("heuristics.db").exists());
    }

    #[test]
    fn pid_file_lifecycle() {
        let dir = tempfile::tempdir().unwrap();
        let pid_path = dir.path().join("test.pid");

        // No existing PID file — should succeed
        check_existing_pid(&pid_path).unwrap();

        // Write our PID
        write_pid_file(&pid_path).unwrap();
        let content = std::fs::read_to_string(&pid_path).unwrap();
        assert_eq!(content, std::process::id().to_string());

        // Stale PID (nonexistent process) — should clean up
        std::fs::write(&pid_path, "999999").unwrap();
        check_existing_pid(&pid_path).unwrap();
        assert!(!pid_path.exists());
    }

    #[test]
    fn pid_file_blocks_duplicate() {
        let dir = tempfile::tempdir().unwrap();
        let pid_path = dir.path().join("test.pid");

        // Write our own PID (a running process)
        std::fs::write(&pid_path, std::process::id().to_string()).unwrap();

        // Should fail because we're still alive
        let result = check_existing_pid(&pid_path);
        assert!(result.is_err());
    }
}
