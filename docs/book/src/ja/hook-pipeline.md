# フックパイプライン

`precc-hook` バイナリはPRECCの中核です。Claude Codeとシェルの間に位置し、すべてのbashコマンドを5ミリ秒以内に処理します。

## Claude Codeがフックを呼び出す方法

Claude CodeはPreToolUseフックをサポートしています。これは実行前にツール入力を検査・変更できる外部プログラムです。Claudeがbashコマンドを実行しようとすると、stdinで `precc-hook` にJSONを送信し、stdoutからレスポンスを読み取ります。

## パイプラインステージ

```
Claude Code
    |
    v
+---------------------------+
| 1. Parse JSON stdin       |  Read the command from Claude Code
+---------------------------+
    |
    v
+---------------------------+
| 2. Skill matching         |  Query heuristics.db for matching skills (Pillar 4)
+---------------------------+
    |
    v
+---------------------------+
| 3. Directory correction   |  Resolve correct working directory (Pillar 1)
+---------------------------+
    |
    v
+---------------------------+
| 4. GDB check              |  Detect debug opportunities (Pillar 2)
+---------------------------+
    |
    v
+---------------------------+
| 5. RTK rewriting          |  Apply command rewrites for token savings
+---------------------------+
    |
    v
+---------------------------+
| 6. Emit JSON stdout       |  Return modified command to Claude Code
+---------------------------+
    |
    v
  Shell executes corrected command
```

## 例：JSONの入出力

### 入力（Claude Codeから）

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECCは現在のディレクトリに `Cargo.toml` がないが、`./myapp/Cargo.toml` が存在することを検出します。

### 出力（Claude Codeへ）

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

変更が不要な場合、`updatedInput.command` は空となり、Claude Codeは元のコマンドを使用します。

## ステージ詳細

### ステージ1：JSON解析

stdinから完全なJSONオブジェクトを読み取ります。`tool_input.command` を抽出します。パースに失敗した場合、フックは即座に終了し、Claude Codeは元のコマンドを使用します（フェイルオープン設計）。

### ステージ2：スキルマッチング

SQLiteヒューリスティクスデータベースに、トリガーパターンがコマンドに一致するスキルを問い合わせます。スキルは優先度順にチェックされます。組み込みTOMLスキルとマイニングされたスキルの両方が評価されます。

### ステージ3：ディレクトリ修正

ビルドコマンド（`cargo`、`go`、`make`、`npm`、`python` など）に対して、期待されるプロジェクトファイルが現在のディレクトリに存在するかチェックします。存在しない場合、近くのディレクトリをスキャンして最も近い一致を見つけ、`cd <dir> &&` を先頭に追加します。

ディレクトリスキャンは5秒のTTLを持つキャッシュされたファイルシステムインデックスを使用して高速を維持します。

### ステージ4：GDBチェック

コマンドがクラッシュを引き起こす可能性がある場合（例：デバッグバイナリの実行）、PRECCはGDBラッパーを提案または注入して、生のクラッシュログの代わりに構造化されたデバッグ出力をキャプチャできます。

### ステージ5：RTK書き換え

RTK（Rewrite Toolkit）ルールを適用して、冗長なコマンドを短縮し、ノイズの多い出力を抑制し、トークン効率のためにコマンドを再構成します。

### ステージ6：JSON出力

変更されたコマンドをJSONにシリアライズしてstdoutに書き込みます。変更がなかった場合、出力はClaude Codeに元のコマンドを使用するよう信号を送ります。

## パフォーマンス

パイプライン全体が5ミリ秒（p99）以内に完了します。主な最適化：

- ロックフリーの並行読み取りのためのSQLite WALモード
- スキルマッチングのためのプリコンパイル済み正規表現パターン
- キャッシュされたファイルシステムスキャン（5秒TTL）
- ホットパスにネットワーク呼び出しなし
- フェイルオープン：エラーが発生しても元のコマンドにフォールスルー

## フックの手動テスト

フックを直接呼び出すことができます：

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
