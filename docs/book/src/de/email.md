# E-Mail

PRECC kann Berichte und Dateien per E-Mail senden. Dies erfordert eine einmalige SMTP-Einrichtung.

## Einrichtung

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

## Konfigurationsdatei

Die Konfiguration wird unter `~/.config/precc/mail.toml` gespeichert:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

Sie können diese Datei direkt bearbeiten:

```bash
$EDITOR ~/.config/precc/mail.toml
```

Verwenden Sie für Gmail ein [App-Passwort](https://support.google.com/accounts/answer/185833) anstelle Ihres Kontopassworts.

## Berichte senden

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## Dateien senden

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## SSH-Relay-Unterstützung

Wenn Ihr Rechner keinen SMTP-Server direkt erreichen kann (z.B. hinter einer Firmen-Firewall), unterstützt PRECC die Weiterleitung über einen SSH-Tunnel:

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

PRECC stellt den SSH-Tunnel vor dem Senden automatisch her.
