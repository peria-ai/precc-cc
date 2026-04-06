# Compressione

`precc compress` riduce CLAUDE.md e altri file di contesto per diminuire l'uso di token quando Claude Code li carica. Questa è una funzionalità Pro.

## Uso base

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

## Prova a secco

Anteprima delle modifiche senza alterare i file:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## Ripristino

Gli originali vengono salvati automaticamente. Per ripristinarli:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## Cosa viene compresso

Il compressore applica diverse trasformazioni:

- Rimuove spazi bianchi e righe vuote ridondanti
- Accorcia le espressioni verbose preservando il significato
- Condensa tabelle e liste
- Rimuove commenti e formattazione decorativa
- Preserva tutti i blocchi di codice, i percorsi e gli identificatori tecnici

L'output compresso è comunque leggibile -- non è minificato né offuscato.

## File specifici

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
