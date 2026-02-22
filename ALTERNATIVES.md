# PRECC Design Alternatives

This document records the key design decisions made for PRECC, the alternatives considered, and the rationale for each choice.

## 1. Compiled Rust Binary vs Shell Script

**Decision**: Replace the `rtk-rewrite.sh` shell script hook with a compiled Rust binary (`precc-hook`).

**Alternatives considered**:

| Option | Latency | SQLite | Regex | Maintainability |
|--------|---------|--------|-------|-----------------|
| Shell script (status quo) | 20-40ms | No (needs sqlite3 CLI) | grep only | Fragile, hard to test |
| Python script | 50-100ms | Yes (sqlite3 module) | Yes | Good, but startup cost |
| Compiled Rust binary | < 5ms | Yes (rusqlite) | Yes (regex crate) | Excellent, type-safe |

**Rationale**: The hook runs on every single Bash tool call. At 20-40ms the shell script is already noticeable; adding SQLite queries and regex matching would push it to 50-100ms. A compiled binary amortizes startup to near-zero and provides direct SQLite access without shelling out. Rust's type system also prevents the class of bugs that plague shell scripts (unquoted variables, missing error handling, etc.).

**Trade-off**: Requires a compile step before installation. Mitigated by providing `cargo install --path .` and pre-built binaries.

## 2. SQLite vs Key-Value Store

**Decision**: Use SQLite for both `history.db` and `heuristics.db`.

**Alternatives considered**:

| Option | Query flexibility | Concurrency | Deployment | Size |
|--------|------------------|-------------|------------|------|
| SQLite | Full SQL | WAL mode (concurrent reads) | Single file, zero config | ~1MB library |
| sled (embedded KV) | Key prefix scan only | Built-in | Single directory | ~500KB |
| JSON files | Manual parsing | File locking needed | Multiple files | Zero deps |
| RocksDB | Key prefix scan | Built-in | Single directory | ~5MB library |

**Rationale**: PRECC needs relational queries — joining failures with fixes, aggregating patterns by project type, computing confidence scores. SQL is the natural fit. SQLite is the most widely deployed database engine in the world, adds minimal binary size, and is already used by RTK for tracking. Using the same engine simplifies the dependency tree and developer mental model.

The key-value alternatives would require reimplementing relational logic in application code, leading to more bugs and worse performance for aggregate queries.

**Trade-off**: SQLite's write concurrency is limited (one writer at a time). Mitigated by WAL mode (concurrent readers, serialized writers) and the fact that only the miner writes to `history.db` in bulk — the hook only reads.

## 3. Workspace of 4 Crates vs Monolith

**Decision**: Structure the project as a Cargo workspace with 4 crates: `precc-core`, `precc-hook`, `precc-cli`, `precc-miner`.

**Alternatives considered**:

| Option | Build time | Binary size | Code sharing | Complexity |
|--------|-----------|-------------|--------------|------------|
| Single binary (monolith) | One target | One large binary | Implicit | Low |
| 4 crates (workspace) | Incremental | 3 small binaries + lib | Explicit via precc-core | Medium |
| Separate repos | Independent | Independent | Git submodule/crate | High |

**Rationale**: The three binaries have fundamentally different requirements:
- **precc-hook**: Must be tiny and fast (< 5ms). Cannot afford to link CLI parsing or filesystem watching.
- **precc-cli**: Needs clap for argument parsing, rich formatting for reports.
- **precc-miner**: Needs filesystem watching (notify crate), long-running daemon support.

A monolith would bloat the hook binary with unnecessary dependencies. Separate repos would make cross-crate changes painful. The workspace strikes the right balance: shared code in `precc-core`, independent binaries that only link what they need.

**Trade-off**: Slightly more complex project structure. Mitigated by clear naming conventions and the workspace Cargo.toml managing shared dependencies.

## 4. GDB Hook Integration vs Standalone CLI

**Decision**: Implement GDB debugging as a CLI command (`precc debug`) rather than as an automatic hook rewrite.

**Alternatives considered**:

| Option | Safety | Discoverability | Flexibility |
|--------|--------|-----------------|-------------|
| Auto-rewrite in hook | Dangerous (silent behavior change) | Invisible | Low (one-size-fits-all) |
| CLI command (`precc debug`) | Safe (explicit invocation) | Clear | High (user controls args) |
| Claude Code MCP server | Safe | Medium | High |

**Rationale**: Automatically rewriting `cargo test` to launch GDB would be surprising and potentially dangerous (GDB can modify process state). A CLI command is explicit — Claude must intentionally choose to use `precc debug` instead of running the binary directly. This also allows the user to see what GDB commands will be generated before execution.

The hook can still *suggest* using `precc debug` (via `permissionDecisionReason`), but the actual invocation remains an explicit choice.

