//! Pillar 3: Failure pattern learning — JSONL session log mining.
//!
//! Parses Claude Code session logs to extract failure-fix pairs
//! and stores them in history.db.

use anyhow::{Context, Result};
use rusqlite::Connection;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::Path;

/// A tool use event extracted from a session.
#[derive(Debug, Clone)]
pub struct ToolEvent {
    pub tool: String,
    pub command: Option<String>,
    pub output: Option<String>,
    pub exit_code: Option<i32>,
    pub is_failure: bool,
}

/// A failure-fix pair detected during mining.
#[derive(Debug)]
pub struct FailureFixPair {
    pub failure_command: String,
    pub failure_output: String,
    pub fix_command: String,
    pub pattern_hash: String,
    pub project_type: Option<String>,
}

/// Result of mining a single session.
#[derive(Debug)]
pub enum MineResult {
    /// Session was already mined and `force` was not set.
    Skipped,
    /// Session was processed, with this many failure-fix pairs found.
    Processed { pairs: usize, events: usize },
}

/// Mine a single JSONL session file and insert results into history.db.
///
/// If `force` is true, any existing record for this session is deleted before
/// re-mining, allowing the session to be processed again from scratch.
pub fn mine_session(conn: &Connection, session_path: &Path, force: bool) -> Result<MineResult> {
    let session_id = session_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Check if already mined
    let already_mined: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM sessions WHERE session_id = ?1",
        [&session_id],
        |r| r.get(0),
    )?;

    if already_mined {
        if force {
            // Delete orphaned events first (no ON DELETE CASCADE in schema),
            // then delete the session row.
            conn.execute(
                "DELETE FROM events WHERE session_id = (SELECT id FROM sessions WHERE session_id = ?1)",
                [&session_id],
            )?;
            conn.execute("DELETE FROM sessions WHERE session_id = ?1", [&session_id])?;
        } else {
            return Ok(MineResult::Skipped);
        }
    }

    let content = std::fs::read_to_string(session_path)
        .with_context(|| format!("failed to read {}", session_path.display()))?;

    let events = extract_tool_events(&content);
    let pairs = find_failure_fix_pairs(&events);

    // Insert session record regardless of whether events were found,
    // so empty sessions are not rescanned on every --all run.
    let project = detect_project_from_path(session_path);
    let now = crate::skills::chrono_now();

    conn.execute(
        "INSERT INTO sessions (session_id, project, started_at, mined_at)
         VALUES (?1, ?2, ?3, ?3)",
        rusqlite::params![session_id, project, now],
    )?;
    let db_session_id = conn.last_insert_rowid();

    if events.is_empty() {
        return Ok(MineResult::Processed {
            pairs: 0,
            events: 0,
        });
    }

    // Insert events
    for event in &events {
        conn.execute(
            "INSERT INTO events (session_id, timestamp, tool, input_json, output_json, exit_code, is_failure)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                db_session_id,
                now,
                event.tool,
                event.command.as_deref().unwrap_or(""),
                event.output.as_deref(),
                event.exit_code,
                event.is_failure,
            ],
        )?;
    }

    // Insert failure-fix pairs
    let mut count = 0;
    for pair in &pairs {
        // Upsert: increment occurrences if pattern already exists
        let existing: Option<i64> = conn
            .query_row(
                "SELECT id FROM failure_fix_pairs WHERE pattern_hash = ?1",
                [&pair.pattern_hash],
                |r| r.get(0),
            )
            .ok();

        if let Some(id) = existing {
            conn.execute(
                "UPDATE failure_fix_pairs SET occurrences = occurrences + 1, updated_at = ?2 WHERE id = ?1",
                rusqlite::params![id, now],
            )?;
        } else {
            conn.execute(
                "INSERT INTO failure_fix_pairs
                 (failure_event, fix_event, pattern_hash, failure_command, failure_output, fix_command, project_type, created_at, updated_at)
                 VALUES (NULL, NULL, ?1, ?2, ?3, ?4, ?5, ?6, ?6)",
                rusqlite::params![
                    pair.pattern_hash,
                    pair.failure_command,
                    pair.failure_output,
                    pair.fix_command,
                    pair.project_type,
                    now,
                ],
            )?;
        }
        count += 1; // count every matched pair (insert or upsert)
    }

    Ok(MineResult::Processed {
        pairs: count,
        events: events.len(),
    })
}

/// Find all unmined session JSONL files under `~/.claude/projects/`.
pub fn find_session_files() -> Result<Vec<std::path::PathBuf>> {
    let home = std::env::var("HOME").context("HOME not set")?;
    let projects_dir = std::path::PathBuf::from(&home).join(".claude/projects");

    if !projects_dir.exists() {
        return Ok(Vec::new());
    }

    let mut files = Vec::new();
    collect_jsonl_files(&projects_dir, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_jsonl_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) -> Result<()> {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Ok(()),
    };

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_jsonl_files(&path, out)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("jsonl") {
            out.push(path);
        }
    }

    Ok(())
}

/// Per-tool byte usage statistics.
#[derive(Debug, Clone, Default)]
pub struct ToolBytes {
    pub input_bytes: u64,
    pub output_bytes: u64,
    pub invocations: u64,
}

