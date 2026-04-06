# Телеметрия

PRECC поддерживает анонимную opt-in телеметрию для улучшения инструмента. Данные не собираются без вашего явного согласия.

## Подключение

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## Отключение

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## Проверка статуса

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## Предварительный просмотр отправляемых данных

Перед подключением вы можете увидеть, какие именно данные будут собираться:

```bash
$ precc telemetry preview
Telemetry payload (this session):
{
  "version": "0.3.0",
  "os": "linux",
  "arch": "x86_64",
  "skills_activated": 12,
  "commands_intercepted": 87,
  "pillars_used": [1, 4],
  "avg_hook_latency_ms": 2.3,
  "session_count": 1
}
```

## Что собирается

- Версия PRECC, ОС и архитектура
- Агрегированные счётчики: перехваченные команды, активированные навыки, использованные pillar
- Средняя задержка hook
- Количество сессий

## Что НЕ собирается

- Никакого текста команд или аргументов
- Никаких путей к файлам или имён директорий
- Никаких названий проектов или URL репозиториев
- Никакой персонально идентифицирующей информации (PII)
- Никаких IP-адресов (сервер их не записывает)

## Переопределение через переменную окружения

Для отключения телеметрии без выполнения команды (полезно в CI или общих средах):

```bash
export PRECC_NO_TELEMETRY=1
```

Это имеет приоритет над настройкой согласия.

## Назначение данных

Данные телеметрии отправляются на `https://telemetry.peria.ai/v1/precc` по HTTPS. Данные используются исключительно для понимания паттернов использования и определения приоритетов разработки.
