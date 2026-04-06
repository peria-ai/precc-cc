# Mining

PRECC mines Claude Code session logs to learn failure-fix patterns. When it sees the same mistake again, it applies the fix automatically.

## Ingesting Session Logs

### Ingest a Single File

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### Ingest All Logs

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### Force Re-ingest

To re-process files that were already ingested:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## How Mining Works

1. PRECC reads the session JSONL log file.
2. It identifies command pairs where the first command failed and the second was a corrected retry.
3. It extracts the pattern (what went wrong) and the fix (what Claude did differently).
4. Patterns are stored in `~/.local/share/precc/history.db`.
5. When a pattern reaches a confidence threshold (seen multiple times), it becomes a mined skill in `heuristics.db`.

### Example Pattern

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## The precc-learner Daemon

The `precc-learner` daemon runs in the background and watches for new session logs automatically:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

The daemon uses file system notifications (inotify on Linux, FSEvents on macOS) so it reacts immediately when a session ends.

## From Patterns to Skills

Mined patterns graduate to skills when they meet these criteria:

- Seen at least 3 times across sessions
- Consistent fix pattern (same type of correction each time)
- No false positives detected

You can review skill candidates with:

```bash
$ precc skills advise
```

See [Skills](skills.md) for details on managing skills.

## Data Storage

- **Failure-fix pairs**: `~/.local/share/precc/history.db`
- **Graduated skills**: `~/.local/share/precc/heuristics.db`

Both are SQLite databases in WAL mode for safe concurrent access.
