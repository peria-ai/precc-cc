# PRECC — Predictive Error Correction for Claude Code

PRECC saves **~34% of Claude Code costs** through three pillars: fixing bash commands before they fail, compressing tool output, and reducing context token usage via semantic search and file compression. Ships as a single Rust binary.

## Three Savings Pillars

### Pillar 1: Command Correction & Output Compression
- **Fixes wrong-directory commands** — Detects when `cargo build` or `npm test` is run in the wrong directory and prepends `cd /correct/path &&`
- **Prevents repeated failures** — Learns from past session failures and auto-corrects commands that would fail the same way
- **Compresses CLI output** — Rewrites commands to use RTK for 60-90% smaller output
- **Suggests GDB debugging** — When a command fails repeatedly, suggests `precc debug`

### Pillar 2: Semantic Code Search (cocoindex-code)
- Intercepts `grep`/`rg` and redirects through AST-aware semantic search
- Built into the `precc-hook` binary; no extra scripts needed
- Understands code structure across 28+ languages, saving ~70% of search tokens

### Pillar 3: Context File Compression
- Strips filler words from CLAUDE.md and memory files via `precc compress`
- Reduces tokens loaded on every API call (~30% compression)
- Backups saved automatically, revertible with `precc compress --revert`

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/yijunyu/precc-cc/main/scripts/install.sh | bash
precc init
```

## Usage

Once installed, PRECC works automatically as a PreToolUse hook.

```bash
# Mine existing session history for failure-fix patterns
precc ingest --all

# View what PRECC has learned
precc skills list

# View unified savings report (all three pillars)
precc savings

# Semantic code search (requires cocoindex-code)
ccc init && ccc index
ccc search "authentication middleware"

# Compress context files
precc compress --dry-run   # preview
precc compress             # compress
precc compress --revert    # revert
```

## Measured Results

| Metric | Value |
|--------|-------|
| **Cost savings** | **$296 / $878 (34%)** |
| **Failures prevented** | **352 / 358 (98%)** |
| **Bash calls improved** | **894 / 5,384 (17%)** |
| **Cache reads saved** | **988M / 1.67B tokens (59%)** |
| **Hook latency** | **2.93ms avg (1.77ms overhead)** |

## Links

- GitHub: https://github.com/yijunyu/precc-cc
- ClawHub: https://clawhub.ai/skills/precc
- cocoindex-code: https://github.com/cocoindex-io/cocoindex-code
- token-saver: https://clawhub.ai/skills/token-saver
- RTK: https://github.com/rtk-ai/rtk
