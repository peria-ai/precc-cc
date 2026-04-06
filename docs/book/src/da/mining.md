# Analyse

PRECC analyserer Claude Code-sessionslogfiler for at lære fejl-rettelsesmønstre. Når den ser den samme fejl igen, anvender den rettelsen automatisk.

## Indlæsning af sessionslogfiler

### Indlæs en enkelt fil

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### Indlæs alle logfiler

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### Tvungen genindlæsning

For at genbehandle filer der allerede er indlæst:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## Hvordan analyse virker

1. PRECC læser sessionens JSONL-logfil.
2. Den identificerer kommandopar, hvor den første kommando fejlede og den anden var et korrigeret genforsøg.
3. Den udtrækker mønsteret (hvad der gik galt) og rettelsen (hvad Claude gjorde anderledes).
4. Mønstre gemmes i `~/.local/share/precc/history.db`.
5. Når et mønster når en tillidstærskel (set flere gange), bliver det en lært færdighed i `heuristics.db`.

### Eksempelmønster

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## precc-learner-dæmonen

`precc-learner`-dæmonen kører i baggrunden og overvåger automatisk nye sessionslogfiler:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

Dæmonen bruger filsystemnotifikationer (inotify på Linux, FSEvents på macOS), så den reagerer øjeblikkeligt, når en session slutter.

## Fra mønstre til færdigheder

Lærte mønstre forfremmes til færdigheder, når de opfylder disse kriterier:

- Set mindst 3 gange på tværs af sessioner
- Konsistent rettelsesmønster (samme type korrektion hver gang)
- Ingen falske positiver registreret

Du kan gennemgå færdighedskandidater med:

```bash
$ precc skills advise
```

Se [Skills](skills.md) for detaljer om administration af færdigheder.

## Datalagring

- **Fejl-rettelsespar**: `~/.local/share/precc/history.db`
- **Forfremmede færdigheder**: `~/.local/share/precc/heuristics.db`

Begge er SQLite-databaser i WAL-tilstand for sikker samtidig adgang.
