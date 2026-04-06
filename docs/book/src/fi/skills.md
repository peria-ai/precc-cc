# Taidot

Taidot ovat mallinsovitussääntöjä, joita PRECC käyttää komentojen havaitsemiseen ja korjaamiseen. Ne voivat olla sisäänrakennettuja (toimitettu TOML-tiedostoina) tai opittuja istuntolokeista.

## Sisäänrakennetut taidot

| Taito | Laukeaa kun | Toiminto |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` Rust-projektin ulkopuolella | Lisää `cd` lähimpään `Cargo.toml`-hakemistoon |
| `git-wrong-dir` | `git *` git-repon ulkopuolella | Lisää `cd` lähimpään `.git`-hakemistoon |
| `go-wrong-dir` | `go build/test` Go-moduulin ulkopuolella | Lisää `cd` lähimpään `go.mod`-hakemistoon |
| `make-wrong-dir` | `make` ilman Makefilea cwd:ssä | Lisää `cd` lähimpään Makefile-hakemistoon |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` Node-projektin ulkopuolella | Lisää `cd` lähimpään `package.json`-hakemistoon |
| `python-wrong-dir` | `python/pytest/pip` Python-projektin ulkopuolella | Lisää `cd` lähimpään Python-projektiin |
| `jj-translate` | `git *` jj-colocated-repossa | Kirjoita uudelleen vastaavaksi `jj`-komennoksi |
| `asciinema-gif` | `asciinema rec` | Kirjoita uudelleen muotoon `precc gif` |

## Taitojen listaus

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

## Taidon yksityiskohdat

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

## Taidon vienti TOMLiin

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

## Taidon muokkaus

```bash
$ precc skills edit cargo-wrong-dir
```

Tämä avaa taitomäärittelyn `$EDITOR`-editorissasi. Tallennuksen jälkeen taito ladataan uudelleen automaattisesti.

## Advise-komento

`precc skills advise` analysoi viimeisimmän istuntosi ja ehdottaa uusia taitoja toistuvien mallien perusteella:

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

## Taitojen klusterointi

```bash
$ precc skills cluster
```

Ryhmittelee samankaltaisia opittuja taitoja tarpeettomien tai päällekkäisten mallien tunnistamiseksi.

## Opitut vs. sisäänrakennetut taidot

Sisäänrakennetut taidot toimitetaan PRECCin mukana ja ne on määritelty tiedostoissa `skills/builtin/*.toml`. Ne kattavat yleisimmät väärä-hakemisto-virheet.

Opitut taidot luodaan `precc ingest` -komennolla tai `precc-learner`-daemonilla istuntolokeistasi. Ne tallennetaan tiedostoon `~/.local/share/precc/heuristics.db` ja ovat työnkulullesi ominaisia. Katso [Mining](mining.md) lisätietoja.
