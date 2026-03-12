//! Skill advisor: suggests new skills that could enhance savings
//! and identifies ineffective skills that should be disabled.
//!
//! The advisor analyzes:
//! 1. Failure-fix pairs not yet promoted to skills (unmet opportunities)
//! 2. Frequently occurring commands with no skill coverage (coverage gaps)
//! 3. Skills with high failure rates or zero activations (ineffective)
//! 4. Skills whose savings are negligible relative to their failure cost

use anyhow::Result;
use rusqlite::Connection;

// ─── Suggestions (new skills) ───────────────────────────────────────────────

/// A suggested new skill based on observed patterns.
#[derive(Debug)]
pub struct SkillSuggestion {
    /// Proposed human-readable name.
    pub name: String,
    /// Why this skill is being suggested.
    pub reason: String,
    /// The failure command pattern that triggers it.
    pub trigger_pattern: String,
    /// The proposed fix template.
    pub fix_template: String,
    /// How many times the underlying failure was observed.
    pub occurrences: i64,
    /// Estimated tokens that would be saved per occurrence.
    pub est_tokens_per_hit: f64,
    /// The project type context (e.g. "rust", "node"), if known.
    pub project_type: Option<String>,
}

/// An ineffective skill that should be considered for disabling.
#[derive(Debug)]
pub struct IneffectiveSkill {
    pub skill_id: i64,
    pub name: String,
    pub source: String,
    /// Why this skill is flagged.
    pub reason: String,
    pub activated: i64,
    pub succeeded: i64,
    pub failed: i64,
    /// failure_rate = failed / activated (0.0–1.0)
    pub failure_rate: f64,
    /// Whether auto-disable was applied by this advisor call.
    pub auto_disabled: bool,
}

/// A token optimization recommendation for an existing skill.
#[derive(Debug)]
pub struct TokenOptimization {
    pub skill_name: String,
    pub action_type: String,
    pub current_template: String,
    /// What optimization to apply.
    pub optimization: String,
    /// Estimated token savings per activation from the optimization.
    pub est_savings_per_act: f64,
    /// Number of activations to date (for projected total savings).
    pub activations: i64,
}

/// Full advisor report.
#[derive(Debug)]
pub struct AdvisorReport {
    pub suggestions: Vec<SkillSuggestion>,
    pub ineffective: Vec<IneffectiveSkill>,
    pub optimizations: Vec<TokenOptimization>,
    pub disabled_count: usize,
}

// ─── Thresholds ─────────────────────────────────────────────────────────────

/// Minimum occurrences of a failure pattern before suggesting a skill.
const MIN_SUGGEST_OCCURRENCES: i64 = 2;

/// Minimum activations before judging effectiveness.
const MIN_ACTIVATIONS_FOR_JUDGEMENT: i64 = 3;

/// Failure rate above which a skill is flagged as ineffective.
const INEFFECTIVE_FAILURE_RATE: f64 = 0.30;

/// Activations threshold: skills enabled for a long time with zero
/// activations are flagged as dead weight.
const STALE_ACTIVATION_THRESHOLD: i64 = 0;

/// Tokens saved per prevented failure (conservative).
const TOKENS_PER_PREVENTION: f64 = 250.0;

// ─── Public API ─────────────────────────────────────────────────────────────

/// Run the full advisor analysis and optionally auto-disable ineffective skills.
///
/// `auto_disable`: if true, actually disable skills flagged as ineffective
/// (mined only — builtins are never auto-disabled but are still reported).
pub fn advise(
    heuristics_conn: &Connection,
    history_conn: &Connection,
    auto_disable: bool,
) -> Result<AdvisorReport> {
    let suggestions = suggest_new_skills(heuristics_conn, history_conn)?;
    let (ineffective, disabled_count) = find_ineffective_skills(heuristics_conn, auto_disable)?;
    let optimizations = find_token_optimizations(heuristics_conn)?;

    Ok(AdvisorReport {
        suggestions,
        ineffective,
        optimizations,
        disabled_count,
    })
}

