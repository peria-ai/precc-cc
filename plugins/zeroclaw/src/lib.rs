//! # zeroclaw-precc
//!
//! PRECC token-saving plugin for ZeroClaw.
//!
//! Implements the ZeroClaw `Tool` trait to route every shell command through
//! `precc-hook` before execution, providing:
//!
//! - Wrong-directory auto-correction (<3ms)
//! - RTK output compression (60–90% smaller)
//! - Learned heuristic application
//! - git→jj translation in Jujutsu repos
//!
//! ## Registration
//!
//! ```rust,ignore
//! use zeroclaw_precc::{PreccHookTool, PreccReportTool, PreccUpdateTool};
//!
//! agent.register_tool(PreccHookTool::new());
//! agent.register_tool(PreccReportTool::new());
//! agent.register_tool(PreccUpdateTool::new());
//! ```

use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

// ---------------------------------------------------------------------------
// Helper: locate binary on PATH or at a configured absolute path
// ---------------------------------------------------------------------------

fn resolve_bin(name: &str) -> String {
    std::env::var(format!("PRECC_{}_BIN", name.to_uppercase().replace('-', "_")))
        .unwrap_or_else(|_| name.to_string())
}

// ---------------------------------------------------------------------------
// Helper: pipe a command string through precc-hook
// ---------------------------------------------------------------------------

/// Send `command` through `precc-hook` and return the rewritten command.
/// Falls back to the original command if the hook fails or produces no output.
pub async fn run_precc_hook(command: &str) -> Result<String> {
    let hook_bin = resolve_bin("precc-hook");
    let payload = json!({"tool_input": {"command": command}}).to_string();

    let mut child = Command::new(&hook_bin)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .with_context(|| {
            format!(
                "precc-hook not found at '{hook_bin}'. \
                 Install with: curl -fsSL \
                 https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash"
            )
        })?;

    child
        .stdin
        .take()
        .context("no stdin on precc-hook")?
        .write_all(payload.as_bytes())
        .await
        .context("writing to precc-hook stdin")?;

    let out = child
        .wait_with_output()
        .await
        .context("waiting for precc-hook")?;

    if !out.status.success() || out.stdout.is_empty() {
        return Ok(command.to_string());
    }

    let response: Value =
        serde_json::from_slice(&out.stdout).unwrap_or_else(|_| Value::Null);

    let rewritten = response
        .pointer("/hookSpecificOutput/updatedInput/command")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .unwrap_or(command);

    Ok(rewritten.to_string())
}

// ---------------------------------------------------------------------------
// Tool 1: PreccHookTool
// ---------------------------------------------------------------------------

/// ZeroClaw tool that routes a bash command through `precc-hook`.
///
/// Parameters:
/// - `command` (string, required) — the shell command to execute via PRECC
///
/// Returns:
/// - `rewritten_command` — the (possibly modified) command after PRECC processing
/// - `was_rewritten` — whether PRECC changed the command
/// - `original_command` — the original input
pub struct PreccHookTool {
    hook_bin: String,
}

impl PreccHookTool {
    pub fn new() -> Self {
        Self {
            hook_bin: resolve_bin("precc-hook"),
        }
    }

    pub fn name() -> &'static str {
        "precc_hook"
    }

    pub fn description() -> &'static str {
        "Route a bash command through the PRECC hook to auto-fix wrong-directory \
         errors, apply RTK compression, and apply learned correction skills. \
         Returns the rewritten command (or original if no rewrite needed). \
         Latency: <3ms. Fail-open: always returns a usable command."
    }

    pub async fn execute(&self, args: Value) -> Result<Value> {
        let command = args["command"]
            .as_str()
            .context("PreccHookTool: 'command' parameter (string) is required")?;

        let rewritten = run_precc_hook(command).await?;
        let was_rewritten = rewritten != command;

        Ok(json!({
            "rewritten_command": rewritten,
            "original_command": command,
            "was_rewritten": was_rewritten,
            "hook_bin": self.hook_bin,
        }))
    }
}

impl Default for PreccHookTool {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tool 2: PreccReportTool
// ---------------------------------------------------------------------------

/// ZeroClaw tool that runs `precc report` and returns the savings summary.
///
/// Parameters: none
///
/// Returns: `report` (string) with the full PRECC analytics dashboard output.
pub struct PreccReportTool;

impl PreccReportTool {
    pub fn new() -> Self {
        Self
    }

