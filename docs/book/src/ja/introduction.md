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

### 実測節約（実データ）

<div id="measured-savings" style="display:none">
<table id="measured-summary">
<thead><tr><th>指標</th><th>値</th></tr></thead>
<tbody>
<tr><td>元の出力トークン（PRECCなし）</td><td><span data-measured="original_output_tokens">--</span></td></tr>
<tr><td>実際の出力トークン（PRECCあり）</td><td><span data-measured="actual_output_tokens">--</span></td></tr>
<tr><td>節約されたトークン</td><td><strong><span data-measured="savings_tokens">--</span></strong></td></tr>
<tr><td>節約率</td><td><strong><span data-measured="savings_pct">--</span>%</strong></td></tr>
<tr><td>実測データ</td><td><span data-measured="ground_truth_count">--</span> 回の測定</td></tr>
</tbody>
</table>
</div>

<div id="rewrite-type-breakdown" style="display:none">

#### 書き換えタイプ別

<table id="rewrite-type-table">
<thead><tr><th>タイプ</th><th>回数</th><th>平均節約%</th><th>節約されたトークン</th></tr></thead>
<tbody><tr><td colspan="4"><em>読み込み中...</em></td></tr></tbody>
</table>
</div>

### リリースごとの節約

<table id="version-breakdown" style="display:none">
<thead><tr><th>バージョン</th><th>ユニークユーザー</th><th>フック呼び出し</th><th>節約されたトークン</th><th>節約率</th></tr></thead>
<tbody><tr><td colspan="5"><em>読み込み中...</em></td></tr></tbody>
</table>

<small>これらの数値は匿名化されたテレメトリから自動的に更新されます。</small>

## リンク

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- ウェブサイト: [https://peria.ai](https://peria.ai)
- ドキュメント: [https://precc.cc](https://precc.cc)
