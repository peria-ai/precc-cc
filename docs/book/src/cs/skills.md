# Dovednosti

Dovednosti jsou pravidla porovnávání vzorů, která PRECC používá k detekci a opravě příkazů. Mohou být vestavěné (dodávané jako TOML soubory) nebo naučené z logů relací.

## Vestavěné dovednosti

| Dovednost | Spouští při | Akce |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` mimo projekt Rust | Předřadit `cd` k nejbližšímu adresáři `Cargo.toml` |
| `git-wrong-dir` | `git *` mimo git repozitář | Předřadit `cd` k nejbližšímu adresáři `.git` |
| `go-wrong-dir` | `go build/test` mimo Go modul | Předřadit `cd` k nejbližšímu adresáři `go.mod` |
| `make-wrong-dir` | `make` bez Makefile v cwd | Předřadit `cd` k nejbližšímu adresáři Makefile |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` mimo Node projekt | Předřadit `cd` k nejbližšímu adresáři `package.json` |
| `python-wrong-dir` | `python/pytest/pip` mimo Python projekt | Předřadit `cd` k nejbližšímu Python projektu |
| `jj-translate` | `git *` v jj-colocated repozitáři | Přepsat na ekvivalentní příkaz `jj` |
| `asciinema-gif` | `asciinema rec` | Přepsat na `precc gif` |

## Seznam dovedností

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

## Detail dovednosti

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

## Export dovednosti do TOML

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

## Úprava dovednosti

```bash
$ precc skills edit cargo-wrong-dir
```

Toto otevře definici dovednosti ve vašem `$EDITOR`. Po uložení je dovednost automaticky znovu načtena.

## Příkaz Advise

`precc skills advise` analyzuje vaši nedávnou relaci a navrhuje nové dovednosti na základě opakujících se vzorů:

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

## Seskupování dovedností

```bash
$ precc skills cluster
```

Seskupuje podobné naučené dovednosti pro identifikaci redundantních nebo překrývajících se vzorů.

## Naučené vs. vestavěné dovednosti

Vestavěné dovednosti jsou dodávány s PRECC a definovány v `skills/builtin/*.toml`. Pokrývají nejběžnější chyby špatného adresáře.

Naučené dovednosti jsou vytvořeny `precc ingest` nebo démonem `precc-learner` z vašich logů relací. Jsou uloženy v `~/.local/share/precc/heuristics.db` a jsou specifické pro váš pracovní postup. Viz [Mining](mining.md) pro podrobnosti.
