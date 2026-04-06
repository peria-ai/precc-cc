# Habilidades

Habilidades são as regras de correspondência de padrões que PRECC usa para detectar e corrigir comandos. Podem ser integradas (distribuídas como arquivos TOML) ou extraídas de logs de sessão.

## Habilidades integradas

| Habilidade | Acionado por | Ação |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` fora de um projeto Rust | Adicionar `cd` para o diretório `Cargo.toml` mais próximo |
| `git-wrong-dir` | `git *` fora de um repositório git | Adicionar `cd` para o diretório `.git` mais próximo |
| `go-wrong-dir` | `go build/test` fora de um módulo Go | Adicionar `cd` para o diretório `go.mod` mais próximo |
| `make-wrong-dir` | `make` sem Makefile no diretório atual | Adicionar `cd` para o diretório Makefile mais próximo |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` fora de um projeto Node | Adicionar `cd` para o diretório `package.json` mais próximo |
| `python-wrong-dir` | `python/pytest/pip` fora de um projeto Python | Adicionar `cd` para o projeto Python mais próximo |
| `jj-translate` | `git *` em um repositório jj co-localizado | Reescrever para o comando `jj` equivalente |
| `asciinema-gif` | `asciinema rec` | Reescrever para `precc gif` |

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

## Mostrar detalhes da habilidade

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

## Exportar uma habilidade para TOML

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

## Editar uma habilidade

```bash
$ precc skills edit cargo-wrong-dir
```

Isso abre a definição da habilidade no seu `$EDITOR`. Após salvar, a habilidade é recarregada automaticamente.

## O comando Advise

`precc skills advise` analisa sua sessão recente e sugere novas habilidades baseadas em padrões repetidos:

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

Agrupa habilidades mineradas semelhantes para ajudar a identificar padrões redundantes ou sobrepostos.

## Habilidades mineradas vs. integradas

Habilidades integradas são distribuídas com PRECC e definidas em `skills/builtin/*.toml`. Elas cobrem os erros de diretório incorreto mais comuns.

Habilidades mineradas são criadas por `precc ingest` ou pelo daemon `precc-learner` a partir dos seus logs de sessão. São armazenadas em `~/.local/share/precc/heuristics.db` e são específicas do seu fluxo de trabalho. Veja [Mineração](mining.md) para detalhes.
