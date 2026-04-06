# Mining

PRECC analizza i log delle sessioni di Claude Code per apprendere pattern errore-correzione. Quando rivede lo stesso errore, applica la correzione automaticamente.

## Acquisizione dei log delle sessioni

### Acquisisci un singolo file

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### Acquisisci tutti i log

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### Riacquisizione forzata

Per rielaborare file già acquisiti:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## Come funziona il mining

1. PRECC legge il file di log della sessione JSONL.
2. Identifica le coppie di comandi in cui il primo è fallito e il secondo era un retry corretto.
3. Estrae il pattern (cosa è andato storto) e la correzione (cosa ha fatto Claude diversamente).
4. I pattern vengono memorizzati in `~/.local/share/precc/history.db`.
5. Quando un pattern raggiunge una soglia di confidenza (visto più volte), diventa una skill appresa in `heuristics.db`.

### Esempio di pattern

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## Il daemon precc-learner

Il daemon `precc-learner` viene eseguito in background e monitora automaticamente i nuovi log delle sessioni:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

Il daemon usa notifiche del file system (inotify su Linux, FSEvents su macOS) quindi reagisce immediatamente quando una sessione termina.

## Da pattern a skill

I pattern appresi vengono promossi a skill quando soddisfano questi criteri:

- Visti almeno 3 volte tra le sessioni
- Pattern di correzione coerente (stesso tipo di correzione ogni volta)
- Nessun falso positivo rilevato

Puoi rivedere le skill candidate con:

```bash
$ precc skills advise
```

Vedi [Skills](skills.md) per dettagli sulla gestione delle skill.

## Archiviazione dati

- **Coppie errore-correzione**: `~/.local/share/precc/history.db`
- **Skill promosse**: `~/.local/share/precc/heuristics.db`

Entrambi sono database SQLite in modalità WAL per un accesso concorrente sicuro.
