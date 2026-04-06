# ライセンス

PRECCは2つのティアを提供します：Community（無料）とPro。

## Communityティア（無料）

Communityティアには以下が含まれます：

- すべての組み込みスキル（ディレクトリ修正、jj変換など）
- Pillar 1とPillar 4を完全サポートするフックパイプライン
- 基本的な `precc savings` サマリー
- `precc ingest` によるセッションマイニング
- 無制限のローカル使用

## Proティア

Proは追加機能をアンロックします：

- **詳細な節約内訳** -- `precc savings --all` コマンドごとの分析付き
- **GIF録画** -- `precc gif` アニメーションターミナルGIFの作成用
- **IPジオフェンスコンプライアンス** -- 規制環境向け
- **メールレポート** -- `precc mail report` アナリティクスを送信
- **GitHub Actions分析** -- `precc gha` 失敗したワークフローのデバッグ用
- **コンテキスト圧縮** -- `precc compress` CLAUDE.mdの最適化用
- **優先サポート**

## ライセンスの有効化

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## ライセンスステータスの確認

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## GitHub Sponsorsアクティベーション

GitHub SponsorsでPRECCをスポンサーしている場合、GitHubメールを通じて自動的にライセンスが有効化されます。キーは不要です。スポンサーメールが一致していることを確認してください：

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## デバイスフィンガープリント

各ライセンスはデバイスフィンガープリントに紐付けられています。以下で確認できます：

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

ライセンスを新しいマシンに移行する必要がある場合は、まず無効化してください：

```bash
precc license deactivate
```

その後、新しいマシンで有効化します。

## ライセンスの有効期限切れ？

Proライセンスの有効期限が切れると、PRECCはCommunityティアに戻ります。すべての組み込みスキルとコア機能は引き続き動作します。Pro固有の機能のみ利用できなくなります。詳細は[FAQ](faq.md)をご覧ください。
