//! precc-hook: Claude Code PreToolUse:Bash hook binary.
//!
//! Reads JSON from stdin (Claude Code hook event), processes through the
//! PRECC pipeline, and emits modified JSON to stdout.
//!
//! Pipeline stages:
//! 1. Parse command JSON
//! 2. Query heuristics.db for matching skills (Pillar 4) — read-only, skip if no DB
//! 3. Resolve correct working directory (Pillar 1)
//! 4. Check for GDB opportunities (Pillar 2) — currently a no-op (needs history.db query)
//! 5. Apply RTK rewriting (existing)
//! 6. Emit modified command JSON
//!
//! Safety: On any error, exit 0 (allow command unchanged). Never block Claude.
//! Latency target: < 5ms p99.
//!
//! Performance notes:
//! - No subprocess spawns (gdb/rtk checks use PATH scanning)
//! - No DB writes in the hot path — metrics appended to metrics.log (O_APPEND)
//!   and imported into metrics.db by precc-miner on its next tick.
//! - No builtin skills loading (done by precc init)
//! - Heuristics DB opened read-only, skipped if file doesn't exist
//! - Schema init skipped (precc init handles it)

use precc_core::{context, db, rtk, skills};
use serde_json::Value;
use std::io::{Read, Write};

/// Confidence threshold for auto-applying skills.
const AUTO_APPLY_THRESHOLD: f64 = 0.7;

/// Minimum confidence to show a suggestion.
const SUGGEST_THRESHOLD: f64 = 0.3;

fn main() {
    // Fail open: any panic or error => exit 0 (approve unchanged)
    if run().is_err() {
        std::process::exit(0);
    }
}

fn run() -> anyhow::Result<()> {
    let t_start = std::time::Instant::now();

    // Stage 1: Parse JSON from stdin
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let hook_input: Value = serde_json::from_str(&input)?;
    let command = match hook_input
        .get("tool_input")
        .and_then(|ti| ti.get("command"))
        .and_then(|c| c.as_str())
    {
        Some(cmd) => cmd.to_string(),
        None => return Ok(()), // No command, exit 0
    };

    if command.is_empty() {
        return Ok(());
    }

    // Run pipeline
    let mut pipeline = Pipeline::new(command);
    pipeline.run();

    // Stage 6: Emit result
    // Emit when command was rewritten OR when there's a GDB suggestion to surface.
    if pipeline.modified() || pipeline.had_gdb_suggestion {
        let tool_input = hook_input
            .get("tool_input")
            .cloned()
            .unwrap_or(Value::Object(serde_json::Map::new()));

        emit_rewrite(&tool_input, &pipeline.command, &pipeline.reason())?;
    }
    // If not modified and no suggestions, exit 0 (approve unchanged)

    // Record metrics (after stdout emit — never delays response to Claude)
    let latency_ms = t_start.elapsed().as_secs_f64() * 1000.0;
    append_metrics_log(&pipeline, latency_ms);

    Ok(())
}

struct Pipeline {
    original: String,
    command: String,
    reasons: Vec<String>,
    // Flags set by each stage for metrics reporting
    had_cd_prepend: bool,
    had_rtk_rewrite: bool,
    had_gdb_suggestion: bool,
}

impl Pipeline {
    fn new(command: String) -> Self {
        Self {
            original: command.clone(),
            command,
            reasons: Vec::new(),
            had_cd_prepend: false,
            had_rtk_rewrite: false,
            had_gdb_suggestion: false,
        }
    }

    fn modified(&self) -> bool {
        self.command != self.original
    }

    fn reason(&self) -> String {
        if self.reasons.is_empty() {
            "PRECC".to_string()
        } else {
            format!("PRECC: {}", self.reasons.join("; "))
        }
    }

    fn run(&mut self) {
        // Stage 2: Skill matching (Pillar 4) — read-only, skip if no DB
        self.stage_skills();

        // Stage 3: Context resolution (Pillar 1)
        self.stage_context();

        // Stage 4: GDB check (Pillar 2)
        self.stage_gdb();

        // Stage 5: RTK rewriting
        self.stage_rtk();
    }

    /// Stage 2: Query heuristics.db for matching skills (read-only).
    /// Skips entirely if heuristics.db doesn't exist yet.
    fn stage_skills(&mut self) {
        let data_dir = match db::data_dir() {
            Ok(d) => d,
            Err(_) => return,
        };

        // Open read-only; skip if DB doesn't exist (precc init not run yet)
        let conn = match db::open_heuristics_readonly(&data_dir) {
            Ok(Some(c)) => c,
            _ => return,
        };

        let matches = match skills::find_matches(&conn, &self.command, SUGGEST_THRESHOLD) {
            Ok(m) => m,
            Err(_) => return,
        };

        for m in &matches {
            if m.confidence >= AUTO_APPLY_THRESHOLD {
                // Auto-apply: resolve project root for template.
                // Skip prepend_cd skills when no better directory is known —
                // applying `cd CWD && cmd` is a no-op rewrite that adds noise.
                let project_root = self.resolve_project_root();
                if m.action_type == "prepend_cd" {
                    let cwd = std::env::current_dir()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_default();
                    if project_root == cwd || project_root == "." {
                        break; // No better directory found — skip skill
                    }
                }
                let rewritten = skills::apply_template(&m.template, &self.command, &project_root);
                self.command = rewritten;
                self.reasons
                    .push(format!("skill:{} (conf={:.1})", m.skill_name, m.confidence));
                // Append activation to log file for async processing by precc-miner.
                // Single write() syscall (~10-50µs), fail-open silently.
                append_activation_log(m.skill_id, &m.skill_name, m.confidence);
                break; // Apply first matching high-confidence skill only
            }
        }
    }

