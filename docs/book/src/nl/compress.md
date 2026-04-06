# Comprimeren

`precc compress` verkleint CLAUDE.md en andere contextbestanden om tokengebruik te verminderen wanneer Claude Code ze laadt. Dit is een Pro-functie.

## Basisgebruik

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

## Proefrun

Bekijk wat er zou veranderen zonder bestanden te wijzigen:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## Terugzetten

Originelen worden automatisch geback-upt. Om ze te herstellen:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## Wat wordt gecomprimeerd

De compressor past meerdere transformaties toe:

- Verwijdert overbodige witruimte en lege regels
- Verkort uitgebreide formuleringen met behoud van betekenis
- Comprimeert tabellen en lijsten
- Verwijdert opmerkingen en decoratieve opmaak
- Behoudt alle codeblokken, paden en technische identifiers

De gecomprimeerde uitvoer is nog steeds leesbaar -- niet geminificeerd of verhuld.

## Specifieke bestanden targeten

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
