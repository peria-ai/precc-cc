# {{i18n:min_title}}

{{i18n:min_intro}}

## {{i18n:min_ingest_title}}

### {{i18n:min_ingest_single_title}}

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### {{i18n:min_ingest_all_title}}

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### {{i18n:min_force_title}}

{{i18n:min_force_body}}

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## {{i18n:min_how_title}}

{{i18n:min_how_body}}

### {{i18n:min_example_title}}

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## {{i18n:min_daemon_title}}

{{i18n:min_daemon_intro}}

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

{{i18n:min_daemon_notify}}

## {{i18n:min_graduation_title}}

{{i18n:min_graduation_body}}

{{i18n:min_graduation_review}}

```bash
$ precc skills advise
```

{{i18n:min_graduation_see_skills}}

## {{i18n:min_storage_title}}

- **{{i18n:min_storage_pairs}}**: `~/.local/share/precc/history.db`
- **{{i18n:min_storage_skills}}**: `~/.local/share/precc/heuristics.db`

{{i18n:min_storage_note}}