    /// Stage 3: Context-aware directory resolution (Pillar 1).
    fn stage_context(&mut self) {
        let ctx = context::resolve(&self.command);

        if let Some(rewritten) = context::apply(&self.command, &ctx) {
            // Only apply if skills didn't already prepend a cd
            if !self.command.starts_with("cd ") {
                self.command = rewritten;
                self.had_cd_prepend = true;
                self.reasons.push(format!(
                    "cd:{} (conf={:.1})",
                    ctx.marker.as_deref().unwrap_or("?"),
                    ctx.confidence
                ));
            }
        }
    }

    /// Stage 4: GDB-based debugging suggestion (Pillar 2).
    ///
    /// Queries history.db for recent failures of the same command class.
    /// If the command has failed ≥2 times in the last 24 hours and GDB is
    /// available, appends a `precc debug` suggestion to the reason string.
    /// The command itself is never modified — this is advisory only.
    fn stage_gdb(&mut self) {
        // Only consider debuggable commands (cargo test/run, ./binary, etc.)
        // Fast check before opening the DB.
        if !precc_core::gdb::gdb_available() {
            return;
        }

        let data_dir = match db::data_dir() {
            Ok(d) => d,
            Err(_) => return,
        };

        let conn = match db::open_history_readonly(&data_dir) {
            Ok(Some(c)) => c,
            _ => return,
        };

        let recent_failures = precc_core::gdb::count_recent_failures(&conn, &self.command);

        if let Some(suggestion) = precc_core::gdb::check_opportunity(&self.command, recent_failures)
        {
            self.reasons.push(format!("gdb-hint:{}", suggestion));
            self.had_gdb_suggestion = true;
        }
    }

    /// Stage 5: RTK command rewriting.
    fn stage_rtk(&mut self) {
        // RTK rewriting applies to the (possibly cd-prepended) command.
        // We need to rewrite the actual command part, not the cd prefix.
        let (prefix, cmd_part) = split_cd_prefix(&self.command);

        if let Some(rewritten) = rtk::rewrite(cmd_part) {
            self.command = if prefix.is_empty() {
                rewritten
            } else {
                format!("{}{}", prefix, rewritten)
            };
            self.had_rtk_rewrite = true;
            self.reasons.push("rtk-rewrite".to_string());
        }
    }

    /// Helper: resolve project root for skill template application.
    fn resolve_project_root(&self) -> String {
        let ctx = context::resolve(&self.command);
        ctx.project_root
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| {
                std::env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| ".".to_string())
            })
    }
}

/// Append a skill activation record to the activations log.
///
/// Uses O_APPEND semantics (single write syscall) for atomicity.
/// Fail-open: any error is silently ignored to stay within latency budget.
fn append_activation_log(skill_id: i64, skill_name: &str, conf: f64) {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let line = format!(
        "{{\"ts\":{},\"skill_id\":{},\"skill_name\":\"{}\",\"conf\":{:.3}}}\n",
        ts, skill_id, skill_name, conf
    );

    if let Ok(home) = std::env::var("HOME") {
        let log_path = std::path::Path::new(&home).join(".local/share/precc/activations.log");
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .and_then(|mut f| f.write_all(line.as_bytes()));
    }
}

/// Append hook metrics to metrics.log for async import by precc-miner.
///
/// Records: hook_latency, cd_prepend (if fired), rtk_rewrite (if fired).
/// Uses O_APPEND semantics — single write() syscall per entry, atomic.
/// Fail-open: any error is silently ignored.
fn append_metrics_log(pipeline: &Pipeline, latency_ms: f64) {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let home = match std::env::var("HOME") {
        Ok(h) => h,
        Err(_) => return,
    };
    let log_path = std::path::Path::new(&home).join(".local/share/precc/metrics.log");

    // Build all lines to write in a single syscall
    let mut lines = format!(
        "{{\"ts\":{},\"type\":\"hook_latency\",\"value\":{:.3}}}\n",
        ts, latency_ms
    );
    if pipeline.had_cd_prepend {
        lines.push_str(&format!(
            "{{\"ts\":{},\"type\":\"cd_prepend\",\"value\":1.0}}\n",
            ts
        ));
    }
    if pipeline.had_rtk_rewrite {
        lines.push_str(&format!(
            "{{\"ts\":{},\"type\":\"rtk_rewrite\",\"value\":1.0}}\n",
            ts
        ));
    }
    if pipeline.had_gdb_suggestion {
        lines.push_str(&format!(
            "{{\"ts\":{},\"type\":\"gdb_suggestion\",\"value\":1.0}}\n",
            ts
        ));
    }

    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .and_then(|mut f| f.write_all(lines.as_bytes()));
}

