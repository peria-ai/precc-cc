# Tölvupóstur

PRECC getur sent skýrslur og skrár með tölvupósti. Þetta krefst stakrar SMTP-uppsetningar.

## Uppsetning

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

## Stillingarskrá

Stillingarnar eru geymdar í `~/.config/precc/mail.toml`:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

Þú getur breytt þessari skrá beint:

```bash
$EDITOR ~/.config/precc/mail.toml
```

Fyrir Gmail, notaðu [forritasamband](https://support.google.com/accounts/answer/185833) frekar en lykilorð reikningsins.

## Sending skýrslna

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## Sending skráa

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## SSH-miðlunarstuðningur

Ef tölvan þín nær ekki til SMTP-þjóns beint (t.d. á bak við eldvegg fyrirtækis), styður PRECC áframsendingu í gegnum SSH-göng:

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

PRECC mun setja upp SSH-göng sjálfkrafa fyrir sendingu.
