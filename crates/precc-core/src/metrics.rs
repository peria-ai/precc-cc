//! Measurement framework for tracking PRECC effectiveness.
//!
//! Records hook latency, skill activations, and pipeline decisions
//! into metrics.db for later analysis via `precc report`.

use anyhow::Result;
use rusqlite::Connection;

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
}
