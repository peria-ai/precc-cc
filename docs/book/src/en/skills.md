# Skills

Skills are the pattern-matching rules that PRECC uses to detect and correct commands. They can be built-in (shipped as TOML files) or mined from session logs.

## Built-in Skills

| Skill | Triggers On | Action |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` outside a Rust project | Prepend `cd` to the nearest `Cargo.toml` directory |
| `git-wrong-dir` | `git *` outside a git repo | Prepend `cd` to the nearest `.git` directory |
| `go-wrong-dir` | `go build/test` outside a Go module | Prepend `cd` to the nearest `go.mod` directory |
| `make-wrong-dir` | `make` without a Makefile in cwd | Prepend `cd` to the nearest Makefile directory |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` outside a Node project | Prepend `cd` to the nearest `package.json` directory |
| `python-wrong-dir` | `python/pytest/pip` outside a Python project | Prepend `cd` to the nearest Python project |
| `jj-translate` | `git *` in a jj-colocated repo | Rewrite to equivalent `jj` command |
| `asciinema-gif` | `asciinema rec` | Rewrite to `precc gif` |

## Listing Skills

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
  9 fix-pytest-path    mined     pytest with wrong test path
```

## Showing Skill Details

```bash
$ precc skills show cargo-wrong-dir
Name:        cargo-wrong-dir
Type:        built-in
Source:      skills/builtin/cargo-wrong-dir.toml
Description: Detects cargo commands run outside a Rust project and prepends
             cd to the directory containing the nearest Cargo.toml.
Trigger:     ^cargo\s+(build|test|clippy|run|check|bench|doc)
Action:      prepend_cd
Marker:      Cargo.toml
Activations: 12
```

## Exporting a Skill to TOML

```bash
$ precc skills export cargo-wrong-dir
[skill]
name = "cargo-wrong-dir"
description = "Prepend cd for cargo commands outside a Rust project"
trigger = "^cargo\\s+(build|test|clippy|run|check|bench|doc)"
action = "prepend_cd"
marker = "Cargo.toml"
priority = 10
```

## Editing a Skill

```bash
$ precc skills edit cargo-wrong-dir
```

This opens the skill definition in your `$EDITOR`. After saving, the skill is reloaded automatically.

## The Advise Command

`precc skills advise` analyzes your recent session and suggests new skills based on repeated patterns:

```bash
$ precc skills advise
Analyzed 47 commands from the last session.

Suggested skills:
  1. docker-wrong-dir: You ran `docker compose up` outside the project root 3 times.
     Suggested trigger: ^docker\s+compose
     Suggested marker: docker-compose.yml

  2. terraform-wrong-dir: You ran `terraform plan` outside the infra directory 2 times.
     Suggested trigger: ^terraform\s+(plan|apply|init)
     Suggested marker: main.tf

Accept suggestion [1/2/skip]?
```

## Clustering Skills

```bash
$ precc skills cluster
```

Groups similar mined skills together to help identify redundant or overlapping patterns.

## Mined vs. Built-in Skills

Built-in skills ship with PRECC and are defined in `skills/builtin/*.toml`. They cover the most common wrong-directory mistakes.

Mined skills are created by `precc ingest` or the `precc-learner` daemon from your session logs. They are stored in `~/.local/share/precc/heuristics.db` and are specific to your workflow. See [Mining](mining.md) for details.
