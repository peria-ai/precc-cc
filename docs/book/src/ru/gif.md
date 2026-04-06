# Запись GIF

`precc gif` создаёт анимированные GIF-записи терминальных сессий из bash-скриптов. Это функция Pro.

## Базовое использование

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

Первый аргумент — bash-скрипт с командами для выполнения. Второй аргумент — максимальная длительность записи.

## Формат скрипта

Скрипт — это стандартный bash-файл:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Симуляция ввода

Для интерактивных команд передайте значения ввода как дополнительные аргументы:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Каждый дополнительный аргумент передаётся как строка stdin, когда скрипт запрашивает ввод.

## Параметры вывода

Выходной файл по умолчанию назван по имени скрипта (`script.gif`). GIF использует тёмную тему терминала со стандартными размерами 80x24.

## Почему GIF вместо asciinema?

Встроенный навык `asciinema-gif` автоматически переписывает `asciinema rec` в `precc gif`. Файлы GIF более портативны — они отображаются inline в README на GitHub, Slack и в письмах без необходимости плеера.
