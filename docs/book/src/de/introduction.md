# Einführung

## Was ist PRECC?

PRECC (Prädiktive Fehlerkorrektur für Claude Code) ist ein Rust-Tool, das Claude Code Bash-Befehle über den offiziellen PreToolUse-Hook-Mechanismus abfängt. Es behebt Fehler *bevor sie auftreten*, spart Token und eliminiert Wiederholungsschleifen.

Kostenlos für Community-Nutzer.

## Das Problem

Claude Code verschwendet erhebliche Token durch vermeidbare Fehler:

- **Falsche Verzeichnisse** -- `cargo build` in einem übergeordneten Verzeichnis ohne `Cargo.toml` ausführen und nach dem Lesen des Fehlers erneut versuchen.
- **Wiederholungsschleifen** -- Ein fehlgeschlagener Befehl erzeugt ausführliche Ausgabe, Claude liest sie, denkt darüber nach und versucht es erneut.
- **Ausführliche Ausgabe** -- Befehle wie `find` oder `ls -R` erzeugen tausende Zeilen, die Claude verarbeiten muss.

## Die vier Säulen

### Kontextkorrektur (cd-prepend)

Erkennt, wenn Befehle wie `cargo build` oder `npm test` im falschen Verzeichnis ausgeführt werden, und stellt `cd /korrekter/pfad &&` voran.

### GDB-Debugging

Erkennt Möglichkeiten, GDB für tieferes Debugging von Segfaults und Abstürzen anzuhängen und liefert strukturierte Debug-Informationen.

### Session-Mining

Analysiert Claude Code-Sitzungsprotokolle nach Fehler-Fix-Paaren. Bei wiederkehrenden Fehlern kennt PRECC die Lösung bereits und wendet sie automatisch an.

### Automatisierungsskills

Eine Bibliothek eingebauter und geminter Skills, die Befehlsmuster erkennen und umschreiben. Skills werden als TOML-Dateien oder SQLite-Zeilen definiert.

## So funktioniert es (30-Sekunden-Version)

1. Claude Code ist im Begriff, einen Bash-Befehl auszuführen.
2. Der PreToolUse-Hook sendet den Befehl als JSON an `precc-hook` über stdin.
3. `precc-hook` verarbeitet den Befehl durch die Pipeline (Skills, Verzeichniskorrektur, Komprimierung) in unter 3 Millisekunden.
4. Der korrigierte Befehl wird als JSON auf stdout zurückgegeben.
5. Claude Code führt den korrigierten Befehl aus.

Claude sieht den Fehler nie. Keine Token verschwendet.

### Adaptive Komprimierung

Wenn ein Befehl nach der Komprimierung fehlschlägt, überspringt PRECC automatisch die Komprimierung beim erneuten Versuch, damit Claude die vollständige unkomprimierte Ausgabe zum Debuggen erhält.

## Live-Nutzungsstatistiken

Aktuelle Version <span data-stat="current_version">--</span>:

| Metrik | Wert |
|---|---|
| Hook-Aufrufe | <span data-stat="total_invocations">--</span> |
| Gesparte Token | <span data-stat="total_tokens_saved">--</span> |
| Sparquote | <span data-stat="saving_pct">--</span>% |
| RTK-Umschreibungen | <span data-stat="rtk_rewrites">--</span> |
| CD-Korrekturen | <span data-stat="cd_prepends">--</span> |
| Hook-Latenz | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| Nutzer | <span data-stat="unique_users">--</span> |

### Gemessene Einsparungen (Realdaten)

<div id="measured-savings" style="display:none">
<table id="measured-summary">
<thead><tr><th>Metrik</th><th>Wert</th></tr></thead>
<tbody>
<tr><td>Original-Ausgabetokens (ohne PRECC)</td><td><span data-measured="original_output_tokens">--</span></td></tr>
<tr><td>Tatsächliche Ausgabetokens (mit PRECC)</td><td><span data-measured="actual_output_tokens">--</span></td></tr>
<tr><td>Gesparte Token</td><td><strong><span data-measured="savings_tokens">--</span></strong></td></tr>
<tr><td>Sparquote</td><td><strong><span data-measured="savings_pct">--</span>%</strong></td></tr>
<tr><td>Realmessungen</td><td><span data-measured="ground_truth_count">--</span> Messungen</td></tr>
</tbody>
</table>
</div>

<div id="rewrite-type-breakdown" style="display:none">

#### Nach Rewrite-Typ

<table id="rewrite-type-table">
<thead><tr><th>Typ</th><th>Anzahl</th><th>Ø Einsparung %</th><th>Gesparte Token</th></tr></thead>
<tbody><tr><td colspan="4"><em>Laden...</em></td></tr></tbody>
</table>
</div>

### Einsparungen pro Version

<table id="version-breakdown" style="display:none">
<thead><tr><th>Version</th><th>Nutzer</th><th>Hook-Aufrufe</th><th>Gesparte Token</th><th>Sparquote</th></tr></thead>
<tbody><tr><td colspan="5"><em>Laden...</em></td></tr></tbody>
</table>

<small>Diese Zahlen werden automatisch aus anonymisierter Telemetrie aktualisiert.</small>

## Links

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Webseite: [https://peria.ai](https://peria.ai)
- Dokumentation: [https://precc.cc](https://precc.cc)
