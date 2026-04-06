# テレメトリ

PRECCはツールの改善に役立つオプトイン方式の匿名テレメトリをサポートしています。明示的に同意しない限り、データは収集されません。

## オプトイン

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## オプトアウト

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## ステータス確認

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## 送信されるデータのプレビュー

オプトインする前に、収集されるデータを正確に確認できます：

```bash
$ precc telemetry preview
Telemetry payload (this session):
{
  "version": "0.3.0",
  "os": "linux",
  "arch": "x86_64",
  "skills_activated": 12,
  "commands_intercepted": 87,
  "pillars_used": [1, 4],
  "avg_hook_latency_ms": 2.3,
  "session_count": 1
}
```

## 収集されるもの

- PRECCバージョン、OS、アーキテクチャ
- 集計カウント：インターセプトされたコマンド、アクティブ化されたスキル、使用された柱
- 平均フックレイテンシ
- セッション数

## 収集されないもの

- コマンドテキストや引数なし
- ファイルパスやディレクトリ名なし
- プロジェクト名やリポジトリURLなし
- 個人を特定できる情報（PII）なし
- IPアドレスなし（サーバーは記録しません）

## 環境変数によるオーバーライド

コマンドを実行せずにテレメトリを無効にするには（CIや共有環境で便利）：

```bash
export PRECC_NO_TELEMETRY=1
```

これは同意設定よりも優先されます。

## データの送信先

テレメトリデータはHTTPSで `https://telemetry.peria.ai/v1/precc` に送信されます。データは使用パターンの理解と開発の優先順位付けにのみ使用されます。
