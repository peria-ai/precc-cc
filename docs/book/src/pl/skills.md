# Umiejętności

Umiejętności to reguły dopasowywania wzorców, których PRECC używa do wykrywania i poprawiania poleceń. Mogą być wbudowane (dostarczane jako pliki TOML) lub wydobyte z logów sesji.

## Wbudowane umiejętności

| Umiejętność | Wyzwalacz | Akcja |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` poza projektem Rust | Dodaj `cd` do najbliższego katalogu `Cargo.toml` |
| `git-wrong-dir` | `git *` poza repozytorium git | Dodaj `cd` do najbliższego katalogu `.git` |
| `go-wrong-dir` | `go build/test` poza modułem Go | Dodaj `cd` do najbliższego katalogu `go.mod` |
| `make-wrong-dir` | `make` bez Makefile w bieżącym katalogu | Dodaj `cd` do najbliższego katalogu Makefile |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` poza projektem Node | Dodaj `cd` do najbliższego katalogu `package.json` |
| `python-wrong-dir` | `python/pytest/pip` poza projektem Python | Dodaj `cd` do najbliższego projektu Python |
| `jj-translate` | `git *` w repozytorium jj-colocated | Przepisz na równoważne polecenie `jj` |
| `asciinema-gif` | `asciinema rec` | Przepisz na `precc gif` |

## Listowanie umiejętności

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

## Wyświetlanie szczegółów umiejętności

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

## Eksportowanie umiejętności do TOML

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

## Edycja umiejętności

```bash
$ precc skills edit cargo-wrong-dir
```

Otwiera definicję umiejętności w twoim `$EDITOR`. Po zapisaniu umiejętność jest automatycznie przeładowywana.

## Polecenie Advise

`precc skills advise` analizuje ostatnią sesję i sugeruje nowe umiejętności na podstawie powtarzających się wzorców:

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

## Grupowanie umiejętności

```bash
$ precc skills cluster
```

Grupuje podobne wydobyte umiejętności w celu identyfikacji redundantnych lub nakładających się wzorców.

## Wydobyte a wbudowane umiejętności

Wbudowane umiejętności są dostarczane z PRECC i zdefiniowane w `skills/builtin/*.toml`. Obejmują najczęstsze błędy niewłaściwego katalogu.

Wydobyte umiejętności są tworzone przez `precc ingest` lub daemon `precc-learner` z logów sesji. Są przechowywane w `~/.local/share/precc/heuristics.db` i są specyficzne dla twojego przepływu pracy. Zobacz [Wydobywanie](mining.md) po szczegóły.
