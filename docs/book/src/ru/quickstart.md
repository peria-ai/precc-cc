# Быстрый старт

Запустите PRECC за 5 минут.

## Шаг 1: Установка

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## Шаг 2: Инициализация

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## Шаг 3: Проверка активности hook

```bash
$ precc skills list
  # Name               Type      Triggers
  1 cargo-wrong-dir    built-in  cargo build/test/clippy outside Rust project
  2 git-wrong-dir      built-in  git * outside a repo
  3 go-wrong-dir       built-in  go build/test outside Go module
  4 make-wrong-dir     built-in  make without Makefile in cwd
  5 npm-wrong-dir      built-in  npm/npx/pnpm/yarn outside Node project
  6 python-wrong-dir   built-in  python/pytest/pip outside Python project
  7 jj-translate       built-in  git * in jj-colocated repo
  8 asciinema-gif      built-in  asciinema rec
```

## Шаг 4: Используйте Claude Code как обычно

Откройте Claude Code и работайте как обычно. PRECC работает тихо в фоне. Когда Claude выполняет команду, которая бы завершилась ошибкой, PRECC исправляет её до выполнения.

### Пример: сборка Cargo в неправильной директории

Допустим, ваш проект находится в `~/projects/myapp/`, и Claude выполняет:

```
cargo build
```

из `~/projects/` (на один уровень выше, `Cargo.toml` там нет).

**Без PRECC:** Claude получает ошибку `could not find Cargo.toml in /home/user/projects or any parent directory`, читает её, рассуждает и повторяет с `cd myapp && cargo build`. Стоимость: ~2 000 потраченных токенов.

**С PRECC:** Hook обнаруживает отсутствующий `Cargo.toml`, находит его в `myapp/` и переписывает команду на:

```
cd /home/user/projects/myapp && cargo build
```

Claude никогда не видит ошибку. Ноль потраченных токенов.

## Шаг 5: Проверьте свою экономию

После сессии посмотрите, сколько токенов PRECC сэкономил:

```bash
$ precc savings
Session Token Savings
=====================
Total estimated savings: 4,312 tokens

Breakdown:
  Pillar 1 (cd prepends):       2,104 tokens  (3 corrections)
  Pillar 4 (skill activations):   980 tokens  (2 activations)
  RTK rewrites:                 1,228 tokens  (5 rewrites)
```

## Следующие шаги

- [Навыки](skills.md) -- Все доступные навыки и как создать свои.
- [Hook Pipeline](hook-pipeline.md) -- Узнайте, что происходит под капотом.
- [Экономия](savings.md) -- Подробный анализ экономии токенов.
