# PRECC — Predictive Error Correction for Claude Code

PRECC saves **~34% of Claude Code costs** by fixing bash commands before they fail and compressing tool output.

## Install

### Option 1: Pre-built binary (recommended)

**Linux / macOS:**

```bash
curl -fsSL https://raw.githubusercontent.com/yijunyu/precc-cc/main/scripts/install.sh | bash
```

**Windows (PowerShell):**

```powershell
iwr -useb https://raw.githubusercontent.com/yijunyu/precc-cc/main/scripts/install.ps1 | iex
```

Then run:

```bash
precc init
```

### Option 2: Claude Code Plugin

```bash
claude plugin marketplace add yijunyu/precc
claude plugin install precc
```

Restart Claude Code to activate the plugin.

### Option 3: Manual (from source)

Requires Rust toolchain.

```bash
git clone https://github.com/yijunyu/precc-cc
cd precc-cc
cargo install --path crates/precc-hook
cargo install --path crates/precc-cli
```

Then add to `~/.claude/settings.json`:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "/home/YOU/.cargo/bin/precc-hook"
          }
        ]
      }
    ]
  }
}
```

Replace `/home/YOU` with your home directory, or use `$(which precc-hook)` to find the installed path.

> **Performance note:** Use the release binary (`cargo install --path crates/precc-hook`) rather than a debug build. The release binary runs in ~3ms; a debug build runs in ~480ms due to unoptimized code.

## Usage

Once installed, PRECC works automatically. Every bash command Claude Code runs passes through the hook, which silently fixes common mistakes and compresses output.

```bash
# Initialize databases and mine existing session history
precc init
precc ingest --all

# Re-mine all sessions from scratch (e.g. after a mining logic update)
precc ingest --all --force

# View what PRECC has learned
precc skills list
precc skills show <name>    # full trigger/action detail
precc skills export <name>  # dump as TOML (for sharing/backup)
precc skills edit <name>    # open in $EDITOR and reimport on save

# View savings report
precc report
```

## What It Does

- **Fixes wrong-directory commands** — Detects when `cargo build` or `npm test` is run in the wrong directory and prepends `cd /correct/path &&`
- **Prevents repeated failures** — Learns from past session failures and auto-corrects commands that would fail the same way
- **Compresses CLI output** — Rewrites commands to use [RTK](https://github.com/rtk-ai/rtk) for 60-90% smaller output, reducing context growth
- **Suggests GDB debugging** — When a command fails repeatedly, suggests `precc debug` instead of edit-compile-retry cycles

## Security

As of v0.2.0, all PRECC databases (`heuristics.db`, `history.db`, `metrics.db`) are
**AES-256 encrypted** via SQLCipher. The encryption key is derived automatically from
your machine ID and username using HKDF-SHA256 — no passphrase required, no key stored
on disk. The databases are unreadable on any other machine.

```
$ precc init
  Encryption: AES-256 (machine-bound key, first 4 bytes: a3f7...)
```

Binary releases have internal strings obfuscated with `obfstr` to reduce information
leakage via `strings(1)`.

## Requirements

- Claude Code (with hooks support)
- Rust toolchain (for building from source only)
- [RTK](https://github.com/rtk-ai/rtk) (optional, for output compression)

## Measured Results

Analyzed across 29 real Claude Code sessions, 5 projects, 5,384 bash calls, $878 total spend:

| Metric | Value |
|--------|-------|
| **Cost savings** | **$296 / $878 (34%)** |
| **Failures prevented** | **352 / 358 (98%)** |
| **Bash calls improved** | **894 / 5,384 (17%)** |
| **Cache reads saved** | **988M / 1.67B tokens (59%)** |
| **Hook latency** | **2.93ms avg (1.77ms overhead)** |

## Changelog

### v0.4.0 — Skills Management & GDB Pillar

- **`precc skills export <name>`** — export any skill to TOML format on stdout,
  matching the `skills/builtin/*.toml` file format; enables sharing between
  machines and pull-request contributions of mined skills
- **`precc skills edit <name>`** — open a skill in `$EDITOR` as TOML, validates
  syntax, reimports on save; skill stats (activation counts) are preserved;
  name renames rejected with a clear error
- **Git wrong-dir skill** — new `git-wrong-dir` builtin covers 20+ git
  subcommands; `context.rs` now maps `git` → `.git` for Pillar 1 cd-prepend
- **Prepend-cd guard** — hook no longer applies a `cd CWD && cmd` no-op rewrite
  when no better project directory is found (was silent wrong behaviour)
- **GDB Pillar 2 re-enabled** — hook now queries `history.db` for recent
  failures of the same command class; when ≥2 failures in the last 24 hours,
  surfaces `"Consider: precc debug …"` in `permissionDecisionReason` so
  Claude sees the advisory; records `gdb_suggestion` metric for `precc report`

### v0.3.0 — Skill Lifecycle & Live Metrics

- **Skill confidence lifecycle** — mined skills auto-promote as they prove useful:
  - `CANDIDATE (0.3)` → `ACTIVE (0.7)` after 5 activations (hook auto-applies)
  - `ACTIVE (0.7)` → `TRUSTED (0.9)` after 20 activations with <5% failure rate
  - Auto-disabled when failure rate exceeds 20% with ≥5 activations
- **Live hook metrics** — `precc report` now shows real hook latency, CD prepend
  counts, and RTK rewrite counts from actual usage (previously "no data")
- **O_APPEND metrics bridge** — hook appends JSONL to `metrics.log` (single
  syscall, ~10µs); miner atomically imports on each tick with no DB writes in
  the hot path
- **Activation tracking fix** — corrected SQL that silently failed to increment
  `skill_stats.activated`, causing lifecycle promotions never to fire
- **Investor demo suite** — `demo/demo.sh` (6-section runnable terminal demo),
  `demo/DEMO_GUIDE.md` (timed presenter script), `demo/PITCH_NARRATIVES.md`
  (5 audience-specific pitch narratives), `demo/session.jsonl` (synthetic session)

### v0.2.0 — Security & Distribution
- **AES-256 database encryption** — all databases encrypted via SQLCipher with a
  machine-bound key derived from HKDF-SHA256; zero user friction, no passphrase
- **Binary hardening** — internal strings obfuscated with `obfstr` in all binaries
- **Pre-built releases** — GitHub Actions CI builds for 5 targets:
  `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `x86_64-apple-darwin`,
  `aarch64-apple-darwin`, `x86_64-pc-windows-msvc`
- **One-line install scripts** — `install.sh` (Linux/macOS) and `install.ps1` (Windows)
  with automatic hook wiring and PATH setup
- **Migration** — `precc init` auto-migrates existing unencrypted databases in place

### v0.1.0 — Initial Release
- PreToolUse:Bash hook pipeline (context resolution, skill matching, RTK rewriting)
- `precc ingest`, `precc skills`, `precc report`, `precc savings`, `precc debug`
- SQLite-backed failure-fix mining and pattern promotion
- Built-in skills and mined skill promotion

## Development

```bash
cargo build              # Build
cargo test               # Test
cargo clippy --all-targets  # Lint
cargo fmt --all --check  # Format check
```

## License

MIT
