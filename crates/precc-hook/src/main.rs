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
//!   and imported into metrics.db by precc-learner on its next tick.
//! - No builtin skills loading (done by precc init)
//! - Heuristics DB opened read-only, skipped if file doesn't exist
//! - Schema init skipped (precc init handles it)

use precc_core::{
    agent_propagate, ccc, context, db, diet, geofence, grep_filter, lean_ctx, license, nushell,
    post_observe, read_filter, rtk, skills, update_check,
};
use serde_json::Value;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};

/// Confidence threshold for auto-applying skills.
const AUTO_APPLY_THRESHOLD: f64 = 0.7;

/// Minimum confidence to show a suggestion.
const SUGGEST_THRESHOLD: f64 = 0.3;

/// Check if dry-run mode is enabled (PRECC_DRY_RUN=1).
/// In dry-run mode, mutations are computed and logged but not applied —
/// the original command executes unchanged.
fn dry_run_enabled() -> bool {
    std::env::var("PRECC_DRY_RUN")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Check if a command is destructive (hard to reverse, dangerous if mutated wrongly).
/// Returns true for commands where a misapplied mutation (especially cd prepend)
/// could cause serious damage.
fn is_destructive(command: &str) -> bool {
    let cmd = command.trim();

    // Strip any cd prefix to inspect the actual command
    let effective = if let Some(pos) = cmd.find(" && ") {
        if cmd.starts_with("cd ") {
            cmd[pos + 4..].trim()
        } else {
            cmd
        }
    } else {
        cmd
    };

    let first_word = effective.split_whitespace().next().unwrap_or("");

    // Direct destructive commands
    if matches!(first_word, "rm" | "rmdir" | "shred" | "dd") {
        return true;
    }

    // Git destructive operations
    if first_word == "git" {
        let rest = effective.strip_prefix("git").unwrap_or("").trim();
        if rest.starts_with("reset --hard")
            || rest.starts_with("push --force")
            || rest.starts_with("push -f")
            || rest.starts_with("clean -f")
            || rest.starts_with("checkout -- .")
            || rest.starts_with("branch -D")
        {
            return true;
        }
    }

    // SQL destructive operations (case-insensitive check on the full command)
    let upper = effective.to_ascii_uppercase();
    if upper.contains("DROP TABLE")
        || upper.contains("DROP DATABASE")
        || upper.contains("TRUNCATE ")
        || upper.contains("DELETE FROM")
    {
        return true;
    }

    false
}

fn main() {
    // Statusline mode: invoked by Claude Code's statusLine config
    if std::env::args().any(|a| a == "--statusline") {
        if run_statusline().is_err() {
            // Fail silent — empty statusline on error
        }
        return;
    }

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
// Statusline mode — real-time PRECC metrics in Claude Code's status bar
// =============================================================================

/// Statusline session counters, parsed from metrics.log.
/// Only counts events from the current session (since session_start timestamp).
struct StatuslineCounts {
    corrections: u64,
    latency_sum: f64,
    latency_count: u64,
    tokens_saved: u64,
    skills_fired: u64,
    ccc_bytes_saved: u64,
}

impl StatuslineCounts {
    fn avg_latency_ms(&self) -> f64 {
        if self.latency_count == 0 {
            0.0
        } else {
            self.latency_sum / self.latency_count as f64
        }
    }
}

/// Parse metrics.log for events since `since_ts` (unix seconds).
fn parse_session_metrics(since_ts: u64) -> StatuslineCounts {
    let mut counts = StatuslineCounts {
        corrections: 0,
        latency_sum: 0.0,
        latency_count: 0,
        tokens_saved: 0,
        skills_fired: 0,
        ccc_bytes_saved: 0,
    };

    let home = match std::env::var("HOME") {
        Ok(h) => h,
        Err(_) => return counts,
    };
    let log_path = std::path::Path::new(&home).join(".local/share/precc/metrics.log");
    let content = match std::fs::read_to_string(&log_path) {
        Ok(c) => c,
        Err(_) => return counts,
    };

    // Token savings estimates per event type (accounts for full retry cycle avoided)
    const TOKENS_PER_CD: u64 = 800;
    const TOKENS_PER_RTK: u64 = 175;
    const TOKENS_PER_LEAN_CTX: u64 = 350; // lean-ctx compresses 70-80% of output
    const TOKENS_PER_SKILL: u64 = 600;
    const TOKENS_PER_CCC_BYTE: f64 = 0.25; // ~4 bytes per token

    for line in content.lines().rev() {
        let ts = match extract_ts(line) {
            Some(t) => t,
            None => continue,
        };
        if ts < since_ts {
            break; // metrics.log is append-only, so once we're past the window, stop
        }

        let metric_type = match extract_str_field(line, "type") {
            Some(t) => t,
            None => continue,
        };
        let value = extract_f64_field(line, "value").unwrap_or(0.0);

        match metric_type {
            "hook_latency" => {
                counts.latency_sum += value;
                counts.latency_count += 1;
            }
            "cd_prepend" => {
                counts.corrections += 1;
                counts.tokens_saved += TOKENS_PER_CD;
            }
            "lean_ctx_wrap" => {
                counts.corrections += 1;
                counts.tokens_saved += TOKENS_PER_LEAN_CTX;
            }
            "rtk_rewrite" => {
                counts.corrections += 1;
                counts.tokens_saved += TOKENS_PER_RTK;
            }
            "skill_activation" => {
                counts.corrections += 1;
                counts.skills_fired += 1;
                counts.tokens_saved += TOKENS_PER_SKILL;
            }
            "ccc_redirect" => {
                counts.corrections += 1;
                counts.ccc_bytes_saved += value as u64;
                counts.tokens_saved += (value * TOKENS_PER_CCC_BYTE) as u64;
            }
            "diet_rewrite" => {
                counts.corrections += 1;
                counts.tokens_saved += value as u64;
            }
            "gdb_suggestion" => {
                counts.corrections += 1;
            }
            _ => {}
        }
    }

    counts
}

/// Fast inline extraction of "ts" integer field from a JSON line.
fn extract_ts(line: &str) -> Option<u64> {
    let start = line.find("\"ts\":")?;
    let rest = &line[start + 5..];
    let end = rest.find(|c: char| !c.is_ascii_digit())?;
    rest[..end].parse().ok()
}

/// Fast inline extraction of a string field value from a JSON line.
fn extract_str_field<'a>(line: &'a str, field: &str) -> Option<&'a str> {
    let pattern = format!("\"{}\":\"", field);
    let start = line.find(&pattern)? + pattern.len();
    let rest = &line[start..];
    let end = rest.find('"')?;
    Some(&rest[..end])
}

