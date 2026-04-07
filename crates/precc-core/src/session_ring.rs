//! Per-session sliding window of recent commands.
//!
//! Used to detect "follow-up diagnostic" patterns: when Claude runs a command,
//! gets compressed output, and then immediately runs the same command in a
//! more verbose mode (`--verbose`, `2>&1`, `--debug`, etc.). This indicates
//! the compression hid important information.

use std::path::{Path, PathBuf};

const RING_CAPACITY: usize = 5;

#[derive(Debug, Clone)]
pub struct SessionEntry {
    pub ts: u64,
    pub cmd_class: String,
    pub cmd: String,
    pub mode_used: String,
}

fn ring_path(data_dir: &Path, session_id: &str) -> PathBuf {
    // Sanitize session_id (no path traversal)
    let safe: String = session_id
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .take(64)
        .collect();
    data_dir.join("sessions").join(format!("{}.jsonl", safe))
}

/// Append a command to the session ring (rotates after RING_CAPACITY).
pub fn push_command(data_dir: &Path, session_id: &str, entry: SessionEntry) {
    if session_id.is_empty() {
        return;
    }
    let path = ring_path(data_dir, session_id);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    // Read existing entries, append new one, keep last N
    let mut entries = read_all(&path);
    entries.push(entry);
    if entries.len() > RING_CAPACITY {
        let drop = entries.len() - RING_CAPACITY;
        entries.drain(0..drop);
    }

    // Write back
    let lines: Vec<String> = entries.iter().map(serialize).collect();
    let _ = std::fs::write(&path, lines.join("\n") + "\n");
}

/// Get the previous (most recent) command in the session ring.
pub fn previous(data_dir: &Path, session_id: &str) -> Option<SessionEntry> {
    let path = ring_path(data_dir, session_id);
    let entries = read_all(&path);
    entries.into_iter().last()
}

fn read_all(path: &Path) -> Vec<SessionEntry> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    content
        .lines()
        .filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }
            let v: serde_json::Value = serde_json::from_str(line).ok()?;
            Some(SessionEntry {
                ts: v.get("ts").and_then(|n| n.as_u64()).unwrap_or(0),
                cmd_class: v.get("cmd_class").and_then(|s| s.as_str())?.to_string(),
                cmd: v.get("cmd").and_then(|s| s.as_str())?.to_string(),
                mode_used: v
                    .get("mode_used")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string(),
            })
        })
        .collect()
}

fn serialize(e: &SessionEntry) -> String {
    format!(
        "{{\"ts\":{},\"cmd_class\":{},\"cmd\":{},\"mode_used\":{}}}",
        e.ts,
        serde_json::to_string(&e.cmd_class).unwrap_or_default(),
        serde_json::to_string(&e.cmd).unwrap_or_default(),
        serde_json::to_string(&e.mode_used).unwrap_or_default(),
    )
}

/// Check if `current_cmd` looks like a verbose follow-up to `previous`.
/// Returns true when the current command is essentially "the same thing
/// but show me more output" — a strong signal that compression hid info.
pub fn is_follow_up_diagnostic(previous: &SessionEntry, current_class: &str, current_cmd: &str) -> bool {
    // Same command class (or first token matches)
    let prev_first = previous.cmd_class.split_whitespace().next().unwrap_or("");
    let curr_first = current_class.split_whitespace().next().unwrap_or("");
    if prev_first.is_empty() || prev_first != curr_first {
        return false;
    }

    // Diagnostic markers in current command
    let markers = [
        " --verbose",
        " -v ",
        " -vv",
        " -vvv",
        " --debug",
        " --trace",
        "RUST_LOG=",
        "DEBUG=",
        " 2>&1",
        " --no-quiet",
        " strace ",
        " ltrace ",
        " | tail",
        " | less",
        " | cat",
    ];
    let cmd_padded = format!(" {} ", current_cmd);
    for m in markers {
        if cmd_padded.contains(m) {
            return true;
        }
    }

    // Or the command is exactly identical (re-run = also diagnostic)
    if previous.cmd == current_cmd {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(cmd: &str, mode: &str) -> SessionEntry {
        SessionEntry {
            ts: 1000,
            cmd_class: cmd.split_whitespace().take(2).collect::<Vec<_>>().join(" "),
            cmd: cmd.to_string(),
            mode_used: mode.to_string(),
        }
    }

    #[test]
    fn ring_capacity() {
        let tmp = tempfile::tempdir().unwrap();
        let sid = "test-session";
        for i in 0..10 {
            push_command(tmp.path(), sid, entry(&format!("cmd {}", i), "rtk"));
        }
        let prev = previous(tmp.path(), sid).unwrap();
        assert_eq!(prev.cmd, "cmd 9");

        let entries = read_all(&ring_path(tmp.path(), sid));
        assert_eq!(entries.len(), RING_CAPACITY);
        assert_eq!(entries[0].cmd, "cmd 5"); // 10 pushed, last 5 kept
    }

    #[test]
    fn detects_verbose_follow_up() {
        let prev = entry("cargo build", "rtk");
        assert!(is_follow_up_diagnostic(&prev, "cargo build", "cargo build --verbose"));
        assert!(is_follow_up_diagnostic(&prev, "cargo build", "RUST_LOG=debug cargo build"));
        assert!(is_follow_up_diagnostic(&prev, "cargo build", "cargo build 2>&1 | tail -50"));
    }

    #[test]
    fn detects_identical_rerun() {
        let prev = entry("git status", "rtk");
        assert!(is_follow_up_diagnostic(&prev, "git status", "git status"));
    }

    #[test]
    fn rejects_unrelated_command() {
        let prev = entry("cargo build", "rtk");
        assert!(!is_follow_up_diagnostic(&prev, "git status", "git status --verbose"));
        assert!(!is_follow_up_diagnostic(&prev, "cargo test", "cargo test"));
    }

    #[test]
    fn empty_session_id_no_op() {
        let tmp = tempfile::tempdir().unwrap();
        push_command(tmp.path(), "", entry("cmd", "rtk"));
        assert!(previous(tmp.path(), "").is_none());
    }
}
