# Email

PRECC can send reports and files via email. This requires a one-time SMTP setup.

## Setup

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

## Configuration File

The configuration is stored at `~/.config/precc/mail.toml`:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

You can edit this file directly:

```bash
$EDITOR ~/.config/precc/mail.toml
```

For Gmail, use an [App Password](https://support.google.com/accounts/answer/185833) rather than your account password.

## Sending Reports

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## Sending Files

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## SSH Relay Support

If your machine cannot reach an SMTP server directly (e.g., behind a corporate firewall), PRECC supports relaying through an SSH tunnel:

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

PRECC will establish the SSH tunnel automatically before sending.
