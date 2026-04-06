# Analiză

PRECC analizează jurnalele sesiunilor Claude Code pentru a învăța tipare eroare-corecție. Când vede aceeași greșeală din nou, aplică automat corecția.

## Ingestia jurnalelor sesiunilor

### Ingestia unui singur fișier

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### Ingestia tuturor jurnalelor

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### Reingestie forțată

Pentru a reprocesa fișierele deja ingerate:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## Cum funcționează analiza

1. PRECC citește fișierul jurnal JSONL al sesiunii.
2. Identifică perechile de comenzi unde prima a eșuat și a doua a fost o reîncercare corectată.
3. Extrage tiparul (ce a mers greșit) și corecția (ce a făcut Claude diferit).
4. Tiparele sunt stocate în `~/.local/share/precc/history.db`.
5. Când un tipar atinge un prag de încredere (văzut de mai multe ori), devine o abilitate învățată în `heuristics.db`.

### Exemplu de tipar

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## Daemon-ul precc-learner

Daemon-ul `precc-learner` rulează în fundal și monitorizează automat jurnalele noi ale sesiunilor:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

Daemon-ul folosește notificări ale sistemului de fișiere (inotify pe Linux, FSEvents pe macOS) astfel încât reacționează imediat când o sesiune se termină.

## De la tipare la abilități

Tiparele învățate sunt promovate la abilități când îndeplinesc aceste criterii:

- Văzute cel puțin de 3 ori în sesiuni diferite
- Tipar de corecție consistent (același tip de corecție de fiecare dată)
- Fără fals pozitive detectate

Puteți revizui candidații la abilități cu:

```bash
$ precc skills advise
```

Vedeți [Skills](skills.md) pentru detalii despre gestionarea abilităților.

## Stocare date

- **Perechi eroare-corecție**: `~/.local/share/precc/history.db`
- **Abilități promovate**: `~/.local/share/precc/heuristics.db`

Ambele sunt baze de date SQLite în mod WAL pentru acces concurent sigur.