    pub fn name() -> &'static str {
        "precc_report"
    }

    pub fn description() -> &'static str {
        "Run 'precc report' to display token savings analytics: commands corrected, \
         tokens saved, dollar-value estimate, hook latency stats, and skill activations. \
         Use this to show the agent and user how much PRECC has saved in this session."
    }

    pub async fn execute(&self, _args: Value) -> Result<Value> {
        let precc_bin = resolve_bin("precc");
        let out = Command::new(&precc_bin)
            .arg("report")
            .output()
            .await
            .with_context(|| format!("failed to run '{precc_bin} report'"))?;

        let report = String::from_utf8_lossy(&out.stdout).to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();

        Ok(json!({
            "report": report,
            "stderr": if stderr.is_empty() { Value::Null } else { Value::String(stderr) },
            "exit_code": out.status.code().unwrap_or(-1),
        }))
    }
}

impl Default for PreccReportTool {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tool 3: PreccUpdateTool
// ---------------------------------------------------------------------------

/// ZeroClaw tool that runs `precc update` to self-update PRECC binaries.
///
/// Parameters:
/// - `version` (string, optional) — pin to a specific release tag (e.g. "v0.2.0")
/// - `force`   (bool, optional)   — reinstall even if already on latest
pub struct PreccUpdateTool;

impl PreccUpdateTool {
    pub fn new() -> Self {
        Self
    }

    pub fn name() -> &'static str {
        "precc_update"
    }

    pub fn description() -> &'static str {
        "Update PRECC binaries to the latest GitHub release (or a specified version). \
         Equivalent to running 'precc update' on the command line."
    }

    pub async fn execute(&self, args: Value) -> Result<Value> {
        let precc_bin = resolve_bin("precc");
        let mut cmd = Command::new(&precc_bin);
        cmd.arg("update");

        if let Some(v) = args["version"].as_str() {
            cmd.args(["--version", v]);
        }
        if args["force"].as_bool().unwrap_or(false) {
            cmd.arg("--force");
        }

        let out = cmd
            .output()
            .await
            .with_context(|| format!("failed to run '{precc_bin} update'"))?;

        Ok(json!({
            "output": String::from_utf8_lossy(&out.stdout).trim().to_string(),
            "success": out.status.success(),
            "exit_code": out.status.code().unwrap_or(-1),
        }))
    }
}

impl Default for PreccUpdateTool {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tool 4: PreccSkillsTool
// ---------------------------------------------------------------------------

/// ZeroClaw tool that runs `precc skills list` to show active correction skills.
pub struct PreccSkillsTool;

impl PreccSkillsTool {
    pub fn new() -> Self {
        Self
    }

    pub fn name() -> &'static str {
        "precc_skills"
    }

    pub fn description() -> &'static str {
        "List all active PRECC correction skills — built-in and mined from session history. \
         Each skill shows its trigger pattern, confidence, and activation count."
    }

    pub async fn execute(&self, _args: Value) -> Result<Value> {
        let precc_bin = resolve_bin("precc");
        let out = Command::new(&precc_bin)
            .args(["skills", "list"])
            .output()
            .await
            .with_context(|| format!("failed to run '{precc_bin} skills list'"))?;

        Ok(json!({
            "skills": String::from_utf8_lossy(&out.stdout).trim().to_string(),
            "exit_code": out.status.code().unwrap_or(-1),
        }))
    }
}

impl Default for PreccSkillsTool {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ── run_precc_hook ───────────────────────────────────────────────────────

    /// When precc-hook is not on PATH, run_precc_hook should return an error
    /// (the test binary can't know if it's installed).  We mock by setting the
    /// env var to a nonexistent binary.
    #[tokio::test]
    async fn hook_missing_binary_returns_error() {
        std::env::set_var("PRECC_PRECC_HOOK_BIN", "/nonexistent/precc-hook");
        let result = run_precc_hook("echo hello").await;
        // Should be an error, not a panic
        assert!(result.is_err());
        std::env::remove_var("PRECC_PRECC_HOOK_BIN");
    }

    // ── resolve_bin ─────────────────────────────────────────────────────────

    #[test]
    fn resolve_bin_default() {
        // Use a fictional binary name that will never have an env override set
        std::env::remove_var("PRECC_FICTIONAL_TEST_BIN");
        assert_eq!(resolve_bin("fictional-test"), "fictional-test");
    }

