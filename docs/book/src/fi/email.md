# Sähköposti

PRECC voi lähettää raportteja ja tiedostoja sähköpostitse. Tämä vaatii kertaluonteisen SMTP-asetuksen.

## Asetukset

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

## Asetustiedosto

Asetukset tallennetaan tiedostoon `~/.config/precc/mail.toml`:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

Voit muokata tätä tiedostoa suoraan:

```bash
$EDITOR ~/.config/precc/mail.toml
```

Gmailissa käytä [sovelluskohtaista salasanaa](https://support.google.com/accounts/answer/185833) tilisi salasanan sijaan.

## Raporttien lähettäminen

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## Tiedostojen lähettäminen

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## SSH-välitystuki

Jos koneesi ei pääse SMTP-palvelimelle suoraan (esim. yrityksen palomuurin takana), PRECC tukee välitystä SSH-tunnelin kautta:

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

PRECC muodostaa SSH-tunnelin automaattisesti ennen lähettämistä.
