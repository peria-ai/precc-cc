//! PostToolUse observability: track tool output sizes, detect waste patterns
//! (repeated commands, oversized outputs), and log metrics for reporting.

use serde_json::Value;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::Path;

/// Threshold in estimated tokens above which an output is flagged as "large".
const LARGE_OUTPUT_THRESHOLD: u64 = 10_000;

/// Maximum number of recent observations to scan for duplicates.
const DEDUP_SCAN_BYTES: u64 = 4096;

/// Estimate tokens from a tool response value.
/// Uses the standard ~4 bytes per token approximation.
pub fn estimate_tokens(tool_response: &Value) -> u64 {
    let serialized = serde_json::to_string(tool_response).unwrap_or_default();
    serialized.len() as u64 / 4
}

/// Estimate tokens from raw byte count (for when we already know the size).
pub fn tokens_from_bytes(bytes: u64) -> u64 {
    bytes / 4
}

/// Check if an output exceeds the large-output threshold.
pub fn is_large_output(estimated_tokens: u64) -> bool {
    estimated_tokens > LARGE_OUTPUT_THRESHOLD
}

/// Compute a deterministic hash of (tool_name, tool_input) for dedup detection.
pub fn hash_command(tool_name: &str, tool_input: &Value) -> u64 {
    let mut hasher = DefaultHasher::new();
    tool_name.hash(&mut hasher);
    // Use compact JSON serialization for deterministic hashing
    let input_str = serde_json::to_string(tool_input).unwrap_or_default();
    input_str.hash(&mut hasher);
    hasher.finish()
}

/// Check if this command was recently observed (duplicate detection).
/// Returns the number of times this command has been seen if > 1, else None.
pub fn check_duplicate(data_dir: &Path, session_id: &str, cmd_hash: u64) -> Option<u32> {
    let log_path = data_dir.join("post_observations.log");

    let content = match std::fs::read_to_string(&log_path) {
        Ok(c) => c,
        Err(_) => return None,
    };

    // Only scan the tail of the file for performance
    let scan_start = if content.len() as u64 > DEDUP_SCAN_BYTES {
        content.len() - DEDUP_SCAN_BYTES as usize
    } else {
        0
    };
    let tail = &content[scan_start..];

    let hash_str = cmd_hash.to_string();
    let mut count = 0u32;
    for line in tail.lines() {
        // Fast check: line must contain both session_id and hash
        if line.contains(session_id) && line.contains(&hash_str) {
            count += 1;
        }
    }

    if count >= 2 {
        Some(count) // command appeared 2+ times already — it's a repeat
    } else {
        None
    }
}

/// Append an observation record to the post_observations log.
/// `compressed` indicates whether the command's output was compressed (RTK/lean-ctx/diet).
/// `cmd_class` is the command class (e.g. "cargo test") for per-class compression tracking.
pub fn append_observation(
    data_dir: &Path,
    session_id: &str,
    tool_name: &str,
    cmd_hash: u64,
    output_bytes: u64,
    estimated_tokens: u64,
    compressed: bool,
    cmd_class: &str,
) {
    let log_path = data_dir.join("post_observations.log");
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let line = format!(
        "{{\"ts\":{},\"sid\":\"{}\",\"tool\":\"{}\",\"hash\":{},\"bytes\":{},\"tokens\":{},\"compressed\":{},\"class\":\"{}\"}}\n",
        ts, session_id, tool_name, cmd_hash, output_bytes, estimated_tokens, compressed, cmd_class
    );

    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .and_then(|mut f| f.write_all(line.as_bytes()));
}

// ─── Measurement stash (PreToolUse → PostToolUse) ──────────────────────────

/// Stash entry: original command info saved by PreToolUse for PostToolUse to read.
#[derive(Debug)]
pub struct StashEntry {
    pub original_cmd: String,
    pub rewritten_cmd: String,
    pub cwd: String,
    pub cmd_class: String,
    pub rewrite_types: Vec<String>,
}

/// Write a measurement stash file for a rewritten command.
/// Called by PreToolUse when the pipeline modifies a command.
/// The stash is keyed by the hash of the rewritten command so PostToolUse can find it.
pub fn write_stash(
    data_dir: &Path,
    rewritten_hash: u64,
    original_cmd: &str,
    rewritten_cmd: &str,
    cwd: &str,
    cmd_class: &str,
    rewrite_types: &[String],
) {
    let stash_dir = data_dir.join("stash");
    let _ = std::fs::create_dir_all(&stash_dir);
    let path = stash_dir.join(format!("{:016x}.json", rewritten_hash));

    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let types_json: Vec<String> = rewrite_types.iter().map(|t| format!("\"{}\"", t)).collect();
    let json = format!(
        "{{\"ts\":{},\"original_cmd\":{},\"rewritten_cmd\":{},\"cwd\":{},\"cmd_class\":{},\"rewrite_types\":[{}]}}",
        ts,
        serde_json::to_string(original_cmd).unwrap_or_default(),
        serde_json::to_string(rewritten_cmd).unwrap_or_default(),
        serde_json::to_string(cwd).unwrap_or_default(),
        serde_json::to_string(cmd_class).unwrap_or_default(),
        types_json.join(",")
    );

    let _ = std::fs::write(&path, json);
}

