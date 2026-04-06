# マイニング

PRECCはClaude Codeのセッションログを解析して失敗-修正パターンを学習します。同じミスを再び見つけると、自動的に修正を適用します。

## セッションログの取り込み

### 単一ファイルの取り込み

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### すべてのログの取り込み

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### 強制再取り込み

すでに取り込まれたファイルを再処理するには：

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## マイニングの仕組み

1. PRECCはセッションJSONLログファイルを読み取ります。
2. 最初のコマンドが失敗し、2番目が修正されたリトライであるコマンドペアを特定します。
3. パターン（何が問題だったか）と修正（Claudeが何を変えたか）を抽出します。
4. パターンは `~/.local/share/precc/history.db` に保存されます。
5. パターンが信頼度の閾値に達すると（複数回確認）、`heuristics.db` のマイニングスキルになります。

### パターンの例

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## precc-learner デーモン

`precc-learner` デーモンはバックグラウンドで実行され、新しいセッションログを自動的に監視します：

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

デーモンはファイルシステム通知（LinuxではinotifyOSではFSEvents）を使用するため、セッション終了時に即座に反応します。

## パターンからスキルへ

マイニングされたパターンは以下の条件を満たすとスキルに昇格します：

- セッション全体で少なくとも3回確認
- 一貫した修正パターン（毎回同じタイプの修正）
- 誤検出なし

スキル候補は以下で確認できます：

```bash
$ precc skills advise
```

スキルの管理の詳細については [Skills](skills.md) を参照してください。

## データストレージ

- **失敗-修正ペア**: `~/.local/share/precc/history.db`
- **昇格したスキル**: `~/.local/share/precc/heuristics.db`

どちらもWALモードのSQLiteデータベースで、安全な並行アクセスが可能です。
