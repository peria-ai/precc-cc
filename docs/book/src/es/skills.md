# Habilidades

Las habilidades son las reglas de coincidencia de patrones que PRECC usa para detectar y corregir comandos. Pueden ser integradas (distribuidas como archivos TOML) o extraídas de registros de sesión.

## Habilidades integradas

| Habilidad | Se activa con | Acción |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` fuera de un proyecto Rust | Anteponer `cd` al directorio del `Cargo.toml` más cercano |
| `git-wrong-dir` | `git *` fuera de un repositorio git | Anteponer `cd` al directorio `.git` más cercano |
| `go-wrong-dir` | `go build/test` fuera de un módulo Go | Anteponer `cd` al directorio del `go.mod` más cercano |
| `make-wrong-dir` | `make` sin un Makefile en el directorio actual | Anteponer `cd` al directorio del Makefile más cercano |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` fuera de un proyecto Node | Anteponer `cd` al directorio del `package.json` más cercano |
| `python-wrong-dir` | `python/pytest/pip` fuera de un proyecto Python | Anteponer `cd` al proyecto Python más cercano |
| `jj-translate` | `git *` en un repositorio jj-colocado | Reescribir al comando `jj` equivalente |
| `asciinema-gif` | `asciinema rec` | Reescribir a `precc gif` |

## Listar habilidades

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

## Mostrar detalles de la habilidad

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

## Exportar una habilidad a TOML

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

## Editar una habilidad

```bash
$ precc skills edit cargo-wrong-dir
```

Esto abre la definición de la habilidad en tu `$EDITOR`. Después de guardar, la habilidad se recarga automáticamente.

## El comando Advise

`precc skills advise` analiza tu sesión reciente y sugiere nuevas habilidades basadas en patrones repetidos:

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

## Agrupar habilidades

```bash
$ precc skills cluster
```

Agrupa habilidades extraídas similares para ayudar a identificar patrones redundantes o superpuestos.

## Habilidades extraídas vs. integradas

Las habilidades integradas se distribuyen con PRECC y están definidas en `skills/builtin/*.toml`. Cubren los errores de directorio incorrecto más comunes.

Las habilidades extraídas son creadas por `precc ingest` o el demonio `precc-learner` a partir de tus registros de sesión. Se almacenan en `~/.local/share/precc/heuristics.db` y son específicas de tu flujo de trabajo. Ver [Minería](mining.md) para más detalles.
