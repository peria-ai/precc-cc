//! RTK command rewriting — ported from rtk-rewrite.sh.
//!
//! Rewrites raw CLI commands to their `rtk` equivalents for token-optimized output.
//! This is the final stage of the hook pipeline.
//!
//! Performance: Uses string prefix matching instead of regex for the hot path.

use std::sync::LazyLock;

/// A rewrite rule: if command starts with `from`, replace that prefix with `to`.
struct RewriteRule {
    from: &'static str,
    to: &'static str,
    /// Estimated tokens saved per rewrite (output compression).
    est_tokens_saved: u32,
}

/// Standard prefix-swap rewrite rules.
/// Checked in order; first match wins (longest prefixes before shorter ones).
static RULES: &[RewriteRule] = &[
    // --- Git commands ---
    RewriteRule {
        from: "git status",
        to: "rtk git status",
        est_tokens_saved: 160,
    },
    RewriteRule {
        from: "git diff",
        to: "rtk git diff",
        est_tokens_saved: 160,
    },
    RewriteRule {
        from: "git log",
        to: "rtk git log",
        est_tokens_saved: 160,
    },
    RewriteRule {
        from: "git show",
        to: "rtk git show",
        est_tokens_saved: 160,
    },
    RewriteRule {
        from: "git add",
        to: "rtk git add",
        est_tokens_saved: 60,
    },
    RewriteRule {
        from: "git commit",
        to: "rtk git commit",
        est_tokens_saved: 60,
    },
    RewriteRule {
        from: "git push",
        to: "rtk git push",
        est_tokens_saved: 60,
    },
    RewriteRule {
        from: "git pull",
        to: "rtk git pull",
        est_tokens_saved: 60,
    },
    RewriteRule {
        from: "git branch",
        to: "rtk git branch",
        est_tokens_saved: 60,
    },
    RewriteRule {
        from: "git fetch",
        to: "rtk git fetch",
        est_tokens_saved: 60,
    },
    RewriteRule {
        from: "git stash",
        to: "rtk git stash",
        est_tokens_saved: 60,
    },
    RewriteRule {
        from: "git worktree",
        to: "rtk git worktree",
        est_tokens_saved: 60,
    },
    // --- GitHub CLI ---
    RewriteRule {
        from: "gh pr",
        to: "rtk gh pr",
        est_tokens_saved: 200,
    },
    RewriteRule {
        from: "gh issue",
        to: "rtk gh issue",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "gh run",
        to: "rtk gh run",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "gh repo",
        to: "rtk gh repo",
        est_tokens_saved: 120,
    },
    RewriteRule {
        from: "gh api",
        to: "rtk gh api",
        est_tokens_saved: 150,
    },
    RewriteRule {
        from: "gh release",
        to: "rtk gh release",
        est_tokens_saved: 120,
    },
    // --- Cargo ---
    RewriteRule {
        from: "cargo test",
        to: "rtk cargo test",
        est_tokens_saved: 420,
    },
    RewriteRule {
        from: "cargo build",
        to: "rtk cargo build",
        est_tokens_saved: 420,
    },
    RewriteRule {
        from: "cargo clippy",
        to: "rtk cargo clippy",
        est_tokens_saved: 420,
    },
    RewriteRule {
        from: "cargo check",
        to: "rtk cargo check",
        est_tokens_saved: 300,
    },
    RewriteRule {
        from: "cargo run",
        to: "rtk cargo run",
        est_tokens_saved: 200,
    },
    RewriteRule {
        from: "cargo install",
        to: "rtk cargo install",
        est_tokens_saved: 150,
    },
    RewriteRule {
        from: "cargo fmt",
        to: "rtk cargo fmt",
        est_tokens_saved: 60,
    },
    // --- npm/yarn ---
    RewriteRule {
        from: "npm test",
        to: "rtk vitest run",
        est_tokens_saved: 420,
    },
    RewriteRule {
        from: "npm run",
        to: "rtk npm run",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "npm exec",
        to: "rtk npm exec",
        est_tokens_saved: 150,
    },
    RewriteRule {
        from: "npm install",
        to: "rtk npm install",
        est_tokens_saved: 150,
    },
    RewriteRule {
        from: "yarn test",
        to: "rtk vitest run",
        est_tokens_saved: 420,
    },
    RewriteRule {
        from: "yarn add",
        to: "rtk yarn add",
        est_tokens_saved: 100,
    },
    // --- npx (standalone) ---
    RewriteRule {
        from: "npx tsc",
        to: "rtk tsc",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "npx eslint",
        to: "rtk lint",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "npx biome",
        to: "rtk lint",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "npx prettier",
        to: "rtk prettier",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "npx next build",
        to: "rtk next",
        est_tokens_saved: 400,
    },
    RewriteRule {
        from: "npx vitest",
        to: "rtk vitest run",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "npx playwright",
        to: "rtk playwright",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "npx prisma",
        to: "rtk prisma",
        est_tokens_saved: 180,
    },
    // --- pnpm ---
    RewriteRule {
        from: "pnpm tsc",
        to: "rtk tsc",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "pnpm lint",
        to: "rtk lint",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "pnpm prettier",
        to: "rtk prettier",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "pnpm next build",
        to: "rtk next",
        est_tokens_saved: 400,
    },
    RewriteRule {
        from: "pnpm vitest",
        to: "rtk vitest run",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "pnpm playwright",
        to: "rtk playwright",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "pnpm prisma",
        to: "rtk prisma",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "pnpm test",
        to: "rtk vitest run",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "pnpm list",
        to: "rtk pnpm list",
        est_tokens_saved: 100,
    },
    RewriteRule {
        from: "pnpm ls",
        to: "rtk pnpm ls",
        est_tokens_saved: 100,
    },
    RewriteRule {
        from: "pnpm outdated",
        to: "rtk pnpm outdated",
        est_tokens_saved: 100,
    },
    RewriteRule {
        from: "pnpm install",
        to: "rtk pnpm install",
        est_tokens_saved: 100,
    },
    // --- Standalone JS/TS tooling ---
    RewriteRule {
        from: "tsc",
        to: "rtk tsc",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "eslint",
        to: "rtk lint",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "biome",
        to: "rtk lint",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "prettier",
        to: "rtk prettier",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "next build",
        to: "rtk next",
        est_tokens_saved: 400,
    },
    RewriteRule {
        from: "vitest",
        to: "rtk vitest run",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "jest",
        to: "rtk vitest run",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "playwright",
        to: "rtk playwright",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "prisma",
        to: "rtk prisma",
        est_tokens_saved: 180,
    },
    // --- Python ---
    RewriteRule {
        from: "python3 -m mypy",
        to: "rtk mypy",
        est_tokens_saved: 200,
    },
    RewriteRule {
        from: "python -m mypy",
        to: "rtk mypy",
        est_tokens_saved: 200,
    },
    RewriteRule {
        from: "mypy",
        to: "rtk mypy",
        est_tokens_saved: 200,
    },
    RewriteRule {
        from: "ruff check",
        to: "rtk ruff check",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "ruff format",
        to: "rtk ruff format",
        est_tokens_saved: 150,
    },
    RewriteRule {
        from: "python -m pytest",
        to: "rtk pytest",
        est_tokens_saved: 380,
    },
    RewriteRule {
        from: "pytest",
        to: "rtk pytest",
        est_tokens_saved: 380,
    },
    RewriteRule {
        from: "uv pip list",
        to: "rtk pip list",
        est_tokens_saved: 150,
    },
    RewriteRule {
        from: "uv pip outdated",
        to: "rtk pip outdated",
        est_tokens_saved: 150,
    },
    RewriteRule {
        from: "uv pip install",
        to: "rtk pip install",
        est_tokens_saved: 150,
    },
    RewriteRule {
        from: "pip3 list",
        to: "rtk pip list",
        est_tokens_saved: 150,
    },
    RewriteRule {
        from: "pip3 outdated",
        to: "rtk pip outdated",
        est_tokens_saved: 150,
    },
    RewriteRule {
        from: "pip3 install",
        to: "rtk pip install",
        est_tokens_saved: 150,
    },
    RewriteRule {
        from: "pip install",
        to: "rtk pip install",
        est_tokens_saved: 150,
    },
    RewriteRule {
        from: "pip list",
        to: "rtk pip list",
        est_tokens_saved: 150,
    },
    // --- Go ---
    RewriteRule {
        from: "go test",
        to: "rtk go test",
        est_tokens_saved: 380,
    },
    RewriteRule {
        from: "go build",
        to: "rtk go build",
        est_tokens_saved: 300,
    },
    RewriteRule {
        from: "go vet",
        to: "rtk go vet",
        est_tokens_saved: 150,
    },
    RewriteRule {
        from: "golangci-lint",
        to: "rtk golangci-lint",
        est_tokens_saved: 300,
    },
    // --- File operations ---
    RewriteRule {
        from: "cat",
        to: "rtk read",
        est_tokens_saved: 50,
    },
    RewriteRule {
        from: "head",
        to: "rtk read",
        est_tokens_saved: 50,
    },
    RewriteRule {
        from: "tail",
        to: "rtk read",
        est_tokens_saved: 50,
    },
    RewriteRule {
        from: "ls",
        to: "rtk ls",
        est_tokens_saved: 40,
    },
    RewriteRule {
        from: "find",
        to: "rtk find",
        est_tokens_saved: 80,
    },
    RewriteRule {
        from: "tree",
        to: "rtk tree",
        est_tokens_saved: 80,
    },
    RewriteRule {
        from: "diff",
        to: "rtk diff",
        est_tokens_saved: 100,
    },
    // --- rg/grep ---
    RewriteRule {
        from: "rg",
        to: "rtk grep",
        est_tokens_saved: 90,
    },
    RewriteRule {
        from: "grep",
        to: "rtk grep",
        est_tokens_saved: 90,
    },
    // --- Containers ---
    RewriteRule {
        from: "docker compose ps",
        to: "rtk docker compose ps",
        est_tokens_saved: 150,
    },
    RewriteRule {
        from: "docker compose logs",
        to: "rtk docker compose logs",
        est_tokens_saved: 200,
    },
    RewriteRule {
        from: "docker compose build",
        to: "rtk docker compose build",
        est_tokens_saved: 400,
    },
    RewriteRule {
        from: "docker build",
        to: "rtk docker build",
        est_tokens_saved: 500,
    },
    RewriteRule {
        from: "docker run",
        to: "rtk docker run",
        est_tokens_saved: 200,
    },
    RewriteRule {
        from: "docker exec",
        to: "rtk docker exec",
        est_tokens_saved: 150,
    },
    RewriteRule {
        from: "docker ps",
        to: "rtk docker ps",
        est_tokens_saved: 150,
    },
    RewriteRule {
        from: "docker images",
        to: "rtk docker images",
        est_tokens_saved: 150,
    },
    RewriteRule {
        from: "docker logs",
        to: "rtk docker logs",
        est_tokens_saved: 150,
    },
    RewriteRule {
        from: "kubectl describe",
        to: "rtk kubectl describe",
        est_tokens_saved: 300,
    },
    RewriteRule {
        from: "kubectl apply",
        to: "rtk kubectl apply",
        est_tokens_saved: 150,
    },
    RewriteRule {
        from: "kubectl get",
        to: "rtk kubectl get",
        est_tokens_saved: 180,
    },
    RewriteRule {
        from: "kubectl logs",
        to: "rtk kubectl logs",
        est_tokens_saved: 180,
    },
    // --- Network ---
    RewriteRule {
        from: "curl",
        to: "rtk curl",
        est_tokens_saved: 200,
    },
    RewriteRule {
        from: "wget",
        to: "rtk wget",
        est_tokens_saved: 100,
    },
    // --- AWS ---
    RewriteRule {
        from: "aws",
        to: "rtk aws",
        est_tokens_saved: 200,
    },
    // --- PostgreSQL ---
    RewriteRule {
        from: "psql",
        to: "rtk psql",
        est_tokens_saved: 180,
    },
    // --- make ---
    RewriteRule {
        from: "make",
        to: "rtk make",
        est_tokens_saved: 400,
    },
];

