# PRECC Architecture

**P**redictive **E**rror **C**orrection for **C**laude Code

> Extends RTK (Rust Token Killer) with predictive error correction — fixing commands *before* execution, offering GDB as an alternative to debug-print cycles, and learning failure patterns to build reusable automation skills.

## Overview

RTK saves 60-90% of tokens by filtering/compressing CLI output via a `PreToolUse:Bash` hook. However, it only optimizes **output** — it cannot prevent commands from failing in the first place. PRECC addresses three categories of waste that RTK cannot touch:

1. **Wrong-directory failures** — Claude runs `cargo build` in the wrong directory, gets an error, reads the error, tries again with `cd`. Cost: 3-4 wasted tool calls.
2. **Edit-compile-read-edit debug cycles** — Claude adds print statements, recompiles, reads output, edits again. Cost: 4+ tool calls per cycle, often repeated.
3. **Repeated failure patterns** — The same class of error (missing import, wrong flag, etc.) recurs across sessions with no learning.

## Four Pillars

### Pillar 1: Context-Aware Bash

**Problem**: Claude frequently runs build/test commands in the wrong directory.

**Solution**: Before executing any bash command, detect the correct working directory by scanning the filesystem for project markers (`Cargo.toml`, `package.json`, `Makefile`, `go.mod`, etc.) and prepend `cd /correct/path &&` when needed.

**Algorithm**:
```
1. Parse the command to extract the tool being invoked (cargo, npm, make, etc.)
2. Determine which project marker file that tool expects
3. Search upward from CWD, then search known project roots
4. If CWD lacks the marker but a single unambiguous match exists, prepend cd
5. If multiple matches exist, prefer the most recently modified
6. If no match, pass through unchanged (don't break unknown commands)
```

**Confidence scoring**:
- `1.0` — CWD already contains the marker (no rewrite needed)
- `0.9` — Single unambiguous match found
- `0.7` — Multiple matches, most-recent heuristic applied
- `0.0` — No match found, pass through

**Latency budget**: < 1ms (cached filesystem scan)

### Pillar 2: GDB-Based Debugging

**Problem**: When Claude encounters a runtime bug, it enters an edit-compile-read-edit cycle:
1. Edit source to add `println!` / `console.log` (Edit tool call)
2. Recompile (Bash tool call)
3. Run and read output (Bash tool call)
4. Edit source to remove debug prints and fix bug (Edit tool call)

This costs 4+ tool calls per debug cycle, often repeated multiple times.

**Solution**: Generate `.gdbinit` scripts dynamically via `precc debug <binary> [args]` CLI command. GDB can set breakpoints, print variables, and inspect state in a single command — replacing the entire edit-compile-read-edit cycle.

**Implementation**:
```
precc debug target/debug/myapp --args foo bar
  1. Scan recent compiler errors / test failures from history.db
  2. Identify likely crash points or assertion failures
  3. Generate .gdbinit with:
     - Breakpoints at suspected failure locations
     - Variable watches for relevant state
     - Conditional breakpoints for loop-related bugs
  4. Launch: gdb -batch -x .gdbinit target/debug/myapp foo bar
  5. Parse GDB output into structured report
```

**Supported debuggers**: GDB (Rust, C, C++), LLDB (macOS fallback), Node.js `--inspect` (JavaScript/TypeScript)

**Token savings**: 4 tool calls (edit + compile + run + edit) reduced to 1 tool call (`precc debug`).

### Pillar 3: Failure Pattern Learning

**Problem**: Claude Code sessions generate rich data about failures and their fixes, but this data is discarded after each session.

**Solution**: Mine Claude Code's JSONL session logs (`~/.claude/projects/*/`) to extract failure-fix pairs and store them in `history.db` (SQLite).