/// Scan failure-fix pairs that have NOT been promoted to skills yet,
/// and suggest them as potential new skills.
fn suggest_new_skills(
    heuristics_conn: &Connection,
    history_conn: &Connection,
) -> Result<Vec<SkillSuggestion>> {
    // Get all existing skill trigger patterns to check coverage
    let existing_patterns: Vec<String> = {
        let mut stmt = heuristics_conn.prepare(
            "SELECT t.pattern FROM skills s
             JOIN skill_triggers t ON t.skill_id = s.id
             WHERE t.trigger_type = 'command_regex'",
        )?;
        let rows: Vec<String> = stmt
            .query_map([], |r| r.get(0))?
            .filter_map(|r| r.ok())
            .collect();
        rows
    };

    // Get all failure-fix pairs above the threshold, ordered by occurrences
    let mut stmt = history_conn.prepare(
        "SELECT failure_command, failure_output, fix_command, occurrences, project_type
         FROM failure_fix_pairs
         WHERE occurrences >= ?1
         ORDER BY occurrences DESC",
    )?;

    let mut suggestions = Vec::new();

    let rows = stmt.query_map([MIN_SUGGEST_OCCURRENCES], |r| {
        Ok((
            r.get::<_, String>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, String>(2)?,
            r.get::<_, i64>(3)?,
            r.get::<_, Option<String>>(4)?,
        ))
    })?;

    for row in rows {
        let (fail_cmd, _fail_output, fix_cmd, occurrences, project_type) = row?;

        // Skip edit-based fixes (code changes, not command rewrites)
        if fix_cmd.starts_with("edit:") {
            continue;
        }

        // Check if any existing skill already covers this failure command
        let already_covered = existing_patterns.iter().any(|pat| {
            regex::Regex::new(pat)
                .map(|re| re.is_match(&fail_cmd))
                .unwrap_or(false)
        });

        if already_covered {
            continue;
        }

        // Generate a suggested skill
        let first_words: Vec<&str> = fail_cmd.split_whitespace().take(2).collect();
        let name = format!("suggested-{}", first_words.join("-"));
        let trigger = crate::promote::generate_trigger_regex_pub(&fail_cmd);
        let (action_type, template) = crate::promote::generate_action_pub(&fail_cmd, &fix_cmd);

        let reason = if fix_cmd.starts_with("cd ") && fix_cmd.contains("&&") {
            format!(
                "Wrong-dir pattern seen {occurrences}x: `{}` fails, fixed by prepending `cd`",
                truncate(&fail_cmd, 40)
            )
        } else {
            format!(
                "Failure pattern seen {occurrences}x: `{}` -> `{}`",
                truncate(&fail_cmd, 30),
                truncate(&fix_cmd, 30)
            )
        };

        let fix_template = if action_type == "prepend_cd" {
            "cd {{project_root}} && {{original_command}}".to_string()
        } else {
            template
        };

        suggestions.push(SkillSuggestion {
            name,
            reason,
            trigger_pattern: trigger,
            fix_template,
            occurrences,
            est_tokens_per_hit: TOKENS_PER_PREVENTION,
            project_type,
        });
    }

    Ok(suggestions)
}

