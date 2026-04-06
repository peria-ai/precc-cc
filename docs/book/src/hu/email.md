# E-mail

A PRECC e-mailben küldhet jelentéseket és fájlokat. Ehhez egyszeri SMTP-beállítás szükséges.

## Beállítás

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

## Konfigurációs fájl

A konfiguráció a `~/.config/precc/mail.toml` fájlban van tárolva:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

Ezt a fájlt közvetlenül szerkesztheti:

```bash
$EDITOR ~/.config/precc/mail.toml
```

Gmailhez használjon [Alkalmazásjelszót](https://support.google.com/accounts/answer/185833) a fiókjelszó helyett.

## Jelentések küldése

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## Fájlok küldése

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## SSH-relay támogatás

Ha a gépe nem éri el közvetlenül az SMTP-szervert (pl. vállalati tűzfal mögött), a PRECC támogatja az SSH-alagúton keresztüli továbbítást:

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

A PRECC automatikusan létrehozza az SSH-alagutat küldés előtt.