**Data model** (history.db):
```sql
CREATE TABLE sessions (
    id          INTEGER PRIMARY KEY,
    session_id  TEXT UNIQUE NOT NULL,
    project     TEXT,
    started_at  TEXT NOT NULL,
    mined_at    TEXT NOT NULL
);

CREATE TABLE events (
    id          INTEGER PRIMARY KEY,
    session_id  INTEGER REFERENCES sessions(id),
    timestamp   TEXT NOT NULL,
    tool        TEXT NOT NULL,       -- 'Bash', 'Edit', 'Read', etc.
    input_json  TEXT NOT NULL,       -- tool_input as JSON
    output_json TEXT,                -- tool_result as JSON (nullable for pending)
    exit_code   INTEGER,            -- for Bash events
    is_failure  BOOLEAN DEFAULT 0
);

CREATE TABLE failure_fix_pairs (
    id              INTEGER PRIMARY KEY,
    failure_event   INTEGER REFERENCES events(id),
    fix_event       INTEGER REFERENCES events(id),
    pattern_hash    TEXT NOT NULL,       -- normalized hash of the failure
    failure_command TEXT NOT NULL,
    failure_output  TEXT NOT NULL,
    fix_command     TEXT NOT NULL,
    project_type    TEXT,               -- 'rust', 'node', 'python', etc.
    confidence      REAL DEFAULT 0.5,
    occurrences     INTEGER DEFAULT 1,
    created_at      TEXT NOT NULL,
    updated_at      TEXT NOT NULL
);

CREATE INDEX idx_ffp_pattern ON failure_fix_pairs(pattern_hash);
CREATE INDEX idx_ffp_project ON failure_fix_pairs(project_type);
CREATE INDEX idx_events_session ON events(session_id);
```

**Mining algorithm**:
```
For each session JSONL file:
  1. Parse events chronologically
  2. Identify Bash events with non-zero exit codes (failures)
  3. Look ahead for the next Bash/Edit event that "fixes" the failure:
     - Same tool + similar command that succeeds, OR
     - Edit to a file mentioned in the error output
  4. Normalize the failure pattern (strip paths, generalize versions)
  5. Compute pattern_hash for deduplication
  6. Upsert into failure_fix_pairs (increment occurrences if exists)
```

**Mining modes**:
- **On-demand**: `precc ingest [session-file]` — mine a specific session
- **Background daemon**: `precc-miner` — watch for new session files and mine continuously
- **Batch**: `precc ingest --all` — mine all unmined sessions

### Pillar 4: Heuristics DB (Skills)

**Problem**: Even with pattern learning, the hook needs a fast way to look up applicable corrections at command time.

**Solution**: Map failure patterns to automation **skills** stored in `heuristics.db` (SQLite). The hook consults this DB before every bash execution.

**Data model** (heuristics.db):
```sql
CREATE TABLE skills (
    id              INTEGER PRIMARY KEY,
    name            TEXT UNIQUE NOT NULL,
    description     TEXT NOT NULL,
    source          TEXT NOT NULL,       -- 'builtin', 'mined', 'user'
    enabled         BOOLEAN DEFAULT 1,
    priority        INTEGER DEFAULT 100, -- lower = higher priority
    created_at      TEXT NOT NULL,
    updated_at      TEXT NOT NULL
);

CREATE TABLE skill_triggers (
    id          INTEGER PRIMARY KEY,
    skill_id    INTEGER REFERENCES skills(id),
    trigger_type TEXT NOT NULL,  -- 'command_regex', 'error_regex', 'project_type', 'file_exists'
    pattern     TEXT NOT NULL,
    weight      REAL DEFAULT 1.0
);

CREATE TABLE skill_actions (
    id          INTEGER PRIMARY KEY,
    skill_id    INTEGER REFERENCES skills(id),
    action_type TEXT NOT NULL,   -- 'prepend_cd', 'rewrite_command', 'add_flag', 'suggest_alternative'
    template    TEXT NOT NULL,   -- action template with {{placeholders}}
    confidence  REAL DEFAULT 0.5
);

CREATE TABLE skill_stats (
    id          INTEGER PRIMARY KEY,
    skill_id    INTEGER REFERENCES skills(id),
    activated   INTEGER DEFAULT 0,
    succeeded   INTEGER DEFAULT 0,
    failed      INTEGER DEFAULT 0,
    last_used   TEXT
);

CREATE INDEX idx_triggers_type ON skill_triggers(trigger_type);
CREATE INDEX idx_triggers_pattern ON skill_triggers(pattern);
CREATE INDEX idx_skills_enabled ON skills(enabled);
```

