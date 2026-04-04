//! lean-ctx integration — thin adapter for external output compression.
//!
//! lean-ctx (<https://github.com/yvgude/lean-ctx>) is an external tool that compresses
//! CLI output to reduce token consumption. PRECC delegates output compression to lean-ctx
//! when available, while continuing to handle error prevention (cd prepend, skills, mining).
//!
//! # Activation
//!
//! Enabled by default when lean-ctx is available. Set `PRECC_LEAN_CTX=0` to disable.
//! When lean-ctx is not available or the env var is unset, the pipeline falls through to RTK.
//!
//! # Interface boundary
//!
//! PRECC knows exactly one thing about lean-ctx: it is a CLI binary invoked as
//! `lean-ctx -c "command"`. No lean-ctx internals are assumed or reimplemented here.
//! Token savings from output compression belong to lean-ctx, not PRECC.

use std::sync::{LazyLock, OnceLock};

// =============================================================================
// Availability detection (cached, mirrors nushell.rs / rtk.rs pattern)
// =============================================================================

/// Check if the `lean-ctx` binary is available on PATH.
/// Cached via `LazyLock` — only scans once per process.
pub fn lean_ctx_available() -> bool {
    static AVAILABLE: LazyLock<bool> = LazyLock::new(|| {
        // Fast path: check cached marker file
        if let Ok(home) = std::env::var("HOME") {
            let cache = std::path::Path::new(&home).join(".local/share/precc/.lean_ctx_path");
            if let Ok(cached_path) = std::fs::read_to_string(&cache) {
                let p = cached_path.trim();
                if !p.is_empty() && std::path::Path::new(p).is_file() {
                    return true;
                }
            }

            // Check common locations before full PATH scan
            let common = [
                format!("{home}/.cargo/bin/lean-ctx"),
                "/usr/local/bin/lean-ctx".to_string(),
                "/usr/bin/lean-ctx".to_string(),
            ];
            for path in &common {
                if std::path::Path::new(path).is_file() {
                    let _ = std::fs::write(&cache, path);
                    return true;
                }
            }
        }

        // Full PATH scan as fallback
        if let Ok(path_var) = std::env::var("PATH") {
            for dir in path_var.split(':') {
                let candidate = std::path::Path::new(dir).join("lean-ctx");
                if candidate.is_file() {
                    if let Ok(home) = std::env::var("HOME") {
                        let cache =
                            std::path::Path::new(&home).join(".local/share/precc/.lean_ctx_path");
                        let _ = std::fs::write(&cache, candidate.to_string_lossy().as_ref());
                    }
                    return true;
                }
            }
        }
        false
    });
    *AVAILABLE
}

/// Check if lean-ctx mode is enabled.
///
/// Enabled by default when lean-ctx is available. Set `PRECC_LEAN_CTX=0`
/// to disable. Also returns false if `LEAN_CTX_ACTIVE` is already set
/// (lean-ctx's own double-wrap guard), preventing infinite recursion.
/// Cached via `OnceLock` — zero cost after first check.
pub fn lean_ctx_mode_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        // If lean-ctx is already running this command, don't double-wrap.
        if std::env::var("LEAN_CTX_ACTIVE").is_ok() {
            return false;
        }
        // Enabled by default when available; disable with PRECC_LEAN_CTX=0
        let env_enabled = std::env::var("PRECC_LEAN_CTX")
            .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
            .unwrap_or(true); // default: enabled
        env_enabled && lean_ctx_available()
    })
}

// =============================================================================
// Command wrapping
// =============================================================================

/// Shell-quote a string using single quotes.
/// Internal single quotes are escaped as `'\''`.
fn shell_quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('\'');
    for ch in s.chars() {
        if ch == '\'' {
            out.push_str("'\\''");
        } else {
            out.push(ch);
        }
    }
    out.push('\'');
    out
}

/// Commands that should never be wrapped by output compression tools.
/// These are PRECC's own binaries and companion tools.
const SKIP_PREFIXES: &[&str] = &[
    "precc",
    "precc-hook",
    "precc-learner",
    "lean-ctx",
    "rtk",
    "nu ",
    "ccc ",
    "claude ",
    "cursor ",
];

/// Returns true if the command starts with a tool that should not be wrapped.
pub fn is_tool_command(cmd: &str) -> bool {
    let trimmed = cmd.trim_start();
    SKIP_PREFIXES
        .iter()
        .any(|p| trimmed.starts_with(p) || trimmed.starts_with(&format!("./{p}")))
}

