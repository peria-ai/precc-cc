# 마이닝

PRECC는 Claude Code 세션 로그를 마이닝하여 실패-수정 패턴을 학습합니다. 같은 실수를 다시 발견하면 자동으로 수정을 적용합니다.

## 세션 로그 수집

### 단일 파일 수집

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### 모든 로그 수집

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### 강제 재수집

이미 수집된 파일을 재처리하려면:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## 마이닝 작동 방식

1. PRECC가 세션 JSONL 로그 파일을 읽습니다.
2. 첫 번째 명령이 실패하고 두 번째가 수정된 재시도인 명령 쌍을 식별합니다.
3. 패턴(무엇이 잘못되었는지)과 수정(Claude가 무엇을 다르게 했는지)을 추출합니다.
4. 패턴은 `~/.local/share/precc/history.db`에 저장됩니다.
5. 패턴이 신뢰도 임계값에 도달하면 `heuristics.db`의 마이닝 스킬이 됩니다.

### 패턴 예시

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## precc-learner 데몬

`precc-learner` 데몬은 백그라운드에서 실행되며 새 세션 로그를 자동으로 감시합니다:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

데몬은 파일 시스템 알림(Linux의 inotify, macOS의 FSEvents)을 사용하여 세션이 끝나면 즉시 반응합니다.

## 패턴에서 스킬로

마이닝된 패턴은 다음 기준을 충족하면 스킬로 승격됩니다:

- 세션 전체에서 최소 3회 확인
- 일관된 수정 패턴(매번 같은 유형의 수정)
- 오탐지 없음

스킬 후보를 다음으로 검토할 수 있습니다:

```bash
$ precc skills advise
```

스킬 관리에 대한 자세한 내용은 [Skills](skills.md)를 참조하세요.

## 데이터 저장

- **실패-수정 쌍**: `~/.local/share/precc/history.db`
- **승격된 스킬**: `~/.local/share/precc/heuristics.db`

둘 다 안전한 동시 접근을 위해 WAL 모드의 SQLite 데이터베이스입니다.
