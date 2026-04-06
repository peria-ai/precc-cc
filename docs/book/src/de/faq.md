# FAQ

## Ist PRECC sicher?

Ja. PRECC verwendet den offiziellen PreToolUse-Hook-Mechanismus von Claude Code -- denselben Erweiterungspunkt, den Anthropic genau für diesen Zweck entwickelt hat. Der Hook:

- Läuft vollständig offline (keine Netzwerkaufrufe im Hot Path)
- Wird in unter 5 Millisekunden abgeschlossen
- Ist fail-open: bei Problemen wird der ursprüngliche Befehl unverändert ausgeführt
- Ändert nur Befehle, führt sie nie selbst aus
- Speichert Daten lokal in SQLite-Datenbanken

## Funktioniert PRECC mit anderen KI-Coding-Tools?

PRECC ist speziell für Claude Code entwickelt. Es basiert auf dem PreToolUse-Hook-Protokoll, das Claude Code bereitstellt. Es funktioniert nicht mit Cursor, Copilot, Windsurf oder anderen KI-Coding-Tools.

## Welche Daten sendet die Telemetrie?

Telemetrie ist nur Opt-in. Wenn aktiviert, sendet sie:

- PRECC-Version, Betriebssystem und Architektur
- Aggregierte Zähler (abgefangene Befehle, aktivierte Skills)
- Durchschnittliche Hook-Latenz

Sie sendet **keine** Befehlstexte, Dateipfade, Projektnamen oder persönlich identifizierbare Informationen. Sie können die genaue Nutzlast mit `precc telemetry preview` vor der Aktivierung ansehen. Siehe [Telemetrie](telemetry.md) für Details.

## Wie deinstalliere ich PRECC?

??faq_uninstall_a_intro??

1. Hook-Registrierung entfernen:
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. Binärdatei entfernen:
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. Daten entfernen (optional):
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## Meine Lizenz ist abgelaufen. Was passiert?

PRECC kehrt zum Community-Tier zurück. Alle Kernfunktionen funktionieren weiterhin:

- Eingebaute Skills bleiben aktiv
- Die Hook-Pipeline läuft normal
- `precc savings` zeigt die Zusammenfassung
- `precc ingest` und Session-Mining funktionieren

Pro-Funktionen werden bis zur Verlängerung nicht verfügbar:

- `precc savings --all` (detaillierte Aufschlüsselung)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- E-Mail-Berichte

## Der Hook scheint nicht zu laufen. Wie debugge ich?

??faq_debug_a_intro??

1. Prüfen Sie, ob der Hook registriert ist:
   ```bash
   precc init
   ```

2. Testen Sie den Hook manuell:
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. Prüfen Sie, ob die Binärdatei in Ihrem PATH ist:
   ```bash
   which precc-hook
   ```

4. Prüfen Sie die Hook-Konfiguration von Claude Code in `~/.claude/settings.json`.

## Verlangsamt PRECC Claude Code?

Nein. Der Hook wird in unter 5 Millisekunden (p99) abgeschlossen. Dies ist im Vergleich zur Zeit, die Claude für Reasoning und Antwortgenerierung benötigt, nicht wahrnehmbar.

## Kann ich PRECC in CI/CD verwenden?

PRECC ist für interaktive Claude Code-Sitzungen konzipiert. In CI/CD gibt es keine Claude Code-Instanz zum Anhaken. Allerdings kann `precc gha` fehlgeschlagene GitHub Actions-Läufe aus jeder Umgebung analysieren.

## Wie unterscheiden sich geminte Skills von eingebauten Skills?

Eingebaute Skills werden mit PRECC ausgeliefert und decken häufige Falsche-Verzeichnis-Muster ab. Geminte Skills werden aus Ihren spezifischen Sitzungsprotokollen gelernt -- sie erfassen Muster, die einzigartig für Ihren Workflow sind. Beide werden in SQLite gespeichert und identisch von der Hook-Pipeline ausgewertet.

## Kann ich Skills mit meinem Team teilen?

Ja. Exportieren Sie einen Skill mit `precc skills export NAME` als TOML und teilen Sie die Datei. Teammitglieder können sie in ihr `skills/`-Verzeichnis legen oder in ihre Heuristik-Datenbank importieren.
