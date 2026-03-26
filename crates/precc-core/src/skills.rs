//! Pillar 4: Skill matching engine.
//!
//! Queries heuristics.db for skills that match the current command,
//! evaluates triggers, and returns applicable actions sorted by confidence.

use anyhow::Result;
use regex::Regex;
use rusqlite::Connection;
use std::path::Path;

/// A matched skill with its action to apply.
#[derive(Debug)]
pub struct SkillMatch {
    pub skill_name: String,
    pub action_type: String,
    pub template: String,
    pub confidence: f64,
    pub skill_id: i64,
    /// "builtin" or "mined" — used for tier-based gating.
    pub source: String,
    /// Whether this skill interacts with Claude/Anthropic API.
    /// 0 = no/unknown, 1 = direct, 2 = indirect.
    pub claude_interaction: i32,
}

/// Query heuristics.db for skills matching the given command.
///
/// Returns matches sorted by confidence (highest first).
/// Only returns enabled skills with confidence >= `min_confidence`.
pub fn find_matches(
    conn: &Connection,
    command: &str,
    min_confidence: f64,
) -> Result<Vec<SkillMatch>> {
    // Get all enabled skills with command_regex triggers
    let mut stmt = conn.prepare_cached(
        "SELECT s.id, s.name, s.source, t.pattern, t.weight, a.action_type, a.template, a.confidence,
                COALESCE(s.claude_interaction, 0)
         FROM skills s
         JOIN skill_triggers t ON t.skill_id = s.id
         JOIN skill_actions a ON a.skill_id = s.id
         WHERE s.enabled = 1
           AND t.trigger_type = 'command_regex'
           AND a.confidence >= ?1
         ORDER BY s.priority ASC, a.confidence DESC",
    )?;

    let rows = stmt.query_map([min_confidence], |row| {
        Ok(CandidateRow {
            skill_id: row.get(0)?,
            skill_name: row.get(1)?,
            source: row.get(2)?,
            pattern: row.get(3)?,
            weight: row.get(4)?,
            action_type: row.get(5)?,
            template: row.get(6)?,
            confidence: row.get(7)?,
            claude_interaction: row.get(8)?,
        })
    })?;

    let mut matches = Vec::new();
    for row in rows {
        let row = row?;
        // Compile and test the regex trigger
        if let Ok(re) = Regex::new(&row.pattern) {
            if re.is_match(command) {
                // Check file_exists triggers for this skill
                let file_check_ok = check_file_triggers(conn, row.skill_id)?;
                if file_check_ok {
                    matches.push(SkillMatch {
                        skill_name: row.skill_name,
                        source: row.source,
                        action_type: row.action_type,
                        template: row.template,
                        confidence: row.confidence * row.weight,
                        skill_id: row.skill_id,
                        claude_interaction: row.claude_interaction,
                    });
                }
            }
        }
    }

    // Sort by confidence descending
    matches.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
    Ok(matches)
}

struct CandidateRow {
    skill_id: i64,
    skill_name: String,
    source: String,
    pattern: String,
    weight: f64,
    action_type: String,
    template: String,
    confidence: f64,
    claude_interaction: i32,
}

/// Check file_exists triggers for a skill.
/// Returns true if all file_exists conditions are satisfied.
fn check_file_triggers(conn: &Connection, skill_id: i64) -> Result<bool> {
    let mut stmt = conn.prepare_cached(
        "SELECT pattern FROM skill_triggers
         WHERE skill_id = ?1 AND trigger_type = 'file_exists'",
    )?;

    let patterns: Vec<String> = stmt
        .query_map([skill_id], |row| row.get(0))?
        .filter_map(|r| r.ok())
        .collect();

    if patterns.is_empty() {
        return Ok(true);
    }

    let cwd = std::env::current_dir().unwrap_or_default();
    for pattern in &patterns {
        if let Some(negated) = pattern.strip_prefix('!') {
            // '!' prefix means the file must NOT exist
            if cwd.join(negated).exists() {
                return Ok(false);
            }
        } else {
            // File must exist
            if !cwd.join(pattern).exists() {
                return Ok(false);
            }
        }
    }

    Ok(true)
}

