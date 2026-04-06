# Comprimare

`precc compress` micșorează CLAUDE.md și alte fișiere de context pentru a reduce consumul de tokeni când Claude Code le încarcă. Aceasta este o funcție Pro.

## Utilizare de bază

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

## Rulare de test

Previzualizați ce s-ar schimba fără a modifica fișierele:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## Restaurare

Originalele sunt salvate automat. Pentru a le restaura:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## Ce se comprimă

Compresorul aplică mai multe transformări:

- Elimină spațiile și liniile goale redundante
- Scurtează formulările verbose păstrând semnificația
- Condensează tabelele și listele
- Elimină comentariile și formatarea decorativă
- Păstrează toate blocurile de cod, căile și identificatorii tehnici

Ieșirea comprimată este în continuare lizibilă -- nu este minificată sau ofuscată.

## Fișiere specifice

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
