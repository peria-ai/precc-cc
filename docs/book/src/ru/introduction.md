# Введение

## Что такое PRECC?

PRECC (Предиктивная коррекция ошибок для Claude Code) — это инструмент на Rust, который перехватывает bash-команды Claude Code через официальный механизм хуков PreToolUse. Он исправляет ошибки *до их возникновения*, экономя токены и устраняя циклы повторов.

Бесплатно для участников сообщества.

## Проблема

Claude Code тратит значительное количество токенов на предотвратимые ошибки:

- **Ошибки каталога** — выполнение `cargo build` в каталоге без `Cargo.toml`.
- **Циклы повторов** — неудачная команда создаёт многословный вывод, Claude анализирует и повторяет.
- **Многословный вывод** — команды вроде `find` или `ls -R` выводят тысячи строк.

## Четыре столпа

### Исправление контекста (cd-prepend)

Определяет, когда команды вроде `cargo build` или `npm test` выполняются в неправильном каталоге, и добавляет `cd /правильный/путь &&` перед выполнением.

### Отладка GDB

Обнаруживает возможности подключения GDB для углублённой отладки segfault и сбоев.

### Анализ сессий

Анализирует логи сессий Claude Code для поиска пар ошибка-исправление.

### Навыки автоматизации

Библиотека навыков, которые сопоставляют шаблоны команд и перезаписывают их.

## Как это работает (30-секундная версия)

1. Claude Code собирается выполнить bash-команду.
2. Хук PreToolUse отправляет команду в `precc-hook` как JSON.
3. `precc-hook` обрабатывает команду менее чем за 3 миллисекунды.
4. Исправленная команда возвращается как JSON.
5. Claude Code выполняет исправленную команду.

Claude никогда не видит ошибку.

### Адаптивное сжатие

Если команда завершается ошибкой после сжатия, PRECC автоматически пропускает сжатие при следующей попытке, чтобы Claude получил полный несжатый вывод для отладки.

## Статистика использования в реальном времени

Текущая версия <span data-stat="current_version">--</span>:

| Метрика | Значение |
|---|---|
| Вызовы хука | <span data-stat="total_invocations">--</span> |
| Сэкономлено токенов | <span data-stat="total_tokens_saved">--</span> |
| Коэффициент экономии | <span data-stat="saving_pct">--</span>% |
| Перезаписей RTK | <span data-stat="rtk_rewrites">--</span> |
| Коррекций CD | <span data-stat="cd_prepends">--</span> |
| Задержка хука | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| Пользователи | <span data-stat="unique_users">--</span> |

### Измеренная экономия (реальные данные)

<div id="measured-savings" style="display:none">
<table id="measured-summary">
<thead><tr><th>Метрика</th><th>Значение</th></tr></thead>
<tbody>
<tr><td>Исходные токены вывода (без PRECC)</td><td><span data-measured="original_output_tokens">--</span></td></tr>
<tr><td>Фактические токены вывода (с PRECC)</td><td><span data-measured="actual_output_tokens">--</span></td></tr>
<tr><td>Сэкономлено токенов</td><td><strong><span data-measured="savings_tokens">--</span></strong></td></tr>
<tr><td>Коэффициент экономии</td><td><strong><span data-measured="savings_pct">--</span>%</strong></td></tr>
<tr><td>Реальные измерения</td><td><span data-measured="ground_truth_count">--</span> измерений</td></tr>
</tbody>
</table>
</div>

<div id="rewrite-type-breakdown" style="display:none">

#### По типу перезаписи

<table id="rewrite-type-table">
<thead><tr><th>Тип</th><th>Количество</th><th>Ср. экономия %</th><th>Сэкономлено токенов</th></tr></thead>
<tbody><tr><td colspan="4"><em>Загрузка...</em></td></tr></tbody>
</table>
</div>

### Экономия по версиям

<table id="version-breakdown" style="display:none">
<thead><tr><th>Версия</th><th>Пользователи</th><th>Вызовы хука</th><th>Сэкономлено токенов</th><th>Коэффициент экономии</th></tr></thead>
<tbody><tr><td colspan="5"><em>Загрузка...</em></td></tr></tbody>
</table>

<small>Эти цифры обновляются автоматически из анонимизированной телеметрии.</small>

## Ссылки

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Веб-сайт: [https://peria.ai](https://peria.ai)
- Документация: [https://precc.cc](https://precc.cc)
