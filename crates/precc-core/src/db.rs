//! SQLite connection management for PRECC databases.
//!
//! Manages two databases:
//! - `history.db`: Mined failure-fix pairs (Pillar 3)
//! - `heuristics.db`: Automation skills (Pillar 4)
//!
//! Both live at `~/.local/share/precc/`.
//!
//! All databases are AES-256 encrypted via SQLCipher.
//! The encryption key is derived from the machine ID and username via HKDF-SHA256,
//! cached in a process-lifetime OnceLock — never stored on disk.

use anyhow::{Context, Result};
use obfstr::obfstr;
use rusqlite::Connection;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// Default data directory: `~/.local/share/precc/`
pub fn data_dir() -> Result<PathBuf> {
    let home = std::env::var("HOME").context("HOME not set")?;
    Ok(PathBuf::from(home).join(".local/share/precc"))
}

// =============================================================================
// Key derivation — HKDF-SHA256(machine_id || username, info="precc-db-key-v1")
// =============================================================================

static MASTER_KEY: OnceLock<String> = OnceLock::new();

/// Return the AES-256 key as a 64-char hex string (32 bytes).
///
/// Derived once per process from the machine ID and username via HKDF-SHA256.
/// Never written to disk.
pub fn master_key() -> &'static str {
    MASTER_KEY.get_or_init(|| {
        let machine_id = read_machine_id();
        let username = std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown".to_string());
        let ikm = format!("{}{}", machine_id, username);
        let hk = hkdf::Hkdf::<sha2::Sha256>::new(None, ikm.as_bytes());
        let mut okm = [0u8; 32];
        hk.expand(obfstr!("precc-db-key-v1").as_bytes(), &mut okm)
            .expect("HKDF expand failed");
        hex::encode(okm)
    })
}

/// Read the platform machine ID.
fn read_machine_id() -> String {
    #[cfg(target_os = "linux")]
    {
        // Primary: /etc/machine-id (systemd)
        if let Ok(id) = std::fs::read_to_string("/etc/machine-id") {
            let trimmed = id.trim().to_string();
            if !trimmed.is_empty() {
                return trimmed;
            }
        }
        // Fallback: /var/lib/dbus/machine-id
        if let Ok(id) = std::fs::read_to_string("/var/lib/dbus/machine-id") {
            let trimmed = id.trim().to_string();
            if !trimmed.is_empty() {
                return trimmed;
            }
        }
        "unknown-linux".to_string()
    }

    #[cfg(target_os = "macos")]
    {
        // ioreg -rd1 -c IOPlatformExpertDevice | awk '/IOPlatformUUID/ {print $3}'
        let output = std::process::Command::new("ioreg")
            .args(["-rd1", "-c", "IOPlatformExpertDevice"])
            .output();
        if let Ok(out) = output {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                if line.contains("IOPlatformUUID") {
                    // Extract the UUID value: `  "IOPlatformUUID" = "XXXX-..."`
                    if let Some(start) = line.rfind('"') {
                        let after = &line[..start];
                        if let Some(prev) = after.rfind('"') {
                            let uuid = &after[prev + 1..];
                            if !uuid.is_empty() {
                                return uuid.to_string();
                            }
                        }
                    }
                }
            }
        }
        "unknown-macos".to_string()
    }

    #[cfg(target_os = "windows")]
    {
        use winreg::enums::HKEY_LOCAL_MACHINE;
        use winreg::RegKey;
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        if let Ok(key) = hklm.open_subkey("SOFTWARE\\Microsoft\\Cryptography") {
            if let Ok(guid) = key.get_value::<String, _>("MachineGuid") {
                return guid;
            }
        }
        "unknown-windows".to_string()
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        "unknown-platform".to_string()
    }
}

// =============================================================================
// Connection helpers
// =============================================================================