/// Apply a skill action template to produce a modified command.
///
/// Supported placeholders:
/// - `{{original_command}}` — the original command string
/// - `{{project_root}}` — the resolved project root directory
pub fn apply_template(template: &str, original_command: &str, project_root: &str) -> String {
    template
        .replace("{{original_command}}", original_command)
        .replace("{{project_root}}", project_root)
}

/// Record that a skill was activated (for stats tracking).
pub fn record_activation(conn: &Connection, skill_id: i64) -> Result<()> {
    // Ensure the row exists first (INSERT OR IGNORE — no-op if already present)
    conn.execute(
        "INSERT OR IGNORE INTO skill_stats (skill_id, activated, succeeded, failed, last_used)
         VALUES (?1, 0, 0, 0, NULL)",
        [skill_id],
    )?;
    // Now increment the counter
    conn.execute(
        "UPDATE skill_stats SET activated = activated + 1, last_used = datetime('now')
         WHERE skill_id = ?1",
        [skill_id],
    )?;
    Ok(())
}

/// Write a plain-text first-word prefix cache to `data_dir/skill_prefixes.txt`.
///
/// The hook reads this file (a single syscall) to decide whether to open
/// heuristics.db at all.  If the command's first word is not listed, the
/// 7–8ms SQLCipher open cost is skipped entirely.
///
/// Each line is one first-word literal extracted from enabled `command_regex`
/// triggers (e.g. `^cargo\s+` → `cargo`).  A special sentinel line `*` means
/// "at least one skill has a pattern that cannot be reduced to a first-word
/// literal — always open the DB".
pub fn write_skill_prefixes(conn: &Connection, data_dir: &Path) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT t.pattern FROM skills s
         JOIN skill_triggers t ON t.skill_id = s.id
         WHERE s.enabled = 1 AND t.trigger_type = 'command_regex'",
    )?;
    let patterns: Vec<String> = stmt
        .query_map([], |row| row.get(0))?
        .filter_map(|r| r.ok())
        .collect();

    let mut prefixes: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut wildcard = false;

    for pat in &patterns {
        // Strip leading ^ and extract the first literal word.
        // Patterns like "^cargo\s+" → "cargo"
        // Patterns like "^(npm|npx|pnpm|yarn)\s+" → "npm", "npx", "pnpm", "yarn"
        let stripped = pat.strip_prefix('^').unwrap_or(pat);
        if stripped.starts_with('(') {
            // Alternation: extract words from "(a|b|c)\s+" or "(a|b|c)..."
            if let Some(end) = stripped.find(')') {
                let inner = &stripped[1..end];
                for word in inner.split('|') {
                    let w = word.trim();
                    if !w.is_empty()
                        && w.chars()
                            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
                    {
                        prefixes.insert(w.to_string());
                    } else {
                        wildcard = true;
                    }
                }
            } else {
                wildcard = true;
            }
        } else {
            // Single word: take chars up to first non-alphanumeric/non-hyphen/non-underscore
            let word: String = stripped
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
                .collect();
            if word.is_empty() {
                wildcard = true;
            } else {
                prefixes.insert(word);
            }
        }
    }

    let mut out = prefixes.into_iter().collect::<Vec<_>>().join("\n");
    if wildcard {
        if !out.is_empty() {
            out.push('\n');
        }
        out.push('*');
    }

    std::fs::write(data_dir.join("skill_prefixes.txt"), out)?;
    Ok(())
}

/// Load built-in skills from TOML files into heuristics.db (if not already present).
pub fn load_builtin_skills(conn: &Connection, skills_dir: &Path) -> Result<usize> {
    let mut loaded = 0;

    let entries = match std::fs::read_dir(skills_dir) {
        Ok(e) => e,
        Err(_) => return Ok(0),
    };

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }

        let content = std::fs::read_to_string(&path)?;
        if load_skill_toml(conn, &content)? {
            loaded += 1;
        }
    }

    Ok(loaded)
}

