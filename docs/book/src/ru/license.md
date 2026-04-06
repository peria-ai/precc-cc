# Лицензия

PRECC предлагает два уровня: Community (бесплатный) и Pro.

## Уровень Community (бесплатный)

Уровень Community включает:

- Все встроенные навыки (коррекция директории, перевод jj и т.д.)
- Hook pipeline с полной поддержкой Pillar 1 и Pillar 4
- Базовая сводка `precc savings`
- Анализ сессий с помощью `precc ingest`
- Неограниченное локальное использование

## Уровень Pro

Pro открывает дополнительные функции:

- **Подробная разбивка экономии** -- `precc savings --all` с покомандным анализом
- **Запись GIF** -- `precc gif` для создания анимированных GIF терминала
- **IP-геозонирование** -- Для регулируемых сред
- **Отчёты по электронной почте** -- `precc mail report` для отправки аналитики
- **Анализ GitHub Actions** -- `precc gha` для отладки неудачных workflow
- **Сжатие контекста** -- `precc compress` для оптимизации CLAUDE.md
- **Приоритетная поддержка**

## Активация лицензии

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## Проверка статуса лицензии

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## Активация через GitHub Sponsors

Если вы спонсируете PRECC через GitHub Sponsors, ваша лицензия активируется автоматически через ваш email GitHub. Ключ не требуется — просто убедитесь, что ваш email спонсора совпадает:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## Отпечаток устройства

Каждая лицензия привязана к отпечатку устройства. Просмотрите свой:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

Если вам нужно перенести лицензию на новую машину, сначала деактивируйте:

```bash
precc license deactivate
```

Затем активируйте на новой машине.

## Лицензия истекла?

Когда лицензия Pro истекает, PRECC возвращается к уровню Community. Все встроенные навыки и основные функции продолжают работать. Только функции, специфичные для Pro, становятся недоступными. Подробности см. в [FAQ](faq.md).
