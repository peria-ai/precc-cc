# Mining

PRECC analyseert Claude Code-sessielogs om fout-fix-patronen te leren. Wanneer het dezelfde fout opnieuw ziet, past het de fix automatisch toe.

## Sessielogs inlezen

### Eén bestand inlezen

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### Alle logs inlezen

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### Opnieuw inlezen forceren

Om reeds ingelezen bestanden opnieuw te verwerken:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## Hoe mining werkt

1. PRECC leest het JSONL-sessielogbestand.
2. Het identificeert opdrachtparen waarbij de eerste opdracht faalde en de tweede een gecorrigeerde herpoging was.
3. Het extraheert het patroon (wat er misging) en de fix (wat Claude anders deed).
4. Patronen worden opgeslagen in `~/.local/share/precc/history.db`.
5. Wanneer een patroon een betrouwbaarheidsdrempel bereikt, wordt het een ontgonnen vaardigheid in `heuristics.db`.

### Voorbeeldpatroon

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## De precc-learner-daemon

De `precc-learner`-daemon draait op de achtergrond en bewaakt automatisch nieuwe sessielogs:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

De daemon gebruikt bestandssysteemmeldingen (inotify op Linux, FSEvents op macOS) en reageert dus direct wanneer een sessie eindigt.

## Van patronen naar vaardigheden

Ontgonnen patronen worden vaardigheden wanneer ze aan deze criteria voldoen:

- Minstens 3 keer gezien over sessies heen
- Consistent fix-patroon (elke keer hetzelfde type correctie)
- Geen valse positieven gedetecteerd

U kunt vaardigheidskandidaten bekijken met:

```bash
$ precc skills advise
```

Zie [Skills](skills.md) voor details over het beheren van vaardigheden.

## Gegevensopslag

- **Fout-fix-paren**: `~/.local/share/precc/history.db`
- **Gepromoveerde vaardigheden**: `~/.local/share/precc/heuristics.db`

Beide zijn SQLite-databases in WAL-modus voor veilige gelijktijdige toegang.