/// Open a SQLCipher connection with the master key and performance pragmas.
///
/// The PRAGMA key **must** be the very first SQL after open — before any other
/// statement, including schema queries. We send it in its own `execute_batch`
/// call before the WAL/cache batch.
fn open_connection(path: &Path) -> Result<Connection> {
    let conn = Connection::open(path)
        .with_context(|| format!("failed to open database: {}", path.display()))?;

    // PRAGMA key must be first — before any other SQL.
    conn.execute_batch(&format!("PRAGMA key = \"x'{}'\";\n", master_key()))?;

    conn.execute_batch(obfstr!(
        "PRAGMA journal_mode=WAL;\
         PRAGMA synchronous=NORMAL;\
         PRAGMA mmap_size=268435456;\
         PRAGMA cache_size=-8000;"
    ))?;

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

/// Open heuristics.db for read (skipping schema init).
///
/// SQLCipher requires write access to verify/derive the key even for reads,
/// so we open without `SQLITE_OPEN_READ_ONLY`. Read-only semantics are
/// enforced at the application layer — the hook never executes write SQL.
///
/// Returns `None` if the DB file doesn't exist yet.
pub fn open_heuristics_readonly(data_dir: &Path) -> Result<Option<Connection>> {
    let path = data_dir.join("heuristics.db");
    if !path.exists() {
        return Ok(None);
    }
    let conn = Connection::open(&path)
        .with_context(|| format!("failed to open database: {}", path.display()))?;
    conn.execute_batch(&format!("PRAGMA key = \"x'{}'\";\n", master_key()))?;
    Ok(Some(conn))
}

/// Open history.db for read (skipping schema init).
///
/// Returns `None` if the DB file doesn't exist yet.
pub fn open_history_readonly(data_dir: &Path) -> Result<Option<Connection>> {
    let path = data_dir.join("history.db");
    if !path.exists() {
        return Ok(None);
    }
    let conn = Connection::open(&path)
        .with_context(|| format!("failed to open database: {}", path.display()))?;
    conn.execute_batch(&format!("PRAGMA key = \"x'{}'\";\n", master_key()))?;
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
    conn.execute_batch(obfstr!(
        "CREATE TABLE IF NOT EXISTS skills (\
            id              INTEGER PRIMARY KEY,\
            name            TEXT UNIQUE NOT NULL,\
            description     TEXT NOT NULL,\
            source          TEXT NOT NULL,\
            enabled         BOOLEAN DEFAULT 1,\
            priority        INTEGER DEFAULT 100,\
            created_at      TEXT NOT NULL,\
            updated_at      TEXT NOT NULL\
        );\
        CREATE TABLE IF NOT EXISTS skill_triggers (\
            id          INTEGER PRIMARY KEY,\
            skill_id    INTEGER REFERENCES skills(id),\
            trigger_type TEXT NOT NULL,\
            pattern     TEXT NOT NULL,\
            weight      REAL DEFAULT 1.0\
        );\
        CREATE TABLE IF NOT EXISTS skill_actions (\
            id          INTEGER PRIMARY KEY,\
            skill_id    INTEGER REFERENCES skills(id),\
            action_type TEXT NOT NULL,\
            template    TEXT NOT NULL,\
            confidence  REAL DEFAULT 0.5\
        );\
        CREATE TABLE IF NOT EXISTS skill_stats (\
            id          INTEGER PRIMARY KEY,\
            skill_id    INTEGER UNIQUE REFERENCES skills(id),\
            activated   INTEGER DEFAULT 0,\
            succeeded   INTEGER DEFAULT 0,\
            failed      INTEGER DEFAULT 0,\
            last_used   TEXT\
        );\
        CREATE INDEX IF NOT EXISTS idx_triggers_type ON skill_triggers(trigger_type);\
        CREATE INDEX IF NOT EXISTS idx_triggers_pattern ON skill_triggers(pattern);\
        CREATE INDEX IF NOT EXISTS idx_skills_enabled ON skills(enabled);"
    ))?;

    // Additive migration: claude_interaction column for geofence skill filtering.
    // 0 = no/unknown (default), 1 = directly calls Claude/Anthropic API,
    // 2 = indirectly calls Claude (e.g., via a wrapper or MCP server).
    // Silently ignore if column already exists.
    let _ =
        conn.execute_batch("ALTER TABLE skills ADD COLUMN claude_interaction INTEGER DEFAULT 0;");

    Ok(())
}

fn init_history_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(obfstr!(
        "CREATE TABLE IF NOT EXISTS sessions (\
            id          INTEGER PRIMARY KEY,\
            session_id  TEXT UNIQUE NOT NULL,\
            project     TEXT,\
            started_at  TEXT NOT NULL,\
            mined_at    TEXT NOT NULL\
        );\
        CREATE TABLE IF NOT EXISTS events (\
            id          INTEGER PRIMARY KEY,\
            session_id  INTEGER REFERENCES sessions(id),\
            timestamp   TEXT NOT NULL,\
            tool        TEXT NOT NULL,\
            input_json  TEXT NOT NULL,\
            output_json TEXT,\
            exit_code   INTEGER,\
            is_failure  BOOLEAN DEFAULT 0\
        );\
        CREATE TABLE IF NOT EXISTS failure_fix_pairs (\
            id              INTEGER PRIMARY KEY,\
            failure_event   INTEGER REFERENCES events(id),\
            fix_event       INTEGER REFERENCES events(id),\
            pattern_hash    TEXT NOT NULL,\
            failure_command TEXT NOT NULL,\
            failure_output  TEXT NOT NULL,\
            fix_command     TEXT NOT NULL,\
            project_type    TEXT,\
            confidence      REAL DEFAULT 0.5,\
            occurrences     INTEGER DEFAULT 1,\
            created_at      TEXT NOT NULL,\
            updated_at      TEXT NOT NULL\
        );\
        CREATE INDEX IF NOT EXISTS idx_ffp_pattern ON failure_fix_pairs(pattern_hash);\
        CREATE INDEX IF NOT EXISTS idx_ffp_project ON failure_fix_pairs(project_type);\
        CREATE INDEX IF NOT EXISTS idx_events_session ON events(session_id);"
    ))?;

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
    conn.execute_batch(obfstr!(
        "CREATE TABLE IF NOT EXISTS metrics (\
            id          INTEGER PRIMARY KEY,\
            timestamp   TEXT NOT NULL,\
            metric_type TEXT NOT NULL,\
            value       REAL NOT NULL,\
            metadata    TEXT\
        );\
        CREATE INDEX IF NOT EXISTS idx_metrics_type ON metrics(metric_type);\
        CREATE INDEX IF NOT EXISTS idx_metrics_time ON metrics(timestamp);\
        CREATE TABLE IF NOT EXISTS savings_measurements (\
            id                     INTEGER PRIMARY KEY,\
            timestamp              TEXT NOT NULL,\
            cmd_class              TEXT NOT NULL,\
            rewrite_type           TEXT NOT NULL,\
            original_output_tokens INTEGER NOT NULL,\
            actual_output_tokens   INTEGER NOT NULL,\
            savings_tokens         INTEGER NOT NULL,\
            savings_pct            REAL NOT NULL,\
            measurement_method     TEXT NOT NULL\
        );\
        CREATE INDEX IF NOT EXISTS idx_savings_class ON savings_measurements(cmd_class);\
        CREATE INDEX IF NOT EXISTS idx_savings_type ON savings_measurements(rewrite_type);\
        CREATE TABLE IF NOT EXISTS compression_failures (\
            id        INTEGER PRIMARY KEY,\
            timestamp INTEGER NOT NULL,\
            cmd_class TEXT NOT NULL,\
            mode      TEXT,\
            signal    TEXT NOT NULL,\
            detail    TEXT\
        );\
        CREATE INDEX IF NOT EXISTS idx_cf_class_ts ON compression_failures(cmd_class, timestamp);"
    ))?;

    // Additive migrations: add new columns if they don't exist yet.
    // SQLite doesn't support ADD COLUMN IF NOT EXISTS, so we ignore errors.
    let _ = conn.execute_batch("ALTER TABLE savings_measurements ADD COLUMN compression_mode TEXT;");
    let _ = conn.execute_batch("ALTER TABLE savings_measurements ADD COLUMN probe_kind TEXT DEFAULT 'live';");
    let _ = conn.execute_batch("ALTER TABLE savings_measurements ADD COLUMN session_id TEXT;");
    let _ = conn.execute_batch("CREATE INDEX IF NOT EXISTS idx_savings_mode ON savings_measurements(compression_mode);");
    let _ = conn.execute_batch("CREATE INDEX IF NOT EXISTS idx_savings_class_mode ON savings_measurements(cmd_class, compression_mode);");

    // One-shot backfill: derive compression_mode from rewrite_type for existing rows
    let _ = conn.execute_batch(
        "UPDATE savings_measurements SET compression_mode = 'nushell'
            WHERE compression_mode IS NULL AND rewrite_type LIKE 'nushell%';
         UPDATE savings_measurements SET compression_mode = 'diet'
            WHERE compression_mode IS NULL AND rewrite_type = 'diet';
         UPDATE savings_measurements SET compression_mode = 'rtk'
            WHERE compression_mode IS NULL AND (rewrite_type LIKE 'rtk%' OR rewrite_type = 'jj-translate');
         UPDATE savings_measurements SET compression_mode = 'lean-ctx'
            WHERE compression_mode IS NULL AND rewrite_type LIKE 'lean-ctx%';
         UPDATE savings_measurements SET compression_mode = 'adaptive-expand'
            WHERE compression_mode IS NULL AND rewrite_type LIKE 'adaptive-expand%';
         UPDATE savings_measurements SET compression_mode = 'basic'
            WHERE compression_mode IS NULL;"
    );

    Ok(())
}

