//! SQLite connection management for PRECC databases.
//!
//! Manages two databases:
//! - `history.db`: Mined failure-fix pairs (Pillar 3)
//! - `heuristics.db`: Automation skills (Pillar 4)
//!
//! Both live at `~/.local/share/precc/`.

use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::{Path, PathBuf};

/// Default data directory: `~/.local/share/precc/`
pub fn data_dir() -> Result<PathBuf> {
    let home = std::env::var("HOME").context("HOME not set")?;
    Ok(PathBuf::from(home).join(".local/share/precc"))
}

/// Open a SQLite connection with performance-optimized pragmas.
fn open_connection(path: &Path) -> Result<Connection> {
    let conn = Connection::open(path)
        .with_context(|| format!("failed to open database: {}", path.display()))?;

    conn.execute_batch(
        "PRAGMA journal_mode=WAL;
         PRAGMA synchronous=NORMAL;
         PRAGMA mmap_size=268435456;
         PRAGMA cache_size=-8000;",
    )?;

    Ok(conn)
}

/// Open or create the heuristics database with schema.
pub fn open_heuristics(data_dir: &Path) -> Result<Connection> {
    std::fs::create_dir_all(data_dir)?;
    let path = data_dir.join("heuristics.db");
    let conn = open_connection(&path)?;
    init_heuristics_schema(&conn)?;
    Ok(conn)
}

/// Open heuristics.db in read-only mode, skipping schema init.
/// Returns None if the DB file doesn't exist yet.
pub fn open_heuristics_readonly(data_dir: &Path) -> Result<Option<Connection>> {
    let path = data_dir.join("heuristics.db");
    if !path.exists() {
        return Ok(None);
    }
    let conn = Connection::open_with_flags(
        &path,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .with_context(|| format!("failed to open database: {}", path.display()))?;
    Ok(Some(conn))
}

/// Open or create the history database with schema.
pub fn open_history(data_dir: &Path) -> Result<Connection> {
    std::fs::create_dir_all(data_dir)?;
    let path = data_dir.join("history.db");
    let conn = open_connection(&path)?;
    init_history_schema(&conn)?;
    Ok(conn)
}

/// Open or create the metrics database with schema.
pub fn open_metrics(data_dir: &Path) -> Result<Connection> {
    std::fs::create_dir_all(data_dir)?;
    let path = data_dir.join("metrics.db");
    let conn = open_connection(&path)?;
    init_metrics_schema(&conn)?;
    Ok(conn)
}

fn init_heuristics_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS skills (
            id              INTEGER PRIMARY KEY,
            name            TEXT UNIQUE NOT NULL,
            description     TEXT NOT NULL,
            source          TEXT NOT NULL,
            enabled         BOOLEAN DEFAULT 1,
            priority        INTEGER DEFAULT 100,
            created_at      TEXT NOT NULL,
            updated_at      TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS skill_triggers (
            id          INTEGER PRIMARY KEY,
            skill_id    INTEGER REFERENCES skills(id),
            trigger_type TEXT NOT NULL,
            pattern     TEXT NOT NULL,
            weight      REAL DEFAULT 1.0
        );

        CREATE TABLE IF NOT EXISTS skill_actions (
            id          INTEGER PRIMARY KEY,
            skill_id    INTEGER REFERENCES skills(id),
            action_type TEXT NOT NULL,
            template    TEXT NOT NULL,
            confidence  REAL DEFAULT 0.5
        );

        CREATE TABLE IF NOT EXISTS skill_stats (
            id          INTEGER PRIMARY KEY,
            skill_id    INTEGER UNIQUE REFERENCES skills(id),
            activated   INTEGER DEFAULT 0,
            succeeded   INTEGER DEFAULT 0,
            failed      INTEGER DEFAULT 0,
            last_used   TEXT
        );

        CREATE INDEX IF NOT EXISTS idx_triggers_type ON skill_triggers(trigger_type);
        CREATE INDEX IF NOT EXISTS idx_triggers_pattern ON skill_triggers(pattern);
        CREATE INDEX IF NOT EXISTS idx_skills_enabled ON skills(enabled);",
    )?;
    Ok(())
}

