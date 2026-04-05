//! Measurement framework for tracking PRECC effectiveness.
//!
//! Records hook latency, skill activations, and pipeline decisions
//! into metrics.db for later analysis via `precc report`.

use anyhow::Result;
use rusqlite::Connection;
use serde_json;

/// Metric types recorded by the hook.
pub enum MetricType<'a> {
    HookLatency,
    SkillActivation,
    CdPrepend,
    GdbSuggestion,
    RtkRewrite,
    MinerTick,
    /// Custom metric type for tool-specific filters (read_filter, grep_filter, etc.)
    Custom(&'a str),
}

impl MetricType<'_> {
    pub fn as_str(&self) -> &str {
        match self {
            MetricType::HookLatency => "hook_latency",
            MetricType::SkillActivation => "skill_activation",
            MetricType::CdPrepend => "cd_prepend",
            MetricType::GdbSuggestion => "gdb_suggestion",
            MetricType::RtkRewrite => "rtk_rewrite",
            MetricType::MinerTick => "miner_tick",
            MetricType::Custom(s) => s,
        }
    }
}

/// Record a metric into metrics.db.
pub fn record(
    conn: &Connection,
    metric_type: MetricType,
    value: f64,
    metadata: Option<&str>,
) -> Result<()> {
    conn.execute(
        "INSERT INTO metrics (timestamp, metric_type, value, metadata)
         VALUES (datetime('now'), ?1, ?2, ?3)",
        rusqlite::params![metric_type.as_str(), value, metadata],
    )?;
    Ok(())
}

/// Record hook latency in milliseconds.
pub fn record_latency(conn: &Connection, latency_ms: f64) -> Result<()> {
    record(conn, MetricType::HookLatency, latency_ms, None)
}

/// Summary stats for a metric type.
#[derive(Debug)]
pub struct MetricSummary {
    pub count: u64,
    pub total: f64,
    pub avg: f64,
    pub min: f64,
    pub max: f64,
}

