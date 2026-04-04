# CLAUDE.md

This file provides guidance to Claude Code when working with this repository.

## Project Overview

**PRECC (Predictive Error Correction for Claude Code)** extends RTK with predictive error correction. It is a Rust workspace with 4 crates that intercepts Claude Code bash commands via a PreToolUse hook to fix errors before they happen.

## Build & Test

```bash
cargo build                          # Build all crates
cargo build --release                # Release build
cargo test                           # Run all tests
cargo clippy --all-targets           # Lint
cargo fmt --all --check              # Format check
```

## Pre-commit Gate

```bash
cargo fmt --all --check && cargo clippy --all-targets && cargo test
```

## Architecture

Four crates in a workspace:

| Crate | Type | Purpose |
|-------|------|---------|
| `precc-core` | lib | Shared logic: context resolution, skill matching, GDB, DB, mining, metrics, RTK rewriting, advisor, sharing, telemetry |
| `precc-hook` | bin | PreToolUse:Bash hook binary — must complete in < 5ms |
| `precc-cli` | bin | User-facing CLI: `precc ingest`, `precc skills`, `precc debug`, `precc report` |
| `precc-learner` | bin | Background daemon that mines session logs for failure patterns |

### Key Files

- `crates/precc-core/src/lib.rs` — Module re-exports
- `crates/precc-hook/src/main.rs` — Hook entry: reads JSON stdin, runs pipeline, writes JSON stdout
- `crates/precc-cli/src/main.rs` — CLI entry with clap
- `crates/precc-learner/src/main.rs` — Daemon entry

### Data Storage

- `~/.local/share/precc/history.db` — Mined failure-fix pairs (Pillar 3)
- `~/.local/share/precc/heuristics.db` — Automation skills (Pillar 4)
- `skills/builtin/*.toml` — Built-in skill definitions

### Hook Pipeline (precc-hook)

The hook processes each bash command through this pipeline:
1. Parse JSON from stdin
2. Query heuristics.db for matching skills (Pillar 4)
3. Resolve correct working directory (Pillar 1)
4. Check for GDB debug opportunities (Pillar 2)
5. Apply RTK command rewriting (existing behavior)
6. Emit modified command JSON to stdout

### Performance Constraints

- Hook binary must complete in < 5ms (p99)
- SQLite in WAL mode for concurrent reads
- Cached filesystem scans with 5s TTL
- Pre-compiled regex where possible

## Design Documents

- `ARCHITECTURE.md` — Full architectural design with schemas and data flows
- `ALTERNATIVES.md` — Design rationale for all major decisions

## Dependencies

Core: `rusqlite`, `serde`, `serde_json`, `regex`, `toml`, `anyhow`
CLI: `clap`
Miner: `notify`

## Conventions

- All code follows `cargo fmt` formatting
- All warnings treated as errors (`cargo clippy --all-targets`)
- Test with `#[cfg(test)] mod tests` in each module
- Error handling via `anyhow::Result`
- Skills defined as TOML (built-in) or SQLite rows (mined)
