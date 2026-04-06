# 挖掘

PRECC挖掘Claude Code會話日誌以學習失敗-修復模式。當它再次看到同樣的錯誤時，會自動應用修復。

## 導入會話日誌

### 導入單個文件

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### 導入所有日誌

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### 強制重新導入

要重新處理已導入的文件：

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## 挖掘的工作原理

1. PRECC讀取會話JSONL日誌文件。
2. 它識別命令對，其中第一個命令失敗，第二個是糾正後的重試。
3. 它提取模式（出了什麼問題）和修復（Claude做了什麼不同的事）。
4. 模式存儲在 `~/.local/share/precc/history.db` 中。
5. 當模式達到置信閾值（多次出現）時，它成爲 `heuristics.db` 中的挖掘技能。

### 示例模式

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## precc-learner 守護進程

`precc-learner` 守護進程在後臺運行，自動監視新的會話日誌：

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

守護進程使用文件系統通知（Linux上的inotify，macOS上的FSEvents），因此在會話結束時立即做出反應。

## 從模式到技能

挖掘的模式在滿足以下條件時升級爲技能：

- 跨會話至少出現3次
- 一致的修復模式（每次相同類型的糾正）
- 未檢測到誤報

您可以通過以下方式查看技能候選：

```bash
$ precc skills advise
```

有關管理技能的詳細信息，請參見 [Skills](skills.md)。

## 數據存儲

- **失敗-修復對**: `~/.local/share/precc/history.db`
- **升級的技能**: `~/.local/share/precc/heuristics.db`

兩者都是WAL模式的SQLite數據庫，用於安全的併發訪問。