/// Token breakdown across all tool types and assistant text in session logs.
#[derive(Debug, Clone, Default)]
pub struct SessionTokenBreakdown {
    /// Per-tool stats keyed by tool name (Bash, Read, Grep, Glob, Edit, Write, etc.)
    pub tools: HashMap<String, ToolBytes>,
    /// Bytes of assistant reasoning text (non-tool content blocks)
    pub assistant_text_bytes: u64,
    /// Bytes of user text (non-tool content blocks)
    pub user_text_bytes: u64,
    /// Bytes of extended thinking blocks
    pub thinking_bytes: u64,
    /// Total raw bytes across all session files
    pub total_raw_bytes: u64,
}

impl SessionTokenBreakdown {
    /// Convenience: get ToolBytes for a specific tool.
    pub fn tool(&self, name: &str) -> ToolBytes {
        self.tools.get(name).cloned().unwrap_or_default()
    }

    /// Total bytes across all tools (input + output).
    pub fn total_tool_bytes(&self) -> u64 {
        self.tools
            .values()
            .map(|t| t.input_bytes + t.output_bytes)
            .sum()
    }

    /// Total API-relevant content bytes (tools + text + thinking).
    /// Excludes progress events, snapshots, metadata, and JSON framing.
    pub fn api_relevant_bytes(&self) -> u64 {
        self.total_tool_bytes()
            + self.assistant_text_bytes
            + self.user_text_bytes
            + self.thinking_bytes
    }
}

/// Compute a full token breakdown across all session JSONL files.
///
/// Scans every content block in every session and attributes bytes to:
/// - Tool input (by tool name)
/// - Tool output / tool_result (by tool name)
/// - Assistant text (reasoning)
/// - User text
pub fn session_token_breakdown() -> Result<SessionTokenBreakdown> {
    session_token_breakdown_since(None)
}

/// Compute token breakdown, optionally filtering to sessions modified after `since`.
/// When `since` is Some(timestamp), only files with mtime >= timestamp are counted.
/// This ensures the denominator (API tokens) matches the numerator (savings) period.
pub fn session_token_breakdown_since(
    since: Option<std::time::SystemTime>,
) -> Result<SessionTokenBreakdown> {
    let files = find_session_files()?;
    let mut breakdown = SessionTokenBreakdown::default();

    for path in &files {
        if let Some(since_ts) = since {
            if let Ok(meta) = std::fs::metadata(&path) {
                if let Ok(mtime) = meta.modified() {
                    if mtime < since_ts {
                        continue; // Skip files older than the baseline
                    }
                }
            }
        }
        if let Ok(meta) = std::fs::metadata(&path) {
            breakdown.total_raw_bytes += meta.len();
        }
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        accumulate_breakdown(&content, &mut breakdown);
    }

    Ok(breakdown)
}

/// Sum the byte sizes of all Bash command inputs and their tool_result outputs
/// across all session JSONL files. This measures the Bash-specific token footprint.
///
/// Returns `(bash_input_bytes, bash_output_bytes)`.
pub fn bash_total_bytes() -> Result<(u64, u64)> {
    let bd = session_token_breakdown()?;
    let bash = bd.tool("Bash");
    Ok((bash.input_bytes, bash.output_bytes))
}

/// Accumulate per-tool and text byte counts from a single session's JSONL content.
fn accumulate_breakdown(content: &str, bd: &mut SessionTokenBreakdown) {
    // Map tool_use id → tool name for attributing tool_results
    let mut pending_ids: HashMap<String, String> = HashMap::new();

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let parsed: serde_json::Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        // Skip non-message lines (progress, snapshots, system, etc.)
        let msg = match parsed.get("message") {
            Some(m) if !m.is_null() => m,
            _ => continue,
        };

        let role = msg.get("role").and_then(|r| r.as_str()).unwrap_or("");

        let content_val = match msg.get("content") {
            Some(c) => c,
            None => continue,
        };

        // Handle string content (some user messages use plain string)
        if let Some(s) = content_val.as_str() {
            bd.user_text_bytes += s.len() as u64;
            continue;
        }

        let blocks = match content_val.as_array() {
            Some(arr) => arr,
            None => continue,
        };

        for block in blocks {
            let btype = match block.get("type").and_then(|t| t.as_str()) {
                Some(t) => t,
                None => continue,
            };

            match btype {
                "text" => {
                    let len = block
                        .get("text")
                        .and_then(|t| t.as_str())
                        .map(|s| s.len() as u64)
                        .unwrap_or(0);
                    if role == "assistant" {
                        bd.assistant_text_bytes += len;
                    } else {
                        bd.user_text_bytes += len;
                    }
                }
                "thinking" => {
                    let len = block
                        .get("thinking")
                        .and_then(|t| t.as_str())
                        .map(|s| s.len() as u64)
                        .unwrap_or(0);
                    bd.thinking_bytes += len;
                }
                "tool_use" => {
                    let tool_name = match block.get("name").and_then(|n| n.as_str()) {
                        Some(n) => n.to_string(),
                        None => continue,
                    };
                    // Count input bytes (the full serialized input object)
                    let input_len = block
                        .get("input")
                        .map(|i| serde_json::to_string(i).unwrap_or_default().len() as u64)
                        .unwrap_or(0);

                    let entry = bd.tools.entry(tool_name.clone()).or_default();
                    entry.input_bytes += input_len;
                    entry.invocations += 1;

                    if let Some(id) = block.get("id").and_then(|i| i.as_str()) {
                        pending_ids.insert(id.to_string(), tool_name);
                    }
                }
                "tool_result" => {
                    let tool_name = block
                        .get("tool_use_id")
                        .and_then(|i| i.as_str())
                        .and_then(|id| pending_ids.remove(id));

                    let output_len = content_block_text_len(block);

                    if let Some(name) = tool_name {
                        let entry = bd.tools.entry(name).or_default();
                        entry.output_bytes += output_len;
                    }
                }
                _ => {}
            }
        }
    }
}

