# Tömörítés

A `precc compress` összezsugorítja a CLAUDE.md-t és más kontextusfájlokat, hogy csökkentse a tokenhasználatot, amikor a Claude Code betölti őket. Ez egy Pro funkció.

## Alapvető használat

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

## Próbafuttatás

A változtatások előnézete fájlok módosítása nélkül:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## Visszaállítás

Az eredetik automatikusan mentésre kerülnek. A visszaállításhoz:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## Mi kerül tömörítésre

A tömörítő több átalakítást alkalmaz:

- Eltávolítja a felesleges szóközöket és üres sorokat
- Rövidíti a bőbeszédű megfogalmazásokat a jelentés megőrzése mellett
- Tömöríti a táblázatokat és listákat
- Eltávolítja a megjegyzéseket és dekoratív formázást
- Megőrzi az összes kódblokkot, útvonalat és technikai azonosítót

A tömörített kimenet továbbra is ember által olvasható -- nem minifikált vagy obfuszkált.

## Adott fájlok célzása

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