**Skill lifecycle** (implemented in `promote.rs::tick_skill_lifecycle`):
```
1. DISCOVERY:  Miner finds a recurring failure-fix pair (≥3 occurrences)
2. CANDIDATE:  promote_patterns() creates skill with confidence = 0.3
               (not auto-applied by hook; shown as suggestion only)
3. ACTIVE:     tick_skill_lifecycle() promotes to 0.7 after 5 activations
               (hook auto-applies silently at conf ≥ 0.7)
4. TRUSTED:    Promoted to 0.9 after 20 activations with <5% failure rate
5. DISABLED:   Auto-disabled if failure rate exceeds 20% with ≥5 activations
```

**Confidence thresholds**:
- `≥ 0.7` — Auto-apply silently (hook rewrites command)
- `0.3–0.69` — Candidate; not auto-applied
- `< 0.3` — Not surfaced

**Activation tracking** (`skill_stats` table):
- Hook writes `activations.log` (O_APPEND, single syscall, ~10µs) on each skill fire
- Miner atomically renames log → `activations.log.processing`, imports into `skill_stats`
- `record_activation()` uses `INSERT OR IGNORE` + `UPDATE` (compatible with all SQLCipher versions)

**Metrics bridge** (`metrics.log` → `metrics.db`):
- Hook appends one JSONL line per invocation: `{"ts":…,"type":"hook_latency","value":2.93}`
- Optional lines for `cd_prepend` and `rtk_rewrite` events
- Miner imports atomically on each tick; `precc report` reads from `metrics.db`