/// Extract the text length from a tool_result content field.
/// Handles both string and array-of-text-blocks formats.
fn content_block_text_len(block: &serde_json::Value) -> u64 {
    match block.get("content") {
        Some(c) if c.is_string() => c.as_str().unwrap_or("").len() as u64,
        Some(c) if c.is_array() => c
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|item| item.get("text").and_then(|t| t.as_str()))
            .map(|t| t.len() as u64)
            .sum(),
        _ => 0,
    }
}

/// Extract tool use events from JSONL content.
///
/// Merges consecutive tool_use + tool_result pairs into single events
/// so each event has both the command and its output/exit status.
/// Extract Bash (and Edit/Write) tool events from JSONL session content.
///
/// Uses `tool_use_id` / `id` matching to correctly pair tool_use blocks with
/// their tool_result blocks regardless of interleaved non-Bash results.  This
/// handles both the classic top-level session format and the subagent format
/// where Glob/Read/Grep results appear between Bash tool_use and tool_result.
fn extract_tool_events(content: &str) -> Vec<ToolEvent> {
    // First pass: collect every tool_use block we care about, keyed by its id.
    // Value: (tool_name, command_or_path, insertion_order)
    let mut pending: HashMap<String, (String, String, usize)> = HashMap::new();
    // Second pass accumulator: completed events in document order.
    // We store (order_key, ToolEvent) and sort at the end.
    let mut completed: Vec<(usize, ToolEvent)> = Vec::new();
    let mut order = 0usize;

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let parsed: serde_json::Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let msg = match parsed.get("message") {
            Some(m) => m,
            None => continue,
        };
        let blocks = match msg.get("content").and_then(|c| c.as_array()) {
            Some(arr) => arr,
            None => continue,
        };

        for block in blocks {
            let btype = match block.get("type").and_then(|t| t.as_str()) {
                Some(t) => t,
                None => continue,
            };

            if btype == "tool_use" {
                let tool_name = match block.get("name").and_then(|n| n.as_str()) {
                    Some(n) => n,
                    None => continue,
                };
                let id = match block.get("id").and_then(|i| i.as_str()) {
                    Some(i) => i.to_string(),
                    None => continue,
                };
                let input = match block.get("input") {
                    Some(i) => i,
                    None => continue,
                };

                if tool_name == "Bash" {
                    if let Some(cmd) = input.get("command").and_then(|c| c.as_str()) {
                        order += 1;
                        pending.insert(id, ("Bash".to_string(), cmd.to_string(), order));
                    }
                } else if tool_name == "Edit" || tool_name == "Write" {
                    if let Some(path) = input.get("file_path").and_then(|p| p.as_str()) {
                        order += 1;
                        pending.insert(id, (tool_name.to_string(), path.to_string(), order));
                    }
                }
            } else if btype == "tool_result" {
                let tool_use_id = match block.get("tool_use_id").and_then(|i| i.as_str()) {
                    Some(i) => i.to_string(),
                    None => continue,
                };

                if let Some((tool_name, command, seq)) = pending.remove(&tool_use_id) {
                    if tool_name == "Bash" {
                        // Extract output text
                        let raw_content = block.get("content");
                        let text = match raw_content {
                            Some(c) if c.is_string() => c.as_str().unwrap_or("").to_string(),
                            Some(c) if c.is_array() => c
                                .as_array()
                                .unwrap()
                                .iter()
                                .filter_map(|item| item.get("text").and_then(|t| t.as_str()))
                                .collect::<Vec<_>>()
                                .join("\n"),
                            _ => String::new(),
                        };

                        let is_error =
                            block.get("is_error").and_then(|e| e.as_bool()) == Some(true);
                        let exit_code = if is_error {
                            extract_exit_code(&text).or(Some(1))
                        } else {
                            extract_exit_code(&text)
                        };
                        let is_failure = exit_code.map(|c| c != 0).unwrap_or(false);

                        completed.push((
                            seq,
                            ToolEvent {
                                tool: "Bash".to_string(),
                                command: Some(command),
                                output: Some(text),
                                exit_code,
                                is_failure,
                            },
                        ));
                    } else {
                        // Edit / Write — no output needed, just record the file path
                        completed.push((
                            seq,
                            ToolEvent {
                                tool: tool_name,
                                command: Some(command),
                                output: None,
                                exit_code: None,
                                is_failure: false,
                            },
                        ));
                    }
                }
            }
        }
    }

    // Emit in document order (by insertion sequence number).
    completed.sort_by_key(|(seq, _)| *seq);
    completed.into_iter().map(|(_, e)| e).collect()
}

