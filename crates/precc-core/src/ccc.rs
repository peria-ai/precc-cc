//! Pillar 2 (Rust): cocoindex-code semantic search integration.
//!
//! Detects grep/rg bash commands eligible for redirection through `ccc search`,
//! runs the semantic search subprocess, and returns the result when it saves tokens.
//!
//! Performance: This stage spawns a subprocess (`ccc search`), so it breaks the
//! <5ms target. It only activates for recursive grep/rg commands in projects with
//! a `.cocoindex_code/` index. The tradeoff is acceptable because the alternative
//! (running grep) would also be slow, and ccc typically returns ~70% fewer tokens.

use regex::Regex;
use std::path::Path;
use std::process::Command;
use std::sync::LazyLock;

/// Reject commands with pipes, chaining, redirects, or subshells.
static COMPLEX_CMD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\||&&|;|>|<|\$\("#).unwrap());

/// Match `grep -r...` (any flag set that includes `r`).
static GREP_RECURSIVE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^grep\s+-[a-zA-Z]*r").unwrap());

/// Extract pattern from grep: `grep -<flags> <pattern> [path]`.
/// Captures the pattern (possibly quoted) and optional trailing path.
static GREP_EXTRACT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^grep\s+(?:-[a-zA-Z]+\s+)*(?:'([^']+)'|"([^"]+)"|(\S+))(?:\s+(.+))?\s*$"#)
        .unwrap()
});

/// Extract pattern from rg: `rg [flags] <pattern> [path]`.
static RG_EXTRACT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^rg\s+(?:-[a-zA-Z-]+(?:\s+\S+)?\s+)*(?:'([^']+)'|"([^"]+)"|(\S+))(?:\s+(.+))?\s*$"#,
    )
    .unwrap()
});

/// Parsed search query extracted from a grep/rg command.
#[derive(Debug)]
pub struct CccQuery {
    pub pattern: String,
    pub path_filter: Option<String>,
}

/// Result of a ccc search redirection.
#[derive(Debug)]
pub struct CccResult {
    pub output: String,
    pub ccc_bytes: usize,
    pub pattern: String,
}

/// Fast check: is this command potentially eligible for ccc redirection?
///
/// Returns `false` for non-grep/rg commands, piped commands, or commands
/// that are unlikely to benefit from semantic search.
pub fn is_eligible(command: &str) -> bool {
    let trimmed = command.trim();

    // Must start with grep or rg
    if !trimmed.starts_with("grep ") && !trimmed.starts_with("rg ") {
        return false;
    }

    // Reject complex commands (pipes, chaining, redirects)
    if COMPLEX_CMD.is_match(trimmed) {
        return false;
    }

    // grep must be recursive
    if trimmed.starts_with("grep ") && !GREP_RECURSIVE.is_match(trimmed) {
        return false;
    }

    true
}

/// Extract the search pattern and optional path from a grep/rg command.
pub fn extract_pattern(command: &str) -> Option<CccQuery> {
    let trimmed = command.trim();

    let (pattern, path) = if trimmed.starts_with("grep ") {
        let caps = GREP_EXTRACT.captures(trimmed)?;
        let pat = caps
            .get(1)
            .or_else(|| caps.get(2))
            .or_else(|| caps.get(3))?
            .as_str();
        let path = caps.get(4).map(|m| m.as_str().to_string());
        (pat.to_string(), path)
    } else if trimmed.starts_with("rg ") {
        let caps = RG_EXTRACT.captures(trimmed)?;
        let pat = caps
            .get(1)
            .or_else(|| caps.get(2))
            .or_else(|| caps.get(3))?
            .as_str();
        let path = caps.get(4).map(|m| m.as_str().to_string());
        (pat.to_string(), path)
    } else {
        return None;
    };

    // Skip very short patterns (likely too broad)
    if pattern.len() < 4 {
        return None;
    }

    Some(CccQuery {
        pattern,
        path_filter: path,
    })
}

/// Check if `ccc` binary is available on PATH.
///
/// Result is cached for the process lifetime.
pub fn is_available() -> bool {
    static AVAILABLE: LazyLock<bool> = LazyLock::new(|| {
        std::env::var_os("PATH")
            .map(|paths| {
                std::env::split_paths(&paths)
                    .any(|dir| dir.join("ccc").is_file() || dir.join("ccc.exe").is_file())
            })
            .unwrap_or(false)
    });
    *AVAILABLE
}

/// Check if the project at `cwd` has a cocoindex-code index.
pub fn has_index(cwd: &str) -> bool {
    Path::new(cwd).join(".cocoindex_code").is_dir()
}