/// Read a measurement stash file by rewritten command hash.
/// Called by PostToolUse to find the original command.
pub fn read_stash(data_dir: &Path, rewritten_hash: u64) -> Option<StashEntry> {
    let path = data_dir
        .join("stash")
        .join(format!("{:016x}.json", rewritten_hash));
    let content = std::fs::read_to_string(&path).ok()?;
    let v: Value = serde_json::from_str(&content).ok()?;

    // Check staleness (> 60 seconds = stale)
    let ts = v.get("ts").and_then(|t| t.as_u64()).unwrap_or(0);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    if now.saturating_sub(ts) > 60 {
        let _ = std::fs::remove_file(&path);
        return None;
    }

    Some(StashEntry {
        original_cmd: v
            .get("original_cmd")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string(),
        rewritten_cmd: v
            .get("rewritten_cmd")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string(),
        cwd: v
            .get("cwd")
            .and_then(|s| s.as_str())
            .unwrap_or(".")
            .to_string(),
        cmd_class: v
            .get("cmd_class")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string(),
        rewrite_types: v
            .get("rewrite_types")
            .and_then(|a| a.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default(),
    })
}

/// Delete a measurement stash file after use.
pub fn delete_stash(data_dir: &Path, rewritten_hash: u64) {
    let path = data_dir
        .join("stash")
        .join(format!("{:016x}.json", rewritten_hash));
    let _ = std::fs::remove_file(&path);
}

/// Clean up stale stash files (older than 60 seconds).
pub fn cleanup_stale_stashes(data_dir: &Path) {
    let stash_dir = data_dir.join("stash");
    let entries = match std::fs::read_dir(&stash_dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    let now = std::time::SystemTime::now();
    for entry in entries.flatten() {
        if let Ok(meta) = entry.metadata() {
            if let Ok(modified) = meta.modified() {
                if now.duration_since(modified).unwrap_or_default().as_secs() > 60 {
                    let _ = std::fs::remove_file(entry.path());
                }
            }
        }
    }
}

// ─── Read-only safety classifier ───────────────────────────────────────────

/// Check if a command is safe to re-run for measurement purposes.
/// Only allows known read-only commands that produce no side effects.
pub fn is_safe_to_rerun(command: &str) -> bool {
    let cmd = command.trim();
    if cmd.is_empty() {
        return false;
    }

    // Strip output redirection — for measurement we run without writing.
    // (`>`, `>>`, `| tee` would alter the filesystem, which we can avoid by
    // dropping the redirect during the re-run measurement.)
    let cmd_norm = strip_output_redirect(cmd);

    // Handle && and ; chains: every clause must be individually safe.
    // `cd /path` clauses are allowed (they're stateful but produce no output
    // and don't modify anything destructively).
    let has_and = cmd_norm.contains(" && ");
    let has_semi = cmd_norm.contains("; ");
    if has_and || has_semi {
        // Split on whichever separator(s) exist
        let parts: Vec<&str> = if has_and && !has_semi {
            cmd_norm.split(" && ").collect()
        } else if !has_and && has_semi {
            cmd_norm.split("; ").collect()
        } else {
            // Mixed: split on both
            cmd_norm
                .split(" && ")
                .flat_map(|s| s.split("; "))
                .collect()
        };
        return parts.iter().map(|c| c.trim()).all(|clause| {
            if clause.starts_with("cd ") {
                return true;
            }
            is_clause_safe(clause)
        });
    }

    // Reject other command separators
    if cmd_norm.contains(" || ") || cmd_norm.contains(" | tee ") {
        return false;
    }

    is_clause_safe(&cmd_norm)
}

/// Returns a version of the command that is safe to re-run for measurement —
/// strips output redirects so we don't clobber files. Returns None if the
/// command is not measurable at all.
pub fn rerunnable_form(command: &str) -> Option<String> {
    if !is_safe_to_rerun(command) {
        return None;
    }
    Some(strip_output_redirect(command))
}

/// Strip the trailing redirect (`> file`, `>> file`) from a command for
/// measurement purposes. Returns the command without the redirect.
fn strip_output_redirect(cmd: &str) -> String {
    // Walk backwards: find the last `> file` or `>> file` not inside quotes.
    // This is a simplified parser that handles the common cases.
    if let Some(idx) = cmd.rfind(" >> ") {
        return cmd[..idx].trim().to_string();
    }
    if let Some(idx) = cmd.rfind(" > ") {
        // Don't strip 2>&1 redirects (those go to stdout, not files)
        if !cmd[idx..].starts_with(" > &") {
            return cmd[..idx].trim().to_string();
        }
    }
    cmd.to_string()
}

/// Check if a single clause (no `&&` chains) is safe to re-run.
fn is_clause_safe(clause: &str) -> bool {
    let cmd = clause.trim();

    // Handle cd /path && remainder (the && was already stripped at chain level
    // but a single clause might still start with cd)
    let effective = if cmd.starts_with("cd ") {
        // Strip cd /path; the remainder must be safe on its own
        // But if there's no &&, this clause is just `cd /path` which is stateful
        return false;
    } else {
        cmd
    };

    // Strip env assignments and noise prefixes (FOO=bar, sudo, nohup, time)
    let mut words: Vec<&str> = effective.split_whitespace().collect();
    while !words.is_empty() {
        let w = words[0];
        let is_env = w
            .find('=')
            .map(|i| {
                let var = &w[..i];
                !var.is_empty()
                    && var
                        .chars()
                        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
            })
            .unwrap_or(false);
        let is_noise = matches!(w, "sudo" | "time" | "exec");
        // nohup is only noise for the safety check; it doesn't make a cmd unsafe
        let is_nohup = w == "nohup";
        if is_env || is_noise || is_nohup {
            words.remove(0);
        } else {
            break;
        }
    }

    // Strip compression wrappers
    if !words.is_empty() && words[0] == "rtk" {
        words.remove(0);
    }
    if words.len() >= 2 && words[0] == "lean-ctx" && words[1] == "-c" {
        // lean-ctx -c '<inner>' — the inner is the actual command
        // Skip the wrapper and check the inner cmd
        words.drain(0..2);
        // The inner is wrapped in quotes; remove them
        if let Some(inner) = words.first().map(|w| w.trim_matches(|c| c == '\'' || c == '"')) {
            return is_clause_safe(inner);
        }
    }

    if words.is_empty() {
        return false;
    }

    let first = words[0];

    // Single-word safe commands
    let safe_single = [
        // Core file/text utilities
        "ls", "cat", "head", "tail", "find", "grep", "rg", "awk", "wc", "du", "df", "file", "stat",
        "tree", "which", "type", "echo", "printf", "date", "uname", "env", "printenv", "whoami",
        "pwd", "hostname", "id", "uptime", "free", "users", "groups", "history", "alias",
        // Networking read-only
        "ping", "host", "dig", "nslookup", "traceroute", "ip", "ifconfig", "ss", "netstat", "arp",
        // Disk read-only
        "lsblk", "blkid", "lsof", "fuser",
        // System info
        "lscpu", "lspci", "lsusb", "dmidecode",
        // Process inspection
        "ps", "pstree", "pgrep",
        // Misc safe
        "cmp", "comm", "diff", "md5sum", "sha256sum", "shasum", "basename", "dirname", "realpath",
        "readlink", "mkfifo", "mktemp",
        // Archive read-only listings (NO extract — those are below)
        "zcat", "bzcat", "xzcat",
    ];
    if safe_single.contains(&first) {
        return true;
    }

    // Two-word safe commands (first + second)
    if words.len() >= 2 {
        let second = words[1];
        let safe_pairs: &[(&str, &[&str])] = &[
            (
                "git",
                &[
                    "status",
                    "diff",
                    "log",
                    "show",
                    "branch",
                    "describe",
                    "tag",
                    "remote",
                    "rev-parse",
                    "ls-files",
                    "ls-tree",
                    "blame",
                    "config",
                    "shortlog",
                    "reflog",
                    "cat-file",
                    "merge-base",
                    "rev-list",
                ],
            ),
            (
                "cargo",
                &[
                    "check",
                    "clippy",
                    "test",
                    "metadata",
                    "tree",
                    "build", // Idempotent (Cargo caches outputs)
                    "doc",
                    "bench",
                    "search",
                    "outdated",
                    "audit",
                    "version",
                    "--version",
                ],
            ),
            (
                "npm",
                &[
                    "list", "ls", "outdated", "view", "info", "audit", "search", "config", "doctor",
                    "ping", "version",
                ],
            ),
            (
                "pip",
                &["list", "show", "freeze", "search", "check", "config", "--version"],
            ),
            ("python", &["--version", "-V", "-c", "-m"]),
            ("python3", &["--version", "-V", "-c", "-m"]),
            ("node", &["--version", "-v", "-e", "--help"]),
            ("rustc", &["--version", "--print", "-V", "--explain"]),
            (
                "go",
                &[
                    "version", "list", "env", "doc", "vet", "test", "fmt", "tool", "mod",
                ],
            ),
            ("docker", &["ps", "images", "logs", "inspect", "version", "info", "stats", "history"]),
            ("kubectl", &["get", "describe", "logs", "version", "config", "explain", "api-resources"]),
            ("brew", &["list", "info", "search", "outdated", "doctor", "config"]),
            ("apt", &["list", "show", "search", "policy"]),
            ("apt-cache", &["search", "show", "depends", "rdepends"]),
            ("gem", &["list", "info", "search", "which", "environment"]),
            ("composer", &["show", "info", "outdated", "depends"]),
            ("mvn", &["dependency:tree", "help:effective-pom", "--version"]),
            ("gradle", &["dependencies", "tasks", "projects", "--version"]),
            ("gh", &["api", "auth", "browse", "config", "issue", "pr", "release", "repo", "run", "search", "status", "version"]),
            ("aws", &["help", "configure", "sts"]),
            ("systemctl", &["status", "list-units", "list-unit-files", "list-timers", "show", "cat", "is-active", "is-enabled", "is-failed"]),
            ("journalctl", &["--list-boots", "--disk-usage", "--version"]),
            ("clawhub", &["inspect", "list", "search", "show", "version"]),
            ("stripe", &["config", "version", "--help"]),
            ("precc", &["savings", "skills", "report", "license", "telemetry", "analyze", "--version"]),
        ];
        for (cmd_prefix, safe_subcommands) in safe_pairs {
            if first == *cmd_prefix && safe_subcommands.contains(&second) {
                return true;
            }
        }

        // Special cases requiring flag inspection:

        // sed: -n is read-only; -i is in-place edit (destructive)
        if first == "sed" {
            if words.iter().any(|w| w.starts_with("-i") || *w == "--in-place") {
                return false;
            }
            return true;
        }

        // cargo fmt: only safe with --check (default mode rewrites files)
        if first == "cargo" && second == "fmt" {
            return words.iter().any(|w| *w == "--check" || *w == "-q");
        }

        // sqlite3: only safe for SELECT-only queries
        if first == "sqlite3" {
            // If a query is specified, check it's SELECT/PRAGMA only
            for w in &words[1..] {
                let upper = w.to_uppercase();
                if upper.contains("INSERT")
                    || upper.contains("UPDATE")
                    || upper.contains("DELETE")
                    || upper.contains("DROP")
                    || upper.contains("CREATE")
                    || upper.contains("ALTER")
                    || upper.contains("REPLACE")
                {
                    return false;
                }
            }
            // No write keywords found — assume safe (could be opening DB in read mode)
            return true;
        }

        // curl/wget: GET requests are safe; POST/PUT/DELETE are not
        if first == "curl" || first == "wget" {
            for w in &words[1..] {
                if matches!(*w, "-X" | "--request" | "-d" | "--data" | "-F" | "--form" | "-T" | "--upload-file") {
                    return false;
                }
            }
            return true;
        }

        // ssh host '<inner_cmd>' — safe if the inner command is safe.
        // Common form: ssh [opts] host "cmd1; cmd2" or ssh host 'cmd'
        if first == "ssh" {
            // Find the first non-option, non-host argument — that's the inner cmd.
            // Heuristic: scan for the first quoted string (single or double).
            // We need the FULL command text, not just words[], so use the
            // original `clause` string.
            if let Some(inner) = extract_ssh_inner_command(clause) {
                return is_safe_to_rerun(inner.trim());
            }
            // No inner command (interactive ssh) — not measurable
            return false;
        }
    }

    false
}

/// Extract the inner command from `ssh [opts] host '<cmd>'` or `ssh ... "<cmd>"`.
/// Returns None if no quoted command is present.
fn extract_ssh_inner_command(cmd: &str) -> Option<String> {
    // Find the first single or double quote
    let bytes = cmd.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\'' || bytes[i] == b'"' {
            let quote = bytes[i];
            let start = i + 1;
            let mut j = start;
            while j < bytes.len() && bytes[j] != quote {
                j += 1;
            }
            if j < bytes.len() {
                return Some(cmd[start..j].to_string());
            }
            return None;
        }
        i += 1;
    }
    None
}

// ─── Savings measurement logging ───────────────────────────────────────────

/// Append a savings measurement to the log file.
///
/// `compression_mode` is the explicit mode (basic/diet/nushell/lean-ctx/rtk/adaptive-expand).
/// `probe_kind` is "live" (the actual mode that ran) or "probe" (multi-mode probe).
/// `session_id` is the Claude Code session for cross-tool correlation.
pub fn append_savings_measurement(
    data_dir: &Path,
    cmd_class: &str,
    rewrite_type: &str,
    compression_mode: &str,
    probe_kind: &str,
    session_id: &str,
    original_output_tokens: u64,
    actual_output_tokens: u64,
    measurement_method: &str,
    measurement_latency_ms: f64,
) {
    let savings_tokens = original_output_tokens.saturating_sub(actual_output_tokens);
    let savings_pct = if original_output_tokens > 0 {
        savings_tokens as f64 / original_output_tokens as f64 * 100.0
    } else {
        0.0
    };

    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let log_path = data_dir.join("savings_measurements.jsonl");
    let line = format!(
        "{{\"ts\":{},\"cmd_class\":\"{}\",\"rewrite_type\":\"{}\",\"compression_mode\":\"{}\",\"probe_kind\":\"{}\",\"session_id\":\"{}\",\"original_output_tokens\":{},\"actual_output_tokens\":{},\"savings_tokens\":{},\"savings_pct\":{:.1},\"measurement_method\":\"{}\",\"measurement_latency_ms\":{:.1}}}\n",
        ts, cmd_class, rewrite_type, compression_mode, probe_kind, session_id,
        original_output_tokens, actual_output_tokens,
        savings_tokens, savings_pct, measurement_method, measurement_latency_ms
    );

    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .and_then(|mut f| f.write_all(line.as_bytes()));
}

/// Compute actual compression savings from observation logs.
///
/// Compares average output tokens for compressed vs uncompressed commands
/// of the same class. Returns (compressed_total_tokens, uncompressed_total_tokens)
/// where the difference is the measured savings.
///
/// Only works once there's data for both compressed and uncompressed runs
/// of the same command class (adaptive-expand provides uncompressed baselines).
pub fn measured_compression_savings(data_dir: &Path) -> (u64, u64) {
    let log_path = data_dir.join("post_observations.log");
    let content = match std::fs::read_to_string(&log_path) {
        Ok(c) => c,
        Err(_) => return (0, 0),
    };

    // Collect per-class averages for compressed vs uncompressed
    use std::collections::HashMap;
    let mut compressed_by_class: HashMap<String, Vec<u64>> = HashMap::new();
    let mut uncompressed_by_class: HashMap<String, Vec<u64>> = HashMap::new();

    for line in content.lines() {
        let parsed: serde_json::Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        // Only Bash commands
        if parsed.get("tool").and_then(|v| v.as_str()) != Some("Bash") {
            continue;
        }
        let class = match parsed.get("class").and_then(|v| v.as_str()) {
            Some(c) if !c.is_empty() => c.to_string(),
            _ => continue,
        };
        let tokens = parsed.get("tokens").and_then(|v| v.as_u64()).unwrap_or(0);
        let is_compressed = parsed
            .get("compressed")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if is_compressed {
            compressed_by_class.entry(class).or_default().push(tokens);
        } else {
            uncompressed_by_class.entry(class).or_default().push(tokens);
        }
    }

    // For classes that have both compressed and uncompressed data,
    // compute the savings: (avg_uncompressed - avg_compressed) × compressed_count
    let mut total_compressed_tokens: u64 = 0;
    let mut estimated_uncompressed_tokens: u64 = 0;

    for (class, comp_vals) in &compressed_by_class {
        let comp_total: u64 = comp_vals.iter().sum();
        total_compressed_tokens += comp_total;

        if let Some(uncomp_vals) = uncompressed_by_class.get(class) {
            // We have real baselines for this class
            let avg_uncomp = uncomp_vals.iter().sum::<u64>() as f64 / uncomp_vals.len() as f64;
            estimated_uncompressed_tokens += (avg_uncomp as u64) * comp_vals.len() as u64;
        } else {
            // No baseline yet — assume 1:1 (no savings claimed)
            estimated_uncompressed_tokens += comp_total;
        }
    }

    (total_compressed_tokens, estimated_uncompressed_tokens)
}

/// Context window usage percentage above which we flag pressure.
const CONTEXT_PRESSURE_THRESHOLD: u64 = 85;

/// Extract context_window.used_percentage from the hook input JSON.
pub fn context_used_pct(hook_input: &Value) -> Option<u64> {
    hook_input
        .get("context_window")
        .and_then(|cw| cw.get("used_percentage"))
        .and_then(|v| v.as_u64())
}

/// Returns true when context usage exceeds the pressure threshold.
pub fn is_context_pressure(used_pct: u64) -> bool {
    used_pct >= CONTEXT_PRESSURE_THRESHOLD
}

/// Result of analyzing a PostToolUse event.
pub struct WasteReport {
    /// Number of times this command has been seen (Some(n) if duplicate, n >= 2)
    pub duplicate_count: Option<u32>,
    /// Whether the output exceeds the large-output threshold
    pub is_large: bool,
    /// Estimated token count of the tool response
    pub estimated_tokens: u64,
    /// Raw byte count of the tool response
    pub output_bytes: u64,
    /// Context window usage percentage (if available)
    pub context_used_pct: Option<u64>,
}

impl WasteReport {
    /// Whether any waste was detected.
    pub fn has_waste(&self) -> bool {
        self.duplicate_count.is_some()
            || self.is_large
            || self.context_used_pct.map_or(false, is_context_pressure)
    }

    /// Generate advisory context for Claude when waste is detected.
    pub fn advisory_context(&self, tool_name: &str) -> Option<String> {
        if !self.has_waste() {
            return None;
        }

        let mut parts = Vec::new();

        if let Some(count) = self.duplicate_count {
            parts.push(format!(
                "duplicate {} command detected ({} times this session) — consider reusing earlier results",
                tool_name, count
            ));
        }

        if self.is_large {
            parts.push(format!(
                "large output (~{} tokens) — consider using filters or limits to reduce output size",
                self.estimated_tokens
            ));
        }

        if let Some(pct) = self.context_used_pct {
            if is_context_pressure(pct) {
                parts.push(format!(
                    "context window {}% full — consider running /compact or starting a new session",
                    pct
                ));
            }
        }

        Some(format!("[precc] {}", parts.join("; ")))
    }
}

// ─── Compression feedback loop ─────────────────────────────────────────────

/// TTL for compression failure records: skip compression for this many seconds
/// after a command class fails post-compression.
const COMPRESSION_COOLDOWN_SECS: u64 = 300; // 5 minutes

/// Extract the command class (first 1-2 significant words) from a bash command.
/// Used for fuzzy matching: if `cargo test` fails after compression,
/// skip compression for all `cargo test` variants.
///
/// Strips:
/// - `cd /path && ` prefix
/// - Compression wrappers (rtk, lean-ctx)
/// - Leading env assignments (FOO=bar BAZ=qux cmd → cmd)
/// - `sudo`, `nohup`, `time` prefixes
pub fn command_class(command: &str) -> String {
    let cmd = command.trim();
    // Strip cd prefix
    let effective = if let Some(pos) = cmd.find(" && ") {
        if cmd.starts_with("cd ") {
            cmd[pos + 4..].trim()
        } else {
            cmd
        }
    } else {
        cmd
    };

    // Strip compression wrappers (rtk, lean-ctx)
    let unwrapped = effective
        .strip_prefix("rtk ")
        .or_else(|| effective.strip_prefix("lean-ctx -c '"))
        .unwrap_or(effective);

    // Strip leading env assignments and noise prefixes (sudo, nohup, time)
    let mut words: Vec<&str> = unwrapped.split_whitespace().collect();
    while !words.is_empty() {
        let w = words[0];
        // Env assignment: VAR=value (unquoted, no spaces)
        let is_env = w
            .find('=')
            .map(|i| {
                let var = &w[..i];
                !var.is_empty()
                    && var
                        .chars()
                        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
            })
            .unwrap_or(false);
        let is_noise = matches!(w, "sudo" | "nohup" | "time" | "exec");
        if is_env || is_noise {
            words.remove(0);
        } else {
            break;
        }
    }

    match words.len() {
        0 => String::new(),
        1 => words[0].to_string(),
        _ => format!("{} {}", words[0], words[1]),
    }
}

/// Record that a command class failed after compression.
/// Written by PostToolUse when a Bash command has non-zero exit code.
pub fn record_compression_failure(data_dir: &Path, cmd_class: &str) {
    let log_path = data_dir.join("compression_failures.log");
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let line = format!("{} {}\n", ts, cmd_class);

    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .and_then(|mut f| f.write_all(line.as_bytes()));
}

/// Check if a command class recently failed after compression.
/// Returns true if compression should be skipped (cooldown active).
pub fn should_skip_compression(data_dir: &Path, cmd_class: &str) -> bool {
    let log_path = data_dir.join("compression_failures.log");
    let content = match std::fs::read_to_string(&log_path) {
        Ok(c) => c,
        Err(_) => return false,
    };

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Scan from the end (most recent entries first)
    for line in content.lines().rev() {
        let mut parts = line.splitn(2, ' ');
        let ts: u64 = match parts.next().and_then(|s| s.parse().ok()) {
            Some(t) => t,
            None => continue,
        };

        // Stop scanning once we're past the cooldown window
        if now.saturating_sub(ts) > COMPRESSION_COOLDOWN_SECS {
            break;
        }

        let class = parts.next().unwrap_or("");
        if class == cmd_class {
            return true;
        }
    }

    false
}

/// Detect if a Bash tool response indicates a failure (non-zero exit code).
/// Claude Code's tool_response for Bash contains the exit code in the response.
pub fn is_bash_failure(hook_input: &Value) -> bool {
    // Check tool_response.exitCode or tool_response for error patterns
    if let Some(resp) = hook_input.get("tool_response") {
        // Direct exit code field
        if let Some(code) = resp.get("exitCode").and_then(|v| v.as_i64()) {
            return code != 0;
        }
        // String response containing "Exit code" (Claude Code format)
        if let Some(s) = resp.as_str() {
            if s.contains("Exit code") && !s.contains("Exit code 0") {
                return true;
            }
        }
        // Check for error field in response object
        if let Some(err) = resp.get("error") {
            if err.is_string() || (err.is_boolean() && err.as_bool() == Some(true)) {
                return true;
            }
        }
    }

    // Check top-level is_error field
    if let Some(is_err) = hook_input.get("tool_result_is_error") {
        if is_err.as_bool() == Some(true) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // =========================================================================
    // estimate_tokens
    // =========================================================================

    #[test]
    fn estimate_tokens_empty_object() {
        let response = json!({});
        // "{}" is 2 bytes → 0 tokens
        assert_eq!(estimate_tokens(&response), 0);
    }

    #[test]
    fn estimate_tokens_small_response() {
        let response = json!({"output": "hello world", "success": true});
        let tokens = estimate_tokens(&response);
        assert!(tokens > 0);
        assert!(tokens < 100);
    }

    #[test]
    fn estimate_tokens_large_response() {
        let big_output = "x".repeat(40_000); // 40KB
        let response = json!({"output": big_output});
        let tokens = estimate_tokens(&response);
        assert!(tokens > 9_000); // ~40KB / 4 = ~10K tokens
    }

    #[test]
    fn estimate_tokens_null() {
        let response = json!(null);
        assert_eq!(estimate_tokens(&response), 1); // "null" is 4 bytes
    }

    #[test]
    fn tokens_from_bytes_conversion() {
        assert_eq!(tokens_from_bytes(0), 0);
        assert_eq!(tokens_from_bytes(4), 1);
        assert_eq!(tokens_from_bytes(400), 100);
        assert_eq!(tokens_from_bytes(3), 0); // integer division
    }

    // =========================================================================
    // is_large_output
    // =========================================================================

    #[test]
    fn large_output_threshold() {
        assert!(!is_large_output(0));
        assert!(!is_large_output(5_000));
        assert!(!is_large_output(10_000));
        assert!(is_large_output(10_001));
        assert!(is_large_output(50_000));
    }

    // =========================================================================
    // hash_command
    // =========================================================================

    #[test]
    fn hash_same_command_is_deterministic() {
        let input = json!({"command": "cargo test"});
        let h1 = hash_command("Bash", &input);
        let h2 = hash_command("Bash", &input);
        assert_eq!(h1, h2);
    }

    #[test]
    fn hash_different_commands_differ() {
        let input1 = json!({"command": "cargo test"});
        let input2 = json!({"command": "cargo build"});
        assert_ne!(hash_command("Bash", &input1), hash_command("Bash", &input2));
    }

    #[test]
    fn hash_different_tools_differ() {
        let input = json!({"command": "test"});
        assert_ne!(hash_command("Bash", &input), hash_command("Read", &input));
    }

    #[test]
    fn hash_empty_input() {
        let input = json!({});
        let h = hash_command("Bash", &input);
        assert!(h > 0); // should produce a valid hash
    }

    // =========================================================================
    // check_duplicate + append_observation
    // =========================================================================

    #[test]
    fn no_duplicate_on_first_call() {
        let tmp = tempfile::tempdir().unwrap();
        let result = check_duplicate(tmp.path(), "sess1", 12345);
        assert!(result.is_none());
    }

    #[test]
    fn duplicate_detected_after_append() {
        let tmp = tempfile::tempdir().unwrap();

        // First observation — not a duplicate yet
        append_observation(tmp.path(), "sess1", "Bash", 12345, 100, 25, false, "");
        assert!(check_duplicate(tmp.path(), "sess1", 12345).is_none());

        // Second observation — still only 1 in log, not yet duplicate
        append_observation(tmp.path(), "sess1", "Bash", 12345, 100, 25, false, "");
        // Now there are 2 entries → check_duplicate should detect it
        let result = check_duplicate(tmp.path(), "sess1", 12345);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 2);

        // Third observation
        append_observation(tmp.path(), "sess1", "Bash", 12345, 100, 25, false, "");
        let result = check_duplicate(tmp.path(), "sess1", 12345);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn no_duplicate_different_session() {
        let tmp = tempfile::tempdir().unwrap();
        append_observation(tmp.path(), "sess1", "Bash", 12345, 100, 25, false, "");
        let result = check_duplicate(tmp.path(), "sess2", 12345);
        assert!(result.is_none());
    }

    #[test]
    fn no_duplicate_different_hash() {
        let tmp = tempfile::tempdir().unwrap();
        append_observation(tmp.path(), "sess1", "Bash", 12345, 100, 25, false, "");
        let result = check_duplicate(tmp.path(), "sess1", 99999);
        assert!(result.is_none());
    }

    #[test]
    fn observation_log_format() {
        let tmp = tempfile::tempdir().unwrap();
        append_observation(tmp.path(), "sess1", "Bash", 12345, 500, 125, false, "");

        let content = std::fs::read_to_string(tmp.path().join("post_observations.log")).unwrap();
        let parsed: Value = serde_json::from_str(content.trim()).unwrap();
        assert_eq!(parsed["sid"].as_str().unwrap(), "sess1");
        assert_eq!(parsed["tool"].as_str().unwrap(), "Bash");
        assert_eq!(parsed["hash"].as_u64().unwrap(), 12345);
        assert_eq!(parsed["bytes"].as_u64().unwrap(), 500);
        assert_eq!(parsed["tokens"].as_u64().unwrap(), 125);
        assert!(parsed["ts"].as_u64().is_some());
    }

    // =========================================================================
    // WasteReport
    // =========================================================================

    #[test]
    fn waste_report_no_waste() {
        let report = WasteReport {
            duplicate_count: None,
            is_large: false,
            estimated_tokens: 100,
            output_bytes: 400,
            context_used_pct: None,
        };
        assert!(!report.has_waste());
        assert!(report.advisory_context("Bash").is_none());
    }

    #[test]
    fn waste_report_duplicate() {
        let report = WasteReport {
            duplicate_count: Some(3),
            is_large: false,
            estimated_tokens: 100,
            output_bytes: 400,
            context_used_pct: None,
        };
        assert!(report.has_waste());
        let ctx = report.advisory_context("Bash").unwrap();
        assert!(ctx.contains("duplicate"));
        assert!(ctx.contains("3 times"));
    }

    #[test]
    fn waste_report_large() {
        let report = WasteReport {
            duplicate_count: None,
            is_large: true,
            estimated_tokens: 15_000,
            output_bytes: 60_000,
            context_used_pct: None,
        };
        assert!(report.has_waste());
        let ctx = report.advisory_context("Bash").unwrap();
        assert!(ctx.contains("large output"));
        assert!(ctx.contains("15000"));
    }

    #[test]
    fn waste_report_both() {
        let report = WasteReport {
            duplicate_count: Some(2),
            is_large: true,
            estimated_tokens: 20_000,
            output_bytes: 80_000,
            context_used_pct: None,
        };
        assert!(report.has_waste());
        let ctx = report.advisory_context("Bash").unwrap();
        assert!(ctx.contains("duplicate"));
        assert!(ctx.contains("large output"));
    }

    #[test]
    fn waste_report_advisory_includes_tool_name() {
        let report = WasteReport {
            duplicate_count: Some(2),
            is_large: false,
            estimated_tokens: 100,
            output_bytes: 400,
            context_used_pct: None,
        };
        let ctx = report.advisory_context("Read").unwrap();
        assert!(ctx.contains("Read"));
    }

    // =========================================================================
    // Context pressure
    // =========================================================================

    #[test]
    fn context_pressure_below_threshold() {
        assert!(!is_context_pressure(50));
        assert!(!is_context_pressure(84));
    }

    #[test]
    fn context_pressure_at_threshold() {
        assert!(is_context_pressure(85));
        assert!(is_context_pressure(100));
    }

    #[test]
    fn context_used_pct_from_json() {
        let input = json!({"context_window": {"used_percentage": 92}});
        assert_eq!(context_used_pct(&input), Some(92));
    }

    #[test]
    fn context_used_pct_missing() {
        let input = json!({"tool_name": "Bash"});
        assert_eq!(context_used_pct(&input), None);
    }

    #[test]
    fn waste_report_context_pressure() {
        let report = WasteReport {
            duplicate_count: None,
            is_large: false,
            estimated_tokens: 100,
            output_bytes: 400,
            context_used_pct: Some(90),
        };
        assert!(report.has_waste());
        let ctx = report.advisory_context("Bash").unwrap();
        assert!(ctx.contains("90%"));
        assert!(ctx.contains("/compact"));
    }

    #[test]
    fn waste_report_no_pressure_at_low_context() {
        let report = WasteReport {
            duplicate_count: None,
            is_large: false,
            estimated_tokens: 100,
            output_bytes: 400,
            context_used_pct: Some(50),
        };
        assert!(!report.has_waste());
    }

    // =========================================================================
    // Compression feedback loop
    // =========================================================================

    #[test]
    fn command_class_simple() {
        assert_eq!(command_class("cargo test"), "cargo test");
        assert_eq!(command_class("git status"), "git status");
        assert_eq!(command_class("ls"), "ls");
    }

    #[test]
    fn command_class_strips_cd_prefix() {
        assert_eq!(
            command_class("cd /app && cargo test --release"),
            "cargo test"
        );
    }

    #[test]
    fn command_class_strips_rtk_wrapper() {
        assert_eq!(command_class("rtk cargo test"), "cargo test");
    }

    #[test]
    fn command_class_strips_lean_ctx_wrapper() {
        assert_eq!(command_class("lean-ctx -c 'cargo test'"), "cargo test'");
    }

    #[test]
    fn command_class_strips_env_assignments() {
        assert_eq!(command_class("RUST_BACKTRACE=1 cargo test"), "cargo test");
        assert_eq!(
            command_class("RUST_LOG=debug RUST_BACKTRACE=full cargo run"),
            "cargo run"
        );
        assert_eq!(command_class("DEBUG=1 npm test"), "npm test");
    }

    #[test]
    fn command_class_strips_sudo_nohup_time() {
        assert_eq!(command_class("sudo cp /tmp/a /tmp/b"), "cp /tmp/a");
        assert_eq!(command_class("nohup ./server &"), "./server &");
        assert_eq!(command_class("time cargo build"), "cargo build");
    }

    #[test]
    fn command_class_does_not_strip_lowercase_assignments() {
        // Lowercase var=val isn't a shell env assignment (they must be uppercase
        // by convention; this avoids false positives like `name=value cmd`)
        assert_eq!(command_class("foo=bar baz"), "foo=bar baz");
    }

    #[test]
    fn compression_failure_roundtrip() {
        let tmp = tempfile::tempdir().unwrap();

        // No failures initially
        assert!(!should_skip_compression(tmp.path(), "cargo test"));

        // Record a failure
        record_compression_failure(tmp.path(), "cargo test");

        // Now it should skip
        assert!(should_skip_compression(tmp.path(), "cargo test"));

        // Different class should not be affected
        assert!(!should_skip_compression(tmp.path(), "cargo build"));
    }

    #[test]
    fn is_bash_failure_exit_code() {
        let input = json!({"tool_response": {"exitCode": 1}});
        assert!(is_bash_failure(&input));

        let input = json!({"tool_response": {"exitCode": 0}});
        assert!(!is_bash_failure(&input));
    }

    #[test]
    fn is_bash_failure_string_response() {
        let input = json!({"tool_response": "Exit code 1\nerror: could not compile"});
        assert!(is_bash_failure(&input));

        let input = json!({"tool_response": "Exit code 0\nSuccess"});
        assert!(!is_bash_failure(&input));
    }

    #[test]
    fn is_bash_failure_tool_result_is_error() {
        let input = json!({"tool_result_is_error": true});
        assert!(is_bash_failure(&input));

        let input = json!({"tool_result_is_error": false});
        assert!(!is_bash_failure(&input));
    }

    #[test]
    fn is_bash_failure_no_error() {
        let input = json!({"tool_response": "all good"});
        assert!(!is_bash_failure(&input));
    }

    // =========================================================================
    // Safety classifier (is_safe_to_rerun)
    // =========================================================================

    #[test]
    fn is_safe_to_rerun_read_only_commands() {
        assert!(is_safe_to_rerun("ls -la"));
        assert!(is_safe_to_rerun("cat file.txt"));
        assert!(is_safe_to_rerun("head -20 file.rs"));
        assert!(is_safe_to_rerun("grep -r pattern ."));
        assert!(is_safe_to_rerun("wc -l src/*.rs"));
        assert!(is_safe_to_rerun("find . -name '*.rs'"));
        assert!(is_safe_to_rerun("git status"));
        assert!(is_safe_to_rerun("git diff HEAD~1"));
        assert!(is_safe_to_rerun("git log --oneline -5"));
        assert!(is_safe_to_rerun("cargo test --release"));
        assert!(is_safe_to_rerun("cargo clippy --all-targets"));
    }

    #[test]
    fn is_safe_to_rerun_with_cd_prefix() {
        assert!(is_safe_to_rerun("cd /app && git status"));
        assert!(is_safe_to_rerun("cd /app && cargo test"));
        assert!(is_safe_to_rerun("cd /app && ls -la"));
    }

    #[test]
    fn is_safe_to_rerun_strips_rtk_wrapper() {
        assert!(is_safe_to_rerun("rtk git status"));
        assert!(is_safe_to_rerun("rtk cargo test"));
    }

    #[test]
    fn is_safe_to_rerun_rejects_destructive() {
        assert!(!is_safe_to_rerun("rm -rf /tmp/stuff"));
        assert!(!is_safe_to_rerun("mv file.txt /tmp/"));
        assert!(!is_safe_to_rerun("mkdir -p /tmp/new"));
        assert!(!is_safe_to_rerun("docker run ubuntu"));
    }

    #[test]
    fn is_safe_to_rerun_strips_redirects() {
        // The base command (`ls`, `echo`) is safe — redirects are stripped
        // for the measurement re-run via rerunnable_form().
        assert!(is_safe_to_rerun("ls > output.txt"));
        assert!(is_safe_to_rerun("echo hi >> log.txt"));
    }

    #[test]
    fn rerunnable_form_strips_redirects() {
        assert_eq!(
            rerunnable_form("ls -la > /tmp/out.txt"),
            Some("ls -la".to_string())
        );
        assert_eq!(
            rerunnable_form("git log >> history.log"),
            Some("git log".to_string())
        );
    }

    #[test]
    fn rerunnable_form_unsafe_returns_none() {
        assert_eq!(rerunnable_form("rm -rf /tmp"), None);
    }

    #[test]
    fn is_safe_to_rerun_rejects_unknown() {
        assert!(!is_safe_to_rerun("some-random-binary"));
        // curl GET is now allowed
        assert!(is_safe_to_rerun("curl https://example.com"));
        // curl POST is rejected
        assert!(!is_safe_to_rerun("curl -X POST https://example.com"));
        assert!(!is_safe_to_rerun("curl -d 'data' https://example.com"));
    }

    #[test]
    fn is_safe_to_rerun_handles_chains() {
        // All clauses safe → safe
        assert!(is_safe_to_rerun("ls -la && git status && wc -l Cargo.toml"));
        // Any unsafe clause → unsafe
        assert!(!is_safe_to_rerun("ls -la && rm -rf /tmp/foo"));
        assert!(!is_safe_to_rerun("git status && git push"));
    }

    #[test]
    fn is_safe_to_rerun_strips_env_prefix() {
        assert!(is_safe_to_rerun("RUST_BACKTRACE=1 cargo test"));
        assert!(is_safe_to_rerun("DEBUG=1 LOG=verbose ls -la"));
        // Env prefix doesn't make a destructive cmd safe
        assert!(!is_safe_to_rerun("FOO=bar rm -rf /tmp"));
    }

    #[test]
    fn is_safe_to_rerun_sed_modes() {
        assert!(is_safe_to_rerun("sed -n '1,10p' file.txt"));
        assert!(is_safe_to_rerun("sed 's/foo/bar/' file.txt")); // outputs to stdout
        assert!(!is_safe_to_rerun("sed -i 's/foo/bar/' file.txt"));
        assert!(!is_safe_to_rerun("sed --in-place 's/foo/bar/' file.txt"));
    }

    #[test]
    fn is_safe_to_rerun_cargo_fmt_check_only() {
        assert!(is_safe_to_rerun("cargo fmt --check"));
        assert!(!is_safe_to_rerun("cargo fmt")); // would rewrite files
    }

    #[test]
    fn is_safe_to_rerun_sqlite3_select_only() {
        assert!(is_safe_to_rerun("sqlite3 db.sqlite 'SELECT * FROM users'"));
        assert!(!is_safe_to_rerun(
            "sqlite3 db.sqlite 'INSERT INTO users VALUES (1)'"
        ));
        assert!(!is_safe_to_rerun("sqlite3 db.sqlite 'DROP TABLE users'"));
    }

    #[test]
    fn is_safe_to_rerun_cargo_build_allowed() {
        assert!(is_safe_to_rerun("cargo build"));
        assert!(is_safe_to_rerun("cargo build --release"));
    }

    // =========================================================================
    // Stash mechanism
    // =========================================================================

    #[test]
    fn stash_write_read_delete() {
        let tmp = tempfile::tempdir().unwrap();
        let hash = 0xDEADBEEF_u64;

        // Write
        write_stash(
            tmp.path(), hash,
            "git status", "rtk git status",
            "/home/user/project", "git status",
            &["rtk-rewrite".to_string()],
        );

        // Read
        let entry = read_stash(tmp.path(), hash);
        assert!(entry.is_some());
        let entry = entry.unwrap();
        assert_eq!(entry.original_cmd, "git status");
        assert_eq!(entry.rewritten_cmd, "rtk git status");
        assert_eq!(entry.cwd, "/home/user/project");
        assert_eq!(entry.cmd_class, "git status");
        assert_eq!(entry.rewrite_types, vec!["rtk-rewrite"]);

        // Delete
        delete_stash(tmp.path(), hash);
        assert!(read_stash(tmp.path(), hash).is_none());
    }

    #[test]
    fn stash_no_file_returns_none() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(read_stash(tmp.path(), 12345).is_none());
    }

    #[test]
    fn savings_measurement_append() {
        let tmp = tempfile::tempdir().unwrap();

        append_savings_measurement(
            tmp.path(),
            "git status",
            "rtk-rewrite",
            "rtk",          // compression_mode
            "live",         // probe_kind
            "test-session", // session_id
            500,            // original
            120,            // actual
            "ground_truth",
            15.3,
        );

        let log_path = tmp.path().join("savings_measurements.jsonl");
        assert!(log_path.exists());

        let content = std::fs::read_to_string(&log_path).unwrap();
        let parsed: Value = serde_json::from_str(content.trim()).unwrap();
        assert_eq!(parsed["cmd_class"].as_str().unwrap(), "git status");
        assert_eq!(parsed["rewrite_type"].as_str().unwrap(), "rtk-rewrite");
        assert_eq!(parsed["compression_mode"].as_str().unwrap(), "rtk");
        assert_eq!(parsed["probe_kind"].as_str().unwrap(), "live");
        assert_eq!(parsed["session_id"].as_str().unwrap(), "test-session");
        assert_eq!(parsed["original_output_tokens"].as_u64().unwrap(), 500);
        assert_eq!(parsed["actual_output_tokens"].as_u64().unwrap(), 120);
        assert_eq!(parsed["savings_tokens"].as_u64().unwrap(), 380);
        assert!((parsed["savings_pct"].as_f64().unwrap() - 76.0).abs() < 0.1);
        assert_eq!(parsed["measurement_method"].as_str().unwrap(), "ground_truth");
    }
}
