//! Diet — rule-based trajectory output slimming.
//!
//! Inspired by AgentDiet (Xiao et al., FSE 2026), this module reduces token
//! waste in LLM agent trajectories by rewriting commands to produce leaner
//! output.  Unlike AgentDiet which uses an LLM reflection module *after* tool
//! execution, diet rules operate *before* execution in the PreToolUse hook,
//! achieving zero-cost compression (<1ms).
//!
//! Three waste categories from the AgentDiet taxonomy:
//!   1. **Useless info** — verbose noise (74 PASSED test lines, __pycache__
//!      in directory listings, make entering/leaving messages).
//!   2. **Redundant info** — duplicate output across steps (partially handled
//!      by post_observe duplicate detection).
//!   3. **Expired info** — stale context (partially handled by post_observe
//!      context pressure warnings).
//!
//! This module targets category 1 with rule-based pipe filters and flag
//! injection.  Categories 2 and 3 are addressed by `post_observe.rs`.

use std::sync::OnceLock;

/// Whether diet rewriting is enabled.
/// Enabled by default; set `PRECC_DIET=0` to disable.
pub fn diet_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("PRECC_DIET")
            .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
            .unwrap_or(true)
    })
}

/// How a diet rule transforms the command.
enum DietTransform {
    /// Append a pipe filter: `cmd 2>&1 | filter`
    AppendPipe(&'static str),
    /// Add a flag if not already present.
    AddFlag {
        flag: &'static str,
        /// Skip if command already contains this string.
        already: &'static str,
    },
}

/// A single diet rule.
struct DietRule {
    /// Command must start with one of these prefixes.
    prefixes: &'static [&'static str],
    /// Skip if command contains any of these.
    skip_if: &'static [&'static str],
    /// How to transform the command.
    transform: DietTransform,
    /// Estimated tokens saved per application.
    est_tokens_saved: u32,
}

