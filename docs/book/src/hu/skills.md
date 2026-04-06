# Képességek

A képességek azok a mintaillesztési szabályok, amelyeket a PRECC használ a parancsok észlelésére és javítására. Lehetnek beépítettek (TOML fájlokként szállítva) vagy munkamenet-naplókból bányászottak.

## Beépített képességek

| Képesség | Aktiválódik | Művelet |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` Rust projekten kívül | `cd` hozzáfűzése a legközelebbi `Cargo.toml` könyvtárhoz |
| `git-wrong-dir` | `git *` git tárolón kívül | `cd` hozzáfűzése a legközelebbi `.git` könyvtárhoz |
| `go-wrong-dir` | `go build/test` Go modulon kívül | `cd` hozzáfűzése a legközelebbi `go.mod` könyvtárhoz |
| `make-wrong-dir` | `make` Makefile nélkül az aktuális könyvtárban | `cd` hozzáfűzése a legközelebbi Makefile könyvtárhoz |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` Node projekten kívül | `cd` hozzáfűzése a legközelebbi `package.json` könyvtárhoz |
| `python-wrong-dir` | `python/pytest/pip` Python projekten kívül | `cd` hozzáfűzése a legközelebbi Python projekthez |
| `jj-translate` | `git *` jj-kolokált tárolóban | Átírás egyenértékű `jj` parancsra |
| `asciinema-gif` | `asciinema rec` | Átírás `precc gif` parancsra |

## Képességek listázása

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

## Képesség részleteinek megjelenítése

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

## Képesség exportálása TOML-ba

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

## Képesség szerkesztése

```bash
$ precc skills edit cargo-wrong-dir
```

Ez megnyitja a képesség definícióját a `$EDITOR`-ban. Mentés után a képesség automatikusan újratöltődik.

## Az Advise parancs

A `precc skills advise` elemzi a legutóbbi munkamenetet és új képességeket javasol ismétlődő minták alapján:

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

## Képességek csoportosítása

```bash
$ precc skills cluster
```

Csoportosítja a hasonló bányászott képességeket a redundáns vagy átfedő minták azonosításához.

## Bányászott és beépített képességek

A beépített képességek a PRECC-kel érkeznek és a `skills/builtin/*.toml`-ban vannak definiálva. A leggyakoribb rossz könyvtár hibákat fedik le.

A bányászott képességek a `precc ingest` vagy a `precc-learner` démon által jönnek létre a munkamenet-naplókból. A `~/.local/share/precc/heuristics.db`-ben tárolódnak és specifikusak a munkafolyamatára. Lásd a [Bányászat](mining.md) oldalt a részletekért.
