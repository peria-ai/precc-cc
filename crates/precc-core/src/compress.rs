//! Pillar 3 (Rust): Context file compression.
//!
//! Strips filler words and verbose phrasing from CLAUDE.md and memory files,
//! reducing tokens loaded on every Claude Code API call.
//!
//! Ported from token-saver patterns (MIT-0, by RubenAQuispe).

use anyhow::Result;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

/// Minimum savings percentage to bother compressing a file.
const MIN_SAVINGS_PCT: usize = 5;

/// A replacement rule: regex pattern → replacement string.
struct Rule {
    re: Regex,
    replacement: &'static str,
}

macro_rules! rule {
    ($pat:expr, $rep:expr) => {
        Rule {
            re: Regex::new($pat).unwrap(),
            replacement: $rep,
        }
    };
}

/// All compression patterns, compiled once.
static RULES: LazyLock<Vec<Rule>> = LazyLock::new(|| {
    vec![
        // Filler removal
        rule!(r"(?i)\bplease\b", ""),
        rule!(r"(?i)\bkindly\b", ""),
        rule!(r"(?i)\bjust\b", ""),
        rule!(r"(?i)\bsimply\b", ""),
        rule!(r"(?i)\bbasically\b", ""),
        rule!(r"(?i)\bactually\b", ""),
        rule!(r"(?i)\bIn order to\b", "To"),
        rule!(r"(?i)\bdue to the fact that\b", "because"),
        rule!(r"(?i)\bat this point in time\b", "now"),
        rule!(r"(?i)\bin the event that\b", "if"),
        rule!(r"(?i)\bfor the purpose of\b", "to"),
        rule!(r"(?i)\bwith regard to\b", "re:"),
        rule!(r"(?i)\bin terms of\b", "re:"),
        rule!(r"(?i)\bIt is important to note that\b", "Note:"),
        rule!(r"(?i)\bIt should be noted that\b", "Note:"),
        rule!(r"(?i)\bAs mentioned (?:earlier|previously|above)\b", ""),
        rule!(r"(?i)\bAs you (?:may |might )?know\b", ""),
        // Action patterns
        rule!(r"(?i)\bBefore doing anything else\b", "First"),
        // Common phrases
        rule!(r"(?i)\byou should\b", ""),
        rule!(r"(?i)\bmake sure (?:to |that )?", "ensure "),
        rule!(r"(?i)\bkeep in mind (?:that )?", "note: "),
        rule!(r"(?i)\bfor example\b", "e.g."),
        rule!(r"(?i)\bsuch as\b", "e.g."),
        rule!(r"(?i)\betc\.?\b", "..."),
        rule!(r"(?i)\band so on\b", "..."),
        rule!(r"(?i)\band others?\b", "..."),
        rule!(r"(?i)\bincluding but not limited to\b", "incl."),
        rule!(r"(?i)\bin other words\b", "i.e."),
        rule!(r"(?i)\bthat is to say\b", "i.e."),
        // Whitespace cleanup (applied last)
        rule!(r"  +", " "),
        rule!(r"\n +", "\n"),
        rule!(r" +\n", "\n"),
        rule!(r"\n{3,}", "\n\n"),
    ]
});

/// Result of compressing a single file.
#[derive(Debug)]
pub struct CompressResult {
    pub file: PathBuf,
    pub original_tokens: usize,
    pub compressed_tokens: usize,
    pub saved_tokens: usize,
    pub pct_saved: usize,
}

/// Apply all compression patterns to the given content.
pub fn compress(content: &str) -> String {
    let mut result = content.to_string();
    for rule in RULES.iter() {
        result = rule.re.replace_all(&result, rule.replacement).to_string();
    }
    // Final whitespace cleanup
    result = result.trim().to_string();
    result.push('\n');
    result
}

/// Estimate token count from text length (1 token ≈ 4 bytes).
pub fn estimate_tokens(text: &str) -> usize {
    text.len() / 4
}

/// Discover context files eligible for compression.
///
/// Searches for CLAUDE.md and .claude/memory/*.md files.
pub fn discover_files(project_dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    // CLAUDE.md at project root
    let claude_md = project_dir.join("CLAUDE.md");
    if claude_md.is_file() {
        files.push(claude_md);
    }

    // .claude/CLAUDE.md
    let dot_claude_md = project_dir.join(".claude").join("CLAUDE.md");
    if dot_claude_md.is_file() {
        files.push(dot_claude_md);
    }

    // Memory files
    let mem_dir = project_dir.join(".claude").join("memory");
    if mem_dir.is_dir() {
        if let Ok(entries) = std::fs::read_dir(&mem_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().and_then(|e| e.to_str()) == Some("md") {
                    let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if !name.ends_with(".backup") {
                        files.push(p);
                    }
                }
            }
        }
    }

    // Nested CLAUDE.md in first-level subdirectories
    if let Ok(entries) = std::fs::read_dir(project_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with('.') {
                continue;
            }
            if !entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                continue;
            }
            let sub = entry.path().join("CLAUDE.md");
            if sub.is_file() {
                files.push(sub);
            }
        }
    }

    files
}