/// Find ineffective skills: high failure rate, or zero activations.
/// If `auto_disable` is true, disable mined skills that are clearly harmful.
fn find_ineffective_skills(
    conn: &Connection,
    auto_disable: bool,
) -> Result<(Vec<IneffectiveSkill>, usize)> {
    let mut stmt = conn.prepare(
        "SELECT s.id, s.name, s.source, s.enabled,
                COALESCE(st.activated, 0), COALESCE(st.succeeded, 0), COALESCE(st.failed, 0),
                s.created_at
         FROM skills s
         LEFT JOIN skill_stats st ON st.skill_id = s.id
         WHERE s.enabled = 1
         ORDER BY COALESCE(st.activated, 0) ASC",
    )?;

    struct SkillRow {
        id: i64,
        name: String,
        source: String,
        activated: i64,
        succeeded: i64,
        failed: i64,
        created_at: String,
    }

    let mut ineffective = Vec::new();
    let mut disabled_count = 0;

    let rows: Vec<SkillRow> = stmt
        .query_map([], |r| {
            Ok(SkillRow {
                id: r.get(0)?,
                name: r.get(1)?,
                source: r.get(2)?,
                // _enabled
                activated: r.get(4)?,
                succeeded: r.get(5)?,
                failed: r.get(6)?,
                created_at: r.get::<_, String>(7)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    let now_secs: u64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    for row in &rows {
        let failure_rate = if row.activated > 0 {
            row.failed as f64 / row.activated as f64
        } else {
            0.0
        };

        let mut reason = None;

        // High failure rate with enough data
        if row.activated >= MIN_ACTIVATIONS_FOR_JUDGEMENT && failure_rate > INEFFECTIVE_FAILURE_RATE
        {
            reason = Some(format!(
                "High failure rate: {:.0}% ({} failed / {} activated)",
                failure_rate * 100.0,
                row.failed,
                row.activated
            ));
        }

        // Zero activations for a skill that's been around a while (>7 days)
        if row.activated == STALE_ACTIVATION_THRESHOLD {
            let created_secs: u64 = row.created_at.parse().unwrap_or(0);
            let age_days = if created_secs > 0 {
                (now_secs.saturating_sub(created_secs)) / 86400
            } else {
                0
            };
            if age_days >= 7 {
                reason = Some(format!(
                    "Zero activations in {} days — trigger pattern may not match any commands",
                    age_days
                ));
            }
        }

        if let Some(reason) = reason {
            let mut auto_disabled = false;

            // Auto-disable mined skills with high failure rate
            if auto_disable
                && row.source == "mined"
                && row.activated >= MIN_ACTIVATIONS_FOR_JUDGEMENT
                && failure_rate > INEFFECTIVE_FAILURE_RATE
            {
                let now = crate::skills::chrono_now();
                conn.execute(
                    "UPDATE skills SET enabled = 0, updated_at = ?2 WHERE id = ?1",
                    rusqlite::params![row.id, now],
                )?;
                auto_disabled = true;
                disabled_count += 1;
            }

            ineffective.push(IneffectiveSkill {
                skill_id: row.id,
                name: row.name.clone(),
                source: row.source.clone(),
                reason,
                activated: row.activated,
                succeeded: row.succeeded,
                failed: row.failed,
                failure_rate,
                auto_disabled,
            });
        }
    }

    Ok((ineffective, disabled_count))
}

/// Analyze existing skill templates for token optimization opportunities.
///
/// Checks for patterns like:
/// - Output not piped through tail/head/grep (could be "sliced" to reduce tokens)
/// - Duplicate work that could be cached
/// - Commands that produce verbose output without filtering
fn find_token_optimizations(conn: &Connection) -> Result<Vec<TokenOptimization>> {
    let mut stmt = conn.prepare(
        "SELECT s.name, sa.action_type, sa.template, COALESCE(st.activated, 0)
         FROM skills s
         JOIN skill_actions sa ON sa.skill_id = s.id
         LEFT JOIN skill_stats st ON st.skill_id = s.id
         WHERE s.enabled = 1 AND sa.action_type = 'rewrite_command'
         ORDER BY st.activated DESC",
    )?;

    let mut optimizations = Vec::new();

    let rows: Vec<(String, String, String, i64)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?
        .filter_map(|r| r.ok())
        .collect();

    for (name, action_type, template, activations) in &rows {
        // Check: template produces unfiltered output (no pipe to tail/head/grep)
        let has_output_filter = template.contains("| tail")
            || template.contains("| head")
            || template.contains("| grep")
            || template.contains("2>&1 |");

        let is_verbose_command = template.contains("cargo build")
            || template.contains("cargo test")
            || template.contains("npm test")
            || template.contains("docker build")
            || template.contains("cargo doc");

        if is_verbose_command && !has_output_filter {
            optimizations.push(TokenOptimization {
                skill_name: name.clone(),
                action_type: action_type.clone(),
                current_template: template.clone(),
                optimization: "Slice output: pipe through `| tail -N` or `| grep -E 'error|warning'` to reduce token consumption by ~60%".to_string(),
                est_savings_per_act: 150.0,
                activations: *activations,
            });
        }

        // Check: cargo doc without caching guard
        if template.contains("cargo doc") && !template.contains("if [") {
            optimizations.push(TokenOptimization {
                skill_name: name.clone(),
                action_type: action_type.clone(),
                current_template: template.clone(),
                optimization: "Cache guard: wrap in `if [ -d target/doc ] && [ target/doc -nt Cargo.toml ]; then ... fi` to skip redundant doc generation (~500 tok/hit)".to_string(),
                est_savings_per_act: 500.0,
                activations: *activations,
            });
        }

        // Check: full `cargo build` when `cargo check` would suffice
        if template.contains("cargo build")
            && !template.contains("--release")
            && !template.contains("cargo check")
        {
            optimizations.push(TokenOptimization {
                skill_name: name.clone(),
                action_type: action_type.clone(),
                current_template: template.clone(),
                optimization: "Use `cargo check` instead of `cargo build` when only type-checking is needed — 2-5x faster, ~300 tok saved".to_string(),
                est_savings_per_act: 300.0,
                activations: *activations,
            });
        }
    }

    Ok(optimizations)
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    fn test_dbs() -> (Connection, Connection) {
        let dir1 = tempfile::tempdir().unwrap();
        let dir2 = tempfile::tempdir().unwrap();
        let history = db::open_history(dir1.path()).unwrap();
        let heuristics = db::open_heuristics(dir2.path()).unwrap();
        std::mem::forget(dir1);
        std::mem::forget(dir2);
        (history, heuristics)
    }

    #[test]
    fn no_suggestions_when_empty() {
        let (history, heuristics) = test_dbs();
        let report = advise(&heuristics, &history, false).unwrap();
        assert!(report.suggestions.is_empty());
        assert!(report.ineffective.is_empty());
        assert_eq!(report.disabled_count, 0);
    }

    #[test]
    fn suggests_uncovered_pattern() {
        let (history, heuristics) = test_dbs();
        let now = crate::skills::chrono_now();

        // Insert a session
        history
            .execute(
                "INSERT INTO sessions (session_id, project, started_at, mined_at)
                 VALUES ('test', 'test', ?1, ?1)",
                [&now],
            )
            .unwrap();

        // Insert a failure-fix pair that is NOT covered by any skill
        history
            .execute(
                "INSERT INTO failure_fix_pairs
                 (failure_event, fix_event, pattern_hash, failure_command, failure_output,
                  fix_command, project_type, occurrences, created_at, updated_at)
                 VALUES (NULL, NULL, 'hash1', 'mvn clean install', 'BUILD FAILURE',
                         'mvn clean install -DskipTests', 'java', 3, ?1, ?1)",
                [&now],
            )
            .unwrap();

        let report = advise(&heuristics, &history, false).unwrap();
        assert_eq!(report.suggestions.len(), 1);
        assert!(report.suggestions[0].name.contains("mvn"));
        assert_eq!(report.suggestions[0].occurrences, 3);
    }

    #[test]
    fn no_suggestion_when_already_covered() {
        let (history, heuristics) = test_dbs();
        let now = crate::skills::chrono_now();

        // Insert a skill covering cargo commands
        heuristics
            .execute(
                "INSERT INTO skills (name, description, source, priority, created_at, updated_at)
                 VALUES ('cargo-wrong-dir', 'test', 'builtin', 50, ?1, ?1)",
                [&now],
            )
            .unwrap();
        let sid = heuristics.last_insert_rowid();
        heuristics
            .execute(
                "INSERT INTO skill_triggers (skill_id, trigger_type, pattern, weight)
                 VALUES (?1, 'command_regex', '^cargo\\s+(build|test)', 1.0)",
                [sid],
            )
            .unwrap();

        // Insert a failure-fix pair for cargo build (covered by above)
        history
            .execute(
                "INSERT INTO sessions (session_id, project, started_at, mined_at)
                 VALUES ('test', 'test', ?1, ?1)",
                [&now],
            )
            .unwrap();
        history
            .execute(
                "INSERT INTO failure_fix_pairs
                 (failure_event, fix_event, pattern_hash, failure_command, failure_output,
                  fix_command, project_type, occurrences, created_at, updated_at)
                 VALUES (NULL, NULL, 'hash2', 'cargo build', 'error',
                         'cd /proj && cargo build', 'rust', 5, ?1, ?1)",
                [&now],
            )
            .unwrap();

        let report = advise(&heuristics, &history, false).unwrap();
        assert!(report.suggestions.is_empty());
    }

    #[test]
    fn flags_high_failure_rate_skill() {
        let (history, heuristics) = test_dbs();
        let now = crate::skills::chrono_now();

        heuristics
            .execute(
                "INSERT INTO skills (name, description, source, priority, created_at, updated_at)
                 VALUES ('bad-skill', 'test', 'mined', 200, ?1, ?1)",
                [&now],
            )
            .unwrap();
        let sid = heuristics.last_insert_rowid();
        heuristics
            .execute(
                "INSERT INTO skill_actions (skill_id, action_type, template, confidence)
                 VALUES (?1, 'rewrite_command', 'test', 0.7)",
                [sid],
            )
            .unwrap();
        heuristics
            .execute(
                "INSERT INTO skill_stats (skill_id, activated, succeeded, failed)
                 VALUES (?1, 10, 3, 7)",
                [sid],
            )
            .unwrap();

        let report = advise(&heuristics, &history, false).unwrap();
        assert_eq!(report.ineffective.len(), 1);
        assert_eq!(report.ineffective[0].name, "bad-skill");
        assert!(!report.ineffective[0].auto_disabled);
    }

    #[test]
    fn auto_disables_mined_high_failure() {
        let (history, heuristics) = test_dbs();
        let now = crate::skills::chrono_now();

        heuristics
            .execute(
                "INSERT INTO skills (name, description, source, priority, created_at, updated_at)
                 VALUES ('bad-mined', 'test', 'mined', 200, ?1, ?1)",
                [&now],
            )
            .unwrap();
        let sid = heuristics.last_insert_rowid();
        heuristics
            .execute(
                "INSERT INTO skill_actions (skill_id, action_type, template, confidence)
                 VALUES (?1, 'rewrite_command', 'test', 0.7)",
                [sid],
            )
            .unwrap();
        heuristics
            .execute(
                "INSERT INTO skill_stats (skill_id, activated, succeeded, failed)
                 VALUES (?1, 5, 1, 4)",
                [sid],
            )
            .unwrap();

        let report = advise(&heuristics, &history, true).unwrap();
        assert_eq!(report.disabled_count, 1);
        assert!(report.ineffective[0].auto_disabled);

        // Verify actually disabled in DB
        let enabled: bool = heuristics
            .query_row("SELECT enabled FROM skills WHERE id = ?1", [sid], |r| {
                r.get(0)
            })
            .unwrap();
        assert!(!enabled);
    }

    #[test]
    fn does_not_auto_disable_builtins() {
        let (history, heuristics) = test_dbs();
        let now = crate::skills::chrono_now();

        heuristics
            .execute(
                "INSERT INTO skills (name, description, source, priority, created_at, updated_at)
                 VALUES ('builtin-bad', 'test', 'builtin', 50, ?1, ?1)",
                [&now],
            )
            .unwrap();
        let sid = heuristics.last_insert_rowid();
        heuristics
            .execute(
                "INSERT INTO skill_actions (skill_id, action_type, template, confidence)
                 VALUES (?1, 'rewrite_command', 'test', 0.9)",
                [sid],
            )
            .unwrap();
        heuristics
            .execute(
                "INSERT INTO skill_stats (skill_id, activated, succeeded, failed)
                 VALUES (?1, 10, 2, 8)",
                [sid],
            )
            .unwrap();

        let report = advise(&heuristics, &history, true).unwrap();
        // Should be reported as ineffective but NOT auto-disabled
        assert_eq!(report.ineffective.len(), 1);
        assert!(!report.ineffective[0].auto_disabled);
        assert_eq!(report.disabled_count, 0);

        let enabled: bool = heuristics
            .query_row("SELECT enabled FROM skills WHERE id = ?1", [sid], |r| {
                r.get(0)
            })
            .unwrap();
        assert!(enabled);
    }

    #[test]
    fn flags_stale_zero_activation_skill() {
        let (history, heuristics) = test_dbs();
        // Use a timestamp >7 days ago
        let old_ts = {
            let secs = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                - (8 * 86400); // 8 days ago
            format!("{secs}")
        };

        heuristics
            .execute(
                "INSERT INTO skills (name, description, source, priority, created_at, updated_at)
                 VALUES ('stale-skill', 'test', 'mined', 200, ?1, ?1)",
                [&old_ts],
            )
            .unwrap();
        let sid = heuristics.last_insert_rowid();
        heuristics
            .execute(
                "INSERT INTO skill_actions (skill_id, action_type, template, confidence)
                 VALUES (?1, 'rewrite_command', 'test', 0.3)",
                [sid],
            )
            .unwrap();
        heuristics
            .execute(
                "INSERT INTO skill_stats (skill_id, activated, succeeded, failed)
                 VALUES (?1, 0, 0, 0)",
                [sid],
            )
            .unwrap();

        let report = advise(&heuristics, &history, false).unwrap();
        assert_eq!(report.ineffective.len(), 1);
        assert!(report.ineffective[0].reason.contains("Zero activations"));
    }
}
