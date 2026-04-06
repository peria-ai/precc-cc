# Komprimering

`precc compress` krymper CLAUDE.md och andra kontextfiler för att minska tokenanvändningen när Claude Code laddar dem. Detta är en Pro-funktion.

## Grundläggande användning

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

## Testkörning

Förhandsgranska vad som skulle ändras utan att modifiera filer:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## Återställning

Originalen säkerhetskopieras automatiskt. För att återställa dem:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## Vad som komprimeras

Kompressorn tillämpar flera transformationer:

- Tar bort överflödiga blanksteg och tomma rader
- Förkortar ordrik formulering med bibehållen mening
- Kondenserar tabeller och listor
- Tar bort kommentarer och dekorativ formatering
- Bevarar alla kodblock, sökvägar och tekniska identifierare

Den komprimerade utdatan är fortfarande läsbar för människor -- den är inte minifierad eller obfuskerad.

## Specifika filer

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