/// Extract exit code from text. Handles both "Exit code: N" and "Exit code N".
fn extract_exit_code(text: &str) -> Option<i32> {
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("Exit code:") {
            if let Ok(code) = rest.trim().parse::<i32>() {
                return Some(code);
            }
        } else if let Some(rest) = trimmed.strip_prefix("Exit code ") {
            if let Ok(code) = rest.trim().parse::<i32>() {
                return Some(code);
            }
        }
    }
    None
}

/// Find failure-fix pairs from a sequence of events.
///
/// A pair is: a Bash failure followed by another Bash/Edit event
/// that appears to fix the failure (a Bash command that succeeds
/// with a similar prefix, or an Edit to a file mentioned in the error).
fn find_failure_fix_pairs(events: &[ToolEvent]) -> Vec<FailureFixPair> {
    let mut pairs = Vec::new();

    for (i, event) in events.iter().enumerate() {
        if !event.is_failure {
            continue;
        }

        let failure_cmd = match &event.command {
            Some(c) => c.clone(),
            None => continue,
        };

        let failure_output = event.output.as_deref().unwrap_or("");

        // Look ahead for a fix (within the next 10 events)
        let lookahead = std::cmp::min(i + 10, events.len());
        for candidate in events.iter().take(lookahead).skip(i + 1) {
            // A Bash command that succeeds with similar prefix = fix
            if candidate.tool == "Bash" && !candidate.is_failure {
                if let Some(fix_cmd) = &candidate.command {
                    if commands_similar(&failure_cmd, fix_cmd) {
                        let hash = compute_pattern_hash(&failure_cmd, failure_output);
                        let project_type = detect_project_type(&failure_cmd);
                        pairs.push(FailureFixPair {
                            failure_command: failure_cmd.clone(),
                            failure_output: truncate(failure_output, 500),
                            fix_command: fix_cmd.clone(),
                            pattern_hash: hash,
                            project_type,
                        });
                        break;
                    }
                }
            }

            // An Edit to a file mentioned in the error output = fix
            if candidate.tool == "Edit" || candidate.tool == "Write" {
                if let Some(file_path) = &candidate.command {
                    if failure_output.contains(file_path)
                        || failure_output.contains(&basename(file_path))
                    {
                        let hash = compute_pattern_hash(&failure_cmd, failure_output);
                        let project_type = detect_project_type(&failure_cmd);
                        pairs.push(FailureFixPair {
                            failure_command: failure_cmd.clone(),
                            failure_output: truncate(failure_output, 500),
                            fix_command: format!("edit:{}", file_path),
                            pattern_hash: hash,
                            project_type,
                        });
                        break;
                    }
                }
            }
        }
    }

    pairs
}

/// Check if two commands are "similar" (same tool prefix).
fn commands_similar(a: &str, b: &str) -> bool {
    let prefix_a = first_word(a);
    let prefix_b = first_word(b);
    prefix_a == prefix_b
}

fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn basename(path: &str) -> String {
    Path::new(path)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or(path)
        .to_string()
}

/// Normalize and hash a failure pattern for deduplication.
fn compute_pattern_hash(command: &str, output: &str) -> String {
    // Normalize: strip absolute paths, version numbers, line numbers
    let normalized_cmd = normalize_pattern(command);
    let normalized_output = normalize_pattern(&truncate(output, 200));

    let mut hasher = DefaultHasher::new();
    normalized_cmd.hash(&mut hasher);
    normalized_output.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

/// Strip paths, version numbers, and other variable parts from a pattern.
fn normalize_pattern(s: &str) -> String {
    let mut result = s.to_string();
    // Strip absolute paths
    let path_re = regex::Regex::new(r"/[a-zA-Z0-9_./-]+").unwrap();
    result = path_re.replace_all(&result, "<PATH>").to_string();
    // Strip version numbers
    let ver_re = regex::Regex::new(r"\d+\.\d+\.\d+").unwrap();
    result = ver_re.replace_all(&result, "<VER>").to_string();
    // Strip line:col numbers
    let linecol_re = regex::Regex::new(r":\d+:\d+").unwrap();
    result = linecol_re.replace_all(&result, ":<L>:<C>").to_string();
    result
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        // Find the nearest char boundary at or before max_len
        let mut end = max_len;
        while end > 0 && !s.is_char_boundary(end) {
            end -= 1;
        }
        format!("{}...", &s[..end])
    }
}