/// All diet rules, checked in order; first match wins.
static RULES: &[DietRule] = &[
    // ── Test output: strip passing-test lines ───────────────────────────
    DietRule {
        prefixes: &["pytest", "python -m pytest"],
        skip_if: &["|", ">", "&&", "||", ";", "--tb=no", "-q"],
        transform: DietTransform::AppendPipe(
            r#"grep -vE "^(tests/\S+\s+PASSED|PASSED\s|\.+$|\s+\d+%\|)""#,
        ),
        est_tokens_saved: 300,
    },
    DietRule {
        prefixes: &["go test"],
        skip_if: &["|", ">", "&&", "||", ";"],
        transform: DietTransform::AppendPipe(r#"grep -vE "^--- PASS:|^ok\s""#),
        est_tokens_saved: 200,
    },
    // ── Directory listings: strip build artifacts ────────────────────────
    DietRule {
        prefixes: &["ls ", "ls\t"],
        skip_if: &["|", ">", "&&", "||", ";"],
        transform: DietTransform::AppendPipe(
            r#"grep -vE "(__pycache__|\.pyc$|\.egg-info|node_modules|\.DS_Store)""#,
        ),
        est_tokens_saved: 80,
    },
    DietRule {
        prefixes: &["find "],
        skip_if: &["|", ">", "&&", "||", ";", "-exec", "-delete"],
        transform: DietTransform::AppendPipe(
            r#"grep -vE "(__pycache__|\.pyc$|\.egg-info|node_modules|/\.git/|target/debug/\.fingerprint)""#,
        ),
        est_tokens_saved: 120,
    },
    // ── Build noise: silent make ─────────────────────────────────────────
    DietRule {
        prefixes: &["make "],
        skip_if: &["|", ">", "&&", "||", ";", "-s", "--silent", "--quiet"],
        transform: DietTransform::AddFlag {
            flag: "-s",
            already: "-s",
        },
        est_tokens_saved: 200,
    },
    // ── Cargo build: strip "Fresh" lines ─────────────────────────────────
    DietRule {
        prefixes: &["cargo build", "cargo check"],
        skip_if: &["|", ">", "&&", "||", ";"],
        transform: DietTransform::AppendPipe(r#"grep -vE "^\s+(Fresh|Compiling) \S+ v""#),
        est_tokens_saved: 200,
    },
    // ── pip install: suppress progress ───────────────────────────────────
    DietRule {
        prefixes: &["pip install", "pip3 install", "uv pip install"],
        skip_if: &["|", ">", "&&", "||", ";", "--quiet", "-q"],
        transform: DietTransform::AddFlag {
            flag: "--quiet",
            already: "--quiet",
        },
        est_tokens_saved: 100,
    },
    // ── npm install: suppress audit/fund noise ──────────────────────────
    DietRule {
        prefixes: &["npm install", "npm i "],
        skip_if: &["|", ">", "&&", "||", ";", "--no-audit"],
        transform: DietTransform::AddFlag {
            flag: "--no-audit --no-fund",
            already: "--no-audit",
        },
        est_tokens_saved: 80,
    },
    // ── docker: suppress pull/build progress ────────────────────────────
    DietRule {
        prefixes: &["docker build", "docker pull"],
        skip_if: &["|", ">", "&&", "||", ";", "--quiet", "-q"],
        transform: DietTransform::AddFlag {
            flag: "--quiet",
            already: "--quiet",
        },
        est_tokens_saved: 150,
    },
];

/// Apply diet rules to a command.
///
/// Returns `Some((rewritten_command, est_tokens_saved))` if a rule matched,
/// `None` otherwise.
pub fn apply(command: &str) -> Option<(String, u32)> {
    if !diet_enabled() {
        return None;
    }

    let cmd = command.trim();
    if cmd.is_empty() {
        return None;
    }

    // Skip heredocs — pipe appending would break them
    if cmd.contains("<<") {
        return None;
    }

    for rule in RULES {
        if !rule.prefixes.iter().any(|p| cmd.starts_with(p)) {
            continue;
        }
        if rule.skip_if.iter().any(|s| cmd.contains(s)) {
            continue;
        }

        let rewritten = match &rule.transform {
            DietTransform::AppendPipe(filter) => {
                format!("{cmd} 2>&1 | {filter}")
            }
            DietTransform::AddFlag { flag, already } => {
                if cmd.contains(already) {
                    continue;
                }
                // Insert flag after the first word (command name)
                if let Some(space_idx) = cmd.find(' ') {
                    let (base, rest) = cmd.split_at(space_idx);
                    format!("{base} {flag}{rest}")
                } else {
                    format!("{cmd} {flag}")
                }
            }
        };

        return Some((rewritten, rule.est_tokens_saved));
    }

    None
}

/// Estimate tokens saved if a diet rule would match this command (for telemetry).
pub fn estimate_savings(command: &str) -> u32 {
    if !diet_enabled() {
        return 0;
    }
    let cmd = command.trim();
    if cmd.contains("<<") {
        return 0;
    }
    for rule in RULES {
        if rule.prefixes.iter().any(|p| cmd.starts_with(p))
            && !rule.skip_if.iter().any(|s| cmd.contains(s))
        {
            return rule.est_tokens_saved;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pytest_gets_pipe() {
        let (out, saved) = apply("pytest tests/").unwrap();
        assert!(out.starts_with("pytest tests/ 2>&1 | grep"));
        assert!(saved > 0);
    }

    #[test]
    fn pytest_with_pipe_skipped() {
        assert!(apply("pytest tests/ | head").is_none());
    }

    #[test]
    fn pytest_with_quiet_skipped() {
        assert!(apply("pytest tests/ -q").is_none());
    }

    #[test]
    fn go_test_gets_pipe() {
        let (out, _) = apply("go test ./...").unwrap();
        assert!(out.contains("grep -vE"));
    }

    #[test]
    fn ls_strips_pycache() {
        let (out, _) = apply("ls -la src/").unwrap();
        assert!(out.contains("__pycache__"));
        assert!(out.contains("grep -vE"));
    }

    #[test]
    fn ls_with_pipe_skipped() {
        assert!(apply("ls -la | wc -l").is_none());
    }

    #[test]
    fn find_strips_artifacts() {
        let (out, _) = apply("find . -name '*.py'").unwrap();
        assert!(out.contains("node_modules"));
    }

    #[test]
    fn find_with_exec_skipped() {
        assert!(apply("find . -name '*.py' -exec rm {} ;").is_none());
    }

    #[test]
    fn make_gets_silent_flag() {
        let (out, _) = apply("make all").unwrap();
        assert_eq!(out, "make -s all");
    }

    #[test]
    fn make_already_silent_skipped() {
        assert!(apply("make -s all").is_none());
    }

    #[test]
    fn cargo_build_strips_fresh() {
        let (out, _) = apply("cargo build --release").unwrap();
        assert!(out.contains("grep -vE"));
        assert!(out.contains("Fresh"));
    }

    #[test]
    fn pip_install_gets_quiet() {
        let (out, _) = apply("pip install requests").unwrap();
        assert_eq!(out, "pip --quiet install requests");
    }

    #[test]
    fn pip_already_quiet_skipped() {
        assert!(apply("pip install --quiet requests").is_none());
    }

    #[test]
    fn npm_install_gets_no_audit() {
        let (out, _) = apply("npm install express").unwrap();
        assert!(out.contains("--no-audit"));
        assert!(out.contains("--no-fund"));
    }

    #[test]
    fn docker_build_gets_quiet() {
        let (out, _) = apply("docker build .").unwrap();
        assert!(out.contains("--quiet"));
    }

    #[test]
    fn heredoc_skipped() {
        assert!(apply("cat <<EOF\nhello\nEOF").is_none());
    }

    #[test]
    fn empty_skipped() {
        assert!(apply("").is_none());
        assert!(apply("  ").is_none());
    }

    #[test]
    fn compound_command_skipped() {
        assert!(apply("pytest tests/ && echo done").is_none());
    }

    #[test]
    fn disabled_via_env() {
        // Can't easily test OnceLock, but verify estimate_savings returns 0
        // when diet_enabled() would be false. Tested manually.
    }

    #[test]
    fn estimate_savings_matches_apply() {
        let cmds = &[
            "pytest tests/",
            "go test ./...",
            "ls -la src/",
            "make all",
            "cargo build",
            "echo hello",
        ];
        for cmd in cmds {
            let est = estimate_savings(cmd);
            match apply(cmd) {
                Some((_, saved)) => assert_eq!(est, saved, "mismatch for: {cmd}"),
                None => assert_eq!(est, 0, "expected 0 for: {cmd}"),
            }
        }
    }
}
