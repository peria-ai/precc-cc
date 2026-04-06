# Quickstart

Get PRECC running in 5 minutes.

## Step 1: Install

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## Step 2: Initialize

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## Step 3: Verify the Hook Is Active

```bash
$ precc skills list
  # Name               Type      Triggers
  1 cargo-wrong-dir    built-in  cargo build/test/clippy outside Rust project
  2 git-wrong-dir      built-in  git * outside a repo
  3 go-wrong-dir       built-in  go build/test outside Go module
  4 make-wrong-dir     built-in  make without Makefile in cwd
  5 npm-wrong-dir      built-in  npm/npx/pnpm/yarn outside Node project
  6 python-wrong-dir   built-in  python/pytest/pip outside Python project
  7 jj-translate       built-in  git * in jj-colocated repo
  8 asciinema-gif      built-in  asciinema rec
```

## Step 4: Use Claude Code Normally

Open Claude Code and work as usual. PRECC runs silently in the background. When Claude issues a command that would fail, PRECC corrects it before execution.

### Example: Wrong-Directory Cargo Build

Suppose your project is at `~/projects/myapp/` and Claude issues:

```
cargo build
```

from `~/projects/` (one level too high, no `Cargo.toml` there).

**Without PRECC:** Claude gets the error `could not find Cargo.toml in /home/user/projects or any parent directory`, reads it, reasons about it, then retries with `cd myapp && cargo build`. Cost: ~2,000 tokens wasted.

**With PRECC:** The hook detects the missing `Cargo.toml`, finds it in `myapp/`, and rewrites the command to:

```
cd /home/user/projects/myapp && cargo build
```

Claude never sees an error. Zero tokens wasted.

## Step 5: Check Your Savings

After a session, see how many tokens PRECC saved:

```bash
$ precc savings
Session Token Savings
=====================
Total estimated savings: 4,312 tokens

Breakdown:
  Pillar 1 (cd prepends):       2,104 tokens  (3 corrections)
  Pillar 4 (skill activations):   980 tokens  (2 activations)
  RTK rewrites:                 1,228 tokens  (5 rewrites)
```

## Next Steps

- [Skills](skills.md) -- See all available skills and how to create your own.
- [Hook Pipeline](hook-pipeline.md) -- Understand what happens under the hood.
- [Savings](savings.md) -- Detailed token savings analysis.
