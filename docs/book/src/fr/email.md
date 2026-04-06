# E-mail

PRECC peut envoyer des rapports et des fichiers par e-mail. Cela nécessite une configuration SMTP unique.

## Configuration

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

## Fichier de configuration

La configuration est stockée dans `~/.config/precc/mail.toml` :

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

Vous pouvez modifier ce fichier directement :

```bash
$EDITOR ~/.config/precc/mail.toml
```

Pour Gmail, utilisez un [mot de passe d'application](https://support.google.com/accounts/answer/185833) plutôt que votre mot de passe de compte.

## Envoi de rapports

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## Envoi de fichiers

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## Support de relais SSH

Si votre machine ne peut pas atteindre un serveur SMTP directement (par exemple, derrière un pare-feu d'entreprise), PRECC prend en charge le relais via un tunnel SSH :

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

PRECC établira le tunnel SSH automatiquement avant l'envoi.
