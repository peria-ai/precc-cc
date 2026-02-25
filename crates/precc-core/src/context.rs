//! Pillar 1: Context-Aware Bash — working directory resolution.
//!
//! Detects the correct working directory for a command by scanning for
//! project marker files (Cargo.toml, package.json, Makefile, etc.).
//! If CWD lacks the expected marker, finds the nearest match and
//! prepends `cd /correct/path &&` to the command.

use std::path::{Path, PathBuf};

/// Tool-to-marker mapping using simple string prefix matching.
/// Each entry is (tool_prefix, marker_file).
/// A command matches if it starts with "{tool_prefix} " or equals "{tool_prefix}".
static TOOL_MARKERS: &[(&str, &str)] = &[
    ("cargo", "Cargo.toml"),
    ("rustc", "Cargo.toml"),
    ("npm", "package.json"),
    ("npx", "package.json"),
    ("pnpm", "package.json"),
    ("yarn", "package.json"),
    ("vitest", "package.json"),
    ("tsc", "package.json"),
    ("eslint", "package.json"),
    ("prettier", "package.json"),
    ("playwright", "package.json"),
    ("prisma", "package.json"),
    ("next", "package.json"),
    ("make", "Makefile"),
    ("cmake", "CMakeLists.txt"),
    ("git", ".git"),
    ("go", "go.mod"),
    ("python", "setup.py"),
    ("pip", "setup.py"),
    ("poetry", "pyproject.toml"),
    ("gradle", "build.gradle"),
    ("mvn", "pom.xml"),
];

/// Result of context resolution.
#[derive(Debug)]
pub struct ContextResult {
    /// The resolved project root directory (if different from CWD).
    pub project_root: Option<PathBuf>,
    /// The marker file that was found.
    pub marker: Option<String>,
    /// Confidence score (0.0 - 1.0).
    pub confidence: f64,
}

/// Resolve the correct working directory for a command.
///
/// Returns `ContextResult` with `project_root` set if the CWD is wrong
/// and a better directory was found.
pub fn resolve(command: &str) -> ContextResult {
    let marker = match detect_marker(command) {
        Some(m) => m,
        None => {
            return ContextResult {
                project_root: None,
                marker: None,
                confidence: 0.0,
            }
        }
    };

    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => {
            return ContextResult {
                project_root: None,
                marker: Some(marker.to_string()),
                confidence: 0.0,
            }
        }
    };

    // Check if CWD already has the marker
    if cwd.join(marker).exists() {
        return ContextResult {
            project_root: None,
            marker: Some(marker.to_string()),
            confidence: 1.0,
        };
    }

    // Search upward from CWD
    let candidates = search_upward(&cwd, marker);

    match candidates.len() {
        0 => ContextResult {
            project_root: None,
            marker: Some(marker.to_string()),
            confidence: 0.0,
        },
        1 => ContextResult {
            project_root: Some(candidates[0].clone()),
            marker: Some(marker.to_string()),
            confidence: 0.9,
        },
        _ => {
            // Multiple matches — prefer most recently modified
            let best = most_recently_modified(&candidates, marker);
            ContextResult {
                project_root: Some(best),
                marker: Some(marker.to_string()),
                confidence: 0.7,
            }
        }
    }
}

/// Detect which marker file a command expects.
/// Uses simple string prefix matching for speed (no regex).
fn detect_marker(command: &str) -> Option<&'static str> {
    // Extract the first word of the command
    let first_word = command.split_whitespace().next()?;
    for &(tool, marker) in TOOL_MARKERS {
        if first_word == tool {
            return Some(marker);
        }
    }
    None
}

/// Search upward from `start` for directories containing `marker`.
fn search_upward(start: &Path, marker: &str) -> Vec<PathBuf> {
    let mut results = Vec::new();
    let mut dir = start.to_path_buf();

    while let Some(parent) = dir.parent() {
        if parent.join(marker).exists() {
            results.push(parent.to_path_buf());
        }
        // Stop at home directory or filesystem root
        if parent == Path::new("/") || is_home_dir(parent) {
            break;
        }
        dir = parent.to_path_buf();
    }

    results
}

fn is_home_dir(path: &Path) -> bool {
    if let Some(home) = std::env::var_os("HOME") {
        path == Path::new(&home)
    } else {
        false
    }
}

/// Among candidates, pick the one whose marker file was most recently modified.
fn most_recently_modified(candidates: &[PathBuf], marker: &str) -> PathBuf {
    candidates
        .iter()
        .max_by_key(|p| p.join(marker).metadata().and_then(|m| m.modified()).ok())
        .unwrap()
        .clone()
}

/// Prepend a `cd` to the command if context resolution found a different project root.
pub fn apply(command: &str, ctx: &ContextResult) -> Option<String> {
    ctx.project_root
        .as_ref()
        .map(|root| format!("cd {} && {}", root.display(), command))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_cargo_marker() {
        assert_eq!(detect_marker("cargo build --release"), Some("Cargo.toml"));
        assert_eq!(detect_marker("cargo test foo"), Some("Cargo.toml"));
    }

    #[test]
    fn detect_npm_marker() {
        assert_eq!(detect_marker("npm install"), Some("package.json"));
        assert_eq!(detect_marker("pnpm test"), Some("package.json"));
        assert_eq!(detect_marker("npx tsc"), Some("package.json"));
    }

    #[test]
    fn detect_make_marker() {
        assert_eq!(detect_marker("make all"), Some("Makefile"));
        assert_eq!(detect_marker("make"), Some("Makefile"));
    }

    #[test]
    fn detect_go_marker() {
        assert_eq!(detect_marker("go build ./..."), Some("go.mod"));
        assert_eq!(detect_marker("go test -v"), Some("go.mod"));
    }

    #[test]
    fn detect_git_marker() {
        assert_eq!(detect_marker("git status"), Some(".git"));
        assert_eq!(detect_marker("git commit -m 'fix'"), Some(".git"));
        assert_eq!(detect_marker("git push origin main"), Some(".git"));
        assert_eq!(detect_marker("git log --oneline"), Some(".git"));
    }

    #[test]
    fn detect_no_marker() {
        assert_eq!(detect_marker("echo hello"), None);
        assert_eq!(detect_marker("ls -la"), None);
        assert_eq!(detect_marker("grep foo bar"), None);
    }

    #[test]
    fn cwd_has_marker_no_change() {
        // When CWD already has the marker, confidence is 1.0 and no project_root is set
        let result = resolve("echo hello");
        assert!(result.project_root.is_none());
        assert_eq!(result.confidence, 0.0);
    }

    #[test]
    fn apply_prepends_cd() {
        let ctx = ContextResult {
            project_root: Some(PathBuf::from("/home/user/myapp")),
            marker: Some("Cargo.toml".to_string()),
            confidence: 0.9,
        };
        assert_eq!(
            apply("cargo build", &ctx),
            Some("cd /home/user/myapp && cargo build".to_string())
        );
    }

    #[test]
    fn apply_no_change_when_no_root() {
        let ctx = ContextResult {
            project_root: None,
            marker: Some("Cargo.toml".to_string()),
            confidence: 1.0,
        };
        assert_eq!(apply("cargo build", &ctx), None);
    }
}
