# Навыки

Навыки — это правила сопоставления паттернов, которые PRECC использует для обнаружения и исправления команд. Они могут быть встроенными (поставляются как файлы TOML) или изученными из логов сессий.

## Встроенные навыки

| Навык | Срабатывает при | Действие |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` вне проекта Rust | Добавить `cd` к ближайшей директории `Cargo.toml` |
| `git-wrong-dir` | `git *` вне git-репозитория | Добавить `cd` к ближайшей директории `.git` |
| `go-wrong-dir` | `go build/test` вне модуля Go | Добавить `cd` к ближайшей директории `go.mod` |
| `make-wrong-dir` | `make` без Makefile в текущей директории | Добавить `cd` к ближайшей директории Makefile |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` вне проекта Node | Добавить `cd` к ближайшей директории `package.json` |
| `python-wrong-dir` | `python/pytest/pip` вне проекта Python | Добавить `cd` к ближайшему проекту Python |
| `jj-translate` | `git *` в jj-colocated репозитории | Переписать в эквивалентную команду `jj` |
| `asciinema-gif` | `asciinema rec` | Переписать в `precc gif` |

## Список навыков

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

## Детали навыка

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

## Экспорт навыка в TOML

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

## Редактирование навыка

```bash
$ precc skills edit cargo-wrong-dir
```

Это открывает определение навыка в вашем `$EDITOR`. После сохранения навык перезагружается автоматически.

## Команда Advise

`precc skills advise` анализирует вашу последнюю сессию и предлагает новые навыки на основе повторяющихся паттернов:

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

## Кластеризация навыков

```bash
$ precc skills cluster
```

Группирует похожие изученные навыки для выявления избыточных или пересекающихся паттернов.

## Изученные vs. встроенные навыки

Встроенные навыки поставляются с PRECC и определены в `skills/builtin/*.toml`. Они покрывают наиболее распространённые ошибки неправильной директории.

Изученные навыки создаются `precc ingest` или демоном `precc-learner` из ваших логов сессий. Они хранятся в `~/.local/share/precc/heuristics.db` и специфичны для вашего рабочего процесса. Подробности см. в [Mining](mining.md).
