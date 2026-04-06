# 挖掘

PRECC挖掘Claude Code会话日志以学习失败-修复模式。当它再次看到同样的错误时，会自动应用修复。

## 导入会话日志

### 导入单个文件

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### 导入所有日志

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### 强制重新导入

要重新处理已导入的文件：

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## 挖掘的工作原理

1. PRECC读取会话JSONL日志文件。
2. 它识别命令对，其中第一个命令失败，第二个是纠正后的重试。
3. 它提取模式（出了什么问题）和修复（Claude做了什么不同的事）。
4. 模式存储在 `~/.local/share/precc/history.db` 中。
5. 当模式达到置信阈值（多次出现）时，它成为 `heuristics.db` 中的挖掘技能。

### 示例模式

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## precc-learner 守护进程

`precc-learner` 守护进程在后台运行，自动监视新的会话日志：

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

守护进程使用文件系统通知（Linux上的inotify，macOS上的FSEvents），因此在会话结束时立即做出反应。

## 从模式到技能

挖掘的模式在满足以下条件时升级为技能：

- 跨会话至少出现3次
- 一致的修复模式（每次相同类型的纠正）
- 未检测到误报

您可以通过以下方式查看技能候选：

```bash
$ precc skills advise
```

有关管理技能的详细信息，请参见 [Skills](skills.md)。

## 数据存储

- **失败-修复对**: `~/.local/share/precc/history.db`
- **升级的技能**: `~/.local/share/precc/heuristics.db`

两者都是WAL模式的SQLite数据库，用于安全的并发访问。
