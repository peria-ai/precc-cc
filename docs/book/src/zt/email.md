# 電子郵件

PRECC可以通過電子郵件發送報告和文件。這需要一次性的SMTP設置。

## 設置

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

## 配置文件

配置存儲在 `~/.config/precc/mail.toml`：

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

您可以直接編輯此文件：

```bash
$EDITOR ~/.config/precc/mail.toml
```

對於Gmail，請使用[應用密碼](https://support.google.com/accounts/answer/185833)而不是您的賬戶密碼。

## 發送報告

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## 發送文件

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## SSH中繼支持

如果您的機器無法直接訪問SMTP服務器（例如，在企業防火牆後面），PRECC支持通過SSH隧道中繼：

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

PRECC將在發送前自動建立SSH隧道。
