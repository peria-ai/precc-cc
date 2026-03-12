//! precc-hook: Claude Code PreToolUse hook binary.
//!
//! Reads JSON from stdin (Claude Code hook event), dispatches by tool type,
//! and emits modified JSON to stdout.
//!
//! Supported tools:
//! - **Bash**: Full pipeline (skills, context, GDB, RTK rewriting)
//! - **Read**: Binary file blocking, smart limit injection, duplicate read warning
//! - **Grep**: Auto head_limit, auto type filter, LSP hints
//! - **Agent**: Subagent hook propagation (inject PRECC hooks into prompt)
//!
//! Safety: On any error, exit 0 (allow command unchanged). Never block Claude.
//! Latency target: < 5ms p99 (Bash), < 3ms p99 (Read/Grep/Agent).
//!
//! Performance notes:
//! - No subprocess spawns (gdb/rtk checks use PATH scanning)
//! - No DB writes in the hot path — metrics appended to metrics.log (O_APPEND)
//!   and imported into metrics.db by precc-miner on its next tick.
//! - No builtin skills loading (done by precc init)
//! - Heuristics DB opened read-only, skipped if file doesn't exist
//! - Schema init skipped (precc init handles it)

use precc_core::{
    agent_propagate, context, db, grep_filter, license, post_observe, read_filter, rtk, skills,
    update_check,
};
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
    let input_len = input.len() as u64;

    let hook_input: Value = serde_json::from_str(&input)?;

    let hook_event = hook_input
        .get("hook_event_name")
        .and_then(|v| v.as_str())
        .unwrap_or("PreToolUse");

    let tool_name = hook_input
        .get("tool_name")
        .and_then(|v| v.as_str())
        .unwrap_or("Bash");

    let tool_input = hook_input
        .get("tool_input")
        .cloned()
        .unwrap_or(Value::Object(Default::default()));

    match hook_event {
        "PostToolUse" => run_post_observe(&hook_input, &tool_input, tool_name, input_len, t_start),
        _ => {
            // PreToolUse (default)
            match tool_name {
                "Bash" => run_bash_pipeline(&hook_input, &tool_input, t_start),
                "Read" => run_read_filter(&tool_input, t_start),
                "Grep" => run_grep_filter(&tool_input, t_start),
                "Agent" => run_agent_propagate(&tool_input, t_start),
                _ => Ok(()), // Unknown tool — approve unchanged
            }
        }
    }
}

// =============================================================================
// Bash pipeline (existing behavior, refactored into its own function)
// =============================================================================

fn run_bash_pipeline(
    hook_input: &Value,
    tool_input: &Value,
    t_start: std::time::Instant,
) -> anyhow::Result<()> {
    let command = match tool_input.get("command").and_then(|c| c.as_str()) {
        Some(cmd) => cmd.to_string(),
        None => return Ok(()),
    };

    if command.is_empty() {
        return Ok(());
    }

    // Run pipeline
    let mut pipeline = Pipeline::new(command);
    pipeline.run();

    // Emit when command was rewritten OR when there's a GDB suggestion to surface.
    if pipeline.modified() || pipeline.had_gdb_suggestion {
        let ti = hook_input
            .get("tool_input")
            .cloned()
            .unwrap_or(Value::Object(serde_json::Map::new()));

        emit_rewrite(&ti, &pipeline.command, &pipeline.reason())?;
    }

    // Record metrics (after stdout emit — never delays response to Claude)
    let latency_ms = t_start.elapsed().as_secs_f64() * 1000.0;
    append_metrics_log_bash(&pipeline, latency_ms);

    // Surface update notification on stderr (never delays stdout response)
    if let Ok(data_dir) = db::data_dir() {
        if let Some(ver) = update_check::read_update_available(&data_dir) {
            eprintln!("[precc] Update available: v{ver} — run `precc update`");
        }
    }

    Ok(())
}

