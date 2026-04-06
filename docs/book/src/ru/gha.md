# Анализ GitHub Actions

`precc gha` анализирует неудачные запуски GitHub Actions и предлагает исправления. Это функция Pro.

## Использование

Передайте URL неудачного запуска GitHub Actions:

```bash
$ precc gha https://github.com/myorg/myrepo/actions/runs/12345678
[precc] Fetching run 12345678...
[precc] Run: CI / build (ubuntu-latest)
[precc] Status: failure
[precc] Failed step: Run cargo test

[precc] Log analysis:
  Error: test result: FAILED. 2 passed; 1 failed
  Failed test: tests::integration::test_database_connection
  Cause: thread 'tests::integration::test_database_connection' panicked at
         'called Result::unwrap() on an Err value: Connection refused'

[precc] Suggested fix:
  The test requires a database connection but the CI environment does not
  start a database service. Add a services block to your workflow:

    services:
      postgres:
        image: postgres:15
        ports:
          - 5432:5432
        env:
          POSTGRES_PASSWORD: test
```

## Что делает

1. Разбирает URL запуска GitHub Actions для извлечения owner, repo и run ID.
2. Получает логи запуска через API GitHub (использует `GITHUB_TOKEN`, если задан, иначе публичный доступ).
3. Определяет неудачный шаг и извлекает соответствующие строки ошибок.
4. Анализирует ошибку и предлагает исправление на основе типичных паттернов ошибок CI.

## Поддерживаемые паттерны ошибок

- Отсутствующие сервисные контейнеры (базы данных, Redis и т.д.)
- Неправильная ОС или архитектура раннера
- Отсутствующие переменные окружения или секреты
- Ошибки установки зависимостей
- Таймауты тестов
- Ошибки прав доступа
- Промахи кэша, вызывающие медленные сборки