/// Detect project type from a command string.
fn detect_project_type(command: &str) -> Option<String> {
    let tool = first_word(command);
    match tool {
        "cargo" | "rustc" => Some("rust".to_string()),
        "npm" | "npx" | "pnpm" | "yarn" | "vitest" | "tsc" | "eslint" | "prettier"
        | "playwright" | "prisma" | "next" | "node" => Some("node".to_string()),
        "python" | "pip" | "poetry" | "pytest" => Some("python".to_string()),
        "go" => Some("go".to_string()),
        "make" | "cmake" | "gcc" | "g++" | "clang" => Some("c".to_string()),
        "mvn" | "gradle" => Some("java".to_string()),
        _ => None,
    }
}

fn detect_project_from_path(path: &Path) -> Option<String> {
    // Claude Code session paths contain the project path encoded
    path.parent()
        .and_then(|p| p.file_name())
        .and_then(|f| f.to_str())
        .map(|s| s.replace('-', "/"))
}

/// A PRECC event extracted from a session's permissionDecisionReason strings.
#[derive(Debug)]
pub enum PreccEvent {
    SkillActivation { skill_name: String },
    CdPrepend { marker: String },
    RtkRewrite,
}

/// Extract PRECC events from JSONL session content by scanning permissionDecisionReason fields.
///
/// The hook emits reasons like: `"PRECC: skill:cargo-wrong-dir (conf=0.9); cd:Cargo.toml; rtk-rewrite"`
/// This function parses those strings and returns structured events.
pub fn extract_precc_events(content: &str) -> Vec<PreccEvent> {
    let mut events = Vec::new();

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        // Quick scan: skip lines without the PRECC marker
        if !line.contains("permissionDecisionReason") || !line.contains("PRECC:") {
            continue;
        }

        let parsed: serde_json::Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        // Walk nested structure to find permissionDecisionReason anywhere in the line
        collect_precc_reasons(&parsed, &mut events);
    }

    events
}

fn collect_precc_reasons(value: &serde_json::Value, events: &mut Vec<PreccEvent>) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(reason) = map.get("permissionDecisionReason").and_then(|v| v.as_str()) {
                parse_precc_reason(reason, events);
            }
            for v in map.values() {
                collect_precc_reasons(v, events);
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr {
                collect_precc_reasons(v, events);
            }
        }
        _ => {}
    }
}

fn parse_precc_reason(reason: &str, events: &mut Vec<PreccEvent>) {
    // Reason format: "PRECC: skill:name (conf=0.9); cd:marker (conf=0.9); rtk-rewrite"
    let body = match reason.strip_prefix("PRECC:") {
        Some(b) => b,
        None => return,
    };

    for token in body.split(';') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }

        if let Some(rest) = token.strip_prefix("skill:") {
            // Extract skill name (up to first space or '(')
            let name = rest
                .split([' ', '('])
                .next()
                .unwrap_or(rest)
                .trim()
                .to_string();
            if !name.is_empty() {
                events.push(PreccEvent::SkillActivation { skill_name: name });
            }
        } else if let Some(rest) = token.strip_prefix("cd:") {
            let marker = rest
                .split([' ', '('])
                .next()
                .unwrap_or(rest)
                .trim()
                .to_string();
            events.push(PreccEvent::CdPrepend { marker });
        } else if token == "rtk-rewrite" {
            events.push(PreccEvent::RtkRewrite);
        }
    }
}

/// Summary of mining results for display.
#[derive(Debug, Default)]
pub struct MiningSummary {
    pub sessions_processed: usize,
    pub sessions_skipped: usize,
    pub events_found: usize,
    pub pairs_found: usize,
    pub precc_events_extracted: usize,
}

/// Mine all sessions. Returns a summary.
///
/// If `force` is true, already-mined sessions are re-mined from scratch.
pub fn mine_all(conn: &Connection, force: bool) -> Result<MiningSummary> {
    let files = find_session_files()?;
    let mut summary = MiningSummary::default();

    for file in &files {
        match mine_session(conn, file, force) {
            Ok(MineResult::Skipped) => summary.sessions_skipped += 1,
            Ok(MineResult::Processed { pairs, events }) => {
                summary.sessions_processed += 1;
                summary.pairs_found += pairs;
                summary.events_found += events;
            }
            Err(_) => summary.sessions_skipped += 1,
        }
    }

    Ok(summary)
}