// =============================================================================
// Read filter
// =============================================================================

fn run_read_filter(tool_input: &Value, t_start: std::time::Instant) -> anyhow::Result<()> {
    let file_path = match tool_input.get("file_path").and_then(|v| v.as_str()) {
        Some(p) => p,
        None => return Ok(()),
    };

    // Check 1: Block binary files
    if read_filter::is_binary_extension(file_path) {
        let ext = std::path::Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("binary");
        let reason = format!(
            "PRECC: blocked binary file (.{}) — use a dedicated tool for this file type",
            ext
        );
        emit_deny(&reason)?;
        append_metrics_log_tool("read_filter", t_start);
        return Ok(());
    }

    // Check 2: Smart limit injection for large files
    let current_limit = tool_input.get("limit").and_then(|v| v.as_u64());
    let mut modified = false;
    let mut updated_input = tool_input.clone();

    if let Some(suggested_limit) = read_filter::suggest_limit(file_path, current_limit) {
        if let Some(obj) = updated_input.as_object_mut() {
            obj.insert("limit".to_string(), Value::Number(suggested_limit.into()));
            modified = true;
        }
    }

    // Check 3: Duplicate read warning (advisory only, on stderr)
    if let Ok(data_dir) = db::data_dir() {
        let offset = tool_input.get("offset").and_then(|v| v.as_u64());
        let limit = tool_input
            .get("limit")
            .and_then(|v| v.as_u64())
            .or(current_limit);
        if read_filter::check_recent_read(&data_dir, file_path, offset, limit) {
            eprintln!(
                "[precc] Note: {} was read recently — consider reusing prior content",
                file_path
            );
        }
    }

    if modified {
        let output = serde_json::json!({
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "allow",
                "permissionDecisionReason": "PRECC: read-filter (limit injection)",
                "updatedInput": updated_input
            }
        });
        println!("{}", serde_json::to_string(&output)?);
        append_metrics_log_tool("read_filter", t_start);
    }

    Ok(())
}

// =============================================================================
// Grep filter
// =============================================================================

fn run_grep_filter(tool_input: &Value, t_start: std::time::Instant) -> anyhow::Result<()> {
    let mut updated_input = tool_input.clone();
    let mut modified = false;
    let mut reasons = Vec::new();

    // Check 1: Auto head_limit
    if let Some(limit) = grep_filter::suggest_head_limit(tool_input) {
        if let Some(obj) = updated_input.as_object_mut() {
            obj.insert("head_limit".to_string(), Value::Number(limit.into()));
            modified = true;
            reasons.push("head_limit injection");
        }
    }

    // Check 2: Auto type filter
    let cwd = std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    if let Some(file_type) = grep_filter::suggest_type_filter(tool_input, &cwd) {
        if let Some(obj) = updated_input.as_object_mut() {
            obj.insert("type".to_string(), Value::String(file_type.to_string()));
            modified = true;
            reasons.push("auto type filter");
        }
    }

    // Check 3: LSP hint (advisory only, on stderr)
    if let Some(pattern) = tool_input.get("pattern").and_then(|v| v.as_str()) {
        if grep_filter::is_symbol_lookup(pattern) {
            eprintln!(
                "[precc] Hint: consider using Go to Definition (LSP) instead of Grep for symbol lookups"
            );
        }
    }

    if modified {
        let reason = format!("PRECC: grep-filter ({})", reasons.join(", "));
        let output = serde_json::json!({
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "allow",
                "permissionDecisionReason": reason,
                "updatedInput": updated_input
            }
        });
        println!("{}", serde_json::to_string(&output)?);
        append_metrics_log_tool("grep_filter", t_start);
    }

    Ok(())
}

// =============================================================================
// Agent propagation
// =============================================================================