**Trade-off**: Requires Claude to learn about the `precc debug` command. Mitigated by documenting it in CLAUDE.md and having the hook suggest it when appropriate.

## 5. Background Daemon vs On-Demand Mining

**Decision**: Support both modes — `precc-miner` daemon for continuous mining, `precc ingest` for on-demand.

**Alternatives considered**:

| Option | Freshness | Resource usage | Setup complexity |
|--------|-----------|----------------|------------------|
| Daemon only | Real-time | Constant (low) | Requires service management |
| On-demand only | Manual | Zero when idle | Simple |
| Both (chosen) | User's choice | User's choice | Medium |

**Rationale**: Different users have different preferences:
- Power users want real-time learning → daemon
- Occasional users want zero overhead → on-demand
- CI environments need batch processing → `precc ingest --all`

Supporting both modes costs little in implementation (the mining logic is in `precc-core`, both binaries just call it) while maximizing user flexibility.

**Trade-off**: Two ways to do the same thing can confuse users. Mitigated by clear documentation and sensible defaults (on-demand by default, daemon opt-in).

## 6. Confidence Thresholds

**Decision**: Three-tier confidence system: auto-apply (≥ 0.7), suggest (0.3-0.7), hidden (< 0.3).

**Alternatives considered**:

| Option | Safety | Efficiency | User burden |
|--------|--------|------------|-------------|
| Always auto-apply | Low (risky for new skills) | Maximum | None |
| Always suggest (never auto) | High | Low (user must approve) | High |
| Three-tier (chosen) | Balanced | Good | Low |
| User-configurable threshold | Maximum flexibility | Varies | Setup required |

**Rationale**: Built-in skills (like `cargo-wrong-dir`) have known-good behavior and should auto-apply. Newly mined skills haven't been validated yet and should only be suggested. The three-tier system provides this graduation automatically through the skill lifecycle.

The 0.7 threshold for auto-apply was chosen based on the principle that a skill should have demonstrated success in at least 5 activations with < 10% failure rate before being trusted to modify commands silently.

**Trade-off**: Fixed thresholds may not suit all users. A future enhancement could allow per-skill or global threshold overrides via config.

## 7. RTK Subsumption Strategy

**Decision**: Port RTK's rewriting logic into `precc-core` as the final pipeline stage, rather than running both hooks in sequence.

**Alternatives considered**:

| Option | Latency | Maintenance | Migration path |
|--------|---------|-------------|----------------|
| Two hooks in sequence | 25-45ms (shell + binary) | Two codebases | Gradual |
| Port to Rust, replace hook | < 5ms | One codebase | Big-bang |
| Port to Rust, keep shell as fallback | < 5ms primary, shell fallback | Two codebases initially | Gradual with safety net |

**Rationale**: Running two hooks in sequence doubles the latency. The RTK rewriting logic is straightforward pattern matching that maps cleanly to Rust's `match` + regex. Porting it into the hook binary eliminates the shell script entirely while maintaining identical behavior.

The key insight is that RTK rewriting is just one stage in the PRECC pipeline — it logically belongs as step 5 (after skill matching, context resolution, and GDB checking).

**Trade-off**: Breaking change for users who have customized `rtk-rewrite.sh`. Mitigated by providing a clear migration guide and keeping the port behaviorally identical.

## 8. Skill Storage Format

**Decision**: TOML files for built-in skills, SQLite rows for mined/user skills.

**Alternatives considered**:

| Option | Human readability | Edit workflow | Version control |
|--------|------------------|---------------|-----------------|
| All TOML files | Excellent | Text editor | Git-friendly |
| All SQLite rows | Poor | CLI tool needed | Binary diff |
| Hybrid (chosen) | Built-ins readable | CLI for mined | Built-ins in git |

**Rationale**: Built-in skills ship with the PRECC source code and should be version-controlled, reviewable, and editable by contributors. TOML is the natural choice for Rust projects (Cargo uses it). Mined skills are auto-generated at high volume and need fast querying — SQLite is the right fit.

The CLI bridges both: `precc skills list` shows all skills regardless of storage, `precc skills export` can dump SQLite skills to TOML for review.

## 9. Session Log Format

**Decision**: Read Claude Code's native JSONL format directly rather than converting to a custom format.

**Rationale**: Claude Code already writes detailed session logs in JSONL format at `~/.claude/projects/*/`. Creating a custom format would mean:
1. Extra conversion step
2. Schema drift risk
3. Doubled storage

By reading JSONL directly, PRECC stays in sync with Claude Code's evolution and avoids the "second source of truth" problem.

**Trade-off**: Tightly coupled to Claude Code's JSONL schema, which could change. Mitigated by version-checking the schema and graceful degradation on unknown fields.
