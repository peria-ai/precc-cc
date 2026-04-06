# Mining

PRECC analysiert Claude Code-Sitzungsprotokolle, um Fehler-Fix-Muster zu lernen. Wenn es denselben Fehler erneut erkennt, wendet es die Lösung automatisch an.

## Sitzungsprotokolle einlesen

### Eine einzelne Datei einlesen

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### Alle Protokolle einlesen

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### Erneutes Einlesen erzwingen

Um bereits eingelesene Dateien erneut zu verarbeiten:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## Wie Mining funktioniert

1. PRECC liest die JSONL-Sitzungsprotokolldatei.
2. Es identifiziert Befehlspaare, bei denen der erste Befehl fehlschlug und der zweite ein korrigierter Versuch war.
3. Es extrahiert das Muster (was schiefging) und die Lösung (was Claude anders machte).
4. Muster werden in `~/.local/share/precc/history.db` gespeichert.
5. Wenn ein Muster einen Konfidenzschwellenwert erreicht (mehrfach gesehen), wird es zu einem gemeinten Skill in `heuristics.db`.

### Beispielmuster

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## Der precc-learner-Daemon

Der `precc-learner`-Daemon läuft im Hintergrund und überwacht automatisch neue Sitzungsprotokolle:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

Der Daemon verwendet Dateisystem-Benachrichtigungen (inotify auf Linux, FSEvents auf macOS) und reagiert daher sofort, wenn eine Sitzung endet.

## Von Mustern zu Skills

Geminte Muster werden zu Skills, wenn sie diese Kriterien erfüllen:

- Mindestens 3 Mal über Sitzungen hinweg gesehen
- Konsistentes Fix-Muster (gleiche Art der Korrektur jedes Mal)
- Keine Fehlalarme erkannt

Sie können Skill-Kandidaten überprüfen mit:

```bash
$ precc skills advise
```

Siehe [Skills](skills.md) für Details zur Verwaltung von Skills.

## Datenspeicherung

- **Fehler-Fix-Paare**: `~/.local/share/precc/history.db`
- **Graduierte Skills**: `~/.local/share/precc/heuristics.db`

Beide sind SQLite-Datenbanken im WAL-Modus für sicheren gleichzeitigen Zugriff.