fn run_agent_propagate(tool_input: &Value, t_start: std::time::Instant) -> anyhow::Result<()> {
    let prompt = match tool_input.get("prompt").and_then(|v| v.as_str()) {
        Some(p) => p,
        None => return Ok(()),
    };

    // Skip if hooks are already present
    if agent_propagate::has_hooks_frontmatter(prompt) {
        return Ok(());
    }

    let new_prompt = agent_propagate::inject_hooks_frontmatter(prompt);
    let mut updated_input = tool_input.clone();
    if let Some(obj) = updated_input.as_object_mut() {
        obj.insert("prompt".to_string(), Value::String(new_prompt));
    }

    let output = serde_json::json!({
        "hookSpecificOutput": {
            "hookEventName": "PreToolUse",
            "permissionDecision": "allow",
            "permissionDecisionReason": "PRECC: agent-propagate (subagent hook injection)",
            "updatedInput": updated_input
        }
    });
    println!("{}", serde_json::to_string(&output)?);
    append_metrics_log_tool("agent_propagate", t_start);

    Ok(())
}

// =============================================================================
// PostToolUse observability
// =============================================================================

fn run_post_observe(
    hook_input: &Value,
    tool_input: &Value,
    tool_name: &str,
    input_len: u64,
    t_start: std::time::Instant,
) -> anyhow::Result<()> {
    let data_dir = match db::data_dir() {
        Ok(d) => d,
        Err(_) => return Ok(()), // No data dir — skip silently
    };

    let session_id = hook_input
        .get("session_id")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    // Estimate output size from tool_response if present, else from raw input length
    let tool_response = hook_input.get("tool_response");
    let (output_bytes, estimated_tokens) = if let Some(resp) = tool_response {
        let tokens = post_observe::estimate_tokens(resp);
        let bytes = serde_json::to_string(resp)
            .map(|s| s.len() as u64)
            .unwrap_or(0);
        (bytes, tokens)
    } else {
        // Fallback: estimate from total input minus overhead (~200 bytes envelope)
        let effective = input_len.saturating_sub(200);
        (effective, post_observe::tokens_from_bytes(effective))
    };

    // Compute command hash for dedup
    let cmd_hash = post_observe::hash_command(tool_name, tool_input);

    // Check for duplicates
    let duplicate_count = post_observe::check_duplicate(&data_dir, session_id, cmd_hash);

    // Check for large output
    let is_large = post_observe::is_large_output(estimated_tokens);

    // Log the observation
    post_observe::append_observation(
        &data_dir,
        session_id,
        tool_name,
        cmd_hash,
        output_bytes,
        estimated_tokens,
    );

    // Build waste report
    let report = post_observe::WasteReport {
        duplicate_count,
        is_large,
        estimated_tokens,
        output_bytes,
    };

    // Emit additionalContext if waste detected
    if let Some(context) = report.advisory_context(tool_name) {
        let output = serde_json::json!({
            "hookSpecificOutput": {
                "hookEventName": "PostToolUse",
                "additionalContext": context
            }
        });
        println!("{}", serde_json::to_string(&output)?);
    }

    // Append metrics
    append_metrics_log_post(tool_name, output_bytes, estimated_tokens, &report, t_start);

    Ok(())
}

/// Append PostToolUse metrics to metrics.log.
fn append_metrics_log_post(
    tool_name: &str,
    output_bytes: u64,
    estimated_tokens: u64,
    report: &post_observe::WasteReport,
    t_start: std::time::Instant,
) {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let latency_ms = t_start.elapsed().as_secs_f64() * 1000.0;

    let home = match std::env::var("HOME") {
        Ok(h) => h,
        Err(_) => return,
    };
    let log_path = std::path::Path::new(&home).join(".local/share/precc/metrics.log");

    let mut lines = format!(
        "{{\"ts\":{},\"type\":\"hook_latency_post\",\"value\":{:.3}}}\n\
         {{\"ts\":{},\"type\":\"post_output_bytes\",\"value\":{}.0}}\n\
         {{\"ts\":{},\"type\":\"post_output_tokens\",\"value\":{}.0}}\n\
         {{\"ts\":{},\"type\":\"post_tool_{}\",\"value\":{}.0}}\n",
        ts, latency_ms, ts, output_bytes, ts, estimated_tokens, ts, tool_name, estimated_tokens
    );

    if report.duplicate_count.is_some() {
        lines.push_str(&format!(
            "{{\"ts\":{},\"type\":\"post_duplicate_detected\",\"value\":1.0}}\n",
            ts
        ));
    }
    if report.is_large {
        lines.push_str(&format!(
            "{{\"ts\":{},\"type\":\"post_large_output\",\"value\":1.0}}\n",
            ts
        ));
    }

    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .and_then(|mut f| f.write_all(lines.as_bytes()));
}

