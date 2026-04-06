# E-mail

PRECC może wysyłać raporty i pliki e-mailem. Wymaga jednorazowej konfiguracji SMTP.

## Konfiguracja

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

## Plik konfiguracyjny

Konfiguracja jest przechowywana w `~/.config/precc/mail.toml`:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

Możesz edytować ten plik bezpośrednio:

```bash
$EDITOR ~/.config/precc/mail.toml
```

W przypadku Gmaila użyj [Hasła aplikacji](https://support.google.com/accounts/answer/185833) zamiast hasła do konta.

## Wysyłanie raportów

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## Wysyłanie plików

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## Obsługa przekaźnika SSH

Jeśli Twoja maszyna nie może bezpośrednio połączyć się z serwerem SMTP (np. za firmowym firewallem), PRECC obsługuje przekazywanie przez tunel SSH:

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

PRECC automatycznie ustanowi tunel SSH przed wysłaniem.
