# Analys

PRECC analyserar Claude Code-sessionsloggar för att lära sig fel-rättningsmönster. När den ser samma fel igen tillämpar den rättningen automatiskt.

## Inläsning av sessionsloggar

### Läs in en enskild fil

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### Läs in alla loggar

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### Tvingad ominläsning

För att ombearbeta redan inlästa filer:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## Hur analys fungerar

1. PRECC läser sessionens JSONL-loggfil.
2. Den identifierar kommandopar där det första kommandot misslyckades och det andra var ett korrigerat omförsök.
3. Den extraherar mönstret (vad som gick fel) och rättningen (vad Claude gjorde annorlunda).
4. Mönster lagras i `~/.local/share/precc/history.db`.
5. När ett mönster når en konfidenströskel (setts flera gånger) blir det en inlärd färdighet i `heuristics.db`.

### Exempelmönster

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## precc-learner-demonen

`precc-learner`-demonen körs i bakgrunden och övervakar automatiskt nya sessionsloggar:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

Demonen använder filsystemsnotifieringar (inotify på Linux, FSEvents på macOS) så den reagerar omedelbart när en session avslutas.

## Från mönster till färdigheter

Inlärda mönster befordras till färdigheter när de uppfyller dessa kriterier:

- Setts minst 3 gånger över sessioner
- Konsekvent rättningsmönster (samma typ av korrigering varje gång)
- Inga falska positiver upptäckta

Du kan granska färdighetskandidater med:

```bash
$ precc skills advise
```

Se [Skills](skills.md) för detaljer om hantering av färdigheter.

## Datalagring

- **Fel-rättningspar**: `~/.local/share/precc/history.db`
- **Befordrade färdigheter**: `~/.local/share/precc/heuristics.db`

Båda är SQLite-databaser i WAL-läge för säker samtidig åtkomst.