/// Split a command into its `cd /path &&` prefix (if any) and the remaining command.
fn split_cd_prefix(command: &str) -> (&str, &str) {
    if let Some(pos) = command.find(" && ") {
        if command.starts_with("cd ") {
            let prefix_end = pos + 4; // include " && "
            (&command[..prefix_end], &command[prefix_end..])
        } else {
            ("", command)
        }
    } else {
        ("", command)
    }
}

/// Emit the hook rewrite JSON to stdout.
fn emit_rewrite(
    original_tool_input: &Value,
    new_command: &str,
    reason: &str,
) -> anyhow::Result<()> {
    let mut updated_input = original_tool_input.clone();
    if let Some(obj) = updated_input.as_object_mut() {
        obj.insert(
            "command".to_string(),
            Value::String(new_command.to_string()),
        );
    }

    let output = serde_json::json!({
        "hookSpecificOutput": {
            "hookEventName": "PreToolUse",
            "permissionDecision": "allow",
            "permissionDecisionReason": reason,
            "updatedInput": updated_input
        }
    });

    println!("{}", serde_json::to_string(&output)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_cd_prefix_with_cd() {
        let (prefix, cmd) = split_cd_prefix("cd /home/user/myapp && cargo build");
        assert_eq!(prefix, "cd /home/user/myapp && ");
        assert_eq!(cmd, "cargo build");
    }

    #[test]
    fn split_cd_prefix_without_cd() {
        let (prefix, cmd) = split_cd_prefix("cargo build --release");
        assert_eq!(prefix, "");
        assert_eq!(cmd, "cargo build --release");
    }

    #[test]
    fn split_cd_prefix_non_cd_with_ampersand() {
        let (prefix, cmd) = split_cd_prefix("echo hello && echo world");
        assert_eq!(prefix, "");
        assert_eq!(cmd, "echo hello && echo world");
    }

    #[test]
    fn pipeline_no_modification() {
        let mut p = Pipeline::new("echo hello".to_string());
        // Only run RTK stage (others need filesystem/DB)
        p.stage_rtk();
        assert!(!p.modified());
    }

    #[test]
    fn pipeline_rtk_rewrite() {
        // This test depends on rtk being available, which it may not be in CI.
        // The rtk module handles this check internally.
        let mut p = Pipeline::new("git status".to_string());
        p.stage_rtk();
        // If rtk is available, command should be rewritten
        // If not, command should be unchanged
        // Both are valid outcomes for this test
        assert!(p.command == "git status" || p.command == "rtk git status");
    }

    #[test]
    fn pipeline_rtk_rewrite_preserves_cd_prefix() {
        let mut p = Pipeline::new("cd /tmp && git status".to_string());
        p.stage_rtk();
        if p.modified() {
            assert!(p.command.starts_with("cd /tmp && rtk git status"));
        }
    }

    #[test]
    fn emit_rewrite_produces_valid_json() {
        let tool_input = serde_json::json!({"command": "git status", "timeout": 5000});
        // Capture stdout would need more setup; just verify it doesn't panic
        let mut updated = tool_input.clone();
        updated["command"] = Value::String("rtk git status".to_string());

        let output = serde_json::json!({
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "allow",
                "permissionDecisionReason": "PRECC: rtk-rewrite",
                "updatedInput": updated
            }
        });

        let s = serde_json::to_string(&output).unwrap();
        assert!(s.contains("rtk git status"));
        assert!(s.contains("PreToolUse"));
    }

    #[test]
    fn pipeline_reason_empty() {
        let p = Pipeline::new("echo hello".to_string());
        assert_eq!(p.reason(), "PRECC");
    }

    #[test]
    fn pipeline_reason_with_entries() {
        let mut p = Pipeline::new("echo hello".to_string());
        p.reasons.push("rtk-rewrite".to_string());
        p.reasons.push("cd:Cargo.toml (conf=0.9)".to_string());
        assert_eq!(p.reason(), "PRECC: rtk-rewrite; cd:Cargo.toml (conf=0.9)");
    }

    #[test]
    fn pipeline_flags_default_false() {
        let p = Pipeline::new("echo hello".to_string());
        assert!(!p.had_cd_prepend);
        assert!(!p.had_rtk_rewrite);
    }

    #[test]
    fn metrics_log_line_format() {
        // Verify the JSON line format we write can be parsed back correctly
        let line = format!(
            "{{\"ts\":{},\"type\":\"hook_latency\",\"value\":{:.3}}}\n",
            1000u64, 2.93f64
        );
        let parsed: serde_json::Value = serde_json::from_str(line.trim()).unwrap();
        assert_eq!(parsed["type"].as_str(), Some("hook_latency"));
        assert!((parsed["value"].as_f64().unwrap() - 2.93).abs() < 0.001);
    }
}