fn init_history_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS sessions (
            id          INTEGER PRIMARY KEY,
            session_id  TEXT UNIQUE NOT NULL,
            project     TEXT,
            started_at  TEXT NOT NULL,
            mined_at    TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS events (
            id          INTEGER PRIMARY KEY,
            session_id  INTEGER REFERENCES sessions(id),
            timestamp   TEXT NOT NULL,
            tool        TEXT NOT NULL,
            input_json  TEXT NOT NULL,
            output_json TEXT,
            exit_code   INTEGER,
            is_failure  BOOLEAN DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS failure_fix_pairs (
            id              INTEGER PRIMARY KEY,
            failure_event   INTEGER REFERENCES events(id),
            fix_event       INTEGER REFERENCES events(id),
            pattern_hash    TEXT NOT NULL,
            failure_command TEXT NOT NULL,
            failure_output  TEXT NOT NULL,
            fix_command     TEXT NOT NULL,
            project_type    TEXT,
            confidence      REAL DEFAULT 0.5,
            occurrences     INTEGER DEFAULT 1,
            created_at      TEXT NOT NULL,
            updated_at      TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_ffp_pattern ON failure_fix_pairs(pattern_hash);
        CREATE INDEX IF NOT EXISTS idx_ffp_project ON failure_fix_pairs(project_type);
        CREATE INDEX IF NOT EXISTS idx_events_session ON events(session_id);",
    )?;

    // Additive migrations: add new columns if they don't exist yet.
    // SQLite doesn't support ADD COLUMN IF NOT EXISTS, so we ignore errors.
    let _ = conn
        .execute_batch("ALTER TABLE sessions ADD COLUMN precc_events_extracted BOOLEAN DEFAULT 0;");
    let _ = conn.execute_batch(
        "ALTER TABLE failure_fix_pairs ADD COLUMN precc_prevented INTEGER DEFAULT 0;",
    );

    Ok(())
}

fn init_metrics_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS metrics (
            id          INTEGER PRIMARY KEY,
            timestamp   TEXT NOT NULL,
            metric_type TEXT NOT NULL,
            value       REAL NOT NULL,
            metadata    TEXT
        );

        CREATE INDEX IF NOT EXISTS idx_metrics_type ON metrics(metric_type);
        CREATE INDEX IF NOT EXISTS idx_metrics_time ON metrics(timestamp);",
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_heuristics_creates_tables() {
        let dir = tempfile::tempdir().unwrap();
        let conn = open_heuristics(dir.path()).unwrap();
        // Verify tables exist by querying them
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM skills", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn open_history_creates_tables() {
        let dir = tempfile::tempdir().unwrap();
        let conn = open_history(dir.path()).unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM failure_fix_pairs", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn open_metrics_creates_tables() {
        let dir = tempfile::tempdir().unwrap();
        let conn = open_metrics(dir.path()).unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM metrics", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn creates_data_dir_if_missing() {
        let dir = tempfile::tempdir().unwrap();
        let subdir = dir.path().join("nested/precc");
        assert!(!subdir.exists());
        open_heuristics(&subdir).unwrap();
        assert!(subdir.exists());
        assert!(subdir.join("heuristics.db").exists());
    }

    #[test]
    fn idempotent_schema_creation() {
        let dir = tempfile::tempdir().unwrap();
        // Open twice — second open should not fail
        let _conn1 = open_heuristics(dir.path()).unwrap();
        let _conn2 = open_heuristics(dir.path()).unwrap();
    }

    #[test]
    fn data_dir_uses_home() {
        // Just verify it doesn't panic; actual path depends on env
        let _ = data_dir();
    }
}
