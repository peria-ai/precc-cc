# Komprimieren

`precc compress` verkleinert CLAUDE.md und andere Kontextdateien, um den Token-Verbrauch zu reduzieren, wenn Claude Code sie lädt. Dies ist eine Pro-Funktion.

## Grundlegende Verwendung

```bash
$ precc compress .
[precc] Scanning directory: .
[precc] Found 3 context files:
         CLAUDE.md (2,847 tokens -> 1,203 tokens, -57.7%)
         ARCHITECTURE.md (4,112 tokens -> 2,044 tokens, -50.3%)
         ALTERNATIVES.md (3,891 tokens -> 1,967 tokens, -49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
[precc] Files compressed. Use --revert to restore originals.
```

## Probelauf

Vorschau der Änderungen ohne Dateien zu modifizieren:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## Zurücksetzen

Originale werden automatisch gesichert. Um sie wiederherzustellen:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## Was wird komprimiert

Der Kompressor wendet mehrere Transformationen an:

- Entfernt überflüssige Leerzeichen und Leerzeilen
- Kürzt ausführliche Formulierungen unter Beibehaltung der Bedeutung
- Verdichtet Tabellen und Listen
- Entfernt Kommentare und dekorative Formatierung
- Behält alle Codeblöcke, Pfade und technische Bezeichner bei

Die komprimierte Ausgabe ist weiterhin menschenlesbar -- sie ist weder minifiziert noch verschleiert.

## Bestimmte Dateien auswählen

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
