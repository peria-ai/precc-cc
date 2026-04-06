# 技能

技能是PRECC用来检测和纠正命令的模式匹配规则。它们可以是内置的（作为TOML文件分发）或从会话日志中挖掘的。

## 内置技能

| 技能 | 触发条件 | 动作 |
|-------|-------------|--------|
| `cargo-wrong-dir` | 在Rust项目外运行 `cargo build/test/clippy` | 在命令前添加 `cd` 到最近的 `Cargo.toml` 目录 |
| `git-wrong-dir` | 在git仓库外运行 `git *` | 在命令前添加 `cd` 到最近的 `.git` 目录 |
| `go-wrong-dir` | 在Go模块外运行 `go build/test` | 在命令前添加 `cd` 到最近的 `go.mod` 目录 |
| `make-wrong-dir` | 当前目录没有Makefile时运行 `make` | 在命令前添加 `cd` 到最近的Makefile目录 |
| `npm-wrong-dir` | 在Node项目外运行 `npm/npx/pnpm/yarn` | 在命令前添加 `cd` 到最近的 `package.json` 目录 |
| `python-wrong-dir` | 在Python项目外运行 `python/pytest/pip` | 在命令前添加 `cd` 到最近的Python项目 |
| `jj-translate` | 在jj共存仓库中运行 `git *` | 重写为等效的 `jj` 命令 |
| `asciinema-gif` | `asciinema rec` | 重写为 `precc gif` |

## 列出技能

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

## 显示技能详情

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

## 将技能导出为TOML

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

## 编辑技能

```bash
$ precc skills edit cargo-wrong-dir
```

这将在您的 `$EDITOR` 中打开技能定义。保存后，技能会自动重新加载。

## Advise 命令

`precc skills advise` 分析您最近的会话，并根据重复模式建议新技能：

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

## 聚类技能

```bash
$ precc skills cluster
```

将相似的挖掘技能分组，帮助识别冗余或重叠的模式。

## 挖掘技能与内置技能

内置技能随PRECC一起分发，定义在 `skills/builtin/*.toml` 中。它们涵盖了最常见的目录错误。

挖掘技能由 `precc ingest` 或 `precc-learner` 守护进程从您的会话日志创建。它们存储在 `~/.local/share/precc/heuristics.db` 中，特定于您的工作流程。详情请参阅[挖掘](mining.md)。
