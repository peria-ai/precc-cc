//! Nushell integration — experimental alternative to RTK for token-optimized output.
//!
//! Instead of rewriting commands to `rtk <cmd>` (external binary), this module wraps
//! commands as `nu -c "<translated>"` so nushell's structured output replaces verbose
//! bash text. Many CLI tools already have compact/structured output modes that nushell
//! can leverage natively.
//!
//! # Activation
//!
//! Set `PRECC_NUSHELL=1` to enable nushell mode (replaces RTK stage in the pipeline).
//! When no nushell translation matches, the pipeline falls through to RTK as usual.
//!
//! # Design
//!
//! - Curated `NuRule` table (not generic bash→nushell translation)
//! - `nu -c "..."` wrapping inside bash (same subprocess pattern as RTK)
//! - No upstream Claude Code changes required
//! - Graceful fallback to RTK for unmatched commands

use std::sync::{LazyLock, OnceLock};

// =============================================================================
// Nushell availability detection (cached, mirrors rtk_available() pattern)
// =============================================================================

/// Check if the `nu` binary is available on PATH.
/// Cached via `LazyLock` — only scans once per process.
pub fn nu_available() -> bool {
    static AVAILABLE: LazyLock<bool> = LazyLock::new(|| {
        // Fast path: check cached marker file
        if let Ok(home) = std::env::var("HOME") {
            let cache = std::path::Path::new(&home).join(".local/share/precc/.nu_path");
            if let Ok(cached_path) = std::fs::read_to_string(&cache) {
                let p = cached_path.trim();
                if !p.is_empty() && std::path::Path::new(p).is_file() {
                    return true;
                }
            }

            // Check common locations before full PATH scan
            let common = [
                format!("{home}/.cargo/bin/nu"),
                "/usr/local/bin/nu".to_string(),
                "/usr/bin/nu".to_string(),
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
                let candidate = std::path::Path::new(dir).join("nu");
                if candidate.is_file() {
                    if let Ok(home) = std::env::var("HOME") {
                        let cache = std::path::Path::new(&home).join(".local/share/precc/.nu_path");
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

/// Check if nushell mode is enabled via `PRECC_NUSHELL` env var.
/// Cached via `OnceLock` — zero cost after first check.
pub fn nushell_mode_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("PRECC_NUSHELL")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true") || v == "benchmark")
            .unwrap_or(false)
            && nu_available()
    })
}

// =============================================================================
// Safety checks — reject bash-isms that break in nushell
// =============================================================================

/// Fast check whether a command is safe to wrap in `nu -c`.
///
/// Rejects commands containing bash-specific constructs that nushell doesn't support:
/// heredocs, command substitution, brace expansion, process substitution, and
/// shell builtins that have no nushell equivalent.
pub fn is_nu_safe(command: &str) -> bool {
    // Heredocs
    if command.contains("<<") {
        return false;
    }
    // Command substitution
    if command.contains("$(") {
        return false;
    }
    // Brace expansion / parameter expansion
    if command.contains("${") {
        return false;
    }
    // Process substitution
    if command.contains("<(") || command.contains(">(") {
        return false;
    }
    // Backtick command substitution
    if command.contains('`') {
        return false;
    }
    // Shell builtins with no nushell equivalent
    let first_word = command.split_whitespace().next().unwrap_or("");
    if matches!(
        first_word,
        "source" | "." | "export" | "eval" | "exec" | "alias" | "unset" | "trap" | "set"
    ) {
        return false;
    }
    true
}

// =============================================================================
// Nushell translation rules
// =============================================================================

/// A nushell rewrite rule: if bash command starts with `from`, translate to `nu_command`.
struct NuRule {
    /// Bash command prefix to match.
    from: &'static str,
    /// Nushell-compatible command (may use compact output flags instead of nu pipes).
    nu_command: &'static str,
    /// Estimated tokens saved by RTK for the same command (baseline comparison).
    pub rtk_baseline: u32,
    /// Fraction of original output remaining after nushell processing (for what-if).
    /// e.g., 0.40 means 60% reduction.
    pub compression_ratio: f64,
}

/// Curated nushell translations for the highest-value RTK scenarios.
/// Each translation has been chosen to produce compact, structured output.
/// Checked in order; first match wins (longest prefixes first).
static NU_RULES: &[NuRule] = &[
    // --- Cargo/Rust (highest RTK savings: 420t) ---
    NuRule { from: "cargo test", nu_command: "cargo test 2>&1 | nu -c 'lines | where {|l| ($l | str contains \"test result\") or ($l | str contains \"FAILED\") or ($l | str contains \"error\")}'", rtk_baseline: 420, compression_ratio: 0.30 },
    NuRule { from: "cargo clippy", nu_command: "cargo clippy --message-format=short 2>&1", rtk_baseline: 420, compression_ratio: 0.40 },
    NuRule { from: "cargo build", nu_command: "cargo build --message-format=short 2>&1", rtk_baseline: 420, compression_ratio: 0.40 },
    NuRule { from: "cargo check", nu_command: "cargo check --message-format=short 2>&1", rtk_baseline: 300, compression_ratio: 0.40 },
    NuRule { from: "cargo run", nu_command: "cargo run --message-format=short 2>&1", rtk_baseline: 200, compression_ratio: 0.50 },
    NuRule { from: "cargo fmt", nu_command: "cargo fmt 2>&1", rtk_baseline: 60, compression_ratio: 0.90 },
    // --- Docker (500t) ---
    NuRule { from: "docker build", nu_command: "docker build --progress=plain 2>&1 | nu -c 'lines | last 20'", rtk_baseline: 500, compression_ratio: 0.25 },
    NuRule { from: "docker compose logs", nu_command: "docker compose logs --tail=30 2>&1", rtk_baseline: 200, compression_ratio: 0.40 },
    NuRule { from: "docker compose ps", nu_command: "docker compose ps --format=json 2>&1 | nu -c 'from json | select Name State Status'", rtk_baseline: 150, compression_ratio: 0.30 },
    NuRule { from: "docker ps", nu_command: "docker ps --format=json 2>&1 | nu -c 'from json | select Names State Status'", rtk_baseline: 150, compression_ratio: 0.30 },
    // --- Make (400t) ---
    NuRule { from: "make", nu_command: "make 2>&1 | nu -c 'lines | where {|l| ($l | str contains \"error\") or ($l | str contains \"warning\") or ($l | str contains \"make:\")}'", rtk_baseline: 400, compression_ratio: 0.30 },
    // --- Go (380t) ---
    NuRule { from: "go test", nu_command: "go test -json ./... 2>&1 | nu -c 'lines | each {|| from json } | where Action == \"fail\" or Action == \"output\" | select Action Package Output?'", rtk_baseline: 380, compression_ratio: 0.30 },
    NuRule { from: "go build", nu_command: "go build 2>&1", rtk_baseline: 300, compression_ratio: 0.90 },
    // --- npm/yarn (420t) ---
    NuRule { from: "npm test", nu_command: "npm test 2>&1 | nu -c 'lines | where {|l| ($l | str contains \"FAIL\") or ($l | str contains \"PASS\") or ($l | str contains \"error\") or ($l | str contains \"Tests:\")}'", rtk_baseline: 420, compression_ratio: 0.30 },
    NuRule { from: "npm run", nu_command: "npm run 2>&1", rtk_baseline: 180, compression_ratio: 0.90 },
    NuRule { from: "yarn test", nu_command: "yarn test 2>&1 | nu -c 'lines | where {|l| ($l | str contains \"FAIL\") or ($l | str contains \"PASS\") or ($l | str contains \"error\") or ($l | str contains \"Tests:\")}'", rtk_baseline: 420, compression_ratio: 0.30 },
    // --- Git (160t) ---
    NuRule { from: "git status", nu_command: "git status --porcelain=v2", rtk_baseline: 160, compression_ratio: 0.35 },
    NuRule { from: "git diff", nu_command: "git diff --stat", rtk_baseline: 160, compression_ratio: 0.40 },
    NuRule { from: "git log", nu_command: "git log --oneline -20", rtk_baseline: 160, compression_ratio: 0.35 },
    NuRule { from: "git show", nu_command: "git show --stat", rtk_baseline: 160, compression_ratio: 0.40 },
    // --- Python (380t) ---
    NuRule { from: "pytest", nu_command: "pytest --tb=short -q 2>&1", rtk_baseline: 380, compression_ratio: 0.35 },
    NuRule { from: "python -m pytest", nu_command: "python -m pytest --tb=short -q 2>&1", rtk_baseline: 380, compression_ratio: 0.35 },
    // --- kubectl ---
    NuRule { from: "kubectl get", nu_command: "kubectl get -o json 2>&1 | nu -c 'from json | get items | select metadata.name status.phase?'", rtk_baseline: 180, compression_ratio: 0.30 },
    NuRule { from: "kubectl describe", nu_command: "kubectl describe 2>&1 | nu -c 'lines | where {|l| ($l | str starts-with \"Name:\") or ($l | str starts-with \"Status:\") or ($l | str starts-with \"Events:\") or ($l | str contains \"Error\")}'", rtk_baseline: 300, compression_ratio: 0.25 },
    // --- System Admin: File operations ---
    NuRule { from: "ls", nu_command: "nu -c 'ls | select name type size modified'", rtk_baseline: 40, compression_ratio: 0.50 },
    NuRule { from: "find", nu_command: "nu -c 'glob **/* | where {|p| $p | str contains \"PATTERN\"}'", rtk_baseline: 80, compression_ratio: 0.60 },
    NuRule { from: "tree", nu_command: "nu -c 'ls **/* | select name type | sort-by name'", rtk_baseline: 80, compression_ratio: 0.50 },
    NuRule { from: "du", nu_command: "du -sh 2>&1", rtk_baseline: 40, compression_ratio: 0.50 },
    NuRule { from: "df", nu_command: "df -h 2>&1", rtk_baseline: 40, compression_ratio: 0.60 },
    NuRule { from: "wc", nu_command: "wc -l 2>&1", rtk_baseline: 30, compression_ratio: 0.40 },
    NuRule { from: "stat", nu_command: "nu -c 'ls -l' 2>&1", rtk_baseline: 30, compression_ratio: 0.50 },
    NuRule { from: "file", nu_command: "file 2>&1", rtk_baseline: 20, compression_ratio: 0.90 },
    // --- System Admin: Text processing ---
    NuRule { from: "cat", nu_command: "cat 2>&1", rtk_baseline: 50, compression_ratio: 0.90 },
    NuRule { from: "head", nu_command: "head 2>&1", rtk_baseline: 50, compression_ratio: 0.90 },
    NuRule { from: "tail", nu_command: "tail 2>&1", rtk_baseline: 50, compression_ratio: 0.90 },
    NuRule { from: "sort", nu_command: "sort 2>&1", rtk_baseline: 30, compression_ratio: 0.90 },
    NuRule { from: "uniq", nu_command: "uniq 2>&1", rtk_baseline: 30, compression_ratio: 0.80 },
    NuRule { from: "cut", nu_command: "cut 2>&1", rtk_baseline: 30, compression_ratio: 0.80 },
    NuRule { from: "sed", nu_command: "sed 2>&1", rtk_baseline: 30, compression_ratio: 0.90 },
    NuRule { from: "awk", nu_command: "awk 2>&1", rtk_baseline: 30, compression_ratio: 0.80 },
    // --- System Admin: Search ---
    NuRule { from: "rg", nu_command: "rg --json 2>&1 | nu -c 'lines | each {|| from json } | where type == \"match\" | get data.lines.text'", rtk_baseline: 90, compression_ratio: 0.40 },
    NuRule { from: "grep", nu_command: "grep -n 2>&1", rtk_baseline: 90, compression_ratio: 0.70 },
    NuRule { from: "ag", nu_command: "ag --column 2>&1", rtk_baseline: 90, compression_ratio: 0.70 },
    NuRule { from: "fd", nu_command: "fd 2>&1", rtk_baseline: 50, compression_ratio: 0.80 },
    // --- System Admin: Process management ---
    NuRule { from: "ps", nu_command: "nu -c 'ps | select pid name cpu mem | sort-by cpu -r | first 20'", rtk_baseline: 60, compression_ratio: 0.35 },
    NuRule { from: "top", nu_command: "top -b -n 1 2>&1 | nu -c 'lines | first 20'", rtk_baseline: 80, compression_ratio: 0.30 },
    NuRule { from: "htop", nu_command: "ps aux --sort=-%cpu 2>&1 | head -20", rtk_baseline: 80, compression_ratio: 0.30 },
    // --- System Admin: Network ---
    NuRule { from: "curl", nu_command: "curl -s 2>&1", rtk_baseline: 200, compression_ratio: 0.80 },
    NuRule { from: "wget", nu_command: "wget -q 2>&1", rtk_baseline: 100, compression_ratio: 0.70 },
    NuRule { from: "ping", nu_command: "ping -c 3 2>&1", rtk_baseline: 50, compression_ratio: 0.50 },
    NuRule { from: "ss", nu_command: "ss -tlnp 2>&1 | nu -c 'lines | skip 1 | parse \"{state} {recv} {send} {local} {peer} {process}\" | select state local process'", rtk_baseline: 80, compression_ratio: 0.40 },
    NuRule { from: "netstat", nu_command: "netstat -tlnp 2>&1 | nu -c 'lines | skip 2 | first 20'", rtk_baseline: 80, compression_ratio: 0.40 },
    NuRule { from: "ip", nu_command: "ip -brief -c addr 2>&1", rtk_baseline: 60, compression_ratio: 0.40 },
    NuRule { from: "dig", nu_command: "dig +short 2>&1", rtk_baseline: 60, compression_ratio: 0.30 },
    NuRule { from: "nslookup", nu_command: "nslookup 2>&1 | nu -c 'lines | where {|l| ($l | str contains \"Address\") or ($l | str contains \"Name\")}'", rtk_baseline: 40, compression_ratio: 0.40 },
    // --- System Admin: Disk/Archive ---
    NuRule { from: "tar", nu_command: "tar 2>&1", rtk_baseline: 40, compression_ratio: 0.80 },
    NuRule { from: "zip", nu_command: "zip 2>&1", rtk_baseline: 40, compression_ratio: 0.80 },
    NuRule { from: "unzip", nu_command: "unzip -l 2>&1", rtk_baseline: 40, compression_ratio: 0.70 },
    // --- System Admin: Package managers ---
    NuRule { from: "apt", nu_command: "apt 2>&1 | nu -c 'lines | where {|l| not ($l | str starts-with \"WARNING\")}'", rtk_baseline: 80, compression_ratio: 0.50 },
    NuRule { from: "apt-get", nu_command: "apt-get 2>&1 | nu -c 'lines | where {|l| not ($l | str starts-with \"WARNING\")}'", rtk_baseline: 80, compression_ratio: 0.50 },
    NuRule { from: "brew", nu_command: "brew 2>&1", rtk_baseline: 60, compression_ratio: 0.70 },
    NuRule { from: "pip install", nu_command: "pip install -q 2>&1", rtk_baseline: 150, compression_ratio: 0.40 },
    NuRule { from: "pip list", nu_command: "pip list --format=json 2>&1 | nu -c 'from json | select name version'", rtk_baseline: 150, compression_ratio: 0.40 },
    // --- System Admin: Permissions/Ownership ---
    NuRule { from: "chmod", nu_command: "chmod 2>&1", rtk_baseline: 20, compression_ratio: 0.90 },
    NuRule { from: "chown", nu_command: "chown 2>&1", rtk_baseline: 20, compression_ratio: 0.90 },
    // --- System Admin: Misc ---
    NuRule { from: "diff", nu_command: "diff --stat 2>&1", rtk_baseline: 100, compression_ratio: 0.50 },
    NuRule { from: "env", nu_command: "nu -c '$env | transpose key value | select key value'", rtk_baseline: 60, compression_ratio: 0.50 },
    NuRule { from: "printenv", nu_command: "nu -c '$env | transpose key value | select key value'", rtk_baseline: 60, compression_ratio: 0.50 },
    NuRule { from: "uname", nu_command: "uname -a 2>&1", rtk_baseline: 10, compression_ratio: 0.90 },
    NuRule { from: "which", nu_command: "which 2>&1", rtk_baseline: 10, compression_ratio: 0.90 },
    NuRule { from: "date", nu_command: "nu -c 'date now | format date \"%Y-%m-%d %H:%M:%S\"'", rtk_baseline: 10, compression_ratio: 0.50 },
    // --- System Admin: Services ---
    NuRule { from: "systemctl", nu_command: "systemctl --no-pager 2>&1", rtk_baseline: 80, compression_ratio: 0.60 },
    NuRule { from: "journalctl", nu_command: "journalctl --no-pager -n 30 2>&1", rtk_baseline: 100, compression_ratio: 0.40 },
    NuRule { from: "service", nu_command: "service 2>&1", rtk_baseline: 40, compression_ratio: 0.80 },
    NuRule { from: "lsof", nu_command: "lsof 2>&1 | nu -c 'lines | first 20'", rtk_baseline: 60, compression_ratio: 0.30 },
    NuRule { from: "free", nu_command: "free -h 2>&1", rtk_baseline: 30, compression_ratio: 0.60 },
    NuRule { from: "pgrep", nu_command: "pgrep -a 2>&1", rtk_baseline: 30, compression_ratio: 0.70 },
    NuRule { from: "dpkg", nu_command: "dpkg 2>&1", rtk_baseline: 40, compression_ratio: 0.70 },
    // --- General: Frequently seen in "Other" category ---
    NuRule { from: "echo", nu_command: "echo 2>&1", rtk_baseline: 10, compression_ratio: 0.95 },
    NuRule { from: "pwd", nu_command: "pwd 2>&1", rtk_baseline: 5, compression_ratio: 0.95 },
    NuRule { from: "sudo", nu_command: "sudo 2>&1", rtk_baseline: 30, compression_ratio: 0.85 },
    NuRule { from: "bash", nu_command: "bash 2>&1", rtk_baseline: 30, compression_ratio: 0.85 },
    NuRule { from: "mdbook", nu_command: "mdbook 2>&1 | nu -c 'lines | where {|l| ($l | str contains \"error\") or ($l | str contains \"warning\") or ($l | str contains \"Finished\")}'", rtk_baseline: 80, compression_ratio: 0.30 },
    NuRule { from: "clawhub", nu_command: "clawhub 2>&1", rtk_baseline: 30, compression_ratio: 0.80 },
    NuRule { from: "stripe", nu_command: "stripe 2>&1", rtk_baseline: 60, compression_ratio: 0.70 },
    NuRule { from: "sg", nu_command: "sg 2>&1", rtk_baseline: 40, compression_ratio: 0.70 },
    NuRule { from: "postconf", nu_command: "postconf 2>&1 | nu -c 'lines | first 20'", rtk_baseline: 40, compression_ratio: 0.40 },
    NuRule { from: "doveconf", nu_command: "doveconf 2>&1 | nu -c 'lines | first 20'", rtk_baseline: 40, compression_ratio: 0.40 },
];

/// Helper: check if command starts with prefix as a whole word.
fn matches_prefix(command: &str, prefix: &str) -> bool {
    if command == prefix {
        return true;
    }
    match command.strip_prefix(prefix) {
        Some(rest) => rest.starts_with(' ') || rest.starts_with('\t'),
        None => false,
    }
}

// =============================================================================
// Command wrapping
// =============================================================================

/// Wrap a bash command in its nushell equivalent, if a matching rule exists.
///
/// Returns `Some(wrapped)` if a translation was found and the command is nu-safe.
/// Returns `None` if no rule matches, command isn't safe for nushell, or nu is unavailable.
///
/// The returned string is a bash command that invokes nushell (e.g., `nu -c "..."`
/// or uses compact CLI flags that produce structured output).
pub fn wrap(command: &str) -> Option<String> {
    // Skip if already wrapped in nushell
    if command.starts_with("nu ") || command.contains("| nu ") {
        return None;
    }

    // Skip if not nu-safe
    if !is_nu_safe(command) {
        return None;
    }

    // Skip if nu is unavailable
    if !nu_available() {
        return None;
    }

    // Find matching rule (first match wins)
    for rule in NU_RULES {
        if matches_prefix(command, rule.from) {
            // Some nu_commands already include the original args handling.
            // For simple prefix swaps where the user may have added args
            // (e.g., `cargo build --release`), we need to preserve those args.
            let extra_args = &command[rule.from.len()..];

            // If the nu_command already handles the full invocation (contains pipes or
            // multi-command constructs), use it as-is (args already baked in or not applicable)
            if rule.nu_command.contains('|') || rule.nu_command.contains("nu -c") {
                return Some(rule.nu_command.to_string());
            }

            // Otherwise, append the user's extra args to the nu_command
            if extra_args.is_empty() {
                return Some(rule.nu_command.to_string());
            } else {
                return Some(format!("{}{}", rule.nu_command, extra_args));
            }
        }
    }

    None
}

/// Dry-run: show what a command would be translated to, without availability checks.
/// Returns (from, nu_command, rtk_baseline, compression_ratio).
pub fn translate_preview(command: &str) -> Option<(&'static str, &'static str, u32, f64)> {
    if !is_nu_safe(command) {
        return None;
    }
    for rule in NU_RULES {
        if matches_prefix(command, rule.from) {
            return Some((
                rule.from,
                rule.nu_command,
                rule.rtk_baseline,
                rule.compression_ratio,
            ));
        }
    }
    None
}

/// Rough BPE token estimate (chars / 4, typical for English CLI output).
pub fn estimate_tokens(text: &str) -> usize {
    text.len() / 4
}

/// Return the list of all nushell rules (for benchmark/info display).
pub fn rule_count() -> usize {
    NU_RULES.len()
}

/// Iterate over rules for display purposes.
pub fn rules_summary() -> Vec<(&'static str, &'static str, u32, f64)> {
    NU_RULES
        .iter()
        .map(|r| (r.from, r.nu_command, r.rtk_baseline, r.compression_ratio))
        .collect()
}

// =============================================================================
// What-if historical analysis
// =============================================================================

/// Usage category for command classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UsageCategory {
    SoftwareDev,
    SystemAdmin,
    DataAnalysis,
    OfficeWork,
    Writing,
    Other,
}

impl UsageCategory {
    pub fn name(self) -> &'static str {
        match self {
            Self::SoftwareDev => "Software Dev",
            Self::SystemAdmin => "System Admin",
            Self::DataAnalysis => "Data Analysis",
            Self::OfficeWork => "Office Work",
            Self::Writing => "Writing",
            Self::Other => "Other",
        }
    }

    pub fn all() -> &'static [UsageCategory] {
        &[
            Self::SoftwareDev,
            Self::SystemAdmin,
            Self::DataAnalysis,
            Self::OfficeWork,
            Self::Writing,
            Self::Other,
        ]
    }
}

/// Classify a bash command into a usage category.
pub fn classify_command(command: &str) -> UsageCategory {
    let cmd = command.trim();
    // Strip cd prefix if present
    let cmd = if cmd.starts_with("cd ") {
        if let Some(pos) = cmd.find(" && ") {
            cmd[pos + 4..].trim()
        } else {
            cmd
        }
    } else {
        cmd
    };
    let first = cmd.split_whitespace().next().unwrap_or("");

    // Software development
    if matches!(
        first,
        "cargo"
            | "rustc"
            | "rustup"
            | "git"
            | "gh"
            | "jj"
            | "npm"
            | "npx"
            | "pnpm"
            | "yarn"
            | "bun"
            | "deno"
            | "node"
            | "tsc"
            | "eslint"
            | "biome"
            | "prettier"
            | "vitest"
            | "jest"
            | "playwright"
            | "go"
            | "golangci-lint"
            | "docker"
            | "kubectl"
            | "helm"
            | "make"
            | "cmake"
            | "ninja"
            | "bazel"
            | "pytest"
            | "mypy"
            | "ruff"
            | "flake8"
            | "black"
            | "mvn"
            | "gradle"
            | "javac"
            | "java"
            | "gcc"
            | "g++"
            | "clang"
            | "clang++"
            | "dotnet"
            | "msbuild"
            | "terraform"
            | "ansible"
            | "rtk"
    ) || cmd.starts_with("./target/")
        || cmd.starts_with("./node_modules/")
    {
        return UsageCategory::SoftwareDev;
    }

    // Data analysis
    if matches!(
        first,
        "python" | "python3" | "pip" | "pip3" | "uv" | "conda"
    ) || matches!(first, "psql" | "mysql" | "sqlite3" | "mongosh")
        || matches!(first, "jupyter" | "ipython" | "Rscript" | "R")
    {
        return UsageCategory::DataAnalysis;
    }

    // Office work
    if matches!(first, "precc")
        && (cmd.contains("report")
            || cmd.contains("mail")
            || cmd.contains("savings")
            || cmd.contains("gif"))
    {
        return UsageCategory::OfficeWork;
    }
    if matches!(first, "pandoc" | "wkhtmltopdf" | "libreoffice" | "soffice") {
        return UsageCategory::OfficeWork;
    }

    // Writing
    if (first == "echo" || first == "cat" || first == "printf")
        && (cmd.contains(".md") || cmd.contains(".txt") || cmd.contains(".rst"))
    {
        return UsageCategory::Writing;
    }
    if matches!(first, "pandoc" | "hugo" | "jekyll" | "mkdocs") {
        return UsageCategory::Writing;
    }

    // System admin
    if matches!(
        first,
        "ls" | "find"
            | "tree"
            | "du"
            | "df"
            | "top"
            | "htop"
            | "ps"
            | "kill"
            | "pkill"
            | "systemctl"
            | "journalctl"
            | "service"
            | "chmod"
            | "chown"
            | "chgrp"
            | "mount"
            | "umount"
            | "fdisk"
            | "lsblk"
            | "ip"
            | "ifconfig"
            | "netstat"
            | "ss"
            | "ping"
            | "traceroute"
            | "dig"
            | "nslookup"
            | "curl"
            | "wget"
            | "ssh"
            | "scp"
            | "rsync"
            | "tar"
            | "zip"
            | "unzip"
            | "gzip"
            | "gunzip"
            | "apt"
            | "apt-get"
            | "yum"
            | "dnf"
            | "brew"
            | "pacman"
            | "head"
            | "tail"
            | "wc"
            | "sort"
            | "uniq"
            | "cut"
            | "awk"
            | "sed"
            | "grep"
            | "rg"
            | "ag"
            | "fd"
            | "diff"
            | "patch"
            | "cp"
            | "mv"
            | "rm"
            | "mkdir"
            | "rmdir"
            | "touch"
            | "ln"
            | "cat"
            | "less"
            | "more"
            | "file"
            | "stat"
            | "which"
            | "whereis"
            | "type"
            | "env"
            | "printenv"
            | "uname"
            | "hostname"
            | "whoami"
            | "id"
            | "date"
            | "sudo"
            | "bash"
            | "sh"
            | "echo"
            | "printf"
            | "pwd"
            | "pgrep"
            | "lsof"
            | "free"
            | "dpkg"
            | "rpm"
            | "nohup"
            | "sleep"
            | "test"
            | "true"
            | "false"
            | "xargs"
            | "tee"
    ) {
        return UsageCategory::SystemAdmin;
    }

    // Documentation / publishing tools → Writing
    if matches!(first, "mdbook" | "clawhub" | "sg") {
        return UsageCategory::Writing;
    }

    // Payment / business tools → OfficeWork
    if matches!(first, "stripe") {
        return UsageCategory::OfficeWork;
    }

    UsageCategory::Other
}

/// Aggregated what-if results per usage category.
#[derive(Debug, Clone, Default)]
pub struct WhatIfResult {
    pub total_commands: u64,
    pub bash_tokens: u64,
    pub rtk_tokens: u64,
    pub nushell_tokens: u64,
    pub nushell_matched: u64,
    pub nushell_unmatched: u64,
}

/// Sub-breakdown for software dev commands.
#[derive(Debug, Clone, Default)]
pub struct DevSubBreakdown {
    pub label: String,
    pub commands: u64,
    pub bash_tokens: u64,
    pub rtk_tokens: u64,
    pub nushell_tokens: u64,
}

/// Full what-if analysis result.
#[derive(Debug, Clone)]
pub struct WhatIfAnalysis {
    pub session_count: u64,
    pub by_category: std::collections::HashMap<UsageCategory, WhatIfResult>,
    pub dev_sub: Vec<DevSubBreakdown>,
}

/// Run the what-if analysis across all historical Claude Code session logs.
///
/// Parses every Bash tool_use + tool_result pair, classifies commands,
/// and simulates RTK vs nushell token savings without re-executing.
pub fn what_if_analysis() -> anyhow::Result<WhatIfAnalysis> {
    use crate::mining;
    use std::collections::HashMap;

    let files = mining::find_session_files()?;
    let session_count = files.len() as u64;

    let mut by_category: HashMap<UsageCategory, WhatIfResult> = HashMap::new();
    // Dev sub-breakdown keyed by NuRule.from (or "Other dev")
    let mut dev_sub: HashMap<String, DevSubBreakdown> = HashMap::new();

    for path in &files {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        process_session_for_whatif(&content, &mut by_category, &mut dev_sub);
    }

    // Sort dev sub by command count descending
    let mut dev_sub_vec: Vec<DevSubBreakdown> = dev_sub.into_values().collect();
    dev_sub_vec.sort_by(|a, b| b.commands.cmp(&a.commands));

    Ok(WhatIfAnalysis {
        session_count,
        by_category,
        dev_sub: dev_sub_vec,
    })
}

/// Process a single session JSONL for what-if analysis.
fn process_session_for_whatif(
    content: &str,
    by_category: &mut std::collections::HashMap<UsageCategory, WhatIfResult>,
    dev_sub: &mut std::collections::HashMap<String, DevSubBreakdown>,
) {
    // Map tool_use id → (tool_name, command)
    let mut pending: std::collections::HashMap<String, (String, String)> =
        std::collections::HashMap::new();

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let parsed: serde_json::Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let msg = match parsed.get("message") {
            Some(m) if !m.is_null() => m,
            _ => continue,
        };

        let content_val = match msg.get("content") {
            Some(c) if c.is_array() => c,
            _ => continue,
        };

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
                "tool_use" => {
                    let tool_name = match block.get("name").and_then(|n| n.as_str()) {
                        Some(n) => n.to_string(),
                        None => continue,
                    };
                    if tool_name != "Bash" {
                        continue;
                    }
                    let command = block
                        .get("input")
                        .and_then(|i| i.get("command"))
                        .and_then(|c| c.as_str())
                        .unwrap_or("")
                        .to_string();
                    if command.is_empty() {
                        continue;
                    }
                    if let Some(id) = block.get("id").and_then(|i| i.as_str()) {
                        pending.insert(id.to_string(), (tool_name, command));
                    }
                }
                "tool_result" => {
                    let entry = block
                        .get("tool_use_id")
                        .and_then(|i| i.as_str())
                        .and_then(|id| pending.remove(id));

                    let (_, command) = match entry {
                        Some(e) => e,
                        None => continue,
                    };

                    // Get output bytes
                    let output_bytes = content_block_text_len(block);
                    if output_bytes == 0 {
                        continue;
                    }

                    let bash_tok = output_bytes / 4;
                    let category = classify_command(&command);
                    let rtk_saved = crate::rtk::tokens_saved(&command) as u64;
                    let rtk_tok = bash_tok.saturating_sub(rtk_saved);

                    // Nushell estimation
                    let (nu_tok, nu_matched) =
                        if let Some((_, _, _, ratio)) = translate_preview(&command) {
                            ((bash_tok as f64 * ratio) as u64, true)
                        } else {
                            // No nu rule: use RTK as fallback
                            (rtk_tok, false)
                        };

                    // Accumulate category totals
                    let cat_result = by_category.entry(category).or_default();
                    cat_result.total_commands += 1;
                    cat_result.bash_tokens += bash_tok;
                    cat_result.rtk_tokens += rtk_tok;
                    cat_result.nushell_tokens += nu_tok;
                    if nu_matched {
                        cat_result.nushell_matched += 1;
                    } else {
                        cat_result.nushell_unmatched += 1;
                    }

                    // Software dev sub-breakdown
                    if category == UsageCategory::SoftwareDev {
                        let sub_key = if let Some((from, _, _, _)) = translate_preview(&command) {
                            from.to_string()
                        } else {
                            "Other dev".to_string()
                        };
                        let sub =
                            dev_sub
                                .entry(sub_key.clone())
                                .or_insert_with(|| DevSubBreakdown {
                                    label: sub_key,
                                    ..Default::default()
                                });
                        sub.commands += 1;
                        sub.bash_tokens += bash_tok;
                        sub.rtk_tokens += rtk_tok;
                        sub.nushell_tokens += nu_tok;
                    }
                }
                _ => {}
            }
        }
    }
}

