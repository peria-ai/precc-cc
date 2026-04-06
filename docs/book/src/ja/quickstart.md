# クイックスタート

5分でPRECCを起動しましょう。

## ステップ1：インストール

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## ステップ2：初期化

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## ステップ3：フックがアクティブか確認

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
```

## ステップ4：Claude Codeを通常通り使用

Claude Codeを開いて通常通り作業してください。PRECCはバックグラウンドで静かに動作します。Claudeが失敗するコマンドを発行すると、PRECCが実行前に修正します。

### 例：間違ったディレクトリでのCargo Build

プロジェクトが `~/projects/myapp/` にあり、Claudeが以下を実行するとします：

```
cargo build
```

`~/projects/` から（1階層上で、そこに `Cargo.toml` はありません）。

**PRECCなし：** Claudeはエラー `could not find Cargo.toml in /home/user/projects or any parent directory` を受け取り、読んで推論し、`cd myapp && cargo build` で再試行します。コスト：約2,000トークンの無駄。

**PRECCあり：** フックが `Cargo.toml` の欠如を検出し、`myapp/` で見つけ、コマンドを以下に書き換えます：

```
cd /home/user/projects/myapp && cargo build
```

Claudeはエラーを見ることがありません。トークンの無駄はゼロ。

## ステップ5：節約を確認

セッション後、PRECCがどれだけのトークンを節約したか確認してください：

```bash
$ precc savings
Session Token Savings
=====================
Total estimated savings: 4,312 tokens

Breakdown:
  Pillar 1 (cd prepends):       2,104 tokens  (3 corrections)
  Pillar 4 (skill activations):   980 tokens  (2 activations)
  RTK rewrites:                 1,228 tokens  (5 rewrites)
```

## 次のステップ

- [スキル](skills.md) -- 利用可能な全スキルと独自スキルの作成方法。
- [フックパイプライン](hook-pipeline.md) -- フードの下で何が起きているかを理解する。
- [節約](savings.md) -- トークン節約の詳細分析。
