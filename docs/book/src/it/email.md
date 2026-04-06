# Email

PRECC può inviare report e file via email. Richiede una configurazione SMTP una tantum.

## Configurazione

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

## File di configurazione

La configurazione è memorizzata in `~/.config/precc/mail.toml`:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

Puoi modificare questo file direttamente:

```bash
$EDITOR ~/.config/precc/mail.toml
```

Per Gmail, usa una [Password per le app](https://support.google.com/accounts/answer/185833) invece della password del tuo account.

## Invio report

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## Invio file

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## Supporto relay SSH

Se la tua macchina non può raggiungere un server SMTP direttamente (es. dietro un firewall aziendale), PRECC supporta l'inoltro tramite tunnel SSH:

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

PRECC stabilirà automaticamente il tunnel SSH prima dell'invio.