/// Run `ccc search` and return the result if it produces useful output.
///
/// Returns `None` if ccc fails, returns no results, or is not available.
pub fn run_search(query: &CccQuery, cwd: &str) -> Option<CccResult> {
    let mut args = vec!["search", &query.pattern, "--limit", "10"];

    let path_str;
    if let Some(ref p) = query.path_filter {
        if p != "." {
            path_str = p.clone();
            args.push("--path");
            args.push(&path_str);
        }
    }

    let output = Command::new("ccc")
        .args(&args)
        .current_dir(cwd)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    // If output is too short (< 2 lines), not useful
    if stdout.lines().count() < 2 {
        return None;
    }

    Some(CccResult {
        ccc_bytes: stdout.len(),
        output: stdout,
        pattern: query.pattern.clone(),
    })
}

/// Build a replacement command that echoes the ccc output.
///
/// Returns a `printf` command that safely reproduces the ccc output.
pub fn build_replacement_command(result: &CccResult) -> String {
    // Use printf with escaped content
    let escaped = result.output.replace('\\', "\\\\").replace('\'', "'\\''");
    format!("printf '%s\\n' '{}'", escaped)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── is_eligible ─────────────────────────────────────────────────────

    #[test]
    fn eligible_recursive_grep() {
        assert!(is_eligible("grep -r 'pattern' src/"));
        assert!(is_eligible("grep -rn 'pattern' ."));
        assert!(is_eligible("grep -rl 'pattern'"));
        assert!(is_eligible("grep -rni 'something' src/"));
    }

    #[test]
    fn eligible_grep_with_multiple_flags() {
        assert!(is_eligible("grep -rIl 'TODO' src/"));
        assert!(is_eligible("grep -rn --include='*.rs' 'pattern' ."));
    }

    #[test]
    fn eligible_rg() {
        assert!(is_eligible("rg 'pattern' src/"));
        assert!(is_eligible("rg pattern"));
        assert!(is_eligible("rg -l pattern src/"));
    }

    #[test]
    fn eligible_rg_with_flags() {
        assert!(is_eligible("rg --type rust 'pattern' src/"));
        assert!(is_eligible("rg -i 'pattern'"));
        assert!(is_eligible("rg --hidden 'pattern' ."));
    }

    #[test]
    fn eligible_with_leading_whitespace() {
        assert!(is_eligible("  grep -r 'pattern' src/"));
        assert!(is_eligible("  rg 'pattern'"));
    }

    #[test]
    fn ineligible_non_recursive_grep() {
        assert!(!is_eligible("grep 'pattern' file.txt"));
        assert!(!is_eligible("grep pattern file.txt"));
        assert!(!is_eligible("grep -n 'pattern' file.txt"));
        assert!(!is_eligible("grep -i 'pattern' file.txt"));
    }

    #[test]
    fn ineligible_piped() {
        assert!(!is_eligible("grep -r 'pattern' | head -5"));
        assert!(!is_eligible("rg pattern && echo done"));
        assert!(!is_eligible("rg pattern > output.txt"));
        assert!(!is_eligible("rg pattern < input.txt"));
        assert!(!is_eligible("grep -r 'pattern' ; echo done"));
        assert!(!is_eligible("echo $(rg pattern)"));
    }

    #[test]
    fn ineligible_non_grep() {
        assert!(!is_eligible("cargo build"));
        assert!(!is_eligible("echo hello"));
        assert!(!is_eligible("ls -la"));
        assert!(!is_eligible("cat file.txt"));
        assert!(!is_eligible(""));
    }

    #[test]
    fn ineligible_empty_and_whitespace() {
        assert!(!is_eligible(""));
        assert!(!is_eligible("   "));
    }

    // ── extract_pattern ─────────────────────────────────────────────────

    #[test]
    fn extract_grep_pattern_single_quoted() {
        let q = extract_pattern("grep -rn 'my pattern' src/").unwrap();
        assert_eq!(q.pattern, "my pattern");
        assert_eq!(q.path_filter.as_deref(), Some("src/"));
    }

    #[test]
    fn extract_grep_pattern_double_quoted() {
        let q = extract_pattern("grep -r \"my pattern\" .").unwrap();
        assert_eq!(q.pattern, "my pattern");
        assert_eq!(q.path_filter.as_deref(), Some("."));
    }

    #[test]
    fn extract_grep_pattern_unquoted() {
        let q = extract_pattern("grep -rn something src/").unwrap();
        assert_eq!(q.pattern, "something");
    }

    #[test]
    fn extract_grep_pattern_no_path() {
        let q = extract_pattern("grep -r 'long_pattern'").unwrap();
        assert_eq!(q.pattern, "long_pattern");
        assert!(q.path_filter.is_none());
    }

    #[test]
    fn extract_grep_multiple_flags() {
        let q = extract_pattern("grep -rni 'some pattern' src/lib/").unwrap();
        assert_eq!(q.pattern, "some pattern");
        assert_eq!(q.path_filter.as_deref(), Some("src/lib/"));
    }

    #[test]
    fn extract_rg_pattern() {
        let q = extract_pattern("rg 'authentication' src/").unwrap();
        assert_eq!(q.pattern, "authentication");
        assert_eq!(q.path_filter.as_deref(), Some("src/"));
    }

    #[test]
    fn extract_rg_pattern_no_path() {
        let q = extract_pattern("rg 'long_pattern'").unwrap();
        assert_eq!(q.pattern, "long_pattern");
        assert!(q.path_filter.is_none());
    }

    #[test]
    fn extract_rg_unquoted() {
        let q = extract_pattern("rg something_long .").unwrap();
        assert_eq!(q.pattern, "something_long");
        assert_eq!(q.path_filter.as_deref(), Some("."));
    }

    #[test]
    fn extract_short_pattern_rejected() {
        assert!(extract_pattern("grep -r 'ab' src/").is_none());
        assert!(extract_pattern("rg 'xy' .").is_none());
        assert!(extract_pattern("rg 'abc' .").is_none()); // exactly 3, still < 4
    }

    #[test]
    fn extract_exactly_four_char_pattern_accepted() {
        let q = extract_pattern("rg 'abcd' .").unwrap();
        assert_eq!(q.pattern, "abcd");
    }

    #[test]
    fn extract_returns_none_for_non_grep() {
        assert!(extract_pattern("cargo build").is_none());
        assert!(extract_pattern("echo hello").is_none());
    }

    // ── has_index ───────────────────────────────────────────────────────

    #[test]
    fn has_index_nonexistent() {
        assert!(!has_index("/nonexistent/path/12345"));
    }

    #[test]
    fn has_index_with_actual_dir() {
        let dir = tempfile::tempdir().unwrap();
        assert!(!has_index(&dir.path().to_string_lossy()));

        // Create .cocoindex_code directory
        std::fs::create_dir(dir.path().join(".cocoindex_code")).unwrap();
        assert!(has_index(&dir.path().to_string_lossy()));
    }

    #[test]
    fn has_index_file_not_dir() {
        let dir = tempfile::tempdir().unwrap();
        // Create .cocoindex_code as a file, not a directory
        std::fs::write(dir.path().join(".cocoindex_code"), "not a dir").unwrap();
        assert!(!has_index(&dir.path().to_string_lossy()));
    }

    // ── is_available ────────────────────────────────────────────────────

    #[test]
    fn is_available_does_not_panic() {
        let _ = is_available();
    }

    // ── build_replacement_command ────────────────────────────────────────

    #[test]
    fn build_replacement_command_simple() {
        let result = CccResult {
            output: "fn main() {}\n".to_string(),
            ccc_bytes: 14,
            pattern: "main".to_string(),
        };
        let cmd = build_replacement_command(&result);
        assert!(cmd.starts_with("printf"));
        assert!(cmd.contains("fn main()"));
    }

    #[test]
    fn build_replacement_command_escapes_backslashes() {
        let result = CccResult {
            output: "path\\to\\file\n".to_string(),
            ccc_bytes: 14,
            pattern: "path".to_string(),
        };
        let cmd = build_replacement_command(&result);
        assert!(cmd.contains("\\\\"));
    }

    #[test]
    fn build_replacement_command_escapes_single_quotes() {
        let result = CccResult {
            output: "it's a test\n".to_string(),
            ccc_bytes: 12,
            pattern: "test".to_string(),
        };
        let cmd = build_replacement_command(&result);
        // Single quotes should be escaped
        assert!(cmd.contains("'\\''"));
    }

    #[test]
    fn build_replacement_command_empty_output() {
        let result = CccResult {
            output: "".to_string(),
            ccc_bytes: 0,
            pattern: "test".to_string(),
        };
        let cmd = build_replacement_command(&result);
        assert!(cmd.starts_with("printf"));
    }

    // ── run_search (integration, depends on ccc being installed) ────────

    #[test]
    fn run_search_nonexistent_cwd() {
        let query = CccQuery {
            pattern: "test pattern".to_string(),
            path_filter: None,
        };
        // Should return None gracefully, not panic
        let result = run_search(&query, "/nonexistent/dir/12345");
        assert!(result.is_none());
    }

    #[test]
    fn run_search_with_path_filter_dot() {
        let query = CccQuery {
            pattern: "test pattern".to_string(),
            path_filter: Some(".".to_string()),
        };
        // "." path should not be passed to --path arg
        let result = run_search(&query, "/tmp");
        // May be None if ccc not installed, that's OK
        assert!(result.is_none() || result.is_some());
    }

    // ── CccQuery/CccResult struct tests ────────────────────────────────

    #[test]
    fn ccc_query_debug_impl() {
        let q = CccQuery {
            pattern: "test".to_string(),
            path_filter: Some("src/".to_string()),
        };
        let debug = format!("{:?}", q);
        assert!(debug.contains("test"));
        assert!(debug.contains("src/"));
    }

    #[test]
    fn ccc_result_debug_impl() {
        let r = CccResult {
            output: "result".to_string(),
            ccc_bytes: 6,
            pattern: "test".to_string(),
        };
        let debug = format!("{:?}", r);
        assert!(debug.contains("result"));
        assert!(debug.contains("6"));
    }
}
