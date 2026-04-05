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
pub fn append_observation(
    data_dir: &Path,
    session_id: &str,
    tool_name: &str,
    cmd_hash: u64,
    output_bytes: u64,
    estimated_tokens: u64,
) {
    let log_path = data_dir.join("post_observations.log");
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let line = format!(
        "{{\"ts\":{},\"sid\":\"{}\",\"tool\":\"{}\",\"hash\":{},\"bytes\":{},\"tokens\":{}}}\n",
        ts, session_id, tool_name, cmd_hash, output_bytes, estimated_tokens
    );

    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .and_then(|mut f| f.write_all(line.as_bytes()));
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

    let words: Vec<&str> = unwrapped.split_whitespace().collect();
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
        append_observation(tmp.path(), "sess1", "Bash", 12345, 100, 25);
        assert!(check_duplicate(tmp.path(), "sess1", 12345).is_none());

        // Second observation — still only 1 in log, not yet duplicate
        append_observation(tmp.path(), "sess1", "Bash", 12345, 100, 25);
        // Now there are 2 entries → check_duplicate should detect it
        let result = check_duplicate(tmp.path(), "sess1", 12345);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 2);

        // Third observation
        append_observation(tmp.path(), "sess1", "Bash", 12345, 100, 25);
        let result = check_duplicate(tmp.path(), "sess1", 12345);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn no_duplicate_different_session() {
        let tmp = tempfile::tempdir().unwrap();
        append_observation(tmp.path(), "sess1", "Bash", 12345, 100, 25);
        let result = check_duplicate(tmp.path(), "sess2", 12345);
        assert!(result.is_none());
    }

    #[test]
    fn no_duplicate_different_hash() {
        let tmp = tempfile::tempdir().unwrap();
        append_observation(tmp.path(), "sess1", "Bash", 12345, 100, 25);
        let result = check_duplicate(tmp.path(), "sess1", 99999);
        assert!(result.is_none());
    }

    #[test]
    fn observation_log_format() {
        let tmp = tempfile::tempdir().unwrap();
        append_observation(tmp.path(), "sess1", "Bash", 12345, 500, 125);

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
}
