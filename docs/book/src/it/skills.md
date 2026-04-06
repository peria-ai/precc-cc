# Skill

Le skill sono le regole di pattern-matching che PRECC usa per rilevare e correggere i comandi. Possono essere integrate (distribuite come file TOML) o apprese dai log delle sessioni.

## Skill integrate

| Skill | Si attiva quando | Azione |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` fuori da un progetto Rust | Preponi `cd` alla directory `Cargo.toml` più vicina |
| `git-wrong-dir` | `git *` fuori da un repo git | Preponi `cd` alla directory `.git` più vicina |
| `go-wrong-dir` | `go build/test` fuori da un modulo Go | Preponi `cd` alla directory `go.mod` più vicina |
| `make-wrong-dir` | `make` senza Makefile nella cwd | Preponi `cd` alla directory Makefile più vicina |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` fuori da un progetto Node | Preponi `cd` alla directory `package.json` più vicina |
| `python-wrong-dir` | `python/pytest/pip` fuori da un progetto Python | Preponi `cd` al progetto Python più vicino |
| `jj-translate` | `git *` in un repo jj-colocated | Riscrivi nel comando `jj` equivalente |
| `asciinema-gif` | `asciinema rec` | Riscrivi in `precc gif` |

## Elenco skill

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

## Dettagli di una skill

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

## Esportazione di una skill in TOML

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

## Modifica di una skill

```bash
$ precc skills edit cargo-wrong-dir
```

Questo apre la definizione della skill nel tuo `$EDITOR`. Dopo il salvataggio, la skill viene ricaricata automaticamente.

## Il comando Advise

`precc skills advise` analizza la tua sessione recente e suggerisce nuove skill basate su pattern ripetuti:

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

## Raggruppamento skill

```bash
$ precc skills cluster
```

Raggruppa skill apprese simili per identificare pattern ridondanti o sovrapposti.

## Skill apprese vs. integrate

Le skill integrate vengono distribuite con PRECC e sono definite in `skills/builtin/*.toml`. Coprono gli errori di directory errata più comuni.

Le skill apprese vengono create da `precc ingest` o dal daemon `precc-learner` dai log delle tue sessioni. Sono memorizzate in `~/.local/share/precc/heuristics.db` e sono specifiche del tuo flusso di lavoro. Vedi [Mining](mining.md) per dettagli.
