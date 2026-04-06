# Сжатие

`precc compress` сжимает CLAUDE.md и другие контекстные файлы для уменьшения расхода токенов при загрузке их Claude Code. Это функция Pro.

## Базовое использование

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

## Пробный запуск

Предварительный просмотр изменений без модификации файлов:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## Восстановление

Оригиналы сохраняются автоматически. Для восстановления:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## Что сжимается

Компрессор применяет несколько преобразований:

- Удаляет избыточные пробелы и пустые строки
- Сокращает многословные формулировки, сохраняя смысл
- Сжимает таблицы и списки
- Убирает комментарии и декоративное форматирование
- Сохраняет все блоки кода, пути и технические идентификаторы

Сжатый вывод остаётся читаемым для человека — он не минифицирован и не обфусцирован.

## Указание конкретных файлов

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
