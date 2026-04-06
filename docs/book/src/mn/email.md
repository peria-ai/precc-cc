# Имэйл

PRECC имэйлээр тайлан, файл илгээх боломжтой. Нэг удаагийн SMTP тохиргоо шаардлагатай.

## Тохиргоо

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

## Тохиргооны файл

Тохиргоо нь `~/.config/precc/mail.toml` файлд хадгалагдана:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

Энэ файлыг шууд засварлах боломжтой:

```bash
$EDITOR ~/.config/precc/mail.toml
```

Gmail-ийн хувьд бүртгэлийн нууц үгийн оронд [App Password](https://support.google.com/accounts/answer/185833) ашиглана уу.

## Тайлан илгээх

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## Файл илгээх

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## SSH relay дэмжлэг

Таны машин SMTP серверт шууд хүрч чадахгүй бол (жишээ нь байгууллагын firewall-ийн ард), PRECC SSH tunnel-ээр дамжуулан relay хийхийг дэмждэг:

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

PRECC илгээхийн өмнө SSH tunnel-ийг автоматаар үүсгэнэ.