/// Estimate tokens saved by rewriting a command.
/// Returns the per-category estimate, or 0 if no rule matches.
pub fn tokens_saved(command: &str) -> u32 {
    let cmd = if command.starts_with("cd ") {
        if let Some(pos) = command.find(" && ") {
            &command[pos + 4..]
        } else {
            command
        }
    } else {
        command
    };

    for rule in RULES {
        if matches_prefix(cmd, rule.from) {
            return rule.est_tokens_saved;
        }
    }
    0
}

/// Attempt to rewrite a command to its RTK equivalent.
/// Returns `Some(rewritten)` if a rewrite was applied, `None` otherwise.
pub fn rewrite(command: &str) -> Option<String> {
    // Skip if already using rtk
    if command.starts_with("rtk ") || command.contains("/rtk ") {
        return None;
    }

    // Never wrap PRECC's own commands or companion tools
    if crate::lean_ctx::is_tool_command(command) {
        return None;
    }

    // Skip heredocs
    if command.contains("<<") {
        return None;
    }

    // Check if rtk is available (cached check)
    if !rtk_available() {
        return None;
    }

    // Try prefix-swap rewrites
    for rule in RULES {
        if matches_prefix(command, rule.from) {
            return Some(command.replacen(rule.from, rule.to, 1));
        }
    }

    None
}

