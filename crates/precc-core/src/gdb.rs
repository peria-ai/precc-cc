//! Pillar 2: GDB-based debugging.
//!
//! Detects commands that could benefit from GDB-based debugging instead
//! of the edit-compile-read-edit cycle. The hook suggests `precc debug`
//! as an alternative when appropriate.
//!
//! Performance: Uses string prefix matching instead of regex.

use std::sync::LazyLock;

/// Repeated failure threshold — if the same command has failed this many
/// times recently, suggest GDB.
const REPEATED_FAILURE_THRESHOLD: u32 = 2;

/// Check if a command could benefit from GDB-based debugging.
///
/// Returns a suggestion string if GDB is appropriate, None otherwise.
pub fn check_opportunity(command: &str, recent_failure_count: u32) -> Option<String> {
    // Only suggest GDB if the command matches a debuggable pattern
    if !is_debuggable(command) {
        return None;
    }

    // Only suggest if the command has failed repeatedly
    if recent_failure_count < REPEATED_FAILURE_THRESHOLD {
        return None;
    }

    // Build suggestion based on command type
    if command.starts_with("cargo test") || command.starts_with("cargo run") {
        Some(format!(
            "This command has failed {} times. Consider: precc debug {}",
            recent_failure_count,
            extract_binary_hint(command)
        ))
    } else {
        Some(format!(
            "This binary has failed {} times. Consider: precc debug {}",
            recent_failure_count, command
        ))
    }
}

/// Check if a command matches a debuggable pattern.
/// Uses string prefix matching for speed (no regex).
fn is_debuggable(command: &str) -> bool {
    let first_word = command.split_whitespace().next().unwrap_or("");

    // cargo test, cargo run
    if first_word == "cargo" {
        let second = command.split_whitespace().nth(1).unwrap_or("");
        return second == "test" || second == "run";
    }

    // ./target/debug/..., ./target/release/...
    if command.starts_with("./target/debug/")
        || command.starts_with("./target/release/")
        || command.starts_with("target/debug/")
        || command.starts_with("target/release/")
    {
        return true;
    }

    // ./binary (starts with ./ followed by alphanumeric)
    if let Some(rest) = command.strip_prefix("./") {
        if let Some(first_char) = rest.chars().next() {
            if first_char.is_alphanumeric() || first_char == '_' {
                return true;
            }
        }
    }

    // node --inspect
    if command.starts_with("node --inspect") {
        return true;
    }

    false
}

/// Extract a hint for what binary to debug from a cargo command.
fn extract_binary_hint(command: &str) -> String {
    if command.starts_with("cargo test") {
        "target/debug/<test-binary>".to_string()
    } else if command.starts_with("cargo run") {
        "target/debug/<binary>".to_string()
    } else {
        command.to_string()
    }
}

/// Check if GDB is available on this system.
/// Uses filesystem PATH check instead of spawning a subprocess for speed.
pub fn gdb_available() -> bool {
    static AVAILABLE: LazyLock<bool> = LazyLock::new(|| find_in_path("gdb"));
    *AVAILABLE
}

/// Check if a binary exists on PATH without spawning a subprocess.
fn find_in_path(binary: &str) -> bool {
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in path_var.split(':') {
            if std::path::Path::new(dir).join(binary).is_file() {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cargo_test_is_debuggable() {
        assert!(is_debuggable("cargo test"));
        assert!(is_debuggable("cargo test my_test"));
        assert!(is_debuggable("cargo run -- --flag"));
    }

    #[test]
    fn binary_is_debuggable() {
        assert!(is_debuggable("./target/debug/myapp"));
        assert!(is_debuggable("target/release/myapp --arg"));
        assert!(is_debuggable("./myapp"));
    }

    #[test]
    fn non_debuggable_commands() {
        assert!(!is_debuggable("cargo build"));
        assert!(!is_debuggable("git status"));
        assert!(!is_debuggable("echo hello"));
        assert!(!is_debuggable("ls -la"));
    }

    #[test]
    fn no_suggestion_below_threshold() {
        assert_eq!(check_opportunity("cargo test", 0), None);
        assert_eq!(check_opportunity("cargo test", 1), None);
    }

    #[test]
    fn suggests_at_threshold() {
        let suggestion = check_opportunity("cargo test", 2);
        assert!(suggestion.is_some());
        assert!(suggestion.unwrap().contains("precc debug"));
    }

    #[test]
    fn no_suggestion_for_non_debuggable() {
        assert_eq!(check_opportunity("cargo build", 5), None);
        assert_eq!(check_opportunity("git status", 10), None);
    }
}
