# Kompresja

`precc compress` zmniejsza CLAUDE.md i inne pliki kontekstowe, aby ograniczyć zużycie tokenów, gdy Claude Code je ładuje. To funkcja Pro.

## Podstawowe użycie

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

## Przebieg próbny

Podgląd zmian bez modyfikowania plików:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## Przywracanie

Oryginały są automatycznie kopiowane. Aby je przywrócić:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## Co jest kompresowane

Kompresor stosuje kilka transformacji:

- Usuwa zbędne białe znaki i puste linie
- Skraca rozwlekłe sformułowania zachowując znaczenie
- Kompresuje tabele i listy
- Usuwa komentarze i dekoracyjne formatowanie
- Zachowuje wszystkie bloki kodu, ścieżki i identyfikatory techniczne

Skompresowany wynik jest nadal czytelny dla człowieka -- nie jest zminifikowany ani zaciemniony.

## Celowanie w konkretne pliki

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
