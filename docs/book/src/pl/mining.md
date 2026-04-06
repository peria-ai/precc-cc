# Eksploracja

PRECC analizuje logi sesji Claude Code, aby uczyć się wzorców awaria-naprawa. Gdy widzi ten sam błąd ponownie, automatycznie stosuje poprawkę.

## Pobieranie logów sesji

### Pobranie jednego pliku

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### Pobranie wszystkich logów

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### Wymuszone ponowne pobranie

Aby ponownie przetworzyć pliki, które zostały już pobrane:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## Jak działa eksploracja

1. PRECC czyta plik logu sesji JSONL.
2. Identyfikuje pary poleceń, w których pierwsze polecenie zawiodło, a drugie było poprawioną ponowną próbą.
3. Wyodrębnia wzorzec (co poszło nie tak) i poprawkę (co Claude zrobił inaczej).
4. Wzorce są przechowywane w `~/.local/share/precc/history.db`.
5. Gdy wzorzec osiągnie próg pewności, staje się wydobytą umiejętnością w `heuristics.db`.

### Przykład wzorca

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## Demon precc-learner

Demon `precc-learner` działa w tle i automatycznie obserwuje nowe logi sesji:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

Demon używa powiadomień systemu plików (inotify na Linuksie, FSEvents na macOS), więc reaguje natychmiast po zakończeniu sesji.

## Od wzorców do umiejętności

Wydobyte wzorce awansują do umiejętności, gdy spełniają te kryteria:

- Widziane co najmniej 3 razy w różnych sesjach
- Spójny wzorzec naprawy (ten sam typ korekty za każdym razem)
- Brak wykrytych fałszywych trafień

Możesz przejrzeć kandydatów na umiejętności za pomocą:

```bash
$ precc skills advise
```

Szczegóły dotyczące zarządzania umiejętnościami znajdziesz w [Skills](skills.md).

## Przechowywanie danych

- **Pary awaria-naprawa**: `~/.local/share/precc/history.db`
- **Awansowane umiejętności**: `~/.local/share/precc/heuristics.db`

Obie to bazy danych SQLite w trybie WAL dla bezpiecznego równoczesnego dostępu.