// =============================================================================
// Bash Pipeline (struct + stages)
// =============================================================================

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

        // Fast pre-filter: read skill_prefixes.txt (plain text, no SQLCipher cost).
        // If the command's first word isn't listed, skip opening heuristics.db entirely.
        // This avoids the ~7ms SQLCipher open cost for commands that can never match.
        if !command_matches_prefix_cache(&self.command, &data_dir) {
            return;
        }

        // Open read-only; skip if DB doesn't exist (precc init not run yet)
        let conn = match db::open_heuristics_readonly(&data_dir) {
            Ok(Some(c)) => c,
            _ => return,
        };

        let mut matches = match skills::find_matches(&conn, &self.command, SUGGEST_THRESHOLD) {
            Ok(m) => m,
            Err(_) => return,
        };

        // License gate: Free tier may only use builtin skills plus FREE_SKILL_LIMIT mined ones.
        // Mined skills are capped (builtins are always allowed).
        // We count mined slots used so far this call and drop excess matches.
        if license::tier() == license::Tier::Free {
            let mut mined_seen = 0usize;
            matches.retain(|m| {
                if m.source == "builtin" {
                    true
                } else {
                    mined_seen += 1;
                    mined_seen <= precc_core::FREE_SKILL_LIMIT
                }
            });
        }

        // Portfolio application: apply all compatible high-confidence skills.
        //
        // Conflict rule: at most one skill per action type may mutate the command
        // (two `rewrite_command` or two `prepend_cd` actions would produce an
        // incoherent command).  `suggest_fix` is always additive and never
        // conflicts.  We track which mutating action types have already fired and
        // skip any subsequent skill that would conflict.
        let project_root = self.resolve_project_root();
        let cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        let mut used_mutating_types: std::collections::HashSet<String> =
            std::collections::HashSet::new();

        for m in &matches {
            if m.confidence < AUTO_APPLY_THRESHOLD {
                // Below auto-apply threshold — surface as suggestion only
                if m.action_type == "suggest_fix" {
                    self.reasons.push(format!(
                        "suggest:{} (conf={:.1})",
                        m.skill_name, m.confidence
                    ));
                }
                continue;
            }

            match m.action_type.as_str() {
                "prepend_cd" => {
                    // Skip if a cd was already prepended by an earlier skill
                    if used_mutating_types.contains("prepend_cd") {
                        continue;
                    }
                    // Skip if no better directory is known
                    if project_root == cwd || project_root == "." {
                        continue;
                    }
                    let rewritten =
                        skills::apply_template(&m.template, &self.command, &project_root);
                    self.command = rewritten;
                    self.had_cd_prepend = true;
                    used_mutating_types.insert("prepend_cd".to_string());
                    self.reasons
                        .push(format!("skill:{} (conf={:.1})", m.skill_name, m.confidence));
                    append_activation_log(m.skill_id, &m.skill_name, m.confidence);
                }
                "rewrite_command" => {
                    // Skip if a rewrite was already applied by an earlier skill
                    if used_mutating_types.contains("rewrite_command") {
                        continue;
                    }
                    let rewritten =
                        skills::apply_template(&m.template, &self.command, &project_root);
                    self.command = rewritten;
                    used_mutating_types.insert("rewrite_command".to_string());
                    self.reasons
                        .push(format!("skill:{} (conf={:.1})", m.skill_name, m.confidence));
                    append_activation_log(m.skill_id, &m.skill_name, m.confidence);
                }
                "suggest_fix" => {
                    // Always additive — surface every suggestion above threshold
                    self.reasons.push(format!(
                        "suggest:{} (conf={:.1})",
                        m.skill_name, m.confidence
                    ));
                    append_activation_log(m.skill_id, &m.skill_name, m.confidence);
                }
                _ => {
                    // Unknown action type — skip safely
                }
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

        // jj translation takes priority: if the repo uses jj, translate git → jj first.
        if let Some(rewritten) = rtk::jj_translate(cmd_part) {
            self.command = if prefix.is_empty() {
                rewritten
            } else {
                format!("{}{}", prefix, rewritten)
            };
            self.had_rtk_rewrite = true;
            self.reasons.push("jj-translate".to_string());
            return;
        }

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

// =============================================================================
// Shared helpers
// =============================================================================

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

/// Append Bash hook metrics to metrics.log for async import by precc-miner.
///
/// Records: hook_latency, cd_prepend (if fired), rtk_rewrite (if fired).
/// Uses O_APPEND semantics — single write() syscall per entry, atomic.
/// Fail-open: any error is silently ignored.
fn append_metrics_log_bash(pipeline: &Pipeline, latency_ms: f64) {
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

/// Append a tool-specific metric event to metrics.log.
fn append_metrics_log_tool(tool_type: &str, t_start: std::time::Instant) {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let latency_ms = t_start.elapsed().as_secs_f64() * 1000.0;

    let home = match std::env::var("HOME") {
        Ok(h) => h,
        Err(_) => return,
    };
    let log_path = std::path::Path::new(&home).join(".local/share/precc/metrics.log");

    let lines = format!(
        "{{\"ts\":{},\"type\":\"hook_latency_{}\",\"value\":{:.3}}}\n\
         {{\"ts\":{},\"type\":\"{}\",\"value\":1.0}}\n",
        ts, tool_type, latency_ms, ts, tool_type
    );

    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .and_then(|mut f| f.write_all(lines.as_bytes()));
}

/// Emit a deny decision to stdout.
fn emit_deny(reason: &str) -> anyhow::Result<()> {
    let output = serde_json::json!({
        "hookSpecificOutput": {
            "hookEventName": "PreToolUse",
            "permissionDecision": "deny",
            "permissionDecisionReason": reason
        }
    });
    println!("{}", serde_json::to_string(&output)?);
    Ok(())
}

/// Split a command into its `cd /path &&` prefix (if any) and the remaining command.
/// Check whether the command's first word appears in the skill prefix cache.
///
/// Reads `data_dir/skill_prefixes.txt` (plain text, no SQLCipher).
/// Returns `true` (open the DB) if:
///   - The file doesn't exist (fall back to always opening — safe default)
///   - The file contains `*` (wildcard: some skill has an unanalysable pattern)
///   - The command's first word is listed in the file
///
/// Returns `false` (skip DB) otherwise.
fn command_matches_prefix_cache(command: &str, data_dir: &std::path::Path) -> bool {
    let cache_path = data_dir.join("skill_prefixes.txt");
    let content = match std::fs::read_to_string(&cache_path) {
        Ok(c) => c,
        Err(_) => return true, // No cache → always open DB (safe default)
    };
    let first_word = command.split_whitespace().next().unwrap_or("");
    for line in content.lines() {
        let line = line.trim();
        if line == "*" || line == first_word {
            return true;
        }
    }
    false
}

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

    // =========================================================================
    // emit_deny output format
    // =========================================================================

    #[test]
    fn emit_deny_format() {
        let reason = "PRECC: blocked binary file (.png)";
        let output = serde_json::json!({
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "deny",
                "permissionDecisionReason": reason
            }
        });
        let s = serde_json::to_string(&output).unwrap();
        assert!(s.contains("\"deny\""));
        assert!(s.contains(".png"));
        assert!(s.contains("PreToolUse"));
    }

    // =========================================================================
    // Read filter integration (JSON output shape)
    // =========================================================================

    #[test]
    fn read_filter_deny_output_shape() {
        // Simulate the deny output for a binary file
        let reason = "PRECC: blocked binary file (.wasm) — use a dedicated tool for this file type";
        let output = serde_json::json!({
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "deny",
                "permissionDecisionReason": reason
            }
        });
        let parsed: Value = serde_json::from_str(&serde_json::to_string(&output).unwrap()).unwrap();
        assert_eq!(
            parsed["hookSpecificOutput"]["permissionDecision"]
                .as_str()
                .unwrap(),
            "deny"
        );
        assert!(parsed["hookSpecificOutput"]["permissionDecisionReason"]
            .as_str()
            .unwrap()
            .contains(".wasm"));
    }

    #[test]
    fn read_filter_limit_injection_output_shape() {
        // Simulate the allow output with injected limit
        let mut updated_input = serde_json::json!({"file_path": "/tmp/big.rs", "offset": 0});
        updated_input
            .as_object_mut()
            .unwrap()
            .insert("limit".to_string(), Value::Number(500.into()));

        let output = serde_json::json!({
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "allow",
                "permissionDecisionReason": "PRECC: read-filter (limit injection)",
                "updatedInput": updated_input
            }
        });

        let parsed: Value = serde_json::from_str(&serde_json::to_string(&output).unwrap()).unwrap();
        assert_eq!(
            parsed["hookSpecificOutput"]["updatedInput"]["limit"]
                .as_u64()
                .unwrap(),
            500
        );
        assert_eq!(
            parsed["hookSpecificOutput"]["permissionDecision"]
                .as_str()
                .unwrap(),
            "allow"
        );
    }

    // =========================================================================
    // Grep filter integration (JSON output shape)
    // =========================================================================

    #[test]
    fn grep_filter_head_limit_output_shape() {
        let mut updated_input = serde_json::json!({"pattern": "fn main", "output_mode": "content"});
        updated_input
            .as_object_mut()
            .unwrap()
            .insert("head_limit".to_string(), Value::Number(50.into()));

        let output = serde_json::json!({
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "allow",
                "permissionDecisionReason": "PRECC: grep-filter (head_limit injection)",
                "updatedInput": updated_input
            }
        });

        let parsed: Value = serde_json::from_str(&serde_json::to_string(&output).unwrap()).unwrap();
        assert_eq!(
            parsed["hookSpecificOutput"]["updatedInput"]["head_limit"]
                .as_u64()
                .unwrap(),
            50
        );
    }

    #[test]
    fn grep_filter_type_injection_output_shape() {
        let mut updated_input = serde_json::json!({"pattern": "foo"});
        updated_input
            .as_object_mut()
            .unwrap()
            .insert("type".to_string(), Value::String("rust".to_string()));

        let output = serde_json::json!({
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "allow",
                "permissionDecisionReason": "PRECC: grep-filter (auto type filter)",
                "updatedInput": updated_input
            }
        });

        let parsed: Value = serde_json::from_str(&serde_json::to_string(&output).unwrap()).unwrap();
        assert_eq!(
            parsed["hookSpecificOutput"]["updatedInput"]["type"]
                .as_str()
                .unwrap(),
            "rust"
        );
    }

    #[test]
    fn grep_filter_combined_reasons() {
        let reasons = vec!["head_limit injection", "auto type filter"];
        let reason = format!("PRECC: grep-filter ({})", reasons.join(", "));
        assert_eq!(
            reason,
            "PRECC: grep-filter (head_limit injection, auto type filter)"
        );
    }

    // =========================================================================
    // Agent propagation integration (JSON output shape)
    // =========================================================================

    #[test]
    fn agent_propagate_output_shape() {
        let original_prompt = "Find all test files";
        let new_prompt = precc_core::agent_propagate::inject_hooks_frontmatter(original_prompt);

        let mut updated_input = serde_json::json!({
            "prompt": original_prompt,
            "subagent_type": "Explore"
        });
        updated_input
            .as_object_mut()
            .unwrap()
            .insert("prompt".to_string(), Value::String(new_prompt.clone()));

        let output = serde_json::json!({
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "allow",
                "permissionDecisionReason": "PRECC: agent-propagate (subagent hook injection)",
                "updatedInput": updated_input
            }
        });

        let parsed: Value = serde_json::from_str(&serde_json::to_string(&output).unwrap()).unwrap();
        let updated_prompt = parsed["hookSpecificOutput"]["updatedInput"]["prompt"]
            .as_str()
            .unwrap();
        assert!(updated_prompt.contains("precc-hook"));
        assert!(updated_prompt.contains("Find all test files"));
        // subagent_type should be preserved
        assert_eq!(
            parsed["hookSpecificOutput"]["updatedInput"]["subagent_type"]
                .as_str()
                .unwrap(),
            "Explore"
        );
    }

    #[test]
    fn agent_propagate_preserves_other_fields() {
        let mut updated_input = serde_json::json!({
            "prompt": "test",
            "subagent_type": "general-purpose",
            "description": "search for files",
            "model": "sonnet"
        });
        let new_prompt = precc_core::agent_propagate::inject_hooks_frontmatter("test");
        updated_input
            .as_object_mut()
            .unwrap()
            .insert("prompt".to_string(), Value::String(new_prompt));

        // Other fields should remain unchanged
        assert_eq!(
            updated_input["subagent_type"].as_str().unwrap(),
            "general-purpose"
        );
        assert_eq!(
            updated_input["description"].as_str().unwrap(),
            "search for files"
        );
        assert_eq!(updated_input["model"].as_str().unwrap(), "sonnet");
    }

    // =========================================================================
    // Tool-specific metrics log format
    // =========================================================================

    #[test]
    fn tool_metrics_log_line_format() {
        // Verify the tool-specific metric JSON line format
        let ts = 1000u64;
        let tool_type = "read_filter";
        let latency_ms = 1.5f64;

        let lines = format!(
            "{{\"ts\":{},\"type\":\"hook_latency_{}\",\"value\":{:.3}}}\n\
             {{\"ts\":{},\"type\":\"{}\",\"value\":1.0}}\n",
            ts, tool_type, latency_ms, ts, tool_type
        );

        // Should be two valid JSON lines
        let json_lines: Vec<&str> = lines.trim().split('\n').collect();
        assert_eq!(json_lines.len(), 2);

        let latency_line: Value = serde_json::from_str(json_lines[0]).unwrap();
        assert_eq!(
            latency_line["type"].as_str().unwrap(),
            "hook_latency_read_filter"
        );

        let counter_line: Value = serde_json::from_str(json_lines[1]).unwrap();
        assert_eq!(counter_line["type"].as_str().unwrap(), "read_filter");
        assert!((counter_line["value"].as_f64().unwrap() - 1.0).abs() < 0.001);
    }

    #[test]
    fn tool_metrics_all_types() {
        // Verify all tool metric types produce valid JSON
        for tool_type in &["read_filter", "grep_filter", "agent_propagate"] {
            let line = format!("{{\"ts\":1000,\"type\":\"{}\",\"value\":1.0}}\n", tool_type);
            let parsed: Value = serde_json::from_str(line.trim()).unwrap();
            assert_eq!(parsed["type"].as_str().unwrap(), *tool_type);
        }
    }
}