/// Extract text length from a tool_result content field.
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

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -- is_nu_safe --

    #[test]
    fn safe_simple_commands() {
        assert!(is_nu_safe("cargo build"));
        assert!(is_nu_safe("git status"));
        assert!(is_nu_safe("ls -la"));
        assert!(is_nu_safe("docker ps"));
        assert!(is_nu_safe("make -j4"));
    }

    #[test]
    fn rejects_heredocs() {
        assert!(!is_nu_safe("cat <<EOF\nhello\nEOF"));
        assert!(!is_nu_safe("cat <<'MARKER'"));
    }

    #[test]
    fn rejects_command_substitution() {
        assert!(!is_nu_safe("echo $(date)"));
        assert!(!is_nu_safe("echo $(whoami)"));
    }

    #[test]
    fn rejects_brace_expansion() {
        assert!(!is_nu_safe("echo ${HOME}"));
        assert!(!is_nu_safe("echo ${VAR:-default}"));
    }

    #[test]
    fn rejects_process_substitution() {
        assert!(!is_nu_safe("diff <(ls dir1) <(ls dir2)"));
        assert!(!is_nu_safe("tee >(wc -l)"));
    }

    #[test]
    fn rejects_backticks() {
        assert!(!is_nu_safe("echo `date`"));
    }

    #[test]
    fn rejects_shell_builtins() {
        assert!(!is_nu_safe("export FOO=bar"));
        assert!(!is_nu_safe("source ~/.bashrc"));
        assert!(!is_nu_safe(". ~/.profile"));
        assert!(!is_nu_safe("eval some_command"));
        assert!(!is_nu_safe("exec bash"));
        assert!(!is_nu_safe("alias ll='ls -la'"));
        assert!(!is_nu_safe("unset VAR"));
        assert!(!is_nu_safe("trap 'echo bye' EXIT"));
        assert!(!is_nu_safe("set -e"));
    }

    // -- matches_prefix --

    #[test]
    fn prefix_matching_whole_word() {
        assert!(matches_prefix("cargo build", "cargo build"));
        assert!(matches_prefix("cargo build --release", "cargo build"));
        assert!(!matches_prefix("cargo builder", "cargo build"));
        assert!(!matches_prefix("cargobuild", "cargo"));
    }

    // -- wrap --

    #[test]
    fn wrap_cargo_build() {
        let preview = translate_preview("cargo build");
        assert!(preview.is_some());
        let (from, to, baseline, ratio) = preview.unwrap();
        assert_eq!(from, "cargo build");
        assert!(to.contains("--message-format=short"));
        assert_eq!(baseline, 420);
        assert!(ratio > 0.0 && ratio < 1.0);
    }

    #[test]
    fn wrap_cargo_build_with_args() {
        let preview = translate_preview("cargo build --release");
        assert!(preview.is_some());
        let (from, _, _, _) = preview.unwrap();
        assert_eq!(from, "cargo build");
    }

    #[test]
    fn wrap_git_status() {
        let preview = translate_preview("git status");
        assert!(preview.is_some());
        let (_, to, _, _) = preview.unwrap();
        assert!(to.contains("--porcelain=v2"));
    }

    #[test]
    fn wrap_git_log() {
        let preview = translate_preview("git log");
        assert!(preview.is_some());
        let (_, to, _, _) = preview.unwrap();
        assert!(to.contains("--oneline"));
    }

    #[test]
    fn wrap_docker_build() {
        let preview = translate_preview("docker build .");
        assert!(preview.is_some());
        let (_, to, _, _) = preview.unwrap();
        assert!(to.contains("--progress=plain"));
    }

    #[test]
    fn wrap_go_test() {
        let preview = translate_preview("go test ./...");
        assert!(preview.is_some());
        let (_, to, _, _) = preview.unwrap();
        assert!(to.contains("-json"));
    }

    #[test]
    fn wrap_make() {
        let preview = translate_preview("make");
        assert!(preview.is_some());
    }

    #[test]
    fn wrap_pytest() {
        let preview = translate_preview("pytest tests/");
        assert!(preview.is_some());
        let (_, to, _, _) = preview.unwrap();
        assert!(to.contains("--tb=short"));
    }

    #[test]
    fn wrap_kubectl_get() {
        let preview = translate_preview("kubectl get pods");
        assert!(preview.is_some());
        let (_, to, _, _) = preview.unwrap();
        assert!(to.contains("-o json"));
    }

    #[test]
    fn wrap_ls() {
        let preview = translate_preview("ls");
        assert!(preview.is_some());
        let (_, to, _, _) = preview.unwrap();
        assert!(to.contains("nu -c"));
    }

    #[test]
    fn no_wrap_for_unknown_command() {
        assert!(translate_preview("my_custom_script.sh").is_none());
        assert!(translate_preview("some_random_binary --flag").is_none());
    }

    #[test]
    fn no_wrap_for_unsafe_command() {
        assert!(translate_preview("export FOO=bar").is_none());
        assert!(translate_preview("echo $(date)").is_none());
    }

    #[test]
    fn skip_already_wrapped() {
        assert!(wrap("nu -c 'ls'").is_none());
        assert!(wrap("cargo build | nu -c 'lines'").is_none());
    }

    // -- estimate_tokens --

    #[test]
    fn token_estimation() {
        assert_eq!(estimate_tokens(""), 0);
        assert_eq!(estimate_tokens("abcd"), 1);
        assert_eq!(estimate_tokens("a".repeat(400).as_str()), 100);
    }

    // -- rules --

    #[test]
    fn rules_non_empty() {
        assert!(rule_count() > 20);
    }

    #[test]
    fn rules_summary_matches_count() {
        assert_eq!(rules_summary().len(), rule_count());
    }

    // -- classify_command --

    #[test]
    fn classify_software_dev() {
        assert_eq!(classify_command("cargo build"), UsageCategory::SoftwareDev);
        assert_eq!(classify_command("git status"), UsageCategory::SoftwareDev);
        assert_eq!(classify_command("npm test"), UsageCategory::SoftwareDev);
        assert_eq!(
            classify_command("docker build ."),
            UsageCategory::SoftwareDev
        );
        assert_eq!(
            classify_command("go test ./..."),
            UsageCategory::SoftwareDev
        );
        assert_eq!(classify_command("make -j4"), UsageCategory::SoftwareDev);
        assert_eq!(classify_command("pytest"), UsageCategory::SoftwareDev);
    }

    #[test]
    fn classify_with_cd_prefix() {
        assert_eq!(
            classify_command("cd /home/user/proj && cargo build"),
            UsageCategory::SoftwareDev
        );
    }

    #[test]
    fn classify_system_admin() {
        assert_eq!(classify_command("ls -la"), UsageCategory::SystemAdmin);
        assert_eq!(
            classify_command("find . -name '*.rs'"),
            UsageCategory::SystemAdmin
        );
        assert_eq!(
            classify_command("grep pattern file"),
            UsageCategory::SystemAdmin
        );
        assert_eq!(
            classify_command("curl https://example.com"),
            UsageCategory::SystemAdmin
        );
    }

    #[test]
    fn classify_data_analysis() {
        assert_eq!(
            classify_command("python script.py"),
            UsageCategory::DataAnalysis
        );
        assert_eq!(
            classify_command("psql -d mydb"),
            UsageCategory::DataAnalysis
        );
    }

    #[test]
    fn classify_other() {
        assert_eq!(classify_command("my_custom_tool"), UsageCategory::Other);
    }

    // -- compression ratios --

    #[test]
    fn compression_ratios_valid() {
        for (_, _, _, ratio) in rules_summary() {
            assert!(ratio > 0.0, "ratio must be positive");
            assert!(ratio <= 1.0, "ratio must be <= 1.0");
        }
    }

    // -- what-if on synthetic data --

    #[test]
    fn whatif_empty_session() {
        let mut by_cat = std::collections::HashMap::new();
        let mut dev_sub = std::collections::HashMap::new();
        process_session_for_whatif("", &mut by_cat, &mut dev_sub);
        assert!(by_cat.is_empty());
    }

    #[test]
    fn whatif_synthetic_session() {
        // Minimal JSONL matching real Claude Code format:
        // Line 1: assistant message with tool_use block
        // Line 2: user message with tool_result block (content as string)
        let line1 = r#"{"message":{"role":"assistant","content":[{"type":"tool_use","id":"t1","name":"Bash","input":{"command":"cargo build"}}]}}"#;
        let output_text = "Compiling precc v0.2.5\n    Finished dev [unoptimized + debuginfo] target(s) in 5.23s\nsome extra verbose output padding to ensure enough bytes for token estimation to be meaningful here padding padding";
        let line2 = format!(
            r#"{{"message":{{"role":"user","content":[{{"type":"tool_result","tool_use_id":"t1","content":"{}"}}]}}}}"#,
            output_text.replace('\n', "\\n")
        );
        let session = format!("{}\n{}", line1, line2);
        let mut by_cat = std::collections::HashMap::new();
        let mut dev_sub = std::collections::HashMap::new();
        process_session_for_whatif(&session, &mut by_cat, &mut dev_sub);
        let sw = by_cat.get(&UsageCategory::SoftwareDev);
        assert!(
            sw.is_some(),
            "expected SoftwareDev category, got: {:?}",
            by_cat.keys().collect::<Vec<_>>()
        );
        let sw = sw.unwrap();
        assert_eq!(sw.total_commands, 1);
        assert!(sw.bash_tokens > 0, "bash_tokens should be > 0");
        // Nushell should produce fewer tokens than raw bash
        assert!(
            sw.nushell_tokens <= sw.bash_tokens,
            "nu {} should be <= bash {}",
            sw.nushell_tokens,
            sw.bash_tokens
        );
        assert_eq!(sw.nushell_matched, 1);
    }
}
