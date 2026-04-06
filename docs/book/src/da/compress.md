# Komprimering

`precc compress` formindsker CLAUDE.md og andre kontekstfiler for at reducere tokenforbruget, når Claude Code indlæser dem. Dette er en Pro-funktion.

## Grundlæggende brug

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

## Testkørsel

Forhåndsvis hvad der ville ændre sig uden at ændre filer:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## Gendannelse

Originalerne sikkerhedskopieres automatisk. For at gendanne dem:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## Hvad komprimeres

Kompressoren anvender flere transformationer:

- Fjerner overflødige mellemrum og tomme linjer
- Forkorter omstændelig formulering med bevarelse af betydning
- Kondenserer tabeller og lister
- Fjerner kommentarer og dekorativ formatering
- Bevarer alle kodeblokke, stier og tekniske identifikatorer

Det komprimerede output er stadig menneskelæsbart -- det er ikke minificeret eller obfuskeret.

## Specifikke filer

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