/// Extract PRECC events from all sessions not yet processed for PRECC events.
///
/// Reads `permissionDecisionReason` strings from session JSONL files and:
/// - Records skill activations via `skills::record_activation()`
/// - Records CD prepends via `metrics::record(CdPrepend, ...)`
/// - Increments `precc_prevented` on matching `failure_fix_pairs` rows
///
/// Sessions are tracked via the `precc_events_extracted` column in `sessions`.
pub fn extract_all_precc_events(
    history_conn: &Connection,
    heuristics_conn: &Connection,
    metrics_conn: &Connection,
) -> Result<usize> {
    use crate::{metrics, skills};

    let files = find_session_files()?;
    let mut total = 0;

    for file in &files {
        let session_id = match file.file_stem().and_then(|s| s.to_str()) {
            Some(id) => id.to_string(),
            None => continue,
        };

        // Check if this session exists in history and hasn't been PRECC-extracted yet
        let row: Option<(i64, bool)> = history_conn
            .query_row(
                "SELECT id, COALESCE(precc_events_extracted, 0)
                 FROM sessions WHERE session_id = ?1",
                [&session_id],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .ok();

        let (db_session_id, already_extracted) = match row {
            Some(r) => r,
            None => continue, // Not mined yet — skip
        };

        if already_extracted {
            continue;
        }

        let content = match std::fs::read_to_string(file) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let events = extract_precc_events(&content);
        let count = events.len();

        for event in events {
            match event {
                PreccEvent::SkillActivation { skill_name } => {
                    // Look up skill_id by name
                    if let Ok(skill_id) = heuristics_conn.query_row(
                        "SELECT id FROM skills WHERE name = ?1",
                        [&skill_name],
                        |r| r.get::<_, i64>(0),
                    ) {
                        let _ = skills::record_activation(heuristics_conn, skill_id);

                        // Also try to increment precc_prevented on matching failure_fix_pairs
                        // by looking for pairs whose failure command starts with the skill's tool prefix
                        let tool = skill_name
                            .split('-')
                            .next()
                            .unwrap_or(&skill_name)
                            .to_string();
                        let _ = history_conn.execute(
                            "UPDATE failure_fix_pairs
                             SET precc_prevented = COALESCE(precc_prevented, 0) + 1
                             WHERE failure_command LIKE ?1 || '%'
                               AND (occurrences > 1 OR precc_prevented > 0)",
                            [&tool],
                        );
                    }
                }
                PreccEvent::CdPrepend { marker } => {
                    let meta = format!("{{\"marker\":\"{}\"}}", marker);
                    let _ = metrics::record(
                        metrics_conn,
                        metrics::MetricType::CdPrepend,
                        1.0,
                        Some(&meta),
                    );
                }
                PreccEvent::RtkRewrite => {
                    // Skip: already counted in metrics.db from the hook
                }
            }
        }

        // Mark session as PRECC-extracted
        let _ = history_conn.execute(
            "UPDATE sessions SET precc_events_extracted = 1 WHERE id = ?1",
            [db_session_id],
        );

        total += count;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_strips_paths() {
        let input = "error at /home/user/project/src/main.rs:42:10";
        let normalized = normalize_pattern(input);
        assert!(normalized.contains("<PATH>"));
        assert!(!normalized.contains("/home/user"));
    }

    #[test]
    fn normalize_strips_versions() {
        let input = "requires rustc 1.75.0";
        let normalized = normalize_pattern(input);
        assert!(normalized.contains("<VER>"));
        assert!(!normalized.contains("1.75.0"));
    }

    #[test]
    fn commands_similar_same_tool() {
        assert!(commands_similar("cargo build", "cargo build --release"));
        assert!(commands_similar("cargo test foo", "cargo test bar"));
        assert!(!commands_similar("cargo build", "git status"));
    }

    #[test]
    fn detect_project_type_rust() {
        assert_eq!(detect_project_type("cargo build"), Some("rust".to_string()));
        assert_eq!(detect_project_type("cargo test"), Some("rust".to_string()));
    }

    #[test]
    fn detect_project_type_node() {
        assert_eq!(detect_project_type("npm install"), Some("node".to_string()));
        assert_eq!(detect_project_type("pnpm test"), Some("node".to_string()));
    }

    #[test]
    fn detect_project_type_unknown() {
        assert_eq!(detect_project_type("echo hello"), None);
    }

    #[test]
    fn pattern_hash_deterministic() {
        let h1 = compute_pattern_hash("cargo build", "error: missing lib");
        let h2 = compute_pattern_hash("cargo build", "error: missing lib");
        assert_eq!(h1, h2);
    }

    #[test]
    fn pattern_hash_different_for_different_errors() {
        let h1 = compute_pattern_hash("cargo build", "error: missing lib");
        let h2 = compute_pattern_hash("cargo build", "error: type mismatch");
        assert_ne!(h1, h2);
    }

    #[test]
    fn find_pairs_basic() {
        let events = vec![
            ToolEvent {
                tool: "Bash".to_string(),
                command: Some("cargo build".to_string()),
                output: Some("error: missing semicolon".to_string()),
                exit_code: Some(1),
                is_failure: true,
            },
            ToolEvent {
                tool: "Edit".to_string(),
                command: Some("src/main.rs".to_string()),
                output: None,
                exit_code: None,
                is_failure: false,
            },
            ToolEvent {
                tool: "Bash".to_string(),
                command: Some("cargo build".to_string()),
                output: Some("Finished".to_string()),
                exit_code: Some(0),
                is_failure: false,
            },
        ];

        let pairs = find_failure_fix_pairs(&events);
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].failure_command, "cargo build");
        assert_eq!(pairs[0].fix_command, "cargo build");
    }

    #[test]
    fn find_pairs_edit_fix() {
        let events = vec![
            ToolEvent {
                tool: "Bash".to_string(),
                command: Some("cargo build".to_string()),
                output: Some("error in src/lib.rs".to_string()),
                exit_code: Some(1),
                is_failure: true,
            },
            ToolEvent {
                tool: "Edit".to_string(),
                command: Some("src/lib.rs".to_string()),
                output: None,
                exit_code: None,
                is_failure: false,
            },
        ];

        let pairs = find_failure_fix_pairs(&events);
        assert_eq!(pairs.len(), 1);
        assert!(pairs[0].fix_command.starts_with("edit:"));
    }

    #[test]
    fn find_pairs_no_fix_found() {
        let events = vec![ToolEvent {
            tool: "Bash".to_string(),
            command: Some("cargo build".to_string()),
            output: Some("error".to_string()),
            exit_code: Some(1),
            is_failure: true,
        }];

        let pairs = find_failure_fix_pairs(&events);
        assert!(pairs.is_empty());
    }

    #[test]
    fn truncate_long_string() {
        let s = "a".repeat(1000);
        let t = truncate(&s, 100);
        assert_eq!(t.len(), 103); // 100 + "..."
        assert!(t.ends_with("..."));
    }

    #[test]
    fn truncate_short_string() {
        let s = "hello";
        let t = truncate(s, 100);
        assert_eq!(t, "hello");
    }

    #[test]
    fn truncate_exact_length() {
        let s = "hello";
        let t = truncate(s, 5);
        assert_eq!(t, "hello");
    }

    #[test]
    fn truncate_empty_string() {
        let t = truncate("", 10);
        assert_eq!(t, "");
    }

    #[test]
    fn truncate_max_len_zero() {
        let t = truncate("hello", 0);
        assert_eq!(t, "...");
    }

    #[test]
    fn truncate_max_len_one() {
        let t = truncate("hello", 1);
        assert_eq!(t, "h...");
    }

    #[test]
    fn truncate_multibyte_utf8_boundary() {
        // "━" is 3 bytes (U+2501). Truncating mid-character should not panic.
        let s = "━━━━━";
        let t = truncate(s, 2);
        assert!(t.ends_with("..."));
    }

    #[test]
    fn truncate_emoji_boundary() {
        // "😀" is 4 bytes. Truncating at byte 3 should round down.
        let s = "😀😀😀";
        let t = truncate(s, 5);
        assert!(t.ends_with("..."));
    }

    #[test]
    fn truncate_mixed_ascii_and_multibyte() {
        let s = "hello━world";
        let t = truncate(s, 6);
        assert!(t.ends_with("..."));
        assert!(t.starts_with("hello"));
    }

    #[test]
    fn extract_exit_code_with_colon() {
        assert_eq!(extract_exit_code("Exit code: 1"), Some(1));
        assert_eq!(extract_exit_code("Exit code: 127"), Some(127));
    }

    #[test]
    fn extract_exit_code_without_colon() {
        assert_eq!(extract_exit_code("Exit code 1"), Some(1));
        assert_eq!(extract_exit_code("Exit code 101\nerror output"), Some(101));
        assert_eq!(
            extract_exit_code("Exit code 127\n/bin/bash: command not found"),
            Some(127)
        );
    }

    #[test]
    fn extract_exit_code_none() {
        assert_eq!(extract_exit_code("Finished build"), None);
        assert_eq!(extract_exit_code(""), None);
    }

    #[test]
    fn merge_tool_use_and_result() {
        // Verify tool_use_id matching: tool_result is paired with its Bash tool_use
        // even when interleaved with other tool results.
        let jsonl = r#"{"type":"assistant","message":{"role":"assistant","content":[{"type":"tool_use","id":"tu_bash","name":"Bash","input":{"command":"cargo build"}}]}}
{"type":"user","message":{"role":"user","content":[{"tool_use_id":"tu_bash","type":"tool_result","content":"Exit code 1\nerror","is_error":true}]}}"#;
        let events = extract_tool_events(jsonl);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].command.as_deref(), Some("cargo build"));
        assert!(events[0].output.is_some());
        assert_eq!(events[0].exit_code, Some(1));
        assert!(events[0].is_failure);
    }

    #[test]
    fn merge_preserves_unmatched_events() {
        // Bash tool_use with no matching tool_result (e.g. truncated session) should be dropped.
        // Edit/Write tool_use with result should still appear.
        let jsonl = r#"{"type":"assistant","message":{"role":"assistant","content":[{"type":"tool_use","id":"tu_bash","name":"Bash","input":{"command":"cargo build"}},{"type":"tool_use","id":"tu_edit","name":"Edit","input":{"file_path":"src/main.rs","old_string":"x","new_string":"y"}}]}}
{"type":"user","message":{"role":"user","content":[{"tool_use_id":"tu_edit","type":"tool_result","content":"File edited"}]}}"#;
        let events = extract_tool_events(jsonl);
        // Only the Edit event is fully paired; the Bash has no result so is dropped.
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].tool, "Edit");
    }

    #[test]
    fn extract_events_from_real_jsonl() {
        // Simulate realistic Claude Code JSONL
        let jsonl = r#"{"type":"assistant","message":{"role":"assistant","content":[{"type":"tool_use","id":"toolu_01","name":"Bash","input":{"command":"cargo build"}}]}}
{"type":"user","message":{"role":"user","content":[{"tool_use_id":"toolu_01","type":"tool_result","content":"Exit code 1\nerror[E0308]: mismatched types","is_error":true}]}}
{"type":"assistant","message":{"role":"assistant","content":[{"type":"tool_use","id":"toolu_02","name":"Edit","input":{"file_path":"src/main.rs","old_string":"x","new_string":"y"}}]}}
{"type":"user","message":{"role":"user","content":[{"tool_use_id":"toolu_02","type":"tool_result","content":"File edited successfully"}]}}
{"type":"assistant","message":{"role":"assistant","content":[{"type":"tool_use","id":"toolu_03","name":"Bash","input":{"command":"cargo build"}}]}}
{"type":"user","message":{"role":"user","content":[{"tool_use_id":"toolu_03","type":"tool_result","content":"Finished `dev` profile","is_error":false}]}}"#;

        let events = extract_tool_events(jsonl);
        // Should have 4 events: Bash(fail), Edit, Bash(success), + tool_result for edit
        // After merge: Bash(fail, merged), Edit (unmerged since next is not result for same tool),
        // tool_result(edit), Bash(success, merged)
        // Actually: tool_use(Bash) + tool_result(Bash) -> merged Bash event
        //           tool_use(Edit) + tool_result(not Edit tool) -> Edit stays, result stays
        //           tool_use(Bash) + tool_result(Bash) -> merged Bash event

        // Count Bash events with both command and failure info
        let bash_with_cmd: Vec<_> = events
            .iter()
            .filter(|e| e.tool == "Bash" && e.command.is_some())
            .collect();
        assert!(
            bash_with_cmd.len() >= 2,
            "Should have at least 2 merged Bash events, got {}",
            bash_with_cmd.len()
        );

        // First Bash should be a failure
        let first_bash = bash_with_cmd[0];
        assert_eq!(first_bash.command.as_deref(), Some("cargo build"));
        assert!(first_bash.is_failure, "First bash should be a failure");
        assert_eq!(first_bash.exit_code, Some(1));

        // Second Bash should be success
        let second_bash = bash_with_cmd[1];
        assert_eq!(second_bash.command.as_deref(), Some("cargo build"));
        assert!(!second_bash.is_failure, "Second bash should be success");
    }

    #[test]
    fn find_pairs_from_real_jsonl() {
        let jsonl = r#"{"type":"assistant","message":{"role":"assistant","content":[{"type":"tool_use","id":"toolu_01","name":"Bash","input":{"command":"cargo build"}}]}}
{"type":"user","message":{"role":"user","content":[{"tool_use_id":"toolu_01","type":"tool_result","content":"Exit code 1\nerror[E0308]: mismatched types in src/main.rs","is_error":true}]}}
{"type":"assistant","message":{"role":"assistant","content":[{"type":"tool_use","id":"toolu_02","name":"Bash","input":{"command":"cargo build"}}]}}
{"type":"user","message":{"role":"user","content":[{"tool_use_id":"toolu_02","type":"tool_result","content":"Finished `dev` profile","is_error":false}]}}"#;

        let events = extract_tool_events(jsonl);
        let pairs = find_failure_fix_pairs(&events);
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].failure_command, "cargo build");
        assert_eq!(pairs[0].fix_command, "cargo build");
    }

    #[test]
    fn extract_precc_events_skill_activation() {
        let jsonl = r#"{"hookSpecificOutput":{"permissionDecisionReason":"PRECC: skill:cargo-wrong-dir (conf=0.9)"}}"#;
        let events = extract_precc_events(jsonl);
        assert_eq!(events.len(), 1);
        match &events[0] {
            PreccEvent::SkillActivation { skill_name } => {
                assert_eq!(skill_name, "cargo-wrong-dir");
            }
            _ => panic!("expected SkillActivation"),
        }
    }

    #[test]
    fn extract_precc_events_cd_prepend() {
        let jsonl = r#"{"hookSpecificOutput":{"permissionDecisionReason":"PRECC: cd:Cargo.toml (conf=0.9)"}}"#;
        let events = extract_precc_events(jsonl);
        assert_eq!(events.len(), 1);
        match &events[0] {
            PreccEvent::CdPrepend { marker } => {
                assert_eq!(marker, "Cargo.toml");
            }
            _ => panic!("expected CdPrepend"),
        }
    }

    #[test]
    fn extract_precc_events_rtk_rewrite() {
        let jsonl = r#"{"hookSpecificOutput":{"permissionDecisionReason":"PRECC: rtk-rewrite"}}"#;
        let events = extract_precc_events(jsonl);
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], PreccEvent::RtkRewrite));
    }

    #[test]
    fn extract_precc_events_multiple_tokens() {
        let jsonl = r#"{"hookSpecificOutput":{"permissionDecisionReason":"PRECC: skill:cargo-wrong-dir (conf=0.9); cd:Cargo.toml (conf=0.9); rtk-rewrite"}}"#;
        let events = extract_precc_events(jsonl);
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn extract_precc_events_ignores_non_precc() {
        let jsonl = r#"{"hookSpecificOutput":{"permissionDecisionReason":"allow"}}"#;
        let events = extract_precc_events(jsonl);
        assert!(events.is_empty());
    }

    #[test]
    fn extract_precc_events_empty_content() {
        let events = extract_precc_events("");
        assert!(events.is_empty());
    }
}
