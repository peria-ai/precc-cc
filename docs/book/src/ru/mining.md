# Анализ

PRECC анализирует логи сессий Claude Code для изучения паттернов ошибок и исправлений. При повторном обнаружении той же ошибки автоматически применяет исправление.

## Загрузка логов сессий

### Загрузить один файл

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### Загрузить все логи

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### Принудительная повторная загрузка

Для повторной обработки уже загруженных файлов:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## Как работает анализ

1. PRECC читает файл лога сессии JSONL.
2. Определяет пары команд, где первая завершилась ошибкой, а вторая — исправленный повтор.
3. Извлекает паттерн (что пошло не так) и исправление (что Claude сделал иначе).
4. Паттерны хранятся в `~/.local/share/precc/history.db`.
5. Когда паттерн достигает порога уверенности (встречен несколько раз), он становится изученным навыком в `heuristics.db`.

### Пример паттерна

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## Демон precc-learner

Демон `precc-learner` работает в фоновом режиме и автоматически отслеживает новые логи сессий:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

Демон использует уведомления файловой системы (inotify на Linux, FSEvents на macOS), поэтому реагирует мгновенно при завершении сессии.

## От паттернов к навыкам

Изученные паттерны повышаются до навыков при выполнении этих критериев:

- Встречены минимум 3 раза в разных сессиях
- Последовательный паттерн исправления (одинаковый тип коррекции каждый раз)
- Ложных срабатываний не обнаружено

Вы можете просмотреть кандидатов в навыки:

```bash
$ precc skills advise
```

Подробности об управлении навыками см. в [Skills](skills.md).

## Хранение данных

- **Пары ошибка-исправление**: `~/.local/share/precc/history.db`
- **Повышенные навыки**: `~/.local/share/precc/heuristics.db`

Оба являются базами данных SQLite в режиме WAL для безопасного параллельного доступа.
