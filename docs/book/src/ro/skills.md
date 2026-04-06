# Abilități

Abilitățile sunt regulile de potrivire a tiparelor pe care PRECC le folosește pentru a detecta și corecta comenzile. Pot fi integrate (livrate ca fișiere TOML) sau învățate din jurnalele sesiunilor.

## Abilități integrate

| Abilitate | Se declanșează la | Acțiune |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` în afara unui proiect Rust | Adaugă `cd` la cel mai apropiat director `Cargo.toml` |
| `git-wrong-dir` | `git *` în afara unui repo git | Adaugă `cd` la cel mai apropiat director `.git` |
| `go-wrong-dir` | `go build/test` în afara unui modul Go | Adaugă `cd` la cel mai apropiat director `go.mod` |
| `make-wrong-dir` | `make` fără Makefile în cwd | Adaugă `cd` la cel mai apropiat director Makefile |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` în afara unui proiect Node | Adaugă `cd` la cel mai apropiat director `package.json` |
| `python-wrong-dir` | `python/pytest/pip` în afara unui proiect Python | Adaugă `cd` la cel mai apropiat proiect Python |
| `jj-translate` | `git *` într-un repo jj-colocated | Rescrie în comanda `jj` echivalentă |
| `asciinema-gif` | `asciinema rec` | Rescrie în `precc gif` |

## Listarea abilităților

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

## Detalii abilitate

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

## Exportarea unei abilități în TOML

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

## Editarea unei abilități

```bash
$ precc skills edit cargo-wrong-dir
```

Aceasta deschide definiția abilității în `$EDITOR`. După salvare, abilitatea este reîncărcată automat.

## Comanda Advise

`precc skills advise` analizează sesiunea recentă și sugerează noi abilități bazate pe tipare repetate:

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

## Gruparea abilităților

```bash
$ precc skills cluster
```

Grupează abilități învățate similare pentru a identifica tipare redundante sau suprapuse.

## Abilități învățate vs. integrate

Abilitățile integrate vin cu PRECC și sunt definite în `skills/builtin/*.toml`. Acoperă cele mai comune greșeli de director.

Abilitățile învățate sunt create de `precc ingest` sau daemon-ul `precc-learner` din jurnalele sesiunilor dvs. Sunt stocate în `~/.local/share/precc/heuristics.db` și sunt specifice fluxului dvs. de lucru. Vedeți [Mining](mining.md) pentru detalii.
