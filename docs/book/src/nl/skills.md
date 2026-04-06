# Vaardigheden

Vaardigheden zijn de patroonherkennungsregels die PRECC gebruikt om commando's te detecteren en te corrigeren. Ze kunnen ingebouwd (meegeleverd als TOML-bestanden) of gedolven uit sessielogs zijn.

## Ingebouwde vaardigheden

| Vaardigheid | Activeert bij | Actie |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` buiten een Rust-project | `cd` toevoegen naar de dichtstbijzijnde `Cargo.toml`-map |
| `git-wrong-dir` | `git *` buiten een git-repo | `cd` toevoegen naar de dichtstbijzijnde `.git`-map |
| `go-wrong-dir` | `go build/test` buiten een Go-module | `cd` toevoegen naar de dichtstbijzijnde `go.mod`-map |
| `make-wrong-dir` | `make` zonder Makefile in de huidige map | `cd` toevoegen naar de dichtstbijzijnde Makefile-map |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` buiten een Node-project | `cd` toevoegen naar de dichtstbijzijnde `package.json`-map |
| `python-wrong-dir` | `python/pytest/pip` buiten een Python-project | `cd` toevoegen naar het dichtstbijzijnde Python-project |
| `jj-translate` | `git *` in een jj-gecoloceerde repo | Herschrijven naar equivalent `jj`-commando |
| `asciinema-gif` | `asciinema rec` | Herschrijven naar `precc gif` |

## Vaardigheden weergeven

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

## Vaardigheiddetails weergeven

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

## Een vaardigheid exporteren naar TOML

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

## Een vaardigheid bewerken

```bash
$ precc skills edit cargo-wrong-dir
```

Dit opent de vaardigheidsdefinitie in uw `$EDITOR`. Na opslaan wordt de vaardigheid automatisch herladen.

## Het Advise-commando

`precc skills advise` analyseert uw recente sessie en suggereert nieuwe vaardigheden op basis van herhaalde patronen:

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

## Vaardigheden groeperen

```bash
$ precc skills cluster
```

Groepeert vergelijkbare gedolven vaardigheden om redundante of overlappende patronen te identificeren.

## Gedolven vs. ingebouwde vaardigheden

Ingebouwde vaardigheden worden meegeleverd met PRECC en zijn gedefinieerd in `skills/builtin/*.toml`. Ze dekken de meest voorkomende verkeerde-map-fouten.

Gedolven vaardigheden worden aangemaakt door `precc ingest` of de `precc-learner`-daemon uit uw sessielogs. Ze worden opgeslagen in `~/.local/share/precc/heuristics.db` en zijn specifiek voor uw workflow. Zie [Mining](mining.md) voor details.
