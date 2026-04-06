# E-mail

PRECC může odesílat zprávy a soubory e-mailem. Vyžaduje jednorázové nastavení SMTP.

## Nastavení

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

## Konfigurační soubor

Konfigurace je uložena v `~/.config/precc/mail.toml`:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

Tento soubor můžete přímo upravit:

```bash
$EDITOR ~/.config/precc/mail.toml
```

Pro Gmail použijte [heslo aplikace](https://support.google.com/accounts/answer/185833) místo hesla účtu.

## Odesílání zpráv

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## Odesílání souborů

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## Podpora SSH relay

Pokud váš počítač nemůže přímo dosáhnout na SMTP server (např. za firemním firewallem), PRECC podporuje přeposílání přes SSH tunel:

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

PRECC automaticky vytvoří SSH tunel před odesláním.
