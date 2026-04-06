# Bányászat

A PRECC a Claude Code munkamenet-naplókat elemzi a hiba-javítás minták megtanulásához. Ha újra ugyanazt a hibát látja, automatikusan alkalmazza a javítást.

## Munkamenet-naplók betöltése

### Egyetlen fájl betöltése

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### Összes napló betöltése

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### Újbóli betöltés kényszerítése

A már betöltött fájlok újrafeldolgozásához:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## Hogyan működik a bányászat

1. A PRECC beolvassa a munkamenet JSONL naplófájlját.
2. Azonosítja azokat a parancspárokat, ahol az első parancs sikertelen volt és a második egy javított újrapróbálkozás.
3. Kivonja a mintát (mi ment rosszul) és a javítást (mit csinált Claude másképp).
4. A minták a `~/.local/share/precc/history.db` fájlban tárolódnak.
5. Ha egy minta eléri a megbízhatósági küszöböt, bányászott készséggé válik a `heuristics.db`-ben.

### Példa minta

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## A precc-learner démon

A `precc-learner` démon a háttérben fut és automatikusan figyeli az új munkamenet-naplókat:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

A démon fájlrendszer-értesítéseket használ (inotify Linuxon, FSEvents macOS-en), így azonnal reagál, amikor egy munkamenet véget ér.

## Mintáktól a készségekig

A bányászott minták készségekké válnak, ha megfelelnek ezeknek a feltételeknek:

- Legalább 3-szor látták különböző munkamenetekben
- Konzisztens javítási minta (minden alkalommal azonos típusú javítás)
- Nem észleltek hamis pozitívot

A készségjelölteket a következővel tekintheti át:

```bash
$ precc skills advise
```

Lásd a [Skills](skills.md) részt a készségek kezelésének részleteiért.

## Adattárolás

- **Hiba-javítás párok**: `~/.local/share/precc/history.db`
- **Előléptetett készségek**: `~/.local/share/precc/heuristics.db`

Mindkettő SQLite adatbázis WAL módban a biztonságos párhuzamos hozzáféréshez.