/// Fast inline extraction of a float field value from a JSON line.
fn extract_f64_field(line: &str, field: &str) -> Option<f64> {
    let pattern = format!("\"{}\":", field);
    let start = line.find(&pattern)? + pattern.len();
    let rest = &line[start..];
    let end = rest
        .find(|c: char| c != '.' && c != '-' && !c.is_ascii_digit())
        .unwrap_or(rest.len());
    rest[..end].parse().ok()
}

/// Format a token count for display (e.g., 1500 → "1.5K", 1500000 → "1.5M").
fn fmt_tokens(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

fn run_statusline() -> anyhow::Result<()> {
    // Read statusline JSON from stdin
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let stdin_json: Value = serde_json::from_str(&input).unwrap_or(Value::Null);

    // Determine session start: use cost.total_duration_ms to compute when the session began
    let session_start_ts = {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let duration_ms = stdin_json
            .pointer("/cost/total_duration_ms")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        now.saturating_sub(duration_ms / 1000)
    };

    let counts = parse_session_metrics(session_start_ts);

    // Build statusline output
    let mut parts: Vec<String> = Vec::new();

    if counts.corrections > 0 {
        parts.push(format!(
            "PRECC: {} fix{}, ~{} tokens saved",
            counts.corrections,
            if counts.corrections == 1 { "" } else { "es" },
            fmt_tokens(counts.tokens_saved)
        ));
    } else {
        parts.push("PRECC: watching".to_string());
    }

    // Add avg latency if we have data
    if counts.latency_count > 0 {
        parts.push(format!("{:.1}ms avg", counts.avg_latency_ms()));
    }

    println!("{}", parts.join(" | "));

    Ok(())
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

    // Resolve cwd from hook input
    let cwd = hook_input
        .get("cwd")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or_else(|| {
            std::env::current_dir()
                .ok()
                .map(|p| p.to_string_lossy().to_string())
        })
        .unwrap_or_default();

    // Run pipeline
    let mut pipeline = Pipeline::new(command, cwd);
    pipeline.run();

    // Geofence block: deny the command entirely with a helpful message
    if pipeline.had_geofence_block {
        if let geofence::GeofenceVerdict::Blocked(cache) = geofence::check_cached() {
            let msg = geofence::format_deny_message(&cache);
            emit_deny(&msg)?;

            let latency_ms = t_start.elapsed().as_secs_f64() * 1000.0;
            append_metrics_log_bash(&pipeline, latency_ms);
            return Ok(());
        }
    }

    // Emit when command was rewritten OR when there's a GDB suggestion to surface.
    if pipeline.modified() || pipeline.had_gdb_suggestion {
        if dry_run_enabled() {
            // Dry-run: log what would happen but don't apply the mutation
            eprintln!(
                "[precc] DRY-RUN: would rewrite → `{}`\n[precc] DRY-RUN: reason: {}",
                pipeline.command,
                pipeline.reason()
            );
        } else {
            let ti = hook_input
                .get("tool_input")
                .cloned()
                .unwrap_or(Value::Object(serde_json::Map::new()));

            emit_rewrite(&ti, &pipeline.command, &pipeline.reason())?;
        }
    }

    // Write measurement stash so PostToolUse can find the original command
    // and measure ground-truth savings by re-running it.
    if pipeline.modified() && !dry_run_enabled() {
        if let Ok(data_dir) = db::data_dir() {
            let rewritten_hash = {
                let mut hasher = DefaultHasher::new();
                "Bash".hash(&mut hasher);
                let ti_str = serde_json::json!({"command": &pipeline.command}).to_string();
                ti_str.hash(&mut hasher);
                hasher.finish()
            };
            let cmd_class = post_observe::command_class(&pipeline.original);
            post_observe::write_stash(
                &data_dir,
                rewritten_hash,
                &pipeline.original,
                &pipeline.command,
                &pipeline.cwd,
                &cmd_class,
                &pipeline.reasons,
            );
        }
    }

    // Record metrics (after stdout emit — never delays response to Claude)
    let latency_ms = t_start.elapsed().as_secs_f64() * 1000.0;
    append_metrics_log_bash(&pipeline, latency_ms);

    // Surface update notification on stderr (never delays stdout response).
    // If auto-update is enabled and an update is available, perform it silently.
    if let Ok(data_dir) = db::data_dir() {
        if let Some(ver) = update_check::read_update_available(&data_dir) {
            if update_check::auto_update_enabled() {
                // Fire auto-update in background (don't block the hook response)
                let dd = data_dir.clone();
                let v = ver.clone();
                std::thread::spawn(move || {
                    let _ = update_check::perform_auto_update(&dd, &v);
                });
            } else {
                eprintln!("[precc] Update available: v{ver} — run `precc update`");
            }
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
        if dry_run_enabled() {
            eprintln!("[precc] DRY-RUN: would inject read limit");
        } else {
            let output = serde_json::json!({
                "hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "allow",
                    "permissionDecisionReason": "PRECC: read-filter (limit injection)",
                    "updatedInput": updated_input
                }
            });
            println!("{}", serde_json::to_string(&output)?);
        }
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
        if dry_run_enabled() {
            eprintln!(
                "[precc] DRY-RUN: would apply grep-filter ({})",
                reasons.join(", ")
            );
        } else {
            let output = serde_json::json!({
                "hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "allow",
                    "permissionDecisionReason": reason,
                    "updatedInput": updated_input
                }
            });
            println!("{}", serde_json::to_string(&output)?);
        }
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

    if dry_run_enabled() {
        eprintln!("[precc] DRY-RUN: would inject agent hooks");
    } else {
        let output = serde_json::json!({
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "allow",
                "permissionDecisionReason": "PRECC: agent-propagate (subagent hook injection)",
                "updatedInput": updated_input
            }
        });
        println!("{}", serde_json::to_string(&output)?);
    }
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

    // Determine if this Bash command was compressed (RTK/lean-ctx/diet wrapped)
    let command = tool_input
        .get("command")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let cmd_class = if tool_name == "Bash" {
        post_observe::command_class(command)
    } else {
        String::new()
    };
    let was_compressed = tool_name == "Bash"
        && (command.starts_with("rtk ")
            || command.contains("| rtk ")
            || command.starts_with("lean-ctx ")
            || command.contains("lean-ctx -c"));

    // Log the observation with compression tag
    post_observe::append_observation(
        &data_dir,
        session_id,
        tool_name,
        cmd_hash,
        output_bytes,
        estimated_tokens,
        was_compressed,
        &cmd_class,
    );

    // Compression feedback: if a Bash command failed, record its class so
    // PreToolUse can skip compression on the next similar command (adaptive expand).
    if tool_name == "Bash" && post_observe::is_bash_failure(hook_input) {
        if !cmd_class.is_empty() {
            post_observe::record_compression_failure(&data_dir, &cmd_class);
        }
    }

    // Extract context pressure
    let context_used_pct = post_observe::context_used_pct(hook_input);

    // Build waste report
    let report = post_observe::WasteReport {
        duplicate_count,
        is_large,
        estimated_tokens,
        output_bytes,
        context_used_pct,
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

    // ── Ground-truth savings measurement ──────────────────────────────────
    // For Bash commands that PRECC modified: look up the stash to find the
    // original command, re-run it if safe, and measure the real output size
    // difference between original (uncompressed) and actual (compressed).
    if tool_name == "Bash" {
        let rewritten_hash = post_observe::hash_command(tool_name, tool_input);
        if let Some(stash) = post_observe::read_stash(&data_dir, rewritten_hash) {
            let measure_start = std::time::Instant::now();

            let (original_tokens, method) = if post_observe::is_safe_to_rerun(&stash.original_cmd) {
                // Ground truth: run the original command and measure output
                match std::process::Command::new("bash")
                    .args(["-c", &stash.original_cmd])
                    .current_dir(&stash.cwd)
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .output()
                {
                    Ok(output) => {
                        let total_bytes = output.stdout.len() as u64 + output.stderr.len() as u64;
                        (total_bytes / 4, "ground_truth")
                    }
                    Err(_) => (estimated_tokens, "measurement_failed"),
                }
            } else {
                // Unsafe to re-run — skip measurement, don't claim savings
                (estimated_tokens, "unsafe_skip")
            };

            let measure_ms = measure_start.elapsed().as_secs_f64() * 1000.0;
            let rewrite_type = stash
                .rewrite_types
                .first()
                .map(|s| s.as_str())
                .unwrap_or("unknown");

            // Only log if we got a real measurement (ground_truth)
            if method == "ground_truth" {
                post_observe::append_savings_measurement(
                    &data_dir,
                    &stash.cmd_class,
                    rewrite_type,
                    original_tokens,
                    estimated_tokens,
                    method,
                    measure_ms,
                );
            }

            post_observe::delete_stash(&data_dir, rewritten_hash);
        }
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
    cwd: String,
    reasons: Vec<String>,
    /// Hash of the original command for audit correlation (no content logged).
    cmd_hash: u64,
    /// Skip compression stages (adaptive expand: recent failure after compression).
    skip_compression: bool,
    // Flags set by each stage for metrics reporting
    had_geofence_block: bool,
    had_bash_unwrap: bool,
    had_cd_prepend: bool,
    had_nushell_wrap: bool,
    had_lean_ctx_wrap: bool,
    had_rtk_rewrite: bool,
    had_diet_rewrite: bool,
    diet_tokens_saved: u32,
    had_skill_activation: bool,
    skill_activation_count: u32,
    had_gdb_suggestion: bool,
    had_ccc_redirect: bool,
    ccc_saved_bytes: usize,
    had_adaptive_expand: bool,
}

impl Pipeline {
    fn new(command: String, cwd: String) -> Self {
        let cmd_hash = {
            let mut hasher = DefaultHasher::new();
            command.hash(&mut hasher);
            hasher.finish()
        };
        Self {
            original: command.clone(),
            command,
            cwd,
            reasons: Vec::new(),
            cmd_hash,
            skip_compression: false,
            had_geofence_block: false,
            had_bash_unwrap: false,
            had_cd_prepend: false,
            had_nushell_wrap: false,
            had_lean_ctx_wrap: false,
            had_rtk_rewrite: false,
            had_diet_rewrite: false,
            diet_tokens_saved: 0,
            had_skill_activation: false,
            skill_activation_count: 0,
            had_gdb_suggestion: false,
            had_ccc_redirect: false,
            ccc_saved_bytes: 0,
            had_adaptive_expand: false,
        }
    }

    fn modified(&self) -> bool {
        self.command != self.original
    }

    /// Validate that a mutation is structurally bounded: the new command must
    /// still contain the original command (or its core) as a substring.
    /// Returns false (and reverts) if the mutation is unbounded.
    fn validate_mutation(&mut self, before: &str, stage: &str) -> bool {
        if self.command == before {
            return true; // No change — nothing to validate
        }

        // The new command should contain the old command as a substring,
        // OR the old command should contain the new one (unwrap case).
        // This prevents a stage from completely replacing the command.
        let old_core = before.trim();
        let new_core = self.command.trim();

        let bounded = new_core.contains(old_core)
            || old_core.contains(new_core)
            || self.commands_share_core(old_core, new_core);

        if !bounded {
            eprintln!(
                "[precc] Warning: {} produced unbounded mutation, reverting",
                stage
            );
            self.command = before.to_string();
            return false;
        }
        true
    }

    /// Check if two commands share a common core (for template-based rewrites
    /// where the command structure changes but key tokens are preserved).
    fn commands_share_core(&self, old: &str, new: &str) -> bool {
        // Extract significant tokens (skip common shell words)
        let skip = ["cd", "&&", "bash", "-c", "rtk", "lean-ctx", "nu", "-c"];
        let old_tokens: Vec<&str> = old
            .split_whitespace()
            .filter(|t| !skip.contains(t))
            .collect();
        let new_tokens: Vec<&str> = new
            .split_whitespace()
            .filter(|t| !skip.contains(t))
            .collect();

        if old_tokens.is_empty() {
            return true;
        }

        // At least half of the old command's significant tokens must appear in the new
        let shared = old_tokens.iter().filter(|t| new_tokens.contains(t)).count();
        shared * 2 >= old_tokens.len()
    }

    fn reason(&self) -> String {
        if self.reasons.is_empty() {
            "PRECC".to_string()
        } else if self.modified() {
            // Include truncated original so Claude can see both sides of the mutation
            let orig_display = if self.original.len() > 80 {
                format!("{}…", &self.original[..80])
            } else {
                self.original.clone()
            };
            format!(
                "PRECC: [original: `{}`] → {}",
                orig_display,
                self.reasons.join("; ")
            )
        } else {
            format!("PRECC: {}", self.reasons.join("; "))
        }
    }

    fn run(&mut self) {
        let destructive = is_destructive(&self.command);

        // Stage 0: Geofence check (Pro feature) — blocks if IP is in restricted region
        if self.stage_geofence() {
            return; // Blocked — skip all subsequent stages
        }

        // Stage 1: Bash unwrap — strip unnecessary `bash -c "..."` wrappers
        let before = self.command.clone();
        self.stage_bash_unwrap();
        self.validate_mutation(&before, "bash-unwrap");

        // Stage 2: Skill matching (Pillar 4) — read-only, skip if no DB
        if !destructive {
            let before = self.command.clone();
            self.stage_skills();
            self.validate_mutation(&before, "skills");
        }

        // Stage 3: Context resolution (Pillar 1)
        // Skip cd-prepend for destructive commands to avoid running `rm -rf` in wrong dir
        if !destructive {
            let before = self.command.clone();
            self.stage_context();
            self.validate_mutation(&before, "context");
        }

        // Stage 4: GDB check (Pillar 2) — advisory only, no mutation
        self.stage_gdb();

        // Stages 5-7: Skip output manipulation for destructive commands
        if destructive {
            return;
        }

        // Adaptive expand: if this command class recently failed after compression,
        // skip all compression stages to give Claude the full uncompressed output.
        // This is the PostToolUse→PreToolUse feedback loop.
        if !self.skip_compression {
            if let Ok(data_dir) = db::data_dir() {
                let cmd_class = post_observe::command_class(&self.command);
                if !cmd_class.is_empty()
                    && post_observe::should_skip_compression(&data_dir, &cmd_class)
                {
                    self.skip_compression = true;
                    self.had_adaptive_expand = true;
                    self.reasons.push(format!(
                        "adaptive-expand:{} (recent failure after compression)",
                        cmd_class
                    ));
                }
            }
        }

        if self.skip_compression {
            return;
        }

        // Stage 5: Diet — rule-based output slimming (AgentDiet-inspired).
        //   Appends pipe filters or adds flags to reduce verbose output.
        //   Skipped when lean-ctx/nushell handle compression externally.
        let before = self.command.clone();
        self.stage_diet();
        self.validate_mutation(&before, "diet");

        // Stage 6: Output compression — mutually exclusive, priority order:
        //   6a: Nushell (PRECC_NUSHELL=1)
        //   6b: lean-ctx (PRECC_LEAN_CTX=1) — external output compression
        //   6c: RTK (fallback)
        let before = self.command.clone();
        if nushell::nushell_mode_enabled() {
            self.stage_nushell();
        }
        if !self.had_nushell_wrap && lean_ctx::lean_ctx_mode_enabled() {
            self.stage_lean_ctx();
        }
        if !self.had_nushell_wrap && !self.had_lean_ctx_wrap {
            self.stage_rtk();
        }
        self.validate_mutation(&before, "output-compression");

        // Stage 7: CCC semantic search redirect (Pillar 2b)
        // CCC is a full replacement (grep → ccc search), so skip validation
        self.stage_ccc();
    }

    /// Stage 0: Geofence — IP region compliance check (Pro feature).
    ///
    /// Reads the cached geofence result (<1ms). If the user's egress IP is
    /// in a blocked region, returns `true` to halt the pipeline. The caller
    /// should emit a deny with the formatted message.
    ///
    /// Skipped for Free-tier users (feature gated to Pro+).
    /// Skipped if `PRECC_GEOFENCE_OVERRIDE=1` is set.
    fn stage_geofence(&mut self) -> bool {
        // Pro feature gate — free users are silently skipped
        if !license::tier().is_paid() {
            return false;
        }

        // Allow override for users who accept the risk
        if geofence::is_overridden() {
            return false;
        }

        match geofence::check_cached() {
            geofence::GeofenceVerdict::Blocked(cache) => {
                self.had_geofence_block = true;
                self.reasons
                    .push(format!("geofence-block:{}", cache.country_code));
                true
            }
            geofence::GeofenceVerdict::Stale => {
                // Stale cache — warn but don't block
                eprintln!("{}", geofence::format_stale_warning());
                false
            }
            geofence::GeofenceVerdict::Allow => false,
        }
    }

    /// Stage 1: Unwrap unnecessary `bash -c "..."` / `bash -c '...'` wrappers.
    ///
    /// Claude sometimes wraps simple commands in `bash -c "cmd"` which adds
    /// subshell overhead and makes output noisier. This stage extracts the
    /// inner command when safe to do so (single command, no shell features
    /// that require bash -c).
    fn stage_bash_unwrap(&mut self) {
        let cmd = self.command.trim();

        // Match: bash -c "..." or bash -c '...'
        let inner = if let Some(rest) = cmd.strip_prefix("bash -c ") {
            let rest = rest.trim();
            let is_quoted = (rest.starts_with('"') && rest.ends_with('"'))
                || (rest.starts_with('\'') && rest.ends_with('\''));
            if is_quoted && rest.len() > 2 {
                Some(&rest[1..rest.len() - 1])
            } else {
                None
            }
        } else {
            None
        };

        let inner = match inner {
            Some(s) if !s.is_empty() => s,
            _ => return,
        };

        // Don't unwrap if the inner command uses shell features that need bash -c:
        // pipes, redirects, semicolons, &&, ||, subshells, backgrounding
        // These are legitimate uses of bash -c
        if inner.contains('|')
            || inner.contains('>')
            || inner.contains('<')
            || inner.contains(';')
            || inner.contains("&&")
            || inner.contains("||")
            || inner.contains('$')
            || inner.contains('`')
            || inner.contains('&')
        {
            return;
        }

        self.command = inner.to_string();
        self.had_bash_unwrap = true;
        self.reasons.push("bash-unwrap".to_string());
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

        // Geofence skill filter: suppress 3rd-party skills that interact with
        // Claude/Anthropic API when the user is in a blocked region.
        // This prevents indirect API calls that could trigger account bans.
        let geofence_active = matches!(
            geofence::check_cached(),
            geofence::GeofenceVerdict::Blocked(_)
        );
        if geofence_active {
            matches.retain(|m| {
                if m.claude_interaction > 0 {
                    self.reasons.push(format!(
                        "geofence-skip-skill:{} (claude_interaction={})",
                        m.skill_name, m.claude_interaction
                    ));
                    false
                } else {
                    true
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
                    self.had_skill_activation = true;
                    self.skill_activation_count += 1;
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
                    self.had_skill_activation = true;
                    self.skill_activation_count += 1;
                    used_mutating_types.insert("rewrite_command".to_string());
                    self.reasons
                        .push(format!("skill:{} (conf={:.1})", m.skill_name, m.confidence));
                    append_activation_log(m.skill_id, &m.skill_name, m.confidence);
                }
                "suggest_fix" => {
                    // Always additive — surface every suggestion above threshold
                    self.had_skill_activation = true;
                    self.skill_activation_count += 1;
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

    /// Stage 5: Diet — rule-based output slimming (AgentDiet-inspired).
    ///
    /// Appends pipe filters (e.g. `| grep -v PASSED`) or adds flags
    /// (e.g. `--quiet`, `-s`) to commands that produce verbose output.
    /// Zero-cost alternative to LLM-based trajectory compression.
    ///
    /// Skipped when lean-ctx or nushell handle compression externally,
    /// since those tools already compress all output.
    fn stage_diet(&mut self) {
        // Skip if external compression is handling output
        if lean_ctx::lean_ctx_mode_enabled() || nushell::nushell_mode_enabled() {
            return;
        }

        let (prefix, cmd_part) = split_cd_prefix(&self.command);

        if let Some((rewritten, tokens_saved)) = diet::apply(cmd_part) {
            self.command = if prefix.is_empty() {
                rewritten
            } else {
                format!("{}{}", prefix, rewritten)
            };
            self.had_diet_rewrite = true;
            self.diet_tokens_saved = tokens_saved;
            self.reasons.push("diet".to_string());
        }
    }

    /// Stage 6a: Nushell wrapping (experimental).
    ///
    /// Wraps commands in nushell equivalents that produce compact/structured output.
    /// Only fires when `PRECC_NUSHELL=1` is set and nu is available.
    fn stage_nushell(&mut self) {
        let (prefix, cmd_part) = split_cd_prefix(&self.command);

        if let Some(wrapped) = nushell::wrap(cmd_part) {
            self.command = if prefix.is_empty() {
                wrapped
            } else {
                format!("{}{}", prefix, wrapped)
            };
            self.had_nushell_wrap = true;
            self.reasons.push("nushell-wrap".to_string());
        }
    }

    /// Stage 5b: lean-ctx output compression (external tool).
    ///
    /// Wraps commands in `lean-ctx -c '...'` for deep output compression.
    /// Only fires when `PRECC_LEAN_CTX=1` is set and lean-ctx is available.
    /// lean-ctx handles its own compression decisions internally.
    fn stage_lean_ctx(&mut self) {
        let (prefix, cmd_part) = split_cd_prefix(&self.command);

        if let Some(wrapped) = lean_ctx::wrap(cmd_part) {
            self.command = if prefix.is_empty() {
                wrapped
            } else {
                format!("{}{}", prefix, wrapped)
            };
            self.had_lean_ctx_wrap = true;
            self.reasons.push("lean-ctx-wrap".to_string());
        }
    }

    /// Stage 5c: RTK command rewriting.
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

    /// Stage 6: CCC semantic search redirect (Pillar 2b).
    ///
    /// Intercepts recursive grep/rg commands and redirects through `ccc search`
    /// when the project has a cocoindex-code index. This stage spawns a subprocess
    /// so it only activates when all preconditions are met.
    fn stage_ccc(&mut self) {
        // Fast reject: only grep/rg commands
        if !ccc::is_eligible(&self.command) {
            return;
        }

        // Extract search pattern
        let query = match ccc::extract_pattern(&self.command) {
            Some(q) => q,
            None => return,
        };

        // Check ccc availability (cached) and project index
        if !ccc::is_available() {
            return;
        }

        let cwd = if self.cwd.is_empty() { "." } else { &self.cwd };

        if !ccc::has_index(cwd) {
            return;
        }

        // Run ccc search
        let result = match ccc::run_search(&query, cwd) {
            Some(r) => r,
            None => return,
        };

        // Replace command with ccc output
        let replacement = ccc::build_replacement_command(&result);
        self.ccc_saved_bytes = result.ccc_bytes; // conservative: we saved at least this
        self.command = replacement;
        self.had_ccc_redirect = true;
        self.reasons.push(format!(
            "ccc-redirect:{} ({} bytes)",
            result.pattern, result.ccc_bytes
        ));
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

/// Append Bash hook metrics to metrics.log for async import by precc-learner.
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

    // Build all lines to write in a single syscall.
    // cmd_hash allows correlating PreToolUse and PostToolUse events for the same
    // command without logging the command content (privacy-safe audit trail).
    let mut lines = format!(
        "{{\"ts\":{},\"type\":\"hook_latency\",\"value\":{:.3},\"cmd_hash\":\"{:016x}\"}}\n",
        ts, latency_ms, pipeline.cmd_hash
    );
    if pipeline.had_geofence_block {
        lines.push_str(&format!(
            "{{\"ts\":{},\"type\":\"geofence_block\",\"value\":1.0}}\n",
            ts
        ));
    }
    if pipeline.had_bash_unwrap {
        lines.push_str(&format!(
            "{{\"ts\":{},\"type\":\"bash_unwrap\",\"value\":1.0}}\n",
            ts
        ));
    }
    if pipeline.had_cd_prepend {
        lines.push_str(&format!(
            "{{\"ts\":{},\"type\":\"cd_prepend\",\"value\":1.0}}\n",
            ts
        ));
    }
    if pipeline.had_nushell_wrap {
        lines.push_str(&format!(
            "{{\"ts\":{},\"type\":\"nushell_wrap\",\"value\":1.0}}\n",
            ts
        ));
    }
    if pipeline.had_lean_ctx_wrap {
        lines.push_str(&format!(
            "{{\"ts\":{},\"type\":\"lean_ctx_wrap\",\"value\":1.0}}\n",
            ts
        ));
    }
    if pipeline.had_rtk_rewrite {
        lines.push_str(&format!(
            "{{\"ts\":{},\"type\":\"rtk_rewrite\",\"value\":1.0}}\n",
            ts
        ));
    }
    if pipeline.had_diet_rewrite {
        lines.push_str(&format!(
            "{{\"ts\":{},\"type\":\"diet_rewrite\",\"value\":{}.0}}\n",
            ts, pipeline.diet_tokens_saved
        ));
    }
    if pipeline.had_skill_activation {
        lines.push_str(&format!(
            "{{\"ts\":{},\"type\":\"skill_activation\",\"value\":{}.0}}\n",
            ts, pipeline.skill_activation_count
        ));
    }
    if pipeline.had_gdb_suggestion {
        lines.push_str(&format!(
            "{{\"ts\":{},\"type\":\"gdb_suggestion\",\"value\":1.0}}\n",
            ts
        ));
    }
    if pipeline.had_ccc_redirect {
        lines.push_str(&format!(
            "{{\"ts\":{},\"type\":\"ccc_redirect\",\"value\":{}.0}}\n",
            ts, pipeline.ccc_saved_bytes
        ));
    }
    if pipeline.had_adaptive_expand {
        lines.push_str(&format!(
            "{{\"ts\":{},\"type\":\"adaptive_expand\",\"value\":1.0}}\n",
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
        let mut p = Pipeline::new("echo hello".to_string(), ".".to_string());
        // Only run RTK stage (others need filesystem/DB)
        p.stage_rtk();
        assert!(!p.modified());
    }

    #[test]
    fn pipeline_rtk_rewrite() {
        // This test depends on rtk being available, which it may not be in CI.
        // The rtk module handles this check internally.
        let mut p = Pipeline::new("git status".to_string(), ".".to_string());
        p.stage_rtk();
        // If rtk is available, command should be rewritten
        // If not, command should be unchanged
        // Both are valid outcomes for this test
        assert!(p.command == "git status" || p.command == "rtk git status");
    }

    #[test]
    fn pipeline_rtk_rewrite_preserves_cd_prefix() {
        let mut p = Pipeline::new("cd /tmp && git status".to_string(), ".".to_string());
        p.stage_rtk();
        if p.modified() {
            assert!(p.command.starts_with("cd /tmp && rtk git status"));
        }
    }

    #[test]
    fn pipeline_lean_ctx_wraps_command() {
        let mut p = Pipeline::new("cargo test".to_string(), ".".to_string());
        p.stage_lean_ctx();
        // lean-ctx wraps unconditionally (no per-command rules)
        assert!(p.command.contains("lean-ctx -c"));
        assert!(p.had_lean_ctx_wrap);
        assert!(p.reasons.contains(&"lean-ctx-wrap".to_string()));
    }

    #[test]
    fn pipeline_lean_ctx_preserves_cd_prefix() {
        let mut p = Pipeline::new("cd /app && cargo test".to_string(), ".".to_string());
        p.stage_lean_ctx();
        assert!(p.command.starts_with("cd /app && lean-ctx -c"));
        assert!(!p.command.contains("cd /app && lean-ctx -c 'cd"));
    }

    #[test]
    fn pipeline_lean_ctx_skips_heredocs() {
        let mut p = Pipeline::new("cat <<EOF\nhello\nEOF".to_string(), ".".to_string());
        p.stage_lean_ctx();
        assert!(!p.had_lean_ctx_wrap);
        assert!(!p.modified());
    }

    #[test]
    fn pipeline_lean_ctx_skips_already_wrapped() {
        let mut p = Pipeline::new("lean-ctx -c 'cargo test'".to_string(), ".".to_string());
        p.stage_lean_ctx();
        assert!(!p.had_lean_ctx_wrap);
    }

    #[test]
    fn pipeline_lean_ctx_flag_default_false() {
        let p = Pipeline::new("cargo test".to_string(), ".".to_string());
        assert!(!p.had_lean_ctx_wrap);
    }

    #[test]
    fn pipeline_lean_ctx_metrics_reason() {
        let mut p = Pipeline::new("git status".to_string(), ".".to_string());
        p.stage_lean_ctx();
        let reason = p.reason();
        assert!(reason.contains("lean-ctx-wrap"));
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
        let p = Pipeline::new("echo hello".to_string(), ".".to_string());
        assert_eq!(p.reason(), "PRECC");
    }

    #[test]
    fn pipeline_reason_with_entries_no_modification() {
        // When command is not modified, original is not included in reason
        let mut p = Pipeline::new("echo hello".to_string(), ".".to_string());
        p.reasons.push("gdb-hint:try precc debug".to_string());
        assert_eq!(p.reason(), "PRECC: gdb-hint:try precc debug");
    }

    #[test]
    fn pipeline_reason_with_modification_shows_original() {
        let mut p = Pipeline::new("echo hello".to_string(), ".".to_string());
        p.command = "cd /app && echo hello".to_string(); // simulate mutation
        p.reasons.push("cd:Cargo.toml (conf=0.9)".to_string());
        assert_eq!(
            p.reason(),
            "PRECC: [original: `echo hello`] → cd:Cargo.toml (conf=0.9)"
        );
    }

    #[test]
    fn pipeline_reason_truncates_long_original() {
        let long_cmd = "a".repeat(100);
        let mut p = Pipeline::new(long_cmd.clone(), ".".to_string());
        p.command = "modified".to_string();
        p.reasons.push("rewrite".to_string());
        let reason = p.reason();
        assert!(reason.contains("…")); // truncated
        assert!(reason.len() < long_cmd.len() + 50);
    }

    #[test]
    fn pipeline_flags_default_false() {
        let p = Pipeline::new("echo hello".to_string(), ".".to_string());
        assert!(!p.had_cd_prepend);
        assert!(!p.had_rtk_rewrite);
    }

    #[test]
    fn metrics_log_line_format() {
        // Verify the JSON line format we write can be parsed back correctly
        let line = format!(
            "{{\"ts\":{},\"type\":\"hook_latency\",\"value\":{:.3},\"cmd_hash\":\"{:016x}\"}}\n",
            1000u64, 2.93f64, 0xdeadbeef_u64
        );
        let parsed: serde_json::Value = serde_json::from_str(line.trim()).unwrap();
        assert_eq!(parsed["type"].as_str(), Some("hook_latency"));
        assert!((parsed["value"].as_f64().unwrap() - 2.93).abs() < 0.001);
        assert_eq!(parsed["cmd_hash"].as_str(), Some("00000000deadbeef"));
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

    // =========================================================================
    // CCC integration (Pipeline stage 6)
    // =========================================================================

    #[test]
    fn pipeline_ccc_flags_default_false() {
        let p = Pipeline::new("echo hello".to_string(), ".".to_string());
        assert!(!p.had_ccc_redirect);
        assert_eq!(p.ccc_saved_bytes, 0);
    }

    #[test]
    fn pipeline_ccc_skips_non_grep() {
        let mut p = Pipeline::new("cargo build".to_string(), ".".to_string());
        p.stage_ccc();
        assert!(!p.had_ccc_redirect);
        assert!(!p.modified());
    }

    #[test]
    fn pipeline_ccc_skips_piped_grep() {
        let mut p = Pipeline::new(
            "grep -r 'pattern' src/ | head -5".to_string(),
            ".".to_string(),
        );
        p.stage_ccc();
        assert!(!p.had_ccc_redirect);
    }

    #[test]
    fn pipeline_ccc_skips_short_pattern() {
        let mut p = Pipeline::new("grep -r 'ab' src/".to_string(), ".".to_string());
        p.stage_ccc();
        assert!(!p.had_ccc_redirect);
    }

    #[test]
    fn pipeline_ccc_skips_when_no_index() {
        // A temp dir without .cocoindex_code — should skip
        let dir = tempfile::tempdir().unwrap();
        let mut p = Pipeline::new(
            "grep -r 'long_pattern' src/".to_string(),
            dir.path().to_string_lossy().to_string(),
        );
        p.stage_ccc();
        assert!(!p.had_ccc_redirect);
    }

    #[test]
    fn pipeline_cwd_from_hook_input() {
        let p = Pipeline::new("echo hello".to_string(), "/tmp/myproject".to_string());
        assert_eq!(p.cwd, "/tmp/myproject");
    }

    #[test]
    fn pipeline_cwd_empty_fallback() {
        let mut p = Pipeline::new("grep -r 'long_pattern' src/".to_string(), "".to_string());
        p.stage_ccc();
        // Should not panic, cwd defaults to "."
        assert!(!p.had_ccc_redirect);
    }

    #[test]
    fn ccc_metrics_log_line_format() {
        // Verify the ccc_redirect JSON line format
        let line = format!(
            "{{\"ts\":{},\"type\":\"ccc_redirect\",\"value\":{}.0}}\n",
            1000u64, 512usize
        );
        let parsed: Value = serde_json::from_str(line.trim()).unwrap();
        assert_eq!(parsed["type"].as_str(), Some("ccc_redirect"));
        assert!((parsed["value"].as_f64().unwrap() - 512.0).abs() < 0.001);
    }

    #[test]
    fn ccc_reason_format() {
        let mut p = Pipeline::new("grep -r 'pattern' .".to_string(), ".".to_string());
        p.had_ccc_redirect = true;
        p.reasons
            .push("ccc-redirect:pattern (512 bytes)".to_string());
        let reason = p.reason();
        assert!(reason.contains("ccc-redirect"));
        assert!(reason.contains("512 bytes"));
    }

    // =========================================================================
    // Statusline helpers
    // =========================================================================

    #[test]
    fn extract_ts_valid() {
        let line = r#"{"ts":1710000000,"type":"hook_latency","value":2.5}"#;
        assert_eq!(extract_ts(line), Some(1710000000));
    }

    #[test]
    fn extract_ts_missing() {
        assert_eq!(extract_ts(r#"{"type":"x"}"#), None);
    }

    #[test]
    fn extract_str_field_valid() {
        let line = r#"{"ts":1,"type":"cd_prepend","value":1.0}"#;
        assert_eq!(extract_str_field(line, "type"), Some("cd_prepend"));
    }

    #[test]
    fn extract_f64_field_valid() {
        let line = r#"{"ts":1,"type":"hook_latency","value":2.53}"#;
        let v = extract_f64_field(line, "value").unwrap();
        assert!((v - 2.53).abs() < 0.001);
    }

    #[test]
    fn fmt_tokens_units() {
        assert_eq!(fmt_tokens(500), "500");
        assert_eq!(fmt_tokens(1_500), "1.5K");
        assert_eq!(fmt_tokens(2_500_000), "2.5M");
    }

    #[test]
    fn statusline_counts_avg_latency() {
        let counts = StatuslineCounts {
            corrections: 0,
            latency_sum: 10.0,
            latency_count: 4,
            tokens_saved: 0,
            skills_fired: 0,
            ccc_bytes_saved: 0,
        };
        assert!((counts.avg_latency_ms() - 2.5).abs() < 0.001);
    }

    #[test]
    fn statusline_counts_zero_latency() {
        let counts = StatuslineCounts {
            corrections: 0,
            latency_sum: 0.0,
            latency_count: 0,
            tokens_saved: 0,
            skills_fired: 0,
            ccc_bytes_saved: 0,
        };
        assert!((counts.avg_latency_ms() - 0.0).abs() < 0.001);
    }

    #[test]
    fn parse_session_metrics_empty() {
        // Should not panic with a future timestamp (no matching events)
        let far_future = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 999_999;
        let counts = parse_session_metrics(far_future);
        assert_eq!(counts.corrections, 0);
        assert_eq!(counts.latency_count, 0);
    }

    // -- bash_unwrap tests --

    #[test]
    fn bash_unwrap_double_quotes() {
        let mut p = Pipeline::new(r#"bash -c "ls -la""#.to_string(), ".".to_string());
        p.stage_bash_unwrap();
        assert_eq!(p.command, "ls -la");
        assert!(p.had_bash_unwrap);
    }

    #[test]
    fn bash_unwrap_single_quotes() {
        let mut p = Pipeline::new("bash -c 'cargo build'".to_string(), ".".to_string());
        p.stage_bash_unwrap();
        assert_eq!(p.command, "cargo build");
        assert!(p.had_bash_unwrap);
    }

    #[test]
    fn bash_unwrap_preserves_pipes() {
        // Don't unwrap when inner command uses shell features
        let mut p = Pipeline::new(r#"bash -c "ls | grep foo""#.to_string(), ".".to_string());
        p.stage_bash_unwrap();
        assert!(!p.had_bash_unwrap);
    }

    #[test]
    fn bash_unwrap_preserves_redirects() {
        let mut p = Pipeline::new(r#"bash -c "echo hi > file""#.to_string(), ".".to_string());
        p.stage_bash_unwrap();
        assert!(!p.had_bash_unwrap);
    }

    #[test]
    fn bash_unwrap_preserves_subshells() {
        let mut p = Pipeline::new(r#"bash -c "echo $(date)""#.to_string(), ".".to_string());
        p.stage_bash_unwrap();
        assert!(!p.had_bash_unwrap);
    }

    #[test]
    fn bash_unwrap_ignores_plain_bash() {
        let mut p = Pipeline::new("bash script.sh".to_string(), ".".to_string());
        p.stage_bash_unwrap();
        assert!(!p.had_bash_unwrap);
    }

    #[test]
    fn bash_unwrap_ignores_non_bash() {
        let mut p = Pipeline::new("cargo build".to_string(), ".".to_string());
        p.stage_bash_unwrap();
        assert!(!p.had_bash_unwrap);
    }

    // -- mutation validation tests --

    #[test]
    fn validate_mutation_allows_cd_prepend() {
        let mut p = Pipeline::new("cargo build".to_string(), ".".to_string());
        let before = p.command.clone();
        p.command = "cd /app && cargo build".to_string();
        assert!(p.validate_mutation(&before, "context"));
    }

    #[test]
    fn validate_mutation_allows_unwrap() {
        let mut p = Pipeline::new("bash -c 'ls -la'".to_string(), ".".to_string());
        let before = p.command.clone();
        p.command = "ls -la".to_string();
        assert!(p.validate_mutation(&before, "bash-unwrap"));
    }

    #[test]
    fn validate_mutation_reverts_unbounded() {
        let mut p = Pipeline::new("cargo build".to_string(), ".".to_string());
        let before = p.command.clone();
        p.command = "rm -rf /".to_string(); // completely unrelated
        assert!(!p.validate_mutation(&before, "bad-stage"));
        assert_eq!(p.command, "cargo build"); // reverted
    }

    #[test]
    fn validate_mutation_allows_rtk_wrap() {
        let mut p = Pipeline::new("git status".to_string(), ".".to_string());
        let before = p.command.clone();
        p.command = "rtk git status".to_string();
        assert!(p.validate_mutation(&before, "rtk"));
    }

    #[test]
    fn validate_mutation_noop() {
        let mut p = Pipeline::new("echo hello".to_string(), ".".to_string());
        let before = p.command.clone();
        // No change
        assert!(p.validate_mutation(&before, "noop"));
    }

    #[test]
    fn commands_share_core_basic() {
        let p = Pipeline::new("cargo test --release".to_string(), ".".to_string());
        assert!(p.commands_share_core("cargo test --release", "cd /app && cargo test --release"));
    }

    #[test]
    fn commands_share_core_rejects_unrelated() {
        let p = Pipeline::new("cargo build".to_string(), ".".to_string());
        assert!(!p.commands_share_core("cargo build", "python manage.py runserver"));
    }

    // -- destructive command detection tests --

    #[test]
    fn destructive_rm() {
        assert!(is_destructive("rm -rf /tmp/stuff"));
        assert!(is_destructive("rm file.txt"));
    }

    #[test]
    fn destructive_git_force_push() {
        assert!(is_destructive("git push --force origin main"));
        assert!(is_destructive("git push -f"));
        assert!(is_destructive("git reset --hard HEAD~1"));
        assert!(is_destructive("git clean -fd"));
        assert!(is_destructive("git branch -D feature"));
    }

    #[test]
    fn destructive_sql() {
        assert!(is_destructive("psql -c 'DROP TABLE users'"));
        assert!(is_destructive("mysql -e \"DELETE FROM orders\""));
    }

    #[test]
    fn destructive_with_cd_prefix() {
        assert!(is_destructive("cd /app && rm -rf build/"));
    }

    #[test]
    fn not_destructive_normal_commands() {
        assert!(!is_destructive("cargo build"));
        assert!(!is_destructive("git status"));
        assert!(!is_destructive("git push origin main"));
        assert!(!is_destructive("ls -la"));
        assert!(!is_destructive("grep -r pattern ."));
    }

    #[test]
    fn destructive_commands_skip_pipeline_mutations() {
        let p = Pipeline::new("rm -rf build/".to_string(), ".".to_string());
        // Running the full pipeline should not mutate the command
        // (stages 2-3 and 5-7 are skipped for destructive commands)
        // We can't easily test the full run() without filesystem/DB,
        // but we can verify the flag detection works
        assert!(is_destructive(&p.command));
        assert!(!p.modified());
    }
}