/// Translate a git command to its jj (Jujutsu) equivalent.
///
/// Only fires when:
/// 1. `jj` is on PATH
/// 2. The current working directory (or any ancestor up to 5 levels) has a `.jj/` directory
///    (i.e., it's a jujutsu-colocated repo)
///
/// Returns `Some(jj_command)` if the git command has a known jj translation,
/// `None` otherwise (caller falls through to RTK rewriting).
pub fn jj_translate(command: &str) -> Option<String> {
    // Must start with "git " to be translatable
    if !command.starts_with("git ") {
        return None;
    }
    // Skip if jj is unavailable or not in a jj repo
    if !jj_available() || !in_jj_repo() {
        return None;
    }

    let rest = &command[4..]; // strip "git "
    translate_git_to_jj(rest)
}

/// Check if `jj` binary is available on PATH (cached).
fn jj_available() -> bool {
    static AVAILABLE: LazyLock<bool> = LazyLock::new(|| {
        if let Ok(home) = std::env::var("HOME") {
            let common = [
                format!("{home}/.cargo/bin/jj"),
                "/usr/local/bin/jj".to_string(),
                "/usr/bin/jj".to_string(),
            ];
            for path in &common {
                if std::path::Path::new(path).is_file() {
                    return true;
                }
            }
        }
        if let Ok(path_var) = std::env::var("PATH") {
            for dir in path_var.split(':') {
                if std::path::Path::new(dir).join("jj").is_file() {
                    return true;
                }
            }
        }
        false
    });
    *AVAILABLE
}

