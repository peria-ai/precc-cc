# PRECC — Predictive Error Correction for Claude Code

PRECC saves **~34% of Claude Code costs** through three pillars: fixing bash commands before they fail, compressing tool output, and reducing context token usage via semantic search and file compression.

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

### Option 3: ClawHub Skill

```bash
clawhub install precc
```

## What It Does

### Pillar 1: Command Correction & Output Compression

- **Fixes wrong-directory commands** — Detects when `cargo build` or `npm test` is run in the wrong directory and prepends `cd /correct/path &&`
- **Prevents repeated failures** — Learns from past session failures and auto-corrects commands that would fail the same way
- **Compresses CLI output** — Rewrites commands to use [RTK](https://github.com/rtk-ai/rtk) for 60-90% smaller output, reducing context growth
- **Suggests GDB debugging** — When a command fails repeatedly, suggests `precc debug` instead of edit-compile-retry cycles

### Pillar 2: Semantic Code Search ([cocoindex-code](https://github.com/cocoindex-io/cocoindex-code))

PRECC's hook automatically intercepts recursive `grep` and `rg` commands. When a project has a cocoindex-code index, the hook redirects through AST-aware semantic search — saving ~70% of search output tokens.

```bash
# Index your project (one-time)
ccc init && ccc index

# Search by meaning instead of text
ccc search "user session management"
ccc search --lang python "error handling"
```

**How it works:** The PreToolUse hook extracts the search pattern from grep/rg, runs `ccc search`, compares output sizes, and returns the smaller result. Savings are logged to `~/.precc/ccc-metrics.jsonl`.

### Pillar 3: Context File Compression ([token-saver](https://clawhub.ai/skills/token-saver))

Strips filler words and verbose phrasing from always-loaded context files (CLAUDE.md, memory files). Since these files are sent with every API call, even small reductions compound across a session.

```bash
# Preview savings
node precc-ts-compress.js --dry-run

# Compress (backups saved as *.backup)
node precc-ts-compress.js

# Revert to originals
node precc-ts-compress.js --revert
```

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

# View unified savings report (all three pillars)
precc savings
```

## Security

As of v0.2.0, all PRECC databases (`heuristics.db`, `history.db`, `metrics.db`) are
**AES-256 encrypted** via SQLCipher. The encryption key is derived automatically from
your machine ID and username using HKDF-SHA256 — no passphrase required, no key stored
on disk. The databases are unreadable on any other machine.

```
$ precc init
  Encryption: AES-256 (machine-bound key, first 4 bytes: a3f7...)
```

## Requirements

- Claude Code (with hooks support)
- [RTK](https://github.com/rtk-ai/rtk) (optional, for output compression)
- [cocoindex-code](https://github.com/cocoindex-io/cocoindex-code) (optional, for AST-driven semantic search)
- Node.js (optional, for context file compression)

## Measured Results

Analyzed across 29 real Claude Code sessions, 5 projects, 5,384 bash calls, $878 total spend:

| Metric | Value |
|--------|-------|
| **Cost savings** | **$296 / $878 (34%)** |
| **Failures prevented** | **352 / 358 (98%)** |
| **Bash calls improved** | **894 / 5,384 (17%)** |
| **Cache reads saved** | **988M / 1.67B tokens (59%)** |
| **Hook latency** | **2.93ms avg (1.77ms overhead)** |

## Acknowledgements

- [cocoindex-code](https://github.com/cocoindex-io/cocoindex-code) — AST-driven semantic code search engine
- [token-saver](https://clawhub.ai/skills/token-saver) — Context file compression patterns (MIT-0, by RubenAQuispe)
- [RTK](https://github.com/rtk-ai/rtk) — CLI output compression toolkit
