# E-mail

PRECC kan sende rapporter og filer via e-mail. Det kræver en engangskonfiguration af SMTP.

## Opsætning

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

Konfigurationen er gemt i `~/.config/precc/mail.toml`:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

Du kan redigere denne fil direkte:

```bash
$EDITOR ~/.config/precc/mail.toml
```

Til Gmail, brug en [App-adgangskode](https://support.google.com/accounts/answer/185833) i stedet for din kontoadgangskode.

## Afsendelse af rapporter

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## Afsendelse af filer

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## SSH-relay-understøttelse

Hvis din maskine ikke kan nå en SMTP-server direkte (f.eks. bag en virksomhedsfirewall), understøtter PRECC videresendelse via en SSH-tunnel:

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

PRECC etablerer automatisk SSH-tunnelen før afsendelse.
