# E-post

PRECC kan skicka rapporter och filer via e-post. Det kräver en engångskonfiguration av SMTP.

## Installation

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

## Konfigurationsfil

Konfigurationen lagras i `~/.config/precc/mail.toml`:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

Du kan redigera denna fil direkt:

```bash
$EDITOR ~/.config/precc/mail.toml
```

För Gmail, använd ett [Applösenord](https://support.google.com/accounts/answer/185833) istället för ditt kontolösenord.

## Skicka rapporter

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## Skicka filer

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## SSH-relästöd

Om din maskin inte kan nå en SMTP-server direkt (t.ex. bakom en företagsbrandvägg), stöder PRECC vidarebefordran genom en SSH-tunnel:

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

PRECC upprättar automatiskt SSH-tunneln före sändning.
