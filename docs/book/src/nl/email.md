# E-mail

PRECC kan rapporten en bestanden per e-mail verzenden. Dit vereist een eenmalige SMTP-configuratie.

## Instelling

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

## Configuratiebestand

De configuratie wordt opgeslagen in `~/.config/precc/mail.toml`:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

U kunt dit bestand rechtstreeks bewerken:

```bash
$EDITOR ~/.config/precc/mail.toml
```

Gebruik voor Gmail een [App-wachtwoord](https://support.google.com/accounts/answer/185833) in plaats van uw accountwachtwoord.

## Rapporten verzenden

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## Bestanden verzenden

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## SSH-relay ondersteuning

Als uw machine geen SMTP-server rechtstreeks kan bereiken (bijv. achter een bedrijfsfirewall), ondersteunt PRECC relay via een SSH-tunnel:

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

PRECC zal de SSH-tunnel automatisch opzetten voor het verzenden.
