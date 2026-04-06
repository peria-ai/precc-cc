# {{i18n:eml_title}}

{{i18n:eml_intro}}

## {{i18n:eml_setup_title}}

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

## {{i18n:eml_config_title}}

{{i18n:eml_config_body}}

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

{{i18n:eml_config_edit}}

```bash
$EDITOR ~/.config/precc/mail.toml
```

{{i18n:eml_gmail_note}}

## {{i18n:eml_sending_reports_title}}

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## {{i18n:eml_sending_files_title}}

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## {{i18n:eml_ssh_relay_title}}

{{i18n:eml_ssh_relay_body}}

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

{{i18n:eml_ssh_relay_auto}}
