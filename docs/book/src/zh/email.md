# 电子邮件

PRECC可以通过电子邮件发送报告和文件。这需要一次性的SMTP设置。

## 设置

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

配置存储在 `~/.config/precc/mail.toml`：

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

您可以直接编辑此文件：

```bash
$EDITOR ~/.config/precc/mail.toml
```

对于Gmail，请使用[应用密码](https://support.google.com/accounts/answer/185833)而不是您的账户密码。

## 发送报告

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## 发送文件

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## SSH中继支持

如果您的机器无法直接访问SMTP服务器（例如，在企业防火墙后面），PRECC支持通过SSH隧道中继：

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

PRECC将在发送前自动建立SSH隧道。
