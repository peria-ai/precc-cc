# སྒྲིག་འཇུག

## Quick Install (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

This downloads the latest release binary for your platform, verifies the SHA256 checksum, and places it in `~/.local/bin/`.

After installation, initialize PRECC:

```bash
precc init
```

`precc init` registers the PreToolUse hook with Claude Code, creates the data directories, and initializes the skills database.

## Install Options

### SHA256 Verification

By default, the installer verifies the binary checksum against the published SHA256 sum. To skip verification (not recommended):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### Custom Install Prefix

Install to a custom location:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### Companion Tools (--extras)

PRECC ships with optional companion tools that significantly increase token savings — most importantly **lean-ctx**, which contributes ~50% saving on real measured workloads. Install them with `--extras`:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

This installs:

| Tool | Purpose |
|------|---------|
| **RTK** | Command rewriting toolkit |
| **lean-ctx** | Deep Bash output compression — measured 50% aggregate saving across 39 command classes; 41–91% on commands with verbose output (git status, find, head -100, etc.) |
| **nushell** | Structured shell for advanced pipelines |
| **cocoindex-code** | Code indexing for faster context resolution |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

Then initialize:

```powershell
precc init
```

## Manual Install

1. Download the release binary for your platform from [GitHub Releases](https://github.com/peria-ai/precc-cc/releases).
2. Verify the SHA256 checksum against the `.sha256` file in the release.
3. Place the binary in a directory on your `PATH` (e.g., `~/.local/bin/`).
4. Run `precc init`.

## Updating

```bash
precc update
```

Force update to a specific version:

```bash
precc update --force --version 0.3.0
```

Enable automatic updates:

```bash
precc update --auto
```

## Verifying Installation

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

If `precc` is not found, ensure `~/.local/bin` is on your `PATH`.
