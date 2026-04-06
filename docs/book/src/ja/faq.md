# よくある質問

## PRECCは安全ですか？

はい。PRECCはClaude Code公式のPreToolUseフックメカニズムを使用しています。Anthropicがまさにこの目的のために設計した拡張ポイントです。フックは：

- 完全にオフラインで動作（ホットパスでのネットワーク呼び出しなし）
- 5ミリ秒未満で完了
- フェイルオープン：問題が発生した場合、元のコマンドがそのまま実行される
- コマンドを変更するだけで、自ら実行することはない
- データはローカルのSQLiteデータベースに保存

## PRECCは他のAIコーディングツールで動作しますか？

PRECCはClaude Code専用に設計されています。Claude Codeが提供するPreToolUseフックプロトコルに依存しています。Cursor、Copilot、Windsurf、その他のAIコーディングツールでは動作しません。

## テレメトリはどのようなデータを送信しますか？

テレメトリはオプトインのみです。有効にすると送信されるもの：

- PRECCバージョン、OS、アーキテクチャ
- 集計カウント（インターセプトされたコマンド、アクティブ化されたスキル）
- 平均フックレイテンシ

コマンドテキスト、ファイルパス、プロジェクト名、個人を特定できる情報は送信**しません**。オプトイン前に `precc telemetry preview` で正確なペイロードを確認できます。詳細は[テレメトリ](telemetry.md)を参照。

## PRECCをアンインストールするには？

??faq_uninstall_a_intro??

1. フック登録を削除：
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. バイナリを削除：
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. データを削除（任意）：
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## ライセンスが期限切れになりました。どうなりますか？

PRECCはCommunityティアに戻ります。すべてのコア機能は引き続き動作します：

- 組み込みスキルはアクティブのまま
- フックパイプラインは正常に動作
- `precc savings` はサマリービューを表示
- `precc ingest` とセッションマイニングは動作

Pro機能は更新まで利用できなくなります：

- `precc savings --all`（詳細な内訳）
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- メールレポート

## フックが動作していないようです。どうやってデバッグしますか？

??faq_debug_a_intro??

1. フックが登録されていることを確認：
   ```bash
   precc init
   ```

2. フックを手動でテスト：
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. バイナリがPATHにあることを確認：
   ```bash
   which precc-hook
   ```

4. `~/.claude/settings.json` のClaude Codeフック設定を確認。

## PRECCはClaude Codeを遅くしますか？

いいえ。フックは5ミリ秒未満（p99）で完了します。Claudeが推論と応答生成に費やす時間と比べると知覚できません。

## CI/CDでPRECCを使用できますか？

PRECCはインタラクティブなClaude Codeセッション向けに設計されています。CI/CDでは、フックするClaude Codeインスタンスがありません。ただし、`precc gha` はどの環境からでも失敗したGitHub Actionsの実行を分析できます。

## マイニングされたスキルと組み込みスキルはどう違いますか？

組み込みスキルはPRECCに同梱され、一般的な間違ったディレクトリパターンをカバーします。マイニングされたスキルはあなたの特定のセッションログから学習されます。両方ともSQLiteに保存され、フックパイプラインで同一に評価されます。

## スキルをチームと共有できますか？

はい。`precc skills export NAME` でスキルをTOMLにエクスポートしてファイルを共有できます。チームメンバーは `skills/` ディレクトリに配置するか、ヒューリスティクスデータベースにインポートできます。
