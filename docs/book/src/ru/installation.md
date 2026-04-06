# Установка

## Быстрая установка (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

Скачивает последний бинарный файл релиза для вашей платформы, проверяет контрольную сумму SHA256 и помещает в `~/.local/bin/`.

После установки инициализируйте PRECC:

```bash
precc init
```

`precc init` регистрирует PreToolUse hook в Claude Code, создаёт директории данных и инициализирует базу навыков.

## Параметры установки

### Проверка SHA256

По умолчанию установщик проверяет контрольную сумму бинарника по опубликованной сумме SHA256. Чтобы пропустить проверку (не рекомендуется):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### Пользовательский префикс установки

Установка в пользовательскую директорию:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### Инструменты-компаньоны (--extras)

PRECC поставляется с дополнительными инструментами-компаньонами. Установите их с `--extras`:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

Это устанавливает:

| Инструмент | Назначение |
|------|---------|
| **RTK** | Набор инструментов для перезаписи команд |
| **lean-ctx** | Сжатие контекста для файлов CLAUDE.md и промптов |
| **nushell** | Структурированная оболочка для продвинутых pipeline |
| **cocoindex-code** | Индексация кода для более быстрого разрешения контекста |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

Затем инициализируйте:

```powershell
precc init
```

## Ручная установка

1. Скачайте бинарный файл релиза для вашей платформы с [GitHub Releases](https://github.com/peria-ai/precc-cc/releases).
2. Проверьте контрольную сумму SHA256 по файлу `.sha256` в релизе.
3. Поместите бинарный файл в директорию из вашего `PATH` (например, `~/.local/bin/`).
4. Выполните `precc init`.

## Обновление

```bash
precc update
```

Принудительное обновление до определённой версии:

```bash
precc update --force --version 0.3.0
```

Включите автоматические обновления:

```bash
precc update --auto
```

## Проверка установки

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

Если `precc` не найден, убедитесь, что `~/.local/bin` находится в вашем `PATH`.
