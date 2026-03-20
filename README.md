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
- **AST-driven semantic code search** — Integrates [cocoindex-code](https://github.com/cocoindex-io/cocoindex-code) for semantic search that understands code structure across 28+ languages, saving 70% tokens vs raw grep

## Security

As of v0.2.0, all PRECC databases (`heuristics.db`, `history.db`, `metrics.db`) are
**AES-256 encrypted** via SQLCipher. The encryption key is derived automatically from
your machine ID and username using HKDF-SHA256 — no passphrase required, no key stored
on disk. The databases are unreadable on any other machine.

```
$ precc init
  Encryption: AES-256 (machine-bound key, first 4 bytes: a3f7...)
```

## Semantic Code Search (cocoindex-code)

PRECC integrates [cocoindex-code](https://github.com/cocoindex-io/cocoindex-code) to give Claude Code AST-aware semantic search. Instead of grep matching raw text, it understands code structure (classes, functions, methods) and supports natural language queries like "authentication middleware" or "database connection pooling".

The installer sets it up automatically. To use manually:

```bash
# Install
pipx install cocoindex-code

# Index your project
ccc init && ccc index

# Search by meaning
ccc search "user session management"
ccc search --lang python --lang typescript "error handling"

# Enable as MCP server for Claude Code
claude mcp add cocoindex-code -- ccc mcp
```

### How it works

PRECC's hook automatically intercepts recursive `grep` and `rg` commands. When a project has a cocoindex-code index (`.cocoindex_code/`), the hook:

1. Extracts the search pattern from the grep/rg command
2. Runs `ccc search` with that pattern (AST-aware, semantic matching)
3. Compares output sizes — if ccc returns fewer bytes, it uses the ccc result
4. Logs the byte/token savings to `~/.precc/ccc-metrics.jsonl`

View savings with:

```bash
precc-ccc-savings.sh
# or via plugin command:
precc ccc-savings
```

## Requirements

- Claude Code (with hooks support)
- [RTK](https://github.com/rtk-ai/rtk) (optional, for output compression)
- [cocoindex-code](https://github.com/cocoindex-io/cocoindex-code) (optional, for AST-driven semantic search)

## Measured Results

Analyzed across 29 real Claude Code sessions, 5 projects, 5,384 bash calls, $878 total spend:

| Metric | Value |
|--------|-------|
| **Cost savings** | **$296 / $878 (34%)** |
| **Failures prevented** | **352 / 358 (98%)** |
| **Bash calls improved** | **894 / 5,384 (17%)** |
| **Cache reads saved** | **988M / 1.67B tokens (59%)** |
| **Hook latency** | **2.93ms avg (1.77ms overhead)** |