## Component Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Claude Code                              │
│                                                              │
│  PreToolUse:Bash ──→ precc-hook (binary, <5ms)              │
│                        │                                     │
│                        ├─ Query heuristics.db (Pillar 4)    │
│                        ├─ Resolve working dir (Pillar 1)    │
│                        ├─ Check GDB opportunity (Pillar 2)  │
│                        ├─ RTK rewriting (existing)          │
│                        ├─ Emit modified JSON to stdout      │
│                        ├─ Append activations.log (O_APPEND) │
│                        └─ Append metrics.log    (O_APPEND)  │
│                                                              │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  precc-cli (user-facing commands)                           │
│    ├─ precc ingest [file|--all]     Mine sessions           │
│    ├─ precc skills [list|show|edit] Manage skills           │
│    ├─ precc debug <binary> [args]   GDB debug helper        │
│    ├─ precc report                  Analytics dashboard     │
│    └─ precc init                    Setup hook + DBs        │
│                                                              │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  precc-miner (background daemon)                            │
│    ├─ Watch ~/.claude/projects/ for new JSONL               │
│    ├─ Mine failure-fix pairs → history.db                   │
│    ├─ Import activations.log → skill_stats (atomic rename)  │
│    ├─ Import metrics.log → metrics.db (atomic rename)       │
│    ├─ Promote patterns → skills in heuristics.db            │
│    └─ Run skill lifecycle: CANDIDATE→ACTIVE→TRUSTED/DISABLED│
│                                                              │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  precc-core (shared library)                                │
│    ├─ context.rs   — Working directory resolution           │
│    ├─ skills.rs    — Skill matching engine                  │
│    ├─ gdb.rs       — GDB script generation                  │
│    ├─ db.rs        — SQLite connection management           │
│    ├─ mining.rs    — JSONL parsing + pattern extraction     │
│    ├─ metrics.rs   — Token/call counting + reporting        │
│    └─ rtk.rs       — RTK rewriting logic (ported from bash) │
│                                                              │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  Storage                                                     │
│    ├─ ~/.local/share/precc/history.db     (Pillar 3)        │
│    ├─ ~/.local/share/precc/heuristics.db  (Pillar 4)        │
│    ├─ ~/.local/share/precc/metrics.db     (hook metrics)    │
│    ├─ ~/.local/share/precc/activations.log (O_APPEND bridge)│
│    ├─ ~/.local/share/precc/metrics.log    (O_APPEND bridge) │
│    └─ skills/builtin/*.toml               (built-in skills) │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Hook Pipeline (precc-hook)

The hook binary is the performance-critical path. It must complete in < 5ms to avoid perceptible latency in Claude Code.

```
Input: JSON on stdin (Claude Code PreToolUse:Bash event)
  │
  ├─ 1. Parse JSON, extract command string
  │
  ├─ 2. SKILL MATCHING (Pillar 4)
  │     Query heuristics.db for skills matching:
  │       - command regex triggers
  │       - project type triggers
  │       - file existence triggers
  │     If skill.confidence ≥ 0.7: apply action
  │     If skill.confidence 0.3-0.7: add suggestion to output
  │
  ├─ 3. CONTEXT RESOLUTION (Pillar 1)
  │     Determine correct working directory
  │     If CWD is wrong: prepend `cd /correct/path &&`
  │
  ├─ 4. GDB CHECK (Pillar 2)
  │     If command matches debug pattern (cargo test, ./binary):
  │       Check if recent failures suggest GDB would help
  │       If so: suggest `precc debug` alternative
  │
  ├─ 5. RTK REWRITING (existing functionality)
  │     Apply rtk command rewrites (git → rtk git, etc.)
  │
  └─ 6. EMIT RESULT
        Output JSON with:
          - updatedInput (modified command)
          - permissionDecision: "allow"
          - permissionDecisionReason: human-readable explanation

Output: JSON on stdout
```

**Performance strategy**:
- Open SQLite in WAL mode with `PRAGMA journal_mode=WAL`
- Keep DB connection in a static/lazy cell (process lifetime)
- Pre-compile regex patterns at build time where possible
- Cache filesystem scans for project markers (TTL: 5s)
- Use `PRAGMA mmap_size` for memory-mapped reads

## Skill System

### Skill Definition Format (TOML)

```toml
[skill]
name = "cargo-wrong-dir"
description = "Detect and fix cargo commands run outside a Rust project"
source = "builtin"
priority = 50

[[triggers]]
type = "command_regex"
pattern = "^cargo\\s+(build|test|clippy|check|run)"
weight = 1.0

[[triggers]]
type = "file_exists"
pattern = "!Cargo.toml"  # '!' means file must NOT exist in CWD
weight = 0.8

[[actions]]
type = "prepend_cd"
template = "cd {{project_root}} && {{original_command}}"
confidence = 0.9
```

### Built-in Skills (shipped with PRECC)

| Skill | Triggers | Action |
|-------|----------|--------|
| `cargo-wrong-dir` | `cargo *` + no Cargo.toml in CWD | Prepend `cd` to nearest Cargo.toml |
| `npm-wrong-dir` | `npm *` / `pnpm *` + no package.json in CWD | Prepend `cd` to nearest package.json |
| `make-wrong-dir` | `make *` + no Makefile in CWD | Prepend `cd` to nearest Makefile |
| `cargo-missing-feature` | `cargo build` + "feature X not found" in recent error | Add `--features X` |
| `git-not-repo` | `git *` + "not a git repository" in recent error | Prepend `cd` to nearest .git |

### Mined Skills (auto-generated from patterns)

The miner generates candidate skills when it detects a failure-fix pair occurring ≥ 3 times across sessions. Generated skills start at confidence 0.3 and are promoted through the lifecycle stages.

## Measurement Framework

PRECC tracks its own effectiveness to validate the four pillars:

### Metrics Collected

```sql
CREATE TABLE metrics (
    id          INTEGER PRIMARY KEY,
    timestamp   TEXT NOT NULL,
    metric_type TEXT NOT NULL,   -- 'hook_latency', 'skill_activation', 'cd_prepend', 'gdb_suggestion', 'rtk_rewrite'
    value       REAL NOT NULL,
    metadata    TEXT             -- JSON blob for context
);

CREATE INDEX idx_metrics_type ON metrics(metric_type);
CREATE INDEX idx_metrics_time ON metrics(timestamp);
```

### Success Criteria

| Metric | Target | Measurement |
|--------|--------|-------------|
| Token savings vs RTK-only | 30-50% additional | Compare session token counts before/after PRECC |
| Bash calls per session | 15-25% fewer | Count tool calls in mined sessions |
| Failed bash calls | 40-60% fewer | Track failure rate in hook metrics |
| Hook latency | < 5ms p99 | Measure in hook binary |
| Skills accumulated | Growing library | Count active skills per project type |

### Reporting

`precc report` generates a dashboard showing:
- Token savings breakdown by pillar
- Skill activation counts and success rates
- Top failure patterns (not yet covered by skills)
- Hook latency percentiles
- Comparison: sessions with PRECC vs without

## Data Flow Diagrams

### Session Mining Flow

```
~/.claude/projects/*/session.jsonl
         │
         ▼
    precc-miner (or precc ingest)
         │
         ├─ Parse JSONL events
         ├─ Identify failures (exit_code != 0)
         ├─ Find fix events (lookahead)
         ├─ Normalize patterns
         │
         ▼
    history.db (failure_fix_pairs)
         │
         ├─ Pattern occurs ≥ 3 times?
         │     YES → Generate candidate skill
         │     NO  → Wait for more data
         │
         ▼
    heuristics.db (skills)