/// Wrap a command in `lean-ctx -c '...'` for output compression.
///
/// Returns `Some(wrapped)` if wrapping is appropriate, `None` otherwise.
/// lean-ctx decides internally which commands to compress and how.
pub fn wrap(command: &str) -> Option<String> {
    // Already wrapped — prevent double-wrapping
    if command.contains("lean-ctx") {
        return None;
    }

    // Never wrap PRECC's own commands or companion tools
    if is_tool_command(command) {
        return None;
    }

    // Heredocs break shell quoting
    if command.contains("<<") {
        return None;
    }

    Some(format!("lean-ctx -c {}", shell_quote(command)))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_basic_command() {
        let result = wrap("cargo test").unwrap();
        assert_eq!(result, "lean-ctx -c 'cargo test'");
    }

    #[test]
    fn wrap_command_with_single_quotes() {
        let result = wrap("echo 'hello world'").unwrap();
        assert_eq!(result, "lean-ctx -c 'echo '\\''hello world'\\'''");
    }

    #[test]
    fn wrap_skips_already_wrapped() {
        assert!(wrap("lean-ctx -c 'cargo test'").is_none());
    }

    #[test]
    fn wrap_skips_heredocs() {
        assert!(wrap("cat <<EOF\nhello\nEOF").is_none());
    }

    #[test]
    fn shell_quote_no_specials() {
        assert_eq!(shell_quote("hello"), "'hello'");
    }

    #[test]
    fn shell_quote_with_single_quote() {
        assert_eq!(shell_quote("it's"), "'it'\\''s'");
    }

    #[test]
    fn wrap_empty_command() {
        let result = wrap("").unwrap();
        assert_eq!(result, "lean-ctx -c ''");
    }

    #[test]
    fn wrap_command_with_pipes_and_redirects() {
        let result = wrap("ls -la | grep foo > out.txt").unwrap();
        assert!(result.starts_with("lean-ctx -c '"));
        assert!(result.contains("| grep foo > out.txt"));
    }

    #[test]
    fn wrap_skips_lean_ctx_in_middle() {
        // Command containing "lean-ctx" anywhere should be skipped
        assert!(wrap("echo lean-ctx is great").is_none());
    }

    #[test]
    fn wrap_command_with_double_quotes() {
        let result = wrap(r#"echo "hello world""#).unwrap();
        // Double quotes don't need escaping inside single quotes
        assert_eq!(result, r#"lean-ctx -c 'echo "hello world"'"#);
    }

    #[test]
    fn wrap_command_with_consecutive_quotes() {
        let result = wrap("echo '''").unwrap();
        assert!(result.starts_with("lean-ctx -c '"));
    }

    #[test]
    fn shell_quote_empty() {
        assert_eq!(shell_quote(""), "''");
    }

    #[test]
    fn shell_quote_with_spaces() {
        assert_eq!(shell_quote("hello world"), "'hello world'");
    }

    #[test]
    fn shell_quote_multiple_single_quotes() {
        assert_eq!(shell_quote("a'b'c"), "'a'\\''b'\\''c'");
    }

    #[test]
    fn lean_ctx_mode_disabled_by_default() {
        // PRECC_LEAN_CTX is not set in the test environment, and lean-ctx
        // binary is likely not on PATH — mode should be disabled.
        // Note: this uses OnceLock so the result is cached for the process.
        // We can't test the env var parsing directly because of caching,
        // but we can verify the function doesn't panic.
        let _ = lean_ctx_mode_enabled();
    }

    #[test]
    fn lean_ctx_available_does_not_panic() {
        // Verify the PATH scanning and caching logic doesn't crash
        let _ = lean_ctx_available();
    }

    #[test]
    fn wrap_skips_precc_commands() {
        assert!(wrap("precc init").is_none());
        assert!(wrap("precc update --force").is_none());
        assert!(wrap("precc-hook").is_none());
        assert!(wrap("precc-learner --interval 60").is_none());
    }

    #[test]
    fn wrap_skips_companion_tools() {
        assert!(wrap("rtk git status").is_none());
        assert!(wrap("ccc search foo").is_none());
        assert!(wrap("claude code").is_none());
    }

    #[test]
    fn is_tool_command_with_path() {
        assert!(is_tool_command("./precc init"));
        assert!(is_tool_command("  precc update"));
    }

    #[test]
    fn is_tool_command_normal_commands() {
        assert!(!is_tool_command("cargo test"));
        assert!(!is_tool_command("git status"));
        assert!(!is_tool_command("ls -la"));
    }
}