// =============================================================================
// Migration: re-encrypt a plain SQLite DB with SQLCipher
// =============================================================================

/// Attempt to migrate an existing unencrypted (plain SQLite) database to SQLCipher.
///
/// Detection: if `PRAGMA key` returns "not an error" but a SELECT fails with
/// "file is not a database", the file is plaintext.
///
/// Migration uses the `sqlcipher_export()` idiom:
/// 1. Open the plaintext DB without a key pragma.
/// 2. ATTACH an encrypted copy.
/// 3. `SELECT sqlcipher_export('encrypted')` — copies all tables/indexes.
/// 4. Replace the original file with the encrypted copy.
///
/// Returns `true` if migration was performed, `false` if not needed.
pub fn migrate_to_encrypted(path: &Path) -> Result<bool> {
    if !path.exists() {
        return Ok(false);
    }

    // Try opening with encryption — if it succeeds (schema query works),
    // the file is already encrypted.
    {
        let test = Connection::open(path)?;
        test.execute_batch(&format!("PRAGMA key = \"x'{}'\";\n", master_key()))?;
        if test
            .query_row("SELECT COUNT(*) FROM sqlite_master", [], |r| {
                r.get::<_, i64>(0)
            })
            .is_ok()
        {
            return Ok(false); // already encrypted, nothing to do
        }
        // fall through — likely plaintext
    }

    // Open the plaintext file without key
    let plain = Connection::open(path)
        .with_context(|| format!("failed to open plaintext db: {}", path.display()))?;

    let encrypted_path = path.with_extension("db.enc");

    // Attach an empty encrypted DB and export everything
    plain.execute_batch(&format!(
        "ATTACH DATABASE '{}' AS encrypted KEY \"x'{}'\";\
         SELECT sqlcipher_export('encrypted');\
         DETACH DATABASE encrypted;",
        encrypted_path.display(),
        master_key()
    ))?;

    // Swap files atomically
    std::fs::rename(&encrypted_path, path)
        .with_context(|| format!("failed to replace {} with encrypted copy", path.display()))?;

    Ok(true)
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

    #[test]
    fn master_key_is_64_hex_chars() {
        let key = master_key();
        assert_eq!(key.len(), 64);
        assert!(key.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn master_key_is_stable() {
        // Calling master_key() twice returns the same value
        assert_eq!(master_key(), master_key());
    }

    #[test]
    fn db_is_encrypted() {
        // Verify the DB file does NOT start with the plain SQLite magic bytes.
        // Plain SQLite header: "SQLite format 3\000" (16 bytes).
        let dir = tempfile::tempdir().unwrap();
        let _conn = open_heuristics(dir.path()).unwrap();
        let db_path = dir.path().join("heuristics.db");
        let header = std::fs::read(&db_path).unwrap();
        let plain_magic = b"SQLite format 3";
        assert!(
            !header.starts_with(plain_magic),
            "DB header must not be plain SQLite magic — encryption not applied"
        );
    }
}
