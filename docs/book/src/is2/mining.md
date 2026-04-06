# Greining

PRECC greinir lotunarskrár Claude Code til að læra villu-lagfæringar mynstur. Þegar það sér sömu villu aftur beitir það lagfæringunni sjálfkrafa.

## Innlesning lotunarskráa

### Lesa inn staka skrá

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### Lesa inn alla loga

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### Þvinguð endurinnlesning

Til að endurvinna skrár sem þegar voru lesnar inn:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## Hvernig greining virkar

1. PRECC les JSONL-lotuskrána.
2. Það auðkennir skipanapar þar sem fyrri skipun mistókst og sú seinni var leiðrétt endurtilraun.
3. Það dregur út mynstur (hvað fór úrskeiðis) og lagfæringuna (hvað Claude gerði öðruvísi).
4. Mynstur eru geymd í `~/.local/share/precc/history.db`.
5. Þegar mynstur nær trúverðugleikaþröskuldi (séð margsinnis), verður það lærð þekking í `heuristics.db`.

### Dæmi um mynstur

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## precc-learner þjónninn

`precc-learner` þjónninn keyrir í bakgrunni og fylgist sjálfkrafa með nýjum lotunarskrám:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

Þjónninn notar tilkynningar skráarkerfis (inotify á Linux, FSEvents á macOS) svo hann bregst strax við þegar lota endar.

## Frá mynstri til þekkingar

Lærð mynstur eru færð upp í þekkingu þegar þau uppfylla þessi skilyrði:

- Séð að minnsta kosti 3 sinnum á milli lota
- Stöðugt lagfæringarmynstur (sama tegund leiðréttingar í hvert skipti)
- Engar rangar jákvæðar uppgötvanir

Þú getur skoðað þekkingarframbjóðendur með:

```bash
$ precc skills advise
```

Sjá [Skills](skills.md) fyrir nánari upplýsingar um stjórnun þekkingar.

## Gagnageymsla

- **Villu-lagfæringar pör**: `~/.local/share/precc/history.db`
- **Uppfærð þekking**: `~/.local/share/precc/heuristics.db`

Báðir eru SQLite-gagnagrunnar í WAL-ham fyrir öruggan samhliða aðgang.