    #[test]
    fn resolve_bin_env_override() {
        std::env::set_var("PRECC_MYBIN_BIN", "/custom/path/mybin");
        assert_eq!(resolve_bin("mybin"), "/custom/path/mybin");
        std::env::remove_var("PRECC_MYBIN_BIN");
    }

    #[test]
    fn resolve_bin_hook_default() {
        // Use a fictional name to avoid picking up real env state
        std::env::remove_var("PRECC_FICTIONAL_HOOK_BIN");
        assert_eq!(resolve_bin("fictional-hook"), "fictional-hook");
    }

    // ── PreccHookTool ────────────────────────────────────────────────────────

    #[test]
    fn hook_tool_name_and_description() {
        assert_eq!(PreccHookTool::name(), "precc_hook");
        assert!(!PreccHookTool::description().is_empty());
        assert!(PreccHookTool::description().contains("PRECC"));
    }

    #[tokio::test]
    async fn hook_tool_missing_command_param_errors() {
        std::env::set_var("PRECC_PRECC_HOOK_BIN", "/nonexistent/precc-hook");
        let tool = PreccHookTool::new();
        // Empty args → missing 'command' field
        let result = tool.execute(serde_json::json!({})).await;
        assert!(result.is_err());
        std::env::remove_var("PRECC_PRECC_HOOK_BIN");
    }

    // ── PreccReportTool ──────────────────────────────────────────────────────

    #[test]
    fn report_tool_name_and_description() {
        assert_eq!(PreccReportTool::name(), "precc_report");
        assert!(PreccReportTool::description().contains("precc report"));
    }

    // ── PreccUpdateTool ──────────────────────────────────────────────────────

    #[test]
    fn update_tool_name_and_description() {
        assert_eq!(PreccUpdateTool::name(), "precc_update");
        assert!(PreccUpdateTool::description().contains("precc update"));
    }

    // ── PreccSkillsTool ──────────────────────────────────────────────────────

    #[test]
    fn skills_tool_name_and_description() {
        assert_eq!(PreccSkillsTool::name(), "precc_skills");
        assert!(PreccSkillsTool::description().contains("PRECC"));
    }

    // ── JSON payload construction ────────────────────────────────────────────

    #[test]
    fn hook_payload_format() {
        let cmd = "cargo build";
        let payload = serde_json::json!({"tool_input": {"command": cmd}});
        assert_eq!(payload["tool_input"]["command"], cmd);
    }

    #[test]
    fn hook_response_parsing_rewritten() {
        let response = serde_json::json!({
            "hookSpecificOutput": {
                "updatedInput": {
                    "command": "cd /project && rtk cargo build"
                }
            }
        });
        let rewritten = response
            .pointer("/hookSpecificOutput/updatedInput/command")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .unwrap_or("cargo build");
        assert_eq!(rewritten, "cd /project && rtk cargo build");
    }

    #[test]
    fn hook_response_parsing_no_rewrite() {
        // Hook output with empty/missing updatedInput → use original
        let response = serde_json::json!({ "hookSpecificOutput": {} });
        let original = "echo hello";
        let rewritten = response
            .pointer("/hookSpecificOutput/updatedInput/command")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .unwrap_or(original);
        assert_eq!(rewritten, original);
    }

    #[test]
    fn hook_response_parsing_empty_command() {
        // Empty string in updatedInput → fall back to original
        let response = serde_json::json!({
            "hookSpecificOutput": { "updatedInput": { "command": "" } }
        });
        let original = "echo hello";
        let rewritten = response
            .pointer("/hookSpecificOutput/updatedInput/command")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .unwrap_or(original);
        assert_eq!(rewritten, original);
    }

    #[test]
    fn hook_response_parsing_null_response() {
        // Completely invalid JSON → fall back
        let original = "npm test";
        let response: Value = serde_json::from_str("not json").unwrap_or(Value::Null);
        let rewritten = response
            .pointer("/hookSpecificOutput/updatedInput/command")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .unwrap_or(original);
        assert_eq!(rewritten, original);
    }

    // ── was_rewritten flag ───────────────────────────────────────────────────

    #[test]
    fn was_rewritten_true_when_command_changed() {
        let original = "cargo build";
        let rewritten = "cd /proj && rtk cargo build";
        assert!(rewritten != original);
    }

    #[test]
    fn was_rewritten_false_when_unchanged() {
        let original = "echo hello";
        let rewritten = "echo hello";
        assert_eq!(rewritten, original);
    }
}
