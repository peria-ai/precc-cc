# {{i18n:sk_title}}

{{i18n:sk_intro}}

## {{i18n:sk_builtin_title}}

| {{i18n:sk_col_skill}} | {{i18n:sk_col_triggers}} | {{i18n:sk_col_action}} |
|-------|-------------|--------|
| `cargo-wrong-dir` | {{i18n:sk_cargo_trigger}} | {{i18n:sk_cargo_action}} |
| `git-wrong-dir` | {{i18n:sk_git_trigger}} | {{i18n:sk_git_action}} |
| `go-wrong-dir` | {{i18n:sk_go_trigger}} | {{i18n:sk_go_action}} |
| `make-wrong-dir` | {{i18n:sk_make_trigger}} | {{i18n:sk_make_action}} |
| `npm-wrong-dir` | {{i18n:sk_npm_trigger}} | {{i18n:sk_npm_action}} |
| `python-wrong-dir` | {{i18n:sk_python_trigger}} | {{i18n:sk_python_action}} |
| `jj-translate` | {{i18n:sk_jj_trigger}} | {{i18n:sk_jj_action}} |
| `asciinema-gif` | {{i18n:sk_asciinema_trigger}} | {{i18n:sk_asciinema_action}} |

## {{i18n:sk_listing_title}}

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

## {{i18n:sk_showing_title}}

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

## {{i18n:sk_exporting_title}}

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

## {{i18n:sk_editing_title}}

```bash
$ precc skills edit cargo-wrong-dir
```

{{i18n:sk_editing_body}}

## {{i18n:sk_advise_title}}

{{i18n:sk_advise_body}}

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

## {{i18n:sk_clustering_title}}

```bash
$ precc skills cluster
```

{{i18n:sk_clustering_body}}

## {{i18n:sk_mined_vs_builtin_title}}

{{i18n:sk_mined_vs_builtin_body}}
