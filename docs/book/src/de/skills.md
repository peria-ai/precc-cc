# Skills

Skills sind die Mustererkennungsregeln, die PRECC verwendet, um Befehle zu erkennen und zu korrigieren. Sie können eingebaut (als TOML-Dateien mitgeliefert) oder aus Sitzungsprotokollen gewonnen werden.

## Eingebaute Skills

| Skill | Auslöser | Aktion |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` außerhalb eines Rust-Projekts | `cd` zum nächsten `Cargo.toml`-Verzeichnis voranstellen |
| `git-wrong-dir` | `git *` außerhalb eines Git-Repos | `cd` zum nächsten `.git`-Verzeichnis voranstellen |
| `go-wrong-dir` | `go build/test` außerhalb eines Go-Moduls | `cd` zum nächsten `go.mod`-Verzeichnis voranstellen |
| `make-wrong-dir` | `make` ohne Makefile im aktuellen Verzeichnis | `cd` zum nächsten Makefile-Verzeichnis voranstellen |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` außerhalb eines Node-Projekts | `cd` zum nächsten `package.json`-Verzeichnis voranstellen |
| `python-wrong-dir` | `python/pytest/pip` außerhalb eines Python-Projekts | `cd` zum nächsten Python-Projekt voranstellen |
| `jj-translate` | `git *` in einem jj-kolokierten Repo | Umschreiben zum äquivalenten `jj`-Befehl |
| `asciinema-gif` | `asciinema rec` | Umschreiben zu `precc gif` |

## Skills auflisten

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

## Skill-Details anzeigen

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

## Einen Skill nach TOML exportieren

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

## Einen Skill bearbeiten

```bash
$ precc skills edit cargo-wrong-dir
```

Dies öffnet die Skill-Definition in Ihrem `$EDITOR`. Nach dem Speichern wird der Skill automatisch neu geladen.

## Der Advise-Befehl

`precc skills advise` analysiert Ihre letzte Sitzung und schlägt neue Skills basierend auf wiederkehrenden Mustern vor:

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

## Skills gruppieren

```bash
$ precc skills cluster
```

Gruppiert ähnliche geminte Skills, um redundante oder überlappende Muster zu identifizieren.

## Geminte vs. eingebaute Skills

Eingebaute Skills werden mit PRECC geliefert und sind in `skills/builtin/*.toml` definiert. Sie decken die häufigsten Fehler bei falschen Verzeichnissen ab.

Geminte Skills werden von `precc ingest` oder dem `precc-learner`-Daemon aus Ihren Sitzungsprotokollen erstellt. Sie werden in `~/.local/share/precc/heuristics.db` gespeichert und sind spezifisch für Ihren Workflow. Siehe [Mining](mining.md) für Details.
