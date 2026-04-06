# 技能

技能是PRECC用來檢測和糾正命令的模式匹配規則。它們可以是內置的（作爲TOML文件分發）或從會話日誌中挖掘的。

## 內置技能

| 技能 | 觸發條件 | 動作 |
|-------|-------------|--------|
| `cargo-wrong-dir` | 在Rust項目外運行 `cargo build/test/clippy` | 在命令前添加 `cd` 到最近的 `Cargo.toml` 目錄 |
| `git-wrong-dir` | 在git倉庫外運行 `git *` | 在命令前添加 `cd` 到最近的 `.git` 目錄 |
| `go-wrong-dir` | 在Go模塊外運行 `go build/test` | 在命令前添加 `cd` 到最近的 `go.mod` 目錄 |
| `make-wrong-dir` | 當前目錄沒有Makefile時運行 `make` | 在命令前添加 `cd` 到最近的Makefile目錄 |
| `npm-wrong-dir` | 在Node項目外運行 `npm/npx/pnpm/yarn` | 在命令前添加 `cd` 到最近的 `package.json` 目錄 |
| `python-wrong-dir` | 在Python項目外運行 `python/pytest/pip` | 在命令前添加 `cd` 到最近的Python項目 |
| `jj-translate` | 在jj共存倉庫中運行 `git *` | 重寫爲等效的 `jj` 命令 |
| `asciinema-gif` | `asciinema rec` | 重寫爲 `precc gif` |

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

## 顯示技能詳情

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

## 將技能導出爲TOML

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

## 編輯技能

```bash
$ precc skills edit cargo-wrong-dir
```

這將在您的 `$EDITOR` 中打開技能定義。保存後，技能會自動重新加載。

## Advise 命令

`precc skills advise` 分析您最近的會話，並根據重複模式建議新技能：

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

## 聚類技能

```bash
$ precc skills cluster
```

將相似的挖掘技能分組，幫助識別冗餘或重疊的模式。

## 挖掘技能與內置技能

內置技能隨PRECC一起分發，定義在 `skills/builtin/*.toml` 中。它們涵蓋了最常見的目錄錯誤。

挖掘技能由 `precc ingest` 或 `precc-learner` 守護進程從您的會話日誌創建。它們存儲在 `~/.local/share/precc/heuristics.db` 中，特定於您的工作流程。詳情請參閱[挖掘](mining.md)。
