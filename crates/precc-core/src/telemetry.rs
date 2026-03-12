//! Anonymous, consent-gated telemetry for PRECC.
//!
//! Sends aggregated per-skill ablation data to help optimize usage.
//! **No PII**: no commands, paths, project names, usernames, or machine IDs.
//!
//! Opt-in only: requires `precc telemetry consent`.
//! Override: `PRECC_NO_TELEMETRY=1` disables unconditionally.
//! Rate-limited: at most once per 24 hours.

use anyhow::{Context, Result};
use serde::Serialize;
use std::path::Path;

use crate::{consent, db, metrics};

/// Schema version for the telemetry payload (bump on breaking changes).
const SCHEMA_VERSION: u32 = 1;

/// Rate limit: minimum seconds between telemetry sends.
const RATE_LIMIT_SECS: u64 = 86400; // 24 hours

// ─── Payload types ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct TelemetryPayload {
    pub schema_version: u32,
    pub precc_version: String,
    pub os: String,
    pub arch: String,
    pub tier: String,
    pub skills: Vec<SkillTelemetry>,
    pub pillars: PillarTelemetry,
    pub hook_latency: LatencyTelemetry,
}

#[derive(Debug, Serialize)]
pub struct SkillTelemetry {
    pub name: String,
    pub source: String,
    pub activated: i64,
    pub succeeded: i64,
    pub failed: i64,
    pub est_tokens_saved: f64,
}

#[derive(Debug, Serialize)]
pub struct PillarTelemetry {
    pub rtk_rewrites: i64,
    pub rtk_tokens_saved: f64,
    pub cd_prepends: i64,
    pub cd_tokens_saved: f64,
    pub skill_activations: i64,
    pub skill_tokens_saved: f64,
    pub mined_preventions: i64,
    pub mined_tokens_saved: f64,
}

#[derive(Debug, Serialize)]
pub struct LatencyTelemetry {
    pub p50_ms: f64,
    pub p99_ms: f64,
    pub count: u64,
}

// ─── Per-skill stats query ──────────────────────────────────────────────────

/// Per-skill activation/savings stats, used by both reports and telemetry.
#[derive(Debug, Clone)]
pub struct SkillSavingsRow {
    pub skill_name: String,
    pub source: String,
    pub activated: i64,
    pub succeeded: i64,
    pub failed: i64,
    /// Estimated tokens saved by this skill.
    pub est_tokens_saved: f64,
}

/// Token savings estimate per skill activation, by action type.
const TOKENS_PER_CD_PREPEND: f64 = 300.0;
const TOKENS_PER_SKILL_ACTIVATION: f64 = 250.0;

