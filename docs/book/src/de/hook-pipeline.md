# Hook-Pipeline

Die `precc-hook`-Binary ist der Kern von PRECC. Sie sitzt zwischen Claude Code und der Shell und verarbeitet jeden Bash-Befehl in unter 5 Millisekunden.

## Wie Claude Code den Hook aufruft

Claude Code unterstützt PreToolUse-Hooks -- externe Programme, die Werkzeugeingaben vor der Ausführung inspizieren und ändern können. Wenn Claude einen Bash-Befehl ausführen will, sendet es JSON an `precc-hook` über stdin und liest die Antwort von stdout.

## Pipeline-Stufen

```
Claude Code
    |
    v
+---------------------------+
| 1. Parse JSON stdin       |  Read the command from Claude Code
+---------------------------+
    |
    v
+---------------------------+
| 2. Skill matching         |  Query heuristics.db for matching skills (Pillar 4)
+---------------------------+
    |
    v
+---------------------------+
| 3. Directory correction   |  Resolve correct working directory (Pillar 1)
+---------------------------+
    |
    v
+---------------------------+
| 4. GDB check              |  Detect debug opportunities (Pillar 2)
+---------------------------+
    |
    v
+---------------------------+
| 5. RTK rewriting          |  Apply command rewrites for token savings
+---------------------------+
    |
    v
+---------------------------+
| 6. Emit JSON stdout       |  Return modified command to Claude Code
+---------------------------+
    |
    v
  Shell executes corrected command
```

## Beispiel: JSON Ein- und Ausgabe

### Eingabe (von Claude Code)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC erkennt, dass das aktuelle Verzeichnis kein `Cargo.toml` hat, aber `./myapp/Cargo.toml` existiert.

### Ausgabe (an Claude Code)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

Wenn keine Änderung nötig ist, ist `updatedInput.command` leer und Claude Code verwendet den ursprünglichen Befehl.

## Stufendetails

### Stufe 1: JSON parsen

Liest das vollständige JSON-Objekt von stdin. Extrahiert `tool_input.command`. Bei einem Parsing-Fehler beendet sich der Hook sofort und Claude Code verwendet den ursprünglichen Befehl (Fail-Open-Design).

### Stufe 2: Skill-Matching

Fragt die SQLite-Heuristik-Datenbank nach Skills ab, deren Trigger-Muster zum Befehl passt. Skills werden in Prioritätsreihenfolge geprüft. Sowohl eingebaute TOML-Skills als auch geminte Skills werden ausgewertet.

### Stufe 3: Verzeichniskorrektur

Prüft bei Build-Befehlen (`cargo`, `go`, `make`, `npm`, `python` usw.), ob die erwartete Projektdatei im aktuellen Verzeichnis existiert. Falls nicht, durchsucht es benachbarte Verzeichnisse nach der nächstgelegenen Übereinstimmung und stellt `cd <dir> &&` voran.

Der Verzeichnisscan verwendet einen zwischengespeicherten Dateisystemindex mit 5 Sekunden TTL für hohe Geschwindigkeit.

### Stufe 4: GDB-Prüfung

Wenn der Befehl wahrscheinlich einen Absturz verursacht (z.B. Ausführen einer Debug-Binary), kann PRECC GDB-Wrapper vorschlagen oder injizieren, um strukturierte Debug-Ausgaben statt roher Absturz-Logs zu erfassen.

### Stufe 5: RTK-Umschreibung

Wendet RTK-Regeln (Rewrite Toolkit) an, die ausführliche Befehle kürzen, verrauschte Ausgaben unterdrücken oder Befehle für Token-Effizienz umstrukturieren.

### Stufe 6: JSON ausgeben

Serialisiert den geänderten Befehl zurück zu JSON und schreibt ihn auf stdout. Wenn keine Änderungen vorgenommen wurden, signalisiert die Ausgabe Claude Code, den ursprünglichen Befehl zu verwenden.

## Leistung

Die gesamte Pipeline wird in unter 5 Millisekunden (p99) abgeschlossen. Wichtige Optimierungen:

- SQLite im WAL-Modus für sperrfreie parallele Lesezugriffe
- Vorkompilierte Regex-Muster für Skill-Matching
- Zwischengespeicherte Dateisystem-Scans (5 Sekunden TTL)
- Keine Netzwerkaufrufe im Hot Path
- Fail-Open: Jeder Fehler fällt auf den ursprünglichen Befehl zurück

## Den Hook manuell testen

Sie können den Hook direkt aufrufen:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
