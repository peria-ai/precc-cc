# E-mail

PRECC pode enviar relatórios e arquivos por e-mail. Isso requer uma configuração SMTP única.

## Configuração

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

## Arquivo de configuração

A configuração é armazenada em `~/.config/precc/mail.toml`:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

Você pode editar este arquivo diretamente:

```bash
$EDITOR ~/.config/precc/mail.toml
```

Para Gmail, use uma [Senha de app](https://support.google.com/accounts/answer/185833) em vez da senha da sua conta.

## Enviando relatórios

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## Enviando arquivos

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## Suporte a retransmissão SSH

Se sua máquina não pode acessar um servidor SMTP diretamente (por exemplo, atrás de um firewall corporativo), PRECC suporta retransmissão via túnel SSH:

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

PRECC estabelecerá o túnel SSH automaticamente antes de enviar.
