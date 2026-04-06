# Färdigheter

Färdigheter är mönstermatchningsreglerna som PRECC använder för att upptäcka och korrigera kommandon. De kan vara inbyggda (levereras som TOML-filer) eller inlärda från sessionsloggar.

## Inbyggda färdigheter

| Färdighet | Utlöses vid | Åtgärd |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` utanför ett Rust-projekt | Lägg till `cd` till närmaste `Cargo.toml`-katalog |
| `git-wrong-dir` | `git *` utanför ett git-repo | Lägg till `cd` till närmaste `.git`-katalog |
| `go-wrong-dir` | `go build/test` utanför en Go-modul | Lägg till `cd` till närmaste `go.mod`-katalog |
| `make-wrong-dir` | `make` utan Makefile i cwd | Lägg till `cd` till närmaste Makefile-katalog |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` utanför ett Node-projekt | Lägg till `cd` till närmaste `package.json`-katalog |
| `python-wrong-dir` | `python/pytest/pip` utanför ett Python-projekt | Lägg till `cd` till närmaste Python-projekt |
| `jj-translate` | `git *` i ett jj-colocated repo | Skriv om till motsvarande `jj`-kommando |
| `asciinema-gif` | `asciinema rec` | Skriv om till `precc gif` |

## Lista färdigheter

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

## Färdighetsdetaljer

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

## Exportera färdighet till TOML

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

## Redigera en färdighet

```bash
$ precc skills edit cargo-wrong-dir
```

Detta öppnar färdighetsdefinitionen i din `$EDITOR`. Efter sparande laddas färdigheten om automatiskt.

## Advise-kommandot

`precc skills advise` analyserar din senaste session och föreslår nya färdigheter baserat på upprepade mönster:

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

## Klustring av färdigheter

```bash
$ precc skills cluster
```

Grupperar liknande inlärda färdigheter för att identifiera redundanta eller överlappande mönster.

## Inlärda vs. inbyggda färdigheter

Inbyggda färdigheter levereras med PRECC och definieras i `skills/builtin/*.toml`. De täcker de vanligaste fel-katalog-misstagen.

Inlärda färdigheter skapas av `precc ingest` eller `precc-learner`-demonen från dina sessionsloggar. De lagras i `~/.local/share/precc/heuristics.db` och är specifika för ditt arbetsflöde. Se [Mining](mining.md) för detaljer.
