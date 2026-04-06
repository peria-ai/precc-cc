# E-mail

PRECC poate trimite rapoarte și fișiere prin e-mail. Aceasta necesită o configurare SMTP unică.

## Configurare

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

## Fișier de configurare

Configurația este stocată în `~/.config/precc/mail.toml`:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

Puteți edita acest fișier direct:

```bash
$EDITOR ~/.config/precc/mail.toml
```

Pentru Gmail, folosiți o [Parolă de aplicație](https://support.google.com/accounts/answer/185833) în loc de parola contului.

## Trimiterea rapoartelor

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## Trimiterea fișierelor

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## Suport releu SSH

Dacă mașina dvs. nu poate ajunge la un server SMTP direct (de ex., în spatele unui firewall corporativ), PRECC suportă retransmiterea printr-un tunel SSH:

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

PRECC va stabili automat tunelul SSH înainte de trimitere.
