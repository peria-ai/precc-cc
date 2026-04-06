# はじめに

## PRECCとは？

PRECC (Claude Codeの予測的エラー修正) は、公式のPreToolUseフックメカニズムを通じてClaude Codeのbashコマンドを傍受するRustツールです。エラーを*発生する前に*修正し、トークンを節約してリトライループを排除します。

コミュニティユーザーは無料。

## 問題

Claude Codeは防げるミスで大量のトークンを浪費します：

- **ディレクトリエラー** -- `Cargo.toml`のない親ディレクトリで`cargo build`を実行し、エラーを読んでからリトライ。
- **リトライループ** -- 失敗したコマンドが冗長な出力を生成し、Claudeがそれを読み、推論し、リトライ。
- **冗長な出力** -- `find`や`ls -R`のようなコマンドが数千行を出力。

## 4つの柱

### コンテキスト修正 (cd-prepend)

`cargo build` や `npm test` などのコマンドが間違ったディレクトリで実行された場合を検出し、実行前に `cd /正しい/パス &&` を追加します。

### GDBデバッグ

セグフォルトやクラッシュの詳細なデバッグのためにGDBをアタッチする機会を検出します。

### セッションマイニング

Claude Codeのセッションログを分析して失敗-修正ペアを見つけます。同じミスが再発するとPRECCは自動的に修正を適用します。

### 自動化スキル

コマンドパターンにマッチして書き換えるスキルのライブラリ。TOMLファイルまたはSQLite行として定義されます。

## 仕組み（30秒バージョン）

1. Claude Codeがbashコマンドを実行しようとしています。
2. PreToolUseフックがコマンドをJSON形式で`precc-hook`に送信します。
3. `precc-hook`がパイプラインを通じて3ミリ秒未満でコマンドを処理します。
4. 修正されたコマンドがJSON形式で返されます。
5. Claude Codeが修正されたコマンドを実行します。

Claudeはエラーを見ることがありません。トークンの無駄はありません。

### 適応圧縮

コマンドが圧縮後に失敗した場合、PRECCは次のリトライで圧縮を自動的にスキップし、Claudeがデバッグ用の完全な非圧縮出力を取得できるようにします。

## リアルタイム使用統計

現在のバージョン <span data-stat="current_version">--</span>:

| 指標 | 値 |
|---|---|
| フック呼び出し | <span data-stat="total_invocations">--</span> |
| 節約されたトークン | <span data-stat="total_tokens_saved">--</span> |
| 節約率 | <span data-stat="saving_pct">--</span>% |
| RTK書き換え | <span data-stat="rtk_rewrites">--</span> |
| CD修正 | <span data-stat="cd_prepends">--</span> |
| フックレイテンシ | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| ユニークユーザー | <span data-stat="unique_users">--</span> |

### リリースごとの節約

<table id="version-breakdown" style="display:none">
<thead><tr><th>バージョン</th><th>ユニークユーザー</th><th>フック呼び出し</th><th>節約されたトークン</th><th>節約率</th></tr></thead>
<tbody><tr><td colspan="5"><em>読み込み中...</em></td></tr></tbody>
</table>

<small>数値は推定値です。各予防された失敗は、エラー出力、モデルの推論、リトライコマンドの完全なリトライサイクルを回避します。 これらの数値は匿名化されたテレメトリから自動的に更新されます。</small>

## リンク

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- ウェブサイト: [https://peria.ai](https://peria.ai)
- ドキュメント: [https://precc.cc](https://precc.cc)
