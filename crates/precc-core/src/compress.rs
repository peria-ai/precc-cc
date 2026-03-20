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

    // ── compress() individual patterns ──────────────────────────────────

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
    fn compress_removes_simply_and_actually() {
        let input = "You simply need to actually check the output.\n";
        let output = compress(input);
        assert!(!output.contains("simply"));
        assert!(!output.contains("actually"));
        assert!(output.contains("check the output"));
    }

    #[test]
    fn compress_removes_kindly() {
        let input = "Kindly review the code before merging.\n";
        let output = compress(input);
        assert!(!output.contains("Kindly"));
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
    fn compress_replaces_time_phrases() {
        let input = "At this point in time we should act in the event that it breaks.\n";
        let output = compress(input);
        assert!(output.contains("now"));
        assert!(output.contains("if"));
        assert!(!output.contains("At this point in time"));
        assert!(!output.contains("in the event that"));
    }

    #[test]
    fn compress_replaces_purpose_and_regard_phrases() {
        let input = "For the purpose of testing, with regard to the API.\n";
        let output = compress(input);
        assert!(!output.contains("For the purpose of"));
        assert!(!output.contains("with regard to"));
    }

    #[test]
    fn compress_replaces_note_phrases() {
        let input = "It is important to note that the API is slow. It should be noted that caching helps.\n";
        let output = compress(input);
        assert!(output.contains("Note:"));
        assert!(!output.contains("It is important to note that"));
        assert!(!output.contains("It should be noted that"));
    }

    #[test]
    fn compress_removes_as_mentioned() {
        let input = "As mentioned earlier, this is key. As you may know, it works.\n";
        let output = compress(input);
        assert!(!output.contains("As mentioned earlier"));
        assert!(!output.contains("As you may know"));
    }

    #[test]
    fn compress_replaces_before_doing() {
        let input = "Before doing anything else run the linter.\n";
        let output = compress(input);
        assert!(output.contains("First"));
        assert!(!output.contains("Before doing anything else"));
    }

    #[test]
    fn compress_removes_you_should() {
        let input = "You should always run tests first.\n";
        let output = compress(input);
        assert!(!output.contains("You should"));
    }

    #[test]
    fn compress_replaces_keep_in_mind() {
        let input = "Keep in mind that tests are important.\n";
        let output = compress(input);
        assert!(output.contains("note:"));
        assert!(!output.contains("Keep in mind"));
    }

    #[test]
    fn compress_abbreviates_common_phrases() {
        let input = "For example, such as lists and so on including but not limited to etc.\n";
        let output = compress(input);
        assert!(output.contains("e.g."));
        assert!(output.contains("incl."));
    }

    #[test]
    fn compress_replaces_in_other_words() {
        let input = "In other words, it fails. That is to say, it crashes.\n";
        let output = compress(input);
        assert!(output.contains("i.e."));
        assert!(!output.contains("In other words"));
        assert!(!output.contains("That is to say"));
    }

    #[test]
    fn compress_replaces_and_others() {
        let input = "Functions, classes, and others are exported.\n";
        let output = compress(input);
        assert!(output.contains("..."));
        assert!(!output.contains("and others"));
    }

    #[test]
    fn compress_in_terms_of() {
        let input = "In terms of performance, it is fast.\n";
        let output = compress(input);
        assert!(!output.contains("In terms of"));
    }

    // ── compress() whitespace handling ───────────────────────────────────

    #[test]
    fn compress_collapses_whitespace() {
        let input = "Hello   world\n\n\n\nGoodbye\n";
        let output = compress(input);
        assert!(!output.contains("   "));
        assert!(!output.contains("\n\n\n"));
    }

    #[test]
    fn compress_trims_trailing_spaces() {
        let input = "Hello world   \nGoodbye   \n";
        let output = compress(input);
        assert!(!output.contains("   \n"));
    }

    #[test]
    fn compress_trims_leading_spaces_on_lines() {
        let input = "   Hello world\n   Goodbye\n";
        let output = compress(input);
        // Leading spaces on lines should be collapsed
        assert!(!output.contains("   "));
    }

    #[test]
    fn compress_ends_with_newline() {
        let input = "Hello world";
        let output = compress(input);
        assert!(output.ends_with('\n'));
    }

    // ── compress() case insensitivity ────────────────────────────────────

    #[test]
    fn compress_case_insensitive() {
        let upper = "PLEASE run the tests BASICALLY.\n";
        let lower = "please run the tests basically.\n";
        let mixed = "Please run the tests Basically.\n";

        let out_upper = compress(upper);
        let out_lower = compress(lower);
        let out_mixed = compress(mixed);

        assert!(!out_upper.contains("PLEASE"));
        assert!(!out_lower.contains("please"));
        assert!(!out_mixed.contains("Please"));
        assert!(!out_upper.contains("BASICALLY"));
        assert!(!out_lower.contains("basically"));
        assert!(!out_mixed.contains("Basically"));
    }

    // ── compress() preserves important content ──────────────────────────

    #[test]
    fn compress_preserves_code_blocks() {
        let input = "```bash\ncargo build --release\n```\n";
        let output = compress(input);
        assert!(output.contains("cargo build --release"));
    }

    #[test]
    fn compress_preserves_urls() {
        let input = "See https://github.com/example/repo for details.\n";
        let output = compress(input);
        assert!(output.contains("https://github.com/example/repo"));
    }

    #[test]
    fn compress_preserves_headings() {
        let input = "# Main Title\n\n## Section\n\nContent here.\n";
        let output = compress(input);
        assert!(output.contains("# Main Title"));
        assert!(output.contains("## Section"));
    }

    #[test]
    fn compress_preserves_bullet_lists() {
        let input = "- Item one\n- Item two\n- Item three\n";
        let output = compress(input);
        assert!(output.contains("- Item one"));
        assert!(output.contains("- Item two"));
    }

    #[test]
    fn compress_empty_input() {
        let output = compress("");
        assert_eq!(output, "\n");
    }

    #[test]
    fn compress_already_compact() {
        let input = "Run tests.\n";
        let output = compress(input);
        assert_eq!(output, "Run tests.\n");
    }

    // ── estimate_tokens ─────────────────────────────────────────────────

    #[test]
    fn estimate_tokens_basic() {
        assert_eq!(estimate_tokens("1234"), 1);
        assert_eq!(estimate_tokens("12345678"), 2);
    }

    #[test]
    fn estimate_tokens_empty() {
        assert_eq!(estimate_tokens(""), 0);
    }

    #[test]
    fn estimate_tokens_short() {
        assert_eq!(estimate_tokens("ab"), 0); // 2/4 = 0
        assert_eq!(estimate_tokens("abc"), 0);
    }

    // ── discover_files ──────────────────────────────────────────────────

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
    fn discover_files_finds_dot_claude_md() {
        let dir = tempfile::tempdir().unwrap();
        let dot_claude = dir.path().join(".claude");
        std::fs::create_dir_all(&dot_claude).unwrap();
        std::fs::write(dot_claude.join("CLAUDE.md"), "# Dot Claude\n").unwrap();
        let files = discover_files(dir.path());
        assert_eq!(files.len(), 1);
        assert!(files[0].to_string_lossy().contains(".claude/CLAUDE.md"));
    }

    #[test]
    fn discover_files_finds_both_claude_mds() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("CLAUDE.md"), "# Root\n").unwrap();
        let dot_claude = dir.path().join(".claude");
        std::fs::create_dir_all(&dot_claude).unwrap();
        std::fs::write(dot_claude.join("CLAUDE.md"), "# Dot\n").unwrap();
        let files = discover_files(dir.path());
        assert_eq!(files.len(), 2);
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
    fn discover_files_finds_multiple_memory_files() {
        let dir = tempfile::tempdir().unwrap();
        let mem_dir = dir.path().join(".claude").join("memory");
        std::fs::create_dir_all(&mem_dir).unwrap();
        std::fs::write(mem_dir.join("user.md"), "# User\n").unwrap();
        std::fs::write(mem_dir.join("project.md"), "# Project\n").unwrap();
        std::fs::write(mem_dir.join("feedback.md"), "# Feedback\n").unwrap();
        let files = discover_files(dir.path());
        assert_eq!(files.len(), 3);
    }

    #[test]
    fn discover_files_finds_subdir_claude_md() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("subproject");
        std::fs::create_dir_all(&sub).unwrap();
        std::fs::write(sub.join("CLAUDE.md"), "# Sub\n").unwrap();
        let files = discover_files(dir.path());
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn discover_files_skips_hidden_subdirs() {
        let dir = tempfile::tempdir().unwrap();
        let hidden = dir.path().join(".hidden");
        std::fs::create_dir_all(&hidden).unwrap();
        std::fs::write(hidden.join("CLAUDE.md"), "# Hidden\n").unwrap();
        // .hidden/CLAUDE.md should NOT be found (hidden subdirs skipped)
        // but .claude/CLAUDE.md IS found (special case)
        let files = discover_files(dir.path());
        assert!(files.is_empty());
    }

    #[test]
    fn discover_files_ignores_non_md() {
        let dir = tempfile::tempdir().unwrap();
        let mem_dir = dir.path().join(".claude").join("memory");
        std::fs::create_dir_all(&mem_dir).unwrap();
        std::fs::write(mem_dir.join("notes.txt"), "not md\n").unwrap();
        std::fs::write(mem_dir.join("data.json"), "{}").unwrap();
        let files = discover_files(dir.path());
        assert!(files.is_empty());
    }

    // ── compress_files ──────────────────────────────────────────────────

    #[test]
    fn compress_files_skips_compact_files() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("CLAUDE.md");
        // This content has no filler words — should be skipped (< 5% savings)
        std::fs::write(&file, "# Build\n\n```bash\ncargo build\ncargo test\n```\n").unwrap();
        let results = compress_files(dir.path(), false).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn compress_files_returns_correct_stats() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("CLAUDE.md");
        let original = "Please just basically make sure to run all the tests before doing anything else with the code in order to verify that everything works properly and as you may know this is important.\n";
        std::fs::write(&file, original).unwrap();

        let results = compress_files(dir.path(), true).unwrap();
        assert_eq!(results.len(), 1);
        let r = &results[0];
        assert!(r.saved_tokens > 0);
        assert!(r.pct_saved >= MIN_SAVINGS_PCT);
        assert_eq!(r.original_tokens, estimate_tokens(original));
        assert!(r.compressed_tokens < r.original_tokens);
    }

    #[test]
    fn compress_files_creates_backup() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("CLAUDE.md");
        let original = "Please just basically make sure to run all the tests in order to verify everything works as you should know.\n";
        std::fs::write(&file, original).unwrap();

        compress_files(dir.path(), false).unwrap();

        let backup = dir.path().join("CLAUDE.md.backup");
        assert!(backup.exists());
        let backup_content = std::fs::read_to_string(&backup).unwrap();
        assert_eq!(backup_content, original);
    }

    #[test]
    fn compress_files_does_not_overwrite_existing_backup() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("CLAUDE.md");
        let backup_path = dir.path().join("CLAUDE.md.backup");
        let original_backup = "original backup content\n";

        std::fs::write(&file, "Please just basically make sure to run all the tests in order to verify everything works as you should know.\n").unwrap();
        std::fs::write(&backup_path, original_backup).unwrap();

        compress_files(dir.path(), false).unwrap();

        // Backup should not be overwritten
        let backup_content = std::fs::read_to_string(&backup_path).unwrap();
        assert_eq!(backup_content, original_backup);
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

        // No backup created
        let backup = dir.path().join("CLAUDE.md.backup");
        assert!(!backup.exists());
    }

    #[test]
    fn compress_files_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let results = compress_files(dir.path(), false).unwrap();
        assert!(results.is_empty());
    }

    // ── revert_files ────────────────────────────────────────────────────

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
    fn revert_no_backups() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("CLAUDE.md"), "# Test\n").unwrap();
        let reverted = revert_files(dir.path()).unwrap();
        assert_eq!(reverted, 0);
    }

    #[test]
    fn revert_removes_backup_files() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("CLAUDE.md");
        let backup = dir.path().join("CLAUDE.md.backup");
        std::fs::write(&file, "compressed\n").unwrap();
        std::fs::write(&backup, "original\n").unwrap();

        revert_files(dir.path()).unwrap();
        assert!(!backup.exists());
    }

    #[test]
    fn revert_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let reverted = revert_files(dir.path()).unwrap();
        assert_eq!(reverted, 0);
    }

    // ── compress_files with multiple files ──────────────────────────────

    #[test]
    fn compress_files_processes_multiple() {
        let dir = tempfile::tempdir().unwrap();

        // Root CLAUDE.md
        std::fs::write(
            dir.path().join("CLAUDE.md"),
            "Please just basically make sure to run all the tests in order to verify everything works properly.\n",
        )
        .unwrap();

        // Memory file
        let mem_dir = dir.path().join(".claude").join("memory");
        std::fs::create_dir_all(&mem_dir).unwrap();
        std::fs::write(
            mem_dir.join("user.md"),
            "As you may know, the user should kindly just basically make sure to follow the instructions in order to succeed.\n",
        )
        .unwrap();

        let results = compress_files(dir.path(), true).unwrap();
        assert!(results.len() >= 1); // At least one should have enough savings
    }
}
