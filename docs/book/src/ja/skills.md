# スキル

スキルは、PRECCがコマンドを検出して修正するために使用するパターンマッチングルールです。組み込み（TOMLファイルとして配布）またはセッションログからマイニングされます。

## 組み込みスキル

| スキル | トリガー条件 | アクション |
|-------|-------------|--------|
| `cargo-wrong-dir` | Rustプロジェクト外での `cargo build/test/clippy` | 最も近い `Cargo.toml` ディレクトリへの `cd` を先頭に追加 |
| `git-wrong-dir` | gitリポジトリ外での `git *` | 最も近い `.git` ディレクトリへの `cd` を先頭に追加 |
| `go-wrong-dir` | Goモジュール外での `go build/test` | 最も近い `go.mod` ディレクトリへの `cd` を先頭に追加 |
| `make-wrong-dir` | カレントディレクトリにMakefileがない状態での `make` | 最も近いMakefileディレクトリへの `cd` を先頭に追加 |
| `npm-wrong-dir` | Nodeプロジェクト外での `npm/npx/pnpm/yarn` | 最も近い `package.json` ディレクトリへの `cd` を先頭に追加 |
| `python-wrong-dir` | Pythonプロジェクト外での `python/pytest/pip` | 最も近いPythonプロジェクトへの `cd` を先頭に追加 |
| `jj-translate` | jj共存リポジトリでの `git *` | 同等の `jj` コマンドに書き換え |
| `asciinema-gif` | `asciinema rec` | `precc gif` に書き換え |

## スキルの一覧表示

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

## スキルの詳細表示

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

## スキルをTOMLにエクスポート

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

## スキルの編集

```bash
$ precc skills edit cargo-wrong-dir
```

これにより、`$EDITOR` でスキル定義が開きます。保存後、スキルは自動的にリロードされます。

## Advise コマンド

`precc skills advise` は最近のセッションを分析し、繰り返しパターンに基づいて新しいスキルを提案します：

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

## スキルのクラスタリング

```bash
$ precc skills cluster
```

類似するマイニングされたスキルをグループ化し、冗長または重複するパターンの特定を支援します。

## マイニングスキルと組み込みスキル

組み込みスキルはPRECCに同梱され、`skills/builtin/*.toml` で定義されています。最も一般的なディレクトリ間違いをカバーします。

マイニングスキルは `precc ingest` または `precc-learner` デーモンによってセッションログから作成されます。`~/.local/share/precc/heuristics.db` に保存され、ワークフローに固有です。詳細は[マイニング](mining.md)を参照してください。
