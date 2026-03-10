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

### Option 2: OpenClaw (ClawHub)

If you use [OpenClaw](https://github.com/openclaw/openclaw), install directly from [ClawHub](https://clawhub.ai/skills/precc-token-saver):

```bash
clawdhub install precc-token-saver
```

The skill activates automatically. Every shell command OpenClaw runs is piped through `precc-hook` before execution.

To show savings at any time:

```
@precc report
@precc skills list
@precc savings
@precc update
```

### Option 3: ZeroClaw

If you use [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw), install from the GitHub repo:

```bash
zeroclaw skills install https://github.com/yijunyu/precc-cc
```

The skill adds four tools to the agent (`precc_hook`, `precc_report`, `precc_skills`, `precc_update`) and injects the hook routing instructions into the system prompt automatically.

### Option 4: Claude Code Plugin

```bash
claude plugin marketplace add yijunyu/precc
claude plugin install precc
```

Restart Claude Code to activate the plugin.


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

## Pricing

| Feature | Free | Pro |
|---|---|---|
| Wrong-dir correction | ✓ | ✓ |
| RTK output compression | ✓ | ✓ |
| jj/Jujutsu translation | ✓ | ✓ |
| Built-in skills (8 included) | ✓ | ✓ |
| `precc report` | ✓ | ✓ |
| Session mining (`precc ingest`) | 1 session | Unlimited |
| Mined skills active in hook | 3 max | Unlimited |
| `precc gif` (script → animated GIF) | — | ✓ |
| `precc mail` (email reports) | — | ✓ |
| `precc savings` (dollar estimate) | — | ✓ |

Activate a Pro key:

```bash
precc license activate PRECC-XXXXXXXX-XXXXXXXX-XXXXXXXX-XXXXXXXX
precc license status
```

**Telemetry:** `precc update` sends one anonymous ping (version + OS + arch, no PII) to count active users. Opt out with `PRECC_NO_TELEMETRY=1`.

## Changelog

### v0.1.5 — Update Ping & Telemetry

- Anonymous update-check ping on `precc update` (version/OS/arch only, no PII)
- Opt-out: `PRECC_NO_TELEMETRY=1`
- Ping URL configurable at build time via `PRECC_PING_URL`

### v0.1.4 — License Enforcement & Plugin Marketplace

- **Free/Pro tier gates** — `precc ingest` (1 session free), mined skills capped at 3 on Free, `precc gif` / `precc mail` / `precc savings` require Pro
- **`license::tier()`** — `OnceLock`-cached tier check, zero cost on repeated calls in hook
- **OpenClaw plugin** published to [ClawHub](https://clawhub.ai/skills/precc-token-saver) (`clawdhub install precc-token-saver`)
- **ZeroClaw plugin** — `SKILL.toml` with native shell tools, installable via `zeroclaw skills install`
- `PRECC_LICENSE_SECRET` passed through Linux and macOS release builds

### v0.1.3 — Self-Update & GIF Generation

- **`precc update`** — self-update from GitHub releases; detects platform, downloads correct asset, replaces binaries in-place, verifies new version
- **`precc gif <script> <length>`** — record a bash script with `asciinema` and convert to GIF at a target duration via `agg`
- **`precc mail`** — send reports and attachments via SSH relay SMTP
- **`precc license`** — HMAC-SHA256 machine-bound license key system (activate/status/deactivate)
- Deploy script auto-bumps `Cargo.toml` version to match release tag

### v0.6.0 — Hook Latency Optimization

- **Prefix cache pre-filter** — `precc init` and `precc-miner` write
  `skill_prefixes.txt` (plain text, one first-word per line) to the data
  directory; the hook reads this with a single `read()` syscall and skips
  opening `heuristics.db` entirely when the command's first word is not
  listed, saving ~3ms per invocation for non-matching commands (`echo`,
  `curl`, `ls`, etc.)
- Investigated hook latency: identified `heuristics.db` SQLCipher open
  (~7–8ms per process) as the dominant cost; binary startup is 1.5ms,
  context resolution and RTK are <1ms each

### v0.5.0 — Embedded Skills & Subagent Mining

- **Builtin skills embedded in binary** — `precc init` loads all 6 builtin
  skills via `include_str!` at compile time; no `skills/builtin/` directory
  needed at runtime, protecting IP and simplifying deployment
- **Subagent session mining fixed** — `precc ingest` now correctly mines
  subagent JSONL files (`agent-*.jsonl`); the old positional merge missed
  Bash events when Glob/Read/Grep results interleaved between a Bash
  `tool_use` and its `tool_result`; replaced with `tool_use_id` lookup
  (328 sessions / 233 pairs, up from 71 sessions / 205 pairs)

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

## License

MIT