/// Check if the current working directory is inside a jj-colocated repo
/// (has a `.jj/` directory within 5 ancestor levels).
fn in_jj_repo() -> bool {
    let mut dir = match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => return false,
    };
    for _ in 0..5 {
        if dir.join(".jj").is_dir() {
            return true;
        }
        if !dir.pop() {
            break;
        }
    }
    false
}

/// Check if a command starts with a prefix, followed by whitespace or end of string.
/// This ensures "git status" matches "git status --short" but not "git statusbar".
fn matches_prefix(command: &str, prefix: &str) -> bool {
    if command == prefix {
        return true;
    }
    if command.starts_with(prefix) {
        // Check that the prefix is followed by whitespace
        let next_char = command.as_bytes().get(prefix.len());
        matches!(next_char, Some(b' ') | Some(b'\t') | Some(b'\n'))
    } else {
        false
    }
}

/// Check if the `rtk` binary is available on PATH.
/// Uses a cached marker file to avoid scanning PATH on every invocation.
/// Falls back to PATH scan if the cache is missing.
fn rtk_available() -> bool {
    static AVAILABLE: LazyLock<bool> = LazyLock::new(|| {
        // Fast path: check cached marker file
        if let Ok(home) = std::env::var("HOME") {
            let cache = std::path::Path::new(&home).join(".local/share/precc/.rtk_path");
            if let Ok(cached_path) = std::fs::read_to_string(&cache) {
                let p = cached_path.trim();
                if !p.is_empty() && std::path::Path::new(p).is_file() {
                    return true;
                }
            }

            // Check common locations before full PATH scan
            let common = [
                format!("{home}/.cargo/bin/rtk"),
                "/usr/local/bin/rtk".to_string(),
                "/usr/bin/rtk".to_string(),
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
                let candidate = std::path::Path::new(dir).join("rtk");
                if candidate.is_file() {
                    // Cache for next invocation
                    if let Ok(home) = std::env::var("HOME") {
                        let cache =
                            std::path::Path::new(&home).join(".local/share/precc/.rtk_path");
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

/// Internal: translate git subcommand (everything after "git ") to jj equivalent.
/// Exposed for unit testing without requiring jj installation or a .jj repo.
#[doc(hidden)]
pub fn translate_git_to_jj(rest: &str) -> Option<String> {
    let translated = if rest == "status" || rest.starts_with("status ") {
        rest.replacen("status", "jj st", 1)
    } else if rest == "log" || rest.starts_with("log ") {
        rest.replacen("log", "jj log", 1)
    } else if rest == "diff" || rest.starts_with("diff ") {
        rest.replacen("diff", "jj diff", 1)
    } else if rest.starts_with("commit -a") || rest == "commit" {
        "jj commit".to_string()
    } else if rest.starts_with("add ") || rest == "add" {
        return Some("true # jj: changes are implicitly staged, no git add needed".to_string());
    } else if let Some(branch) = rest.strip_prefix("checkout -b ") {
        format!("jj new -B {}", branch.trim())
    } else if let Some(rev) = rest.strip_prefix("checkout ") {
        format!("jj edit {}", rev.trim())
    } else if let Some(branch) = rest.strip_prefix("switch -c ") {
        format!("jj new -B {}", branch.trim())
    } else if let Some(rev) = rest.strip_prefix("switch ") {
        format!("jj edit {}", rev.trim())
    } else if let Some(name) = rest.strip_prefix("branch -D ") {
        format!("jj bookmark delete {}", name.trim())
    } else if let Some(name) = rest.strip_prefix("branch -d ") {
        format!("jj bookmark delete {}", name.trim())
    } else if rest == "branch" || rest.starts_with("branch ") {
        rest.replacen("branch", "jj bookmark list", 1)
    } else if rest.starts_with("push ") || rest == "push" {
        rest.replacen("push", "jj git push", 1)
    } else if rest.starts_with("fetch ") || rest == "fetch" {
        rest.replacen("fetch", "jj git fetch", 1)
    } else if rest.starts_with("pull ") || rest == "pull" {
        "jj git fetch && jj rebase -d main@origin".to_string()
    } else if let Some(args) = rest.strip_prefix("rebase ") {
        format!("jj rebase -d {}", args.trim())
    } else if let Some(rev) = rest.strip_prefix("cherry-pick ") {
        format!("jj duplicate {} -d @", rev.trim())
    } else if let Some(rev) = rest.strip_prefix("revert ") {
        format!("jj backout -r {}", rev.trim())
    } else if rest.starts_with("stash") {
        "jj new # jj: use 'jj new' to create an anonymous change instead of stash".to_string()
    } else if let Some(path) = rest.strip_prefix("worktree add ") {
        format!("jj workspace add {}", path.trim())
    } else if rest == "worktree list" || rest.starts_with("worktree list ") {
        "jj workspace list".to_string()
    } else if let Some(file) = rest.strip_prefix("blame ") {
        format!("jj file annotate {}", file.trim())
    } else if rest.starts_with("show ") || rest == "show" {
        rest.replacen("show", "jj diff -r", 1)
    } else {
        return None;
    };
    Some(translated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_already_rtk() {
        assert_eq!(rewrite("rtk cargo build"), None);
        assert_eq!(rewrite("/usr/bin/rtk git status"), None);
    }

    #[test]
    fn skip_heredocs() {
        assert_eq!(rewrite("cat <<EOF\nhello\nEOF"), None);
    }

    #[test]
    fn matches_prefix_exact() {
        assert!(matches_prefix("ls", "ls"));
        assert!(matches_prefix("ls -la", "ls"));
        assert!(matches_prefix("git status", "git status"));
        assert!(matches_prefix("git status --short", "git status"));
    }

    #[test]
    fn matches_prefix_no_partial() {
        // Should not match partial words
        assert!(!matches_prefix("lsof", "ls"));
    }

    #[test]
    fn rg_grep_rewrite() {
        // If rtk is available, these should rewrite
        if rtk_available() {
            assert_eq!(
                rewrite("rg pattern src/"),
                Some("rtk grep pattern src/".to_string())
            );
            assert_eq!(
                rewrite("grep -r foo ."),
                Some("rtk grep -r foo .".to_string())
            );
        }
    }

    #[test]
    fn vitest_rewrite() {
        if rtk_available() {
            assert_eq!(rewrite("pnpm vitest"), Some("rtk vitest run".to_string()));
            assert_eq!(
                rewrite("vitest run"),
                Some("rtk vitest run run".to_string())
            );
        }
    }

    #[test]
    fn pnpm_test_rewrite() {
        if rtk_available() {
            assert_eq!(rewrite("pnpm test"), Some("rtk vitest run".to_string()));
        }
    }

    #[test]
    fn tsc_rewrite() {
        if rtk_available() {
            assert_eq!(
                rewrite("tsc --noEmit"),
                Some("rtk tsc --noEmit".to_string())
            );
            assert_eq!(
                rewrite("npx tsc --noEmit"),
                Some("rtk tsc --noEmit".to_string())
            );
            assert_eq!(rewrite("pnpm tsc"), Some("rtk tsc".to_string()));
        }
    }

    #[test]
    fn eslint_rewrite() {
        if rtk_available() {
            assert_eq!(rewrite("eslint src/"), Some("rtk lint src/".to_string()));
            assert_eq!(rewrite("pnpm lint"), Some("rtk lint".to_string()));
        }
    }

    #[test]
    fn prettier_rewrite() {
        if rtk_available() {
            assert_eq!(
                rewrite("prettier --check ."),
                Some("rtk prettier --check .".to_string())
            );
        }
    }

    #[test]
    fn playwright_rewrite() {
        if rtk_available() {
            assert_eq!(
                rewrite("playwright test"),
                Some("rtk playwright test".to_string())
            );
            assert_eq!(
                rewrite("pnpm playwright test"),
                Some("rtk playwright test".to_string())
            );
        }
    }

    #[test]
    fn prisma_rewrite() {
        if rtk_available() {
            assert_eq!(
                rewrite("prisma generate"),
                Some("rtk prisma generate".to_string())
            );
            assert_eq!(
                rewrite("npx prisma db push"),
                Some("rtk prisma db push".to_string())
            );
        }
    }

    #[test]
    fn no_rewrite_for_unknown() {
        // These should always return None regardless of rtk availability
        // because they don't match any rules
        if rtk_available() {
            assert_eq!(rewrite("python script.py"), None);
            assert_eq!(rewrite("echo hello"), None);
        }
    }

    #[test]
    fn tokens_saved_cargo_commands() {
        assert_eq!(tokens_saved("cargo build"), 420);
        assert_eq!(tokens_saved("cargo test --release"), 420);
        assert_eq!(tokens_saved("cargo clippy --all"), 420);
        assert_eq!(tokens_saved("cargo check"), 300);
        assert_eq!(tokens_saved("cargo run"), 200);
        assert_eq!(tokens_saved("cargo fmt"), 60);
    }

    #[test]
    fn tokens_saved_git_commands() {
        assert_eq!(tokens_saved("git status"), 160);
        assert_eq!(tokens_saved("git diff HEAD"), 160);
        assert_eq!(tokens_saved("git log --oneline"), 160);
        assert_eq!(tokens_saved("git add ."), 60);
        assert_eq!(tokens_saved("git commit -m foo"), 60);
    }

    #[test]
    fn tokens_saved_new_rules() {
        assert_eq!(tokens_saved("pytest tests/"), 380);
        assert_eq!(tokens_saved("go test ./..."), 380);
        assert_eq!(tokens_saved("go build ./..."), 300);
        assert_eq!(tokens_saved("go vet ./..."), 150);
        assert_eq!(tokens_saved("golangci-lint run"), 300);
        assert_eq!(tokens_saved("make all"), 400);
        assert_eq!(tokens_saved("docker build ."), 500);
        assert_eq!(tokens_saved("docker exec -it foo bash"), 150);
        assert_eq!(tokens_saved("kubectl describe pod"), 300);
        assert_eq!(tokens_saved("ruff check src/"), 180);
        assert_eq!(tokens_saved("mypy src/"), 200);
        assert_eq!(tokens_saved("wget https://example.com"), 100);
        assert_eq!(tokens_saved("aws s3 ls"), 200);
        assert_eq!(tokens_saved("psql -c 'SELECT 1'"), 180);
        assert_eq!(tokens_saved("diff a.txt b.txt"), 100);
        assert_eq!(tokens_saved("find . -name '*.rs'"), 80);
        assert_eq!(tokens_saved("tree src/"), 80);
        assert_eq!(tokens_saved("biome check ."), 180);
        assert_eq!(tokens_saved("gh repo view"), 120);
        assert_eq!(tokens_saved("gh api /repos"), 150);
        assert_eq!(tokens_saved("git worktree list"), 60);
    }

    #[test]
    fn tokens_saved_unknown_command() {
        assert_eq!(tokens_saved("echo hello"), 0);
        assert_eq!(tokens_saved("python script.py"), 0);
    }

    #[test]
    fn tokens_saved_with_cd_prefix() {
        // tokens_saved should strip the cd prefix before matching
        assert_eq!(tokens_saved("cd /foo && cargo build"), 420);
        assert_eq!(tokens_saved("cd /foo && make"), 400);
    }

    // --- jj_translate tests ---
    // These tests exercise the translation logic directly without requiring
    // jj to be installed or a .jj directory to exist. We test the internal
    // mapping by bypassing the availability/repo checks.

    #[test]
    fn jj_translate_not_git_command() {
        // Non-git commands must not translate (returns None before any check)
        // We test with a fake availability scenario by directly verifying
        // that non-git input is rejected at the first guard.
        // Since jj is not installed in CI, jj_translate returns None for all.
        assert_eq!(jj_translate("cargo build"), None);
        assert_eq!(jj_translate("ls -la"), None);
        assert_eq!(jj_translate("jj log"), None);
    }

    #[test]
    fn jj_translate_mapping_logic() {
        // Test the internal translation mapping via a helper that bypasses
        // the jj_available()/in_jj_repo() guards.
        assert_eq!(translate_git_to_jj("status"), Some("jj st".to_string()));
        assert_eq!(
            translate_git_to_jj("status --short"),
            Some("jj st --short".to_string())
        );
        assert_eq!(translate_git_to_jj("log"), Some("jj log".to_string()));
        assert_eq!(
            translate_git_to_jj("log --oneline"),
            Some("jj log --oneline".to_string())
        );
        assert_eq!(translate_git_to_jj("diff"), Some("jj diff".to_string()));
        assert_eq!(
            translate_git_to_jj("diff HEAD"),
            Some("jj diff HEAD".to_string())
        );
        assert_eq!(translate_git_to_jj("commit"), Some("jj commit".to_string()));
        assert_eq!(
            translate_git_to_jj("commit -am 'msg'"),
            Some("jj commit".to_string())
        );
        assert_eq!(
            translate_git_to_jj("checkout main"),
            Some("jj edit main".to_string())
        );
        assert_eq!(
            translate_git_to_jj("checkout -b feature"),
            Some("jj new -B feature".to_string())
        );
        assert_eq!(
            translate_git_to_jj("switch -c topic"),
            Some("jj new -B topic".to_string())
        );
        assert_eq!(
            translate_git_to_jj("branch"),
            Some("jj bookmark list".to_string())
        );
        assert_eq!(
            translate_git_to_jj("branch -D old"),
            Some("jj bookmark delete old".to_string())
        );
        assert_eq!(
            translate_git_to_jj("push origin main"),
            Some("jj git push origin main".to_string())
        );
        assert_eq!(
            translate_git_to_jj("fetch"),
            Some("jj git fetch".to_string())
        );
        assert_eq!(
            translate_git_to_jj("cherry-pick abc123"),
            Some("jj duplicate abc123 -d @".to_string())
        );
        assert_eq!(
            translate_git_to_jj("revert abc123"),
            Some("jj backout -r abc123".to_string())
        );
        assert_eq!(
            translate_git_to_jj("worktree add ../other"),
            Some("jj workspace add ../other".to_string())
        );
        assert_eq!(
            translate_git_to_jj("worktree list"),
            Some("jj workspace list".to_string())
        );
        assert_eq!(
            translate_git_to_jj("blame src/main.rs"),
            Some("jj file annotate src/main.rs".to_string())
        );
        // Unknown subcommands return None
        assert_eq!(translate_git_to_jj("bisect start"), None);
        assert_eq!(translate_git_to_jj("submodule update"), None);
    }
}
