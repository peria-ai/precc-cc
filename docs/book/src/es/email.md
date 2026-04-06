# Correo electrónico

PRECC puede enviar informes y archivos por correo electrónico. Esto requiere una configuración SMTP única.

## Configuración

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

## Archivo de configuración

La configuración se almacena en `~/.config/precc/mail.toml`:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

Puede editar este archivo directamente:

```bash
$EDITOR ~/.config/precc/mail.toml
```

Para Gmail, use una [Contraseña de aplicación](https://support.google.com/accounts/answer/185833) en lugar de su contraseña de cuenta.

## Envío de informes

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## Envío de archivos

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## Soporte de retransmisión SSH

Si su máquina no puede acceder directamente a un servidor SMTP (por ejemplo, detrás de un firewall corporativo), PRECC soporta la retransmisión a través de un túnel SSH:

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

PRECC establecerá el túnel SSH automáticamente antes de enviar.