/// Compress all discovered context files.
///
/// Returns results for files that had significant savings (>= 5%).
/// Creates `.backup` files before overwriting unless `dry_run` is true.
pub fn compress_files(project_dir: &Path, dry_run: bool) -> Result<Vec<CompressResult>> {
    let files = discover_files(project_dir);
    let mut results = Vec::new();

    for file in &files {
        let original = std::fs::read_to_string(file)?;
        let compressed = compress(&original);
        let orig_tokens = estimate_tokens(&original);
        let comp_tokens = estimate_tokens(&compressed);

        if orig_tokens == 0 {
            continue;
        }

        let saved = orig_tokens.saturating_sub(comp_tokens);
        let pct = saved * 100 / orig_tokens;

        if pct < MIN_SAVINGS_PCT {
            continue;
        }

        if !dry_run {
            // Create backup if none exists
            let backup = PathBuf::from(format!("{}.backup", file.display()));
            if !backup.exists() {
                std::fs::copy(file, &backup)?;
            }
            std::fs::write(file, &compressed)?;
        }

        results.push(CompressResult {
            file: file.clone(),
            original_tokens: orig_tokens,
            compressed_tokens: comp_tokens,
            saved_tokens: saved,
            pct_saved: pct,
        });
    }

    Ok(results)
}

/// Revert all compressed files from their backups.
///
/// Returns the number of files reverted.
pub fn revert_files(project_dir: &Path) -> Result<usize> {
    let files = discover_files(project_dir);
    let mut reverted = 0;

    for file in &files {
        let backup = PathBuf::from(format!("{}.backup", file.display()));
        if backup.exists() {
            std::fs::copy(&backup, file)?;
            std::fs::remove_file(&backup)?;
            reverted += 1;
        }
    }

    Ok(reverted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_removes_filler_words() {
        let input = "Please just make sure to basically run the tests.\n";
        let output = compress(input);
        assert!(!output.contains("Please"));
        assert!(!output.contains("just"));
        assert!(!output.contains("basically"));
        assert!(output.contains("ensure"));
    }

    #[test]
    fn compress_replaces_verbose_phrases() {
        let input = "In order to fix the bug, due to the fact that it fails.\n";
        let output = compress(input);
        assert!(output.contains("To"));
        assert!(output.contains("because"));
        assert!(!output.contains("In order to"));
        assert!(!output.contains("due to the fact that"));
    }

    #[test]
    fn compress_abbreviates_common_phrases() {
        let input = "For example, such as lists and so on including but not limited to etc.\n";
        let output = compress(input);
        assert!(output.contains("e.g."));
        assert!(output.contains("incl."));
    }

    #[test]
    fn compress_collapses_whitespace() {
        let input = "Hello   world\n\n\n\nGoodbye\n";
        let output = compress(input);
        assert!(!output.contains("   "));
        assert!(!output.contains("\n\n\n"));
    }

    #[test]
    fn compress_preserves_code_blocks() {
        // Code blocks should pass through mostly unchanged (no filler words typically)
        let input = "```bash\ncargo build --release\n```\n";
        let output = compress(input);
        assert!(output.contains("cargo build --release"));
    }

    #[test]
    fn estimate_tokens_basic() {
        assert_eq!(estimate_tokens("1234"), 1);
        assert_eq!(estimate_tokens("12345678"), 2);
    }

    #[test]
    fn discover_files_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let files = discover_files(dir.path());
        assert!(files.is_empty());
    }

    #[test]
    fn discover_files_finds_claude_md() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("CLAUDE.md"), "# Test\n").unwrap();
        let files = discover_files(dir.path());
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("CLAUDE.md"));
    }

    #[test]
    fn discover_files_finds_memory_files() {
        let dir = tempfile::tempdir().unwrap();
        let mem_dir = dir.path().join(".claude").join("memory");
        std::fs::create_dir_all(&mem_dir).unwrap();
        std::fs::write(mem_dir.join("note.md"), "# Note\n").unwrap();
        std::fs::write(mem_dir.join("note.md.backup"), "backup").unwrap();
        let files = discover_files(dir.path());
        assert_eq!(files.len(), 1); // backup excluded
    }

    #[test]
    fn compress_and_revert_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("CLAUDE.md");
        let original = "Please just basically make sure to run all the tests before doing anything else with the code in order to verify that everything works.\n";
        std::fs::write(&file, original).unwrap();

        let results = compress_files(dir.path(), false).unwrap();
        assert!(!results.is_empty());

        // File should be compressed
        let compressed = std::fs::read_to_string(&file).unwrap();
        assert!(compressed.len() < original.len());

        // Revert
        let reverted = revert_files(dir.path()).unwrap();
        assert_eq!(reverted, 1);

        // File should match original
        let restored = std::fs::read_to_string(&file).unwrap();
        assert_eq!(restored, original);
    }

    #[test]
    fn compress_files_dry_run_does_not_modify() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("CLAUDE.md");
        let original = "Please just basically make sure to run all the tests in order to verify everything works properly.\n";
        std::fs::write(&file, original).unwrap();

        let results = compress_files(dir.path(), true).unwrap();
        assert!(!results.is_empty());

        // File should be unchanged
        let content = std::fs::read_to_string(&file).unwrap();
        assert_eq!(content, original);
    }
}