/// Get summary statistics for a metric type.
pub fn summary(conn: &Connection, metric_type: MetricType) -> Result<Option<MetricSummary>> {
    let mut stmt = conn.prepare(
        "SELECT COUNT(*), COALESCE(SUM(value), 0), COALESCE(AVG(value), 0),
                COALESCE(MIN(value), 0), COALESCE(MAX(value), 0)
         FROM metrics WHERE metric_type = ?1",
    )?;

    let result = stmt.query_row([metric_type.as_str()], |row| {
        Ok(MetricSummary {
            count: row.get(0)?,
            total: row.get(1)?,
            avg: row.get(2)?,
            min: row.get(3)?,
            max: row.get(4)?,
        })
    })?;

    if result.count == 0 {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}

/// Import pending metrics from `metrics.log` into `metrics.db`.
///
/// The hook writes to `metrics.log` (O_APPEND, no DB overhead) and the learner
/// daemon imports periodically. This function performs the same import on demand
/// so that CLI commands like `precc savings` see up-to-date data even when the
/// daemon isn't running.
///
/// Uses atomic rename to avoid double-counting with a concurrent learner.
/// Returns the number of entries imported.
pub fn import_log(conn: &Connection, data_dir: &std::path::Path) -> Result<usize> {
    let log_path = data_dir.join("metrics.log");
    if !log_path.exists() {
        return Ok(0);
    }

    // Atomically rename so the hook can write a new log concurrently
    let processing_path = data_dir.join("metrics.log.processing");
    if let Err(_e) = std::fs::rename(&log_path, &processing_path) {
        // Another process (learner) may be importing — skip silently
        return Ok(0);
    }

    let content = match std::fs::read_to_string(&processing_path) {
        Ok(c) => c,
        Err(_) => {
            let _ = std::fs::remove_file(&processing_path);
            return Ok(0);
        }
    };

    let mut count = 0;
    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let parsed: serde_json::Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let metric_type = match parsed.get("type").and_then(|v| v.as_str()) {
            Some(t) => t.to_string(),
            None => continue,
        };
        let value = parsed.get("value").and_then(|v| v.as_f64()).unwrap_or(1.0);

        let _ = conn.execute(
            "INSERT INTO metrics (timestamp, metric_type, value, metadata)
             VALUES (datetime('now'), ?1, ?2, NULL)",
            rusqlite::params![metric_type, value],
        );
        count += 1;
    }

    let _ = std::fs::remove_file(&processing_path);
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    fn test_db() -> Connection {
        let dir = tempfile::tempdir().unwrap();
        db::open_metrics(dir.path()).unwrap()
    }

    #[test]
    fn record_and_query_metric() {
        let conn = test_db();
        record(&conn, MetricType::HookLatency, 2.5, None).unwrap();
        record(&conn, MetricType::HookLatency, 3.5, None).unwrap();

        let s = summary(&conn, MetricType::HookLatency).unwrap().unwrap();
        assert_eq!(s.count, 2);
        assert!((s.avg - 3.0).abs() < 0.01);
        assert!((s.min - 2.5).abs() < 0.01);
        assert!((s.max - 3.5).abs() < 0.01);
    }

    #[test]
    fn no_metrics_returns_none() {
        let conn = test_db();
        let s = summary(&conn, MetricType::SkillActivation).unwrap();
        assert!(s.is_none());
    }

    #[test]
    fn record_with_metadata() {
        let conn = test_db();
        record(
            &conn,
            MetricType::SkillActivation,
            1.0,
            Some(r#"{"skill":"cargo-wrong-dir"}"#),
        )
        .unwrap();

        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM metrics WHERE metadata IS NOT NULL",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn custom_metric_type_as_str() {
        assert_eq!(MetricType::Custom("read_filter").as_str(), "read_filter");
        assert_eq!(MetricType::Custom("grep_filter").as_str(), "grep_filter");
        assert_eq!(
            MetricType::Custom("agent_propagate").as_str(),
            "agent_propagate"
        );
    }

    #[test]
    fn record_and_query_custom_metric() {
        let conn = test_db();
        record(&conn, MetricType::Custom("read_filter"), 1.0, None).unwrap();
        record(&conn, MetricType::Custom("read_filter"), 1.0, None).unwrap();
        record(&conn, MetricType::Custom("read_filter"), 1.0, None).unwrap();

        let s = summary(&conn, MetricType::Custom("read_filter"))
            .unwrap()
            .unwrap();
        assert_eq!(s.count, 3);
        assert!((s.total - 3.0).abs() < 0.01);
    }

    #[test]
    fn custom_metrics_isolated_from_each_other() {
        let conn = test_db();
        record(&conn, MetricType::Custom("read_filter"), 1.0, None).unwrap();
        record(&conn, MetricType::Custom("read_filter"), 1.0, None).unwrap();
        record(&conn, MetricType::Custom("grep_filter"), 1.0, None).unwrap();

        let read_s = summary(&conn, MetricType::Custom("read_filter"))
            .unwrap()
            .unwrap();
        assert_eq!(read_s.count, 2);

        let grep_s = summary(&conn, MetricType::Custom("grep_filter"))
            .unwrap()
            .unwrap();
        assert_eq!(grep_s.count, 1);

        let agent_s = summary(&conn, MetricType::Custom("agent_propagate")).unwrap();
        assert!(agent_s.is_none());
    }

    #[test]
    fn custom_metrics_isolated_from_builtin() {
        let conn = test_db();
        record(&conn, MetricType::HookLatency, 2.5, None).unwrap();
        record(&conn, MetricType::Custom("read_filter"), 1.0, None).unwrap();

        let builtin_s = summary(&conn, MetricType::HookLatency).unwrap().unwrap();
        assert_eq!(builtin_s.count, 1);

        let custom_s = summary(&conn, MetricType::Custom("read_filter"))
            .unwrap()
            .unwrap();
        assert_eq!(custom_s.count, 1);
    }

    #[test]
    fn all_builtin_metric_types_as_str() {
        assert_eq!(MetricType::HookLatency.as_str(), "hook_latency");
        assert_eq!(MetricType::SkillActivation.as_str(), "skill_activation");
        assert_eq!(MetricType::CdPrepend.as_str(), "cd_prepend");
        assert_eq!(MetricType::GdbSuggestion.as_str(), "gdb_suggestion");
        assert_eq!(MetricType::RtkRewrite.as_str(), "rtk_rewrite");
        assert_eq!(MetricType::MinerTick.as_str(), "miner_tick");
    }

    #[test]
    fn import_log_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let conn = db::open_metrics(dir.path()).unwrap();

        // Write a fake metrics.log
        let log = dir.path().join("metrics.log");
        std::fs::write(
            &log,
            "{\"ts\":1000,\"type\":\"rtk_rewrite\",\"value\":1.0}\n\
             {\"ts\":1001,\"type\":\"cd_prepend\",\"value\":1.0}\n\
             {\"ts\":1002,\"type\":\"hook_latency\",\"value\":2.5}\n",
        )
        .unwrap();

        let imported = import_log(&conn, dir.path()).unwrap();
        assert_eq!(imported, 3);

        // Verify data is in metrics.db
        let rtk = summary(&conn, MetricType::RtkRewrite).unwrap().unwrap();
        assert_eq!(rtk.count, 1);

        let cd = summary(&conn, MetricType::CdPrepend).unwrap().unwrap();
        assert_eq!(cd.count, 1);

        // Log file should be removed
        assert!(!log.exists());
    }

    #[test]
    fn import_log_no_file() {
        let dir = tempfile::tempdir().unwrap();
        let conn = db::open_metrics(dir.path()).unwrap();
        let imported = import_log(&conn, dir.path()).unwrap();
        assert_eq!(imported, 0);
    }

    #[test]
    fn import_log_skips_bad_lines() {
        let dir = tempfile::tempdir().unwrap();
        let conn = db::open_metrics(dir.path()).unwrap();

        let log = dir.path().join("metrics.log");
        std::fs::write(
            &log,
            "not json\n\
             {\"ts\":1000,\"type\":\"rtk_rewrite\",\"value\":1.0}\n\
             {\"no_type\":true}\n",
        )
        .unwrap();

        let imported = import_log(&conn, dir.path()).unwrap();
        assert_eq!(imported, 1);
    }
}
