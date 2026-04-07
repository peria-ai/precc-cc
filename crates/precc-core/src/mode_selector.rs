//! Adaptive compression mode selector.
//!
//! Queries `savings_measurements` for historical per-mode performance on a
//! command class and returns the mode with the highest average savings_pct.
//! Filters out modes with recent failure events. Caches results for 60s
//! to avoid hitting SQLite on every PreToolUse invocation.

use crate::db;
use crate::mode::CompressionMode;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

/// Minimum measurements required before we trust the data.
const MIN_SAMPLES: i64 = 3;
/// Failure events in last N seconds that disqualify a mode.
const FAILURE_WINDOW_SECS: i64 = 86400;
/// Failure threshold (≥N events disqualifies the mode).
const FAILURE_THRESHOLD: i64 = 2;
/// Cache TTL.
const CACHE_TTL_SECS: u64 = 60;

#[derive(Clone)]
struct CacheEntry {
    mode: Option<CompressionMode>,
    expires_at: Instant,
}

static CACHE: LazyLock<Mutex<HashMap<String, CacheEntry>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Pick the best compression mode for the given command class based on
/// historical measurements. Returns None if no data exists or all modes
/// have been disqualified by recent failures.
pub fn select_best_mode(data_dir: &Path, cmd_class: &str) -> Option<CompressionMode> {
    if cmd_class.is_empty() {
        return None;
    }

    // Cache check
    {
        let cache = CACHE.lock().ok()?;
        if let Some(entry) = cache.get(cmd_class) {
            if entry.expires_at > Instant::now() {
                return entry.mode;
            }
        }
    }

    let result = compute_best_mode(data_dir, cmd_class);

    // Cache write
    if let Ok(mut cache) = CACHE.lock() {
        cache.insert(
            cmd_class.to_string(),
            CacheEntry {
                mode: result,
                expires_at: Instant::now() + Duration::from_secs(CACHE_TTL_SECS),
            },
        );
    }

    result
}

/// Clear the in-memory cache (used by tests).
pub fn clear_cache() {
    if let Ok(mut cache) = CACHE.lock() {
        cache.clear();
    }
}

fn compute_best_mode(data_dir: &Path, cmd_class: &str) -> Option<CompressionMode> {
    let conn = db::open_metrics(data_dir).ok()?;

    // Find disqualified modes (≥THRESHOLD failures in last WINDOW_SECS)
    let now_ts: i64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs() as i64;
    let cutoff = now_ts - FAILURE_WINDOW_SECS;

    let mut disqualified: Vec<String> = Vec::new();
    if let Ok(mut stmt) = conn.prepare(
        "SELECT mode, COUNT(*) FROM compression_failures
         WHERE cmd_class = ?1 AND timestamp >= ?2 AND mode IS NOT NULL
         GROUP BY mode
         HAVING COUNT(*) >= ?3",
    ) {
        let rows = stmt.query_map(rusqlite::params![cmd_class, cutoff, FAILURE_THRESHOLD], |r| {
            r.get::<_, String>(0)
        });
        if let Ok(iter) = rows {
            for r in iter.flatten() {
                disqualified.push(r);
            }
        }
    }

    // Query best mode by avg savings_pct
    let mut stmt = conn
        .prepare(
            "SELECT compression_mode, AVG(savings_pct) AS avg_pct, COUNT(*) AS n
             FROM savings_measurements
             WHERE cmd_class = ?1
               AND compression_mode IS NOT NULL
               AND compression_mode NOT IN ('adaptive-expand', 'basic')
             GROUP BY compression_mode
             HAVING n >= ?2
             ORDER BY avg_pct DESC",
        )
        .ok()?;

    let rows = stmt
        .query_map(rusqlite::params![cmd_class, MIN_SAMPLES], |r| {
            let mode_str: String = r.get(0)?;
            let avg_pct: f64 = r.get(1)?;
            let count: i64 = r.get(2)?;
            Ok((mode_str, avg_pct, count))
        })
        .ok()?;

    for row in rows.flatten() {
        let (mode_str, _avg_pct, _count) = row;
        if disqualified.contains(&mode_str) {
            continue;
        }
        if let Some(mode) = CompressionMode::from_str(&mode_str) {
            return Some(mode);
        }
    }

    None
}

/// Record a compression failure for a command class.
pub fn record_failure(data_dir: &Path, cmd_class: &str, mode: &str, signal: &str, detail: &str) {
    if cmd_class.is_empty() || signal.is_empty() {
        return;
    }
    let conn = match db::open_metrics(data_dir) {
        Ok(c) => c,
        Err(_) => return,
    };
    let ts: i64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let _ = conn.execute(
        "INSERT INTO compression_failures (timestamp, cmd_class, mode, signal, detail)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![ts, cmd_class, mode, signal, detail],
    );
    // Bust cache for this class so next selection sees the failure
    if let Ok(mut cache) = CACHE.lock() {
        cache.remove(cmd_class);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed_measurement(
        conn: &rusqlite::Connection,
        cmd_class: &str,
        mode: &str,
        savings_pct: f64,
    ) {
        conn.execute(
            "INSERT INTO savings_measurements (timestamp, cmd_class, rewrite_type, compression_mode, probe_kind, original_output_tokens, actual_output_tokens, savings_tokens, savings_pct, measurement_method)
             VALUES (datetime('now'), ?1, ?2, ?3, 'live', 1000, 100, 900, ?4, 'ground_truth')",
            rusqlite::params![cmd_class, mode, mode, savings_pct],
        ).unwrap();
    }

    #[test]
    fn no_data_returns_none() {
        let tmp = tempfile::tempdir().unwrap();
        clear_cache();
        let result = select_best_mode(tmp.path(), "cargo test");
        assert_eq!(result, None);
    }

    #[test]
    fn picks_highest_avg_savings_with_quorum() {
        let tmp = tempfile::tempdir().unwrap();
        let conn = db::open_metrics(tmp.path()).unwrap();
        clear_cache();

        // Seed: nushell wins on cargo test (3 samples each, nushell higher)
        for _ in 0..3 {
            seed_measurement(&conn, "cargo test", "rtk", 65.0);
            seed_measurement(&conn, "cargo test", "nushell", 80.0);
        }

        let result = select_best_mode(tmp.path(), "cargo test");
        assert_eq!(result, Some(CompressionMode::Nushell));
    }

    #[test]
    fn rejects_below_quorum() {
        let tmp = tempfile::tempdir().unwrap();
        let conn = db::open_metrics(tmp.path()).unwrap();
        clear_cache();

        // Only 2 samples — below MIN_SAMPLES
        seed_measurement(&conn, "git status", "rtk", 80.0);
        seed_measurement(&conn, "git status", "rtk", 80.0);

        let result = select_best_mode(tmp.path(), "git status");
        assert_eq!(result, None);
    }

    #[test]
    fn failure_disqualifies_mode() {
        let tmp = tempfile::tempdir().unwrap();
        let conn = db::open_metrics(tmp.path()).unwrap();
        clear_cache();

        // RTK has best savings on cargo test, but has failure events
        for _ in 0..3 {
            seed_measurement(&conn, "cargo test", "rtk", 90.0);
            seed_measurement(&conn, "cargo test", "nushell", 70.0);
        }

        // Record 2 failures for rtk
        record_failure(tmp.path(), "cargo test", "rtk", "follow_up_diagnostic", "");
        record_failure(tmp.path(), "cargo test", "rtk", "output_too_small", "");

        // Now nushell should win even though rtk has higher savings
        let result = select_best_mode(tmp.path(), "cargo test");
        assert_eq!(result, Some(CompressionMode::Nushell));
    }
}
