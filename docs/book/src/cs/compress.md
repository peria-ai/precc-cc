# Komprese

`precc compress` zmenšuje CLAUDE.md a další kontextové soubory pro snížení spotřeby tokenů při načítání Claude Code. Toto je funkce Pro.

## Základní použití

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

## Testovací běh

Náhled změn bez úpravy souborů:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## Obnovení

Originály jsou zálohovány automaticky. Pro obnovení:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## Co se komprimuje

Kompresor aplikuje několik transformací:

- Odstraní nadbytečné mezery a prázdné řádky
- Zkrátí rozvláčné formulace při zachování významu
- Zkondenzuje tabulky a seznamy
- Odstraní komentáře a dekorativní formátování
- Zachová všechny bloky kódu, cesty a technické identifikátory

Komprimovaný výstup je stále čitelný pro člověka -- není minifikovaný ani obfuskovaný.

## Konkrétní soubory

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
