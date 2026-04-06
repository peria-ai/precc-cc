# Analýza

PRECC analyzuje logy relací Claude Code pro naučení vzorů chyb a oprav. Když uvidí stejnou chybu znovu, automaticky aplikuje opravu.

## Ingestace logů relací

### Ingestovat jeden soubor

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### Ingestovat všechny logy

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### Vynucená reingestace

Pro opětovné zpracování již ingestovaných souborů:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## Jak analýza funguje

1. PRECC přečte JSONL log soubor relace.
2. Identifikuje páry příkazů, kde první selhal a druhý byl opravené opakování.
3. Extrahuje vzor (co se pokazilo) a opravu (co Claude udělal jinak).
4. Vzory jsou uloženy v `~/.local/share/precc/history.db`.
5. Když vzor dosáhne prahu důvěryhodnosti (viděn vícekrát), stane se naučenou dovedností v `heuristics.db`.

### Příklad vzoru

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## Démon precc-learner

Démon `precc-learner` běží na pozadí a automaticky sleduje nové logy relací:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

Démon používá notifikace souborového systému (inotify na Linuxu, FSEvents na macOS), takže reaguje okamžitě, když relace skončí.

## Od vzorů k dovednostem

Naučené vzory jsou povýšeny na dovednosti, když splní tato kritéria:

- Viděny alespoň 3krát napříč relacemi
- Konzistentní vzor opravy (stejný typ korekce pokaždé)
- Nebyly detekovány žádné falešné pozitivy

Kandidáty na dovednosti si můžete prohlédnout:

```bash
$ precc skills advise
```

Viz [Skills](skills.md) pro podrobnosti o správě dovedností.

## Úložiště dat

- **Páry chyba-oprava**: `~/.local/share/precc/history.db`
- **Povýšené dovednosti**: `~/.local/share/precc/heuristics.db`

Obě jsou SQLite databáze v režimu WAL pro bezpečný souběžný přístup.
