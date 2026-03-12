//! PRECC Core — shared library for Predictive Error Correction for Claude Code.

pub mod advisor;
pub mod consent;
pub mod context;
pub mod db;
pub mod gdb;
pub mod license;
pub mod metrics;
pub mod mining;
pub mod promote;
pub mod rtk;
pub mod sharing;
pub mod skills;
pub mod telemetry;

/// Free-tier cap: maximum number of mined (non-builtin) skills that will be
/// applied per hook invocation. Builtin skills are always applied regardless.
pub const FREE_SKILL_LIMIT: usize = 3;