/// Load built-in skills from embedded TOML strings into heuristics.db.
///
/// Each entry is `(filename_hint, toml_content)`. Skills already present
/// (by name) are skipped. Returns the count of newly inserted skills.
pub fn load_builtin_skills_embedded(conn: &Connection, skills: &[(&str, &str)]) -> Result<usize> {
    let mut loaded = 0;
    for (_name, content) in skills {
        if load_skill_toml(conn, content)? {
            loaded += 1;
        }
    }
    Ok(loaded)
}

/// Parse a skill TOML and update an existing skill in heuristics.db.
///
/// Replaces the skill's metadata, triggers, and actions in-place.
/// The skill's stats row (`skill_stats`) is preserved unchanged.
/// Returns an error if the skill name in the TOML does not match `existing_name`,
/// or if the skill does not exist.
pub fn update_skill_toml(conn: &Connection, existing_name: &str, toml_content: &str) -> Result<()> {
    let doc: SkillDoc = toml::from_str(toml_content)?;

    if doc.skill.name != existing_name {
        anyhow::bail!(
            "skill name in TOML ({:?}) does not match existing name ({:?}); \
             rename is not supported via edit",
            doc.skill.name,
            existing_name
        );
    }

    let skill_id: Option<i64> = conn
        .query_row(
            "SELECT id FROM skills WHERE name = ?1",
            [existing_name],
            |r| r.get(0),
        )
        .ok();

    let skill_id = match skill_id {
        Some(id) => id,
        None => anyhow::bail!("skill '{}' not found", existing_name),
    };

    let now = chrono_now();

    // Update skill metadata
    conn.execute(
        "UPDATE skills SET description = ?2, source = ?3, priority = ?4, updated_at = ?5
         WHERE id = ?1",
        rusqlite::params![
            skill_id,
            doc.skill.description,
            doc.skill.source,
            doc.skill.priority,
            now
        ],
    )?;

    // Replace triggers
    conn.execute("DELETE FROM skill_triggers WHERE skill_id = ?1", [skill_id])?;
    for trigger in &doc.triggers {
        conn.execute(
            "INSERT INTO skill_triggers (skill_id, trigger_type, pattern, weight)
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![skill_id, trigger.r#type, trigger.pattern, trigger.weight],
        )?;
    }

    // Replace actions
    conn.execute("DELETE FROM skill_actions WHERE skill_id = ?1", [skill_id])?;
    for action in &doc.actions {
        conn.execute(
            "INSERT INTO skill_actions (skill_id, action_type, template, confidence)
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![skill_id, action.r#type, action.template, action.confidence],
        )?;
    }

    Ok(())
}

/// Parse a skill TOML and insert into heuristics.db.
/// Returns true if a new skill was inserted, false if it already existed.
fn load_skill_toml(conn: &Connection, toml_content: &str) -> Result<bool> {
    let doc: SkillDoc = toml::from_str(toml_content)?;

    // Check if skill already exists
    let exists: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM skills WHERE name = ?1",
        [&doc.skill.name],
        |r| r.get(0),
    )?;

    if exists {
        return Ok(false);
    }

    let now = chrono_now();

    conn.execute(
        "INSERT INTO skills (name, description, source, priority, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
        rusqlite::params![
            doc.skill.name,
            doc.skill.description,
            doc.skill.source,
            doc.skill.priority,
            now
        ],
    )?;

    let skill_id = conn.last_insert_rowid();

    for trigger in &doc.triggers {
        conn.execute(
            "INSERT INTO skill_triggers (skill_id, trigger_type, pattern, weight)
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![skill_id, trigger.r#type, trigger.pattern, trigger.weight],
        )?;
    }

    for action in &doc.actions {
        conn.execute(
            "INSERT INTO skill_actions (skill_id, action_type, template, confidence)
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![skill_id, action.r#type, action.template, action.confidence],
        )?;
    }

    // Initialize stats row
    conn.execute(
        "INSERT INTO skill_stats (skill_id, activated, succeeded, failed) VALUES (?1, 0, 0, 0)",
        [skill_id],
    )?;

    Ok(true)
}

pub fn chrono_now() -> String {
    // Simple UTC timestamp without pulling in chrono crate
    use std::time::SystemTime;
    let d = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", d.as_secs())
}

/// TOML deserialization structures for skill files.
#[derive(serde::Deserialize)]
struct SkillDoc {
    skill: SkillMeta,
    #[serde(default)]
    triggers: Vec<TriggerDef>,
    #[serde(default)]
    actions: Vec<ActionDef>,
}

#[derive(serde::Deserialize)]
struct SkillMeta {
    name: String,
    description: String,
    source: String,
    #[serde(default = "default_priority")]
    priority: i32,
}

fn default_priority() -> i32 {
    100
}

#[derive(serde::Deserialize)]
struct TriggerDef {
    r#type: String,
    pattern: String,
    #[serde(default = "default_weight")]
    weight: f64,
}

fn default_weight() -> f64 {
    1.0
}

#[derive(serde::Deserialize)]
struct ActionDef {
    r#type: String,
    template: String,
    #[serde(default = "default_confidence")]
    confidence: f64,
}

fn default_confidence() -> f64 {
    0.5
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    fn test_db() -> Connection {
        let dir = tempfile::tempdir().unwrap();
        db::open_heuristics(dir.path()).unwrap()
    }

    #[test]
    fn load_cargo_wrong_dir_skill() {
        let conn = test_db();
        let toml = r#"
[skill]
name = "cargo-wrong-dir"
description = "Fix cargo commands in wrong directory"
source = "builtin"
priority = 50

[[triggers]]
type = "command_regex"
pattern = "^cargo\\s+(build|test|clippy|check)"
weight = 1.0

[[triggers]]
type = "file_exists"
pattern = "!Cargo.toml"
weight = 0.8

[[actions]]
type = "prepend_cd"
template = "cd {{project_root}} && {{original_command}}"
confidence = 0.9
"#;
        assert!(load_skill_toml(&conn, toml).unwrap());
        // Second load should return false (already exists)
        assert!(!load_skill_toml(&conn, toml).unwrap());
    }

    #[test]
    fn find_matches_returns_matching_skills() {
        let conn = test_db();
        let now = chrono_now();

        // Insert a test skill manually
        conn.execute(
            "INSERT INTO skills (name, description, source, priority, created_at, updated_at)
             VALUES ('test-skill', 'test', 'builtin', 100, ?1, ?1)",
            [&now],
        )
        .unwrap();
        let skill_id = conn.last_insert_rowid();

        conn.execute(
            "INSERT INTO skill_triggers (skill_id, trigger_type, pattern, weight)
             VALUES (?1, 'command_regex', '^cargo\\s+build', 1.0)",
            [skill_id],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO skill_actions (skill_id, action_type, template, confidence)
             VALUES (?1, 'rewrite_command', 'rtk cargo build', 0.9)",
            [skill_id],
        )
        .unwrap();

        let matches = find_matches(&conn, "cargo build --release", 0.3).unwrap();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].skill_name, "test-skill");
        assert_eq!(matches[0].confidence, 0.9);
    }

    #[test]
    fn find_matches_filters_by_confidence() {
        let conn = test_db();
        let now = chrono_now();

        conn.execute(
            "INSERT INTO skills (name, description, source, priority, created_at, updated_at)
             VALUES ('low-conf', 'test', 'mined', 100, ?1, ?1)",
            [&now],
        )
        .unwrap();
        let skill_id = conn.last_insert_rowid();

        conn.execute(
            "INSERT INTO skill_triggers (skill_id, trigger_type, pattern, weight)
             VALUES (?1, 'command_regex', '^cargo', 1.0)",
            [skill_id],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO skill_actions (skill_id, action_type, template, confidence)
             VALUES (?1, 'rewrite_command', 'test', 0.2)",
            [skill_id],
        )
        .unwrap();

        // Should be excluded with min_confidence = 0.3
        let matches = find_matches(&conn, "cargo build", 0.3).unwrap();
        assert!(matches.is_empty());

        // Should be included with min_confidence = 0.1
        let matches = find_matches(&conn, "cargo build", 0.1).unwrap();
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn apply_template_replaces_placeholders() {
        let result = apply_template(
            "cd {{project_root}} && {{original_command}}",
            "cargo build",
            "/home/user/myapp",
        );
        assert_eq!(result, "cd /home/user/myapp && cargo build");
    }

    #[test]
    fn no_matches_for_unrelated_command() {
        let conn = test_db();
        let matches = find_matches(&conn, "echo hello", 0.0).unwrap();
        assert!(matches.is_empty());
    }

    #[test]
    fn update_skill_toml_changes_description() {
        let conn = test_db();
        let original = r#"
[skill]
name = "my-skill"
description = "original description"
source = "mined"
priority = 100

[[triggers]]
type = "command_regex"
pattern = "^cargo"
weight = 1.0

[[actions]]
type = "prepend_cd"
template = "cd {{project_root}} && {{original_command}}"
confidence = 0.5
"#;
        load_skill_toml(&conn, original).unwrap();

        let updated = r#"
[skill]
name = "my-skill"
description = "updated description"
source = "mined"
priority = 200

[[triggers]]
type = "command_regex"
pattern = "^cargo\\s+build"
weight = 0.9

[[actions]]
type = "prepend_cd"
template = "cd {{project_root}} && {{original_command}}"
confidence = 0.7
"#;
        update_skill_toml(&conn, "my-skill", updated).unwrap();

        // Verify description and priority changed
        let (desc, pri): (String, i64) = conn
            .query_row(
                "SELECT description, priority FROM skills WHERE name = 'my-skill'",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();
        assert_eq!(desc, "updated description");
        assert_eq!(pri, 200);

        // Verify trigger was replaced
        let trigger_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM skill_triggers WHERE skill_id = \
                 (SELECT id FROM skills WHERE name = 'my-skill')",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(trigger_count, 1);

        // Verify action confidence updated
        let conf: f64 = conn
            .query_row(
                "SELECT confidence FROM skill_actions WHERE skill_id = \
                 (SELECT id FROM skills WHERE name = 'my-skill')",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!((conf - 0.7).abs() < f64::EPSILON);
    }

    #[test]
    fn update_skill_toml_rejects_name_change() {
        let conn = test_db();
        let original = r#"
[skill]
name = "orig-skill"
description = "desc"
source = "mined"
priority = 100
"#;
        load_skill_toml(&conn, original).unwrap();

        let renamed = r#"
[skill]
name = "renamed-skill"
description = "desc"
source = "mined"
priority = 100
"#;
        let result = update_skill_toml(&conn, "orig-skill", renamed);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("rename is not supported"));
    }

    #[test]
    fn update_skill_toml_preserves_stats() {
        let conn = test_db();
        let toml = r#"
[skill]
name = "stats-skill"
description = "desc"
source = "mined"
priority = 100

[[triggers]]
type = "command_regex"
pattern = "^npm"
weight = 1.0

[[actions]]
type = "prepend_cd"
template = "cd {{project_root}} && {{original_command}}"
confidence = 0.5
"#;
        load_skill_toml(&conn, toml).unwrap();

        // Manually bump activation count
        let skill_id: i64 = conn
            .query_row(
                "SELECT id FROM skills WHERE name = 'stats-skill'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        conn.execute(
            "UPDATE skill_stats SET activated = 7 WHERE skill_id = ?1",
            [skill_id],
        )
        .unwrap();

        // Update the skill
        update_skill_toml(&conn, "stats-skill", toml).unwrap();

        // Stats should be preserved
        let activated: i64 = conn
            .query_row(
                "SELECT activated FROM skill_stats WHERE skill_id = ?1",
                [skill_id],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(activated, 7);
    }
}
