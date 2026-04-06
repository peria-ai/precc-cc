# Электронная почта

PRECC может отправлять отчёты и файлы по электронной почте. Требуется однократная настройка SMTP.

## Настройка

```bash
$ precc mail setup
SMTP host: smtp.gmail.com
SMTP port [587]: 587
Username: you@gmail.com
Password: ********
From address [you@gmail.com]: you@gmail.com
[precc] Mail configuration saved to ~/.config/precc/mail.toml
[precc] Sending test email to you@gmail.com...
[precc] Test email sent successfully.
```

## Файл конфигурации

Конфигурация хранится в `~/.config/precc/mail.toml`:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

Вы можете редактировать этот файл напрямую:

```bash
$EDITOR ~/.config/precc/mail.toml
```

Для Gmail используйте [пароль приложения](https://support.google.com/accounts/answer/185833) вместо пароля учётной записи.

## Отправка отчётов

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## Отправка файлов

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## Поддержка SSH-ретрансляции

Если ваша машина не может подключиться к SMTP-серверу напрямую (например, за корпоративным файрволом), PRECC поддерживает пересылку через SSH-туннель:

```toml
[smtp]
host = "localhost"
port = 2525

[ssh_relay]
host = "relay.example.com"
user = "you"
remote_port = 587
local_port = 2525
```

PRECC автоматически установит SSH-туннель перед отправкой.