/// Query per-skill stats from heuristics.db.
pub fn per_skill_stats(heuristics_conn: &rusqlite::Connection) -> Result<Vec<SkillSavingsRow>> {
    let mut stmt = heuristics_conn.prepare(
        "SELECT s.name, s.source, st.activated, st.succeeded, st.failed,
                COALESCE(sa.action_type, 'rewrite')
         FROM skills s
         JOIN skill_stats st ON st.skill_id = s.id
         LEFT JOIN skill_actions sa ON sa.skill_id = s.id
         WHERE st.activated > 0
         ORDER BY st.activated DESC",
    )?;

    let rows = stmt
        .query_map([], |r| {
            let name: String = r.get(0)?;
            let source: String = r.get(1)?;
            let activated: i64 = r.get(2)?;
            let succeeded: i64 = r.get(3)?;
            let failed: i64 = r.get(4)?;
            let action_type: String = r.get(5)?;
            let tokens_per = if action_type == "prepend_cd" {
                TOKENS_PER_CD_PREPEND
            } else {
                TOKENS_PER_SKILL_ACTIVATION
            };
            Ok(SkillSavingsRow {
                skill_name: name,
                source,
                activated,
                succeeded,
                failed,
                est_tokens_saved: activated as f64 * tokens_per,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    Ok(rows)
}

/// Compute p50 and p99 hook latency from metrics.db.
pub fn hook_latency_percentiles(metrics_conn: &rusqlite::Connection) -> Result<LatencyTelemetry> {
    // Count total
    let count: u64 = metrics_conn
        .query_row(
            "SELECT COUNT(*) FROM metrics WHERE metric_type = 'hook_latency'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);

    if count == 0 {
        return Ok(LatencyTelemetry {
            p50_ms: 0.0,
            p99_ms: 0.0,
            count: 0,
        });
    }

    // p50: OFFSET count/2
    let p50_offset = count / 2;
    let p50: f64 = metrics_conn
        .query_row(
            "SELECT value FROM metrics WHERE metric_type = 'hook_latency'
             ORDER BY value ASC LIMIT 1 OFFSET ?1",
            [p50_offset],
            |r| r.get(0),
        )
        .unwrap_or(0.0);

    // p99: OFFSET count*99/100
    let p99_offset = count * 99 / 100;
    let p99: f64 = metrics_conn
        .query_row(
            "SELECT value FROM metrics WHERE metric_type = 'hook_latency'
             ORDER BY value ASC LIMIT 1 OFFSET ?1",
            [p99_offset],
            |r| r.get(0),
        )
        .unwrap_or(0.0);

    Ok(LatencyTelemetry {
        p50_ms: p50,
        p99_ms: p99,
        count,
    })
}

// ─── Payload builder ────────────────────────────────────────────────────────

/// Build the telemetry payload from local databases.  Contains NO PII.
pub fn build_payload(data_dir: &Path) -> Result<TelemetryPayload> {
    let version = env!("CARGO_PKG_VERSION").to_string();
    let os = std::env::consts::OS.to_string();
    let arch = std::env::consts::ARCH.to_string();
    let tier = format!("{:?}", crate::license::tier());

    // Per-skill stats
    let skills_data = if let Ok(conn) = db::open_heuristics(data_dir) {
        per_skill_stats(&conn).unwrap_or_default()
    } else {
        vec![]
    };

    let skills: Vec<SkillTelemetry> = skills_data
        .iter()
        .map(|s| SkillTelemetry {
            name: s.skill_name.clone(),
            source: s.source.clone(),
            activated: s.activated,
            succeeded: s.succeeded,
            failed: s.failed,
            est_tokens_saved: s.est_tokens_saved,
        })
        .collect();

    // Pillar-level aggregates
    let (rtk_rewrites, cd_prepends, skill_activations_total) =
        if let Ok(conn) = db::open_metrics(data_dir) {
            let rtk = metrics::summary(&conn, metrics::MetricType::RtkRewrite)?
                .map(|s| s.count as i64)
                .unwrap_or(0);
            let cd = metrics::summary(&conn, metrics::MetricType::CdPrepend)?
                .map(|s| s.count as i64)
                .unwrap_or(0);
            let sa = metrics::summary(&conn, metrics::MetricType::SkillActivation)?
                .map(|s| s.count as i64)
                .unwrap_or(0);
            (rtk, cd, sa)
        } else {
            (0, 0, 0)
        };

    let mined_preventions: i64 = if let Ok(conn) = db::open_history(data_dir) {
        conn.query_row(
            "SELECT COALESCE(SUM(occurrences - 1), 0) FROM failure_fix_pairs WHERE occurrences > 1",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0)
    } else {
        0
    };

    // Use conservative token model (same as cmd_savings)
    let rtk_avg = 175.0; // weighted avg across rule categories
    let pillars = PillarTelemetry {
        rtk_rewrites,
        rtk_tokens_saved: rtk_rewrites as f64 * rtk_avg,
        cd_prepends,
        cd_tokens_saved: cd_prepends as f64 * TOKENS_PER_CD_PREPEND,
        skill_activations: skill_activations_total,
        skill_tokens_saved: skill_activations_total as f64 * TOKENS_PER_SKILL_ACTIVATION,
        mined_preventions,
        mined_tokens_saved: mined_preventions as f64 * 200.0,
    };

    // Hook latency
    let hook_latency = if let Ok(conn) = db::open_metrics(data_dir) {
        hook_latency_percentiles(&conn)?
    } else {
        LatencyTelemetry {
            p50_ms: 0.0,
            p99_ms: 0.0,
            count: 0,
        }
    };

    Ok(TelemetryPayload {
        schema_version: SCHEMA_VERSION,
        precc_version: version,
        os,
        arch,
        tier,
        skills,
        pillars,
        hook_latency,
    })
}

// ─── Sender ─────────────────────────────────────────────────────────────────

/// Send the payload via a background curl POST.  Fire-and-forget.
fn send(payload: &TelemetryPayload) -> Result<()> {
    let json = serde_json::to_string(payload).context("serializing telemetry")?;

    const TELEMETRY_URL: &str = match option_env!("PRECC_TELEMETRY_URL") {
        Some(u) => u,
        None => "https://precc.goatcounter.com/api/telemetry/v1",
    };

    let _ = std::process::Command::new("curl")
        .args([
            "-fsSL",
            "--max-time",
            "10",
            "-X",
            "POST",
            "-H",
            "Content-Type: application/json",
            "-d",
            &json,
            "--silent",
            "--output",
            "/dev/null",
            TELEMETRY_URL,
        ])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn(); // fire and forget

    Ok(())
}

/// Rate-limit file path.
fn last_sent_path(data_dir: &Path) -> std::path::PathBuf {
    data_dir.join(".telemetry_last_sent")
}

/// Check + send if consent is given and rate limit has elapsed.
pub fn maybe_send(data_dir: &Path) -> Result<()> {
    if !consent::is_telemetry_enabled() {
        return Ok(());
    }

    // Rate limit: check last-sent timestamp
    let marker = last_sent_path(data_dir);
    if marker.exists() {
        if let Ok(meta) = std::fs::metadata(&marker) {
            if let Ok(modified) = meta.modified() {
                let elapsed = std::time::SystemTime::now()
                    .duration_since(modified)
                    .unwrap_or_default()
                    .as_secs();
                if elapsed < RATE_LIMIT_SECS {
                    return Ok(()); // too soon
                }
            }
        }
    }

    let payload = build_payload(data_dir)?;
    send(&payload)?;

    // Touch the marker file
    let _ = std::fs::write(&marker, "");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn build_payload_empty_dbs() {
        let dir = tempfile::tempdir().unwrap();
        let _ = db::open_heuristics(dir.path()).unwrap();
        let _ = db::open_history(dir.path()).unwrap();
        let _ = db::open_metrics(dir.path()).unwrap();

        let payload = build_payload(dir.path()).unwrap();
        assert_eq!(payload.schema_version, SCHEMA_VERSION);
        assert!(payload.skills.is_empty());
        assert_eq!(payload.pillars.rtk_rewrites, 0);
        assert_eq!(payload.hook_latency.count, 0);
    }

    #[test]
    fn payload_serializes_without_pii() {
        let dir = tempfile::tempdir().unwrap();
        let _ = db::open_heuristics(dir.path()).unwrap();
        let _ = db::open_history(dir.path()).unwrap();
        let _ = db::open_metrics(dir.path()).unwrap();

        let payload = build_payload(dir.path()).unwrap();
        let json = serde_json::to_string_pretty(&payload).unwrap();

        // Must not contain any path separators or usernames
        assert!(!json.contains("/home/"));
        assert!(!json.contains("/Users/"));
        assert!(!json.contains("\\Users\\"));
    }
}
