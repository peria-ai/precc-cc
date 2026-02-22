# PRECC — Predictive Error Correction for Claude Code

PRECC saves **~34% of Claude Code costs** by fixing bash commands before they fail and compressing tool output.

## Install

### Option 1: Claude Code Plugin (recommended)

```bash
claude plugin marketplace add yijunyu/precc
claude plugin install precc
```

Then build and install the hook binary (requires Rust toolchain):

```bash
git clone https://github.com/yijunyu/precc && cd precc
cargo install --path crates/precc-hook
cargo install --path crates/precc-cli
```

Restart Claude Code to activate the plugin.

### Option 2: One-line Install

```bash
git clone https://github.com/yijunyu/precc && cd precc && ./scripts/install.sh
```

### Option 3: Manual

Requires Rust toolchain.

```bash
git clone https://github.com/yijunyu/precc
cd precc
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

# View savings report
precc report
```

## What It Does

- **Fixes wrong-directory commands** — Detects when `cargo build` or `npm test` is run in the wrong directory and prepends `cd /correct/path &&`
- **Prevents repeated failures** — Learns from past session failures and auto-corrects commands that would fail the same way
- **Compresses CLI output** — Rewrites commands to use [RTK](https://github.com/rtk-ai/rtk) for 60-90% smaller output, reducing context growth
- **Suggests GDB debugging** — When a command fails repeatedly, suggests `precc debug` instead of edit-compile-retry cycles

## Requirements

- Claude Code (with hooks support)
- Rust toolchain (for building from source)
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

## Development

```bash
cargo build              # Build
cargo test               # Test
cargo clippy --all-targets  # Lint
cargo fmt --all --check  # Format check
```

## License

MIT
