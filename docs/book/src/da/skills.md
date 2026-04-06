# Færdigheder

Færdigheder er mønstermatchingsreglerne som PRECC bruger til at registrere og korrigere kommandoer. De kan være indbyggede (leveret som TOML-filer) eller lært fra sessionslogfiler.

## Indbyggede færdigheder

| Færdighed | Udløses ved | Handling |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` uden for et Rust-projekt | Sæt `cd` foran til nærmeste `Cargo.toml`-mappe |
| `git-wrong-dir` | `git *` uden for et git-repo | Sæt `cd` foran til nærmeste `.git`-mappe |
| `go-wrong-dir` | `go build/test` uden for et Go-modul | Sæt `cd` foran til nærmeste `go.mod`-mappe |
| `make-wrong-dir` | `make` uden Makefile i cwd | Sæt `cd` foran til nærmeste Makefile-mappe |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` uden for et Node-projekt | Sæt `cd` foran til nærmeste `package.json`-mappe |
| `python-wrong-dir` | `python/pytest/pip` uden for et Python-projekt | Sæt `cd` foran til nærmeste Python-projekt |
| `jj-translate` | `git *` i et jj-colocated repo | Omskriv til tilsvarende `jj`-kommando |
| `asciinema-gif` | `asciinema rec` | Omskriv til `precc gif` |

## Liste over færdigheder

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

## Færdighedsdetaljer

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

## Eksport af færdighed til TOML

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

## Redigering af en færdighed

```bash
$ precc skills edit cargo-wrong-dir
```

Dette åbner færdighedsdefinitionen i din `$EDITOR`. Efter lagring genindlæses færdigheden automatisk.

## Advise-kommandoen

`precc skills advise` analyserer din seneste session og foreslår nye færdigheder baseret på gentagne mønstre:

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

## Klyngning af færdigheder

```bash
$ precc skills cluster
```

Grupperer lignende lærte færdigheder for at identificere overflødige eller overlappende mønstre.

## Lærte vs. indbyggede færdigheder

Indbyggede færdigheder leveres med PRECC og er defineret i `skills/builtin/*.toml`. De dækker de mest almindelige forkert-mappe-fejl.

Lærte færdigheder oprettes af `precc ingest` eller `precc-learner`-dæmonen fra dine sessionslogfiler. De gemmes i `~/.local/share/precc/heuristics.db` og er specifikke for din arbejdsgang. Se [Mining](mining.md) for detaljer.