```

### Hook Decision Flow

```
Incoming command: "cargo build"
         │
         ▼
    ┌─ Skills DB lookup ─┐
    │ Match: cargo-wrong-dir │
    │ Confidence: 0.9        │
    │ Action: prepend_cd     │
    └────────────────────────┘
         │
         ▼
    ┌─ Context check ────┐
    │ CWD: /home/user     │
    │ Cargo.toml found at:│
    │   /home/user/myapp  │
    └─────────────────────┘
         │
         ▼
    ┌─ RTK rewrite ──────┐
    │ cargo build →       │
    │ rtk cargo build     │
    └─────────────────────┘
         │
         ▼
    Output: "cd /home/user/myapp && rtk cargo build"
```

## Crate Dependency Graph

```
precc-core (library)
    ├── rusqlite (SQLite)
    ├── serde + serde_json (serialization)
    ├── regex (pattern matching)
    ├── toml (skill definitions)
    └── anyhow (error handling)

precc-hook (binary, performance-critical)
    └── precc-core

precc-cli (binary, user-facing)
    ├── precc-core
    └── clap (CLI parsing)

precc-miner (binary, background daemon)
    ├── precc-core
    └── notify (filesystem watching)
```

## File Layout

```
rtk/
├── ARCHITECTURE.md          # This document
├── ALTERNATIVES.md          # Design rationale
├── README.md                # Project overview
├── CLAUDE.md                # Claude Code instructions
├── Cargo.toml               # Workspace manifest
├── crates/
│   ├── precc-core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs       # Module re-exports
│   │       ├── context.rs   # Pillar 1: working directory
│   │       ├── skills.rs    # Pillar 4: skill matching
│   │       ├── gdb.rs       # Pillar 2: GDB generation
│   │       ├── mining.rs    # Pillar 3: JSONL mining
│   │       ├── db.rs        # SQLite management
│   │       ├── metrics.rs   # Measurement framework
│   │       └── rtk.rs       # RTK rewriting (ported)
│   ├── precc-hook/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs      # Hook binary entry point
│   ├── precc-cli/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs      # CLI entry point
│   └── precc-miner/
│       ├── Cargo.toml
│       └── src/
│           └── main.rs      # Miner daemon entry point
└── skills/
    └── builtin/
        └── cargo-wrong-dir.toml  # Example builtin skill
```
