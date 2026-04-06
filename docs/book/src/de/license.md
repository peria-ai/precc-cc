# Lizenz

PRECC bietet zwei Stufen: Community (kostenlos) und Pro.

## Community-Stufe (kostenlos)

Die Community-Stufe umfasst:

- Alle integrierten Skills (Verzeichniskorrektur, jj-Übersetzung usw.)
- Hook-Pipeline mit voller Pillar-1- und Pillar-4-Unterstützung
- Grundlegende `precc savings`-Zusammenfassung
- Session-Mining mit `precc ingest`
- Unbegrenzte lokale Nutzung

## Pro-Stufe

Pro schaltet zusätzliche Funktionen frei:

- **Detaillierte Einsparungsaufschlüsselung** -- `precc savings --all` mit Analyse pro Befehl
- **GIF-Aufnahme** -- `precc gif` zum Erstellen animierter Terminal-GIFs
- **IP-Geofence-Compliance** -- Für regulierte Umgebungen
- **E-Mail-Berichte** -- `precc mail report` zum Senden von Analysen
- **GitHub-Actions-Analyse** -- `precc gha` zum Debuggen fehlgeschlagener Workflows
- **Kontextkomprimierung** -- `precc compress` zur CLAUDE.md-Optimierung
- **Prioritäts-Support**

## Eine Lizenz aktivieren

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## Lizenzstatus prüfen

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## GitHub-Sponsors-Aktivierung

Wenn Sie PRECC über GitHub Sponsors sponsern, wird Ihre Lizenz automatisch über Ihre GitHub-E-Mail aktiviert. Kein Schlüssel erforderlich -- stellen Sie nur sicher, dass Ihre Sponsor-E-Mail übereinstimmt:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## Geräte-Fingerprint

Jede Lizenz ist an einen Geräte-Fingerprint gebunden. Zeigen Sie Ihren an mit:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

Wenn Sie Ihre Lizenz auf einen neuen Computer übertragen müssen, deaktivieren Sie sie zuerst:

```bash
precc license deactivate
```

Dann aktivieren Sie auf dem neuen Computer.

## Lizenz abgelaufen?

Wenn eine Pro-Lizenz abläuft, kehrt PRECC zur Community-Stufe zurück. Alle integrierten Skills und Kernfunktionen funktionieren weiterhin. Nur Pro-spezifische Funktionen werden nicht verfügbar. Weitere Details finden Sie in den [FAQ](faq.md).
