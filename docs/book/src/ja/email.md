# メール

PRECCはメールでレポートやファイルを送信できます。これには一度だけのSMTP設定が必要です。

## セットアップ

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

## 設定ファイル

設定は `~/.config/precc/mail.toml` に保存されます：

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

このファイルを直接編集できます：

```bash
$EDITOR ~/.config/precc/mail.toml
```

Gmailの場合は、アカウントパスワードではなく[アプリパスワード](https://support.google.com/accounts/answer/185833)を使用してください。

## レポートの送信

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## ファイルの送信

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## SSHリレーサポート

マシンがSMTPサーバーに直接到達できない場合（例：企業ファイアウォールの背後）、PRECCはSSHトンネル経由のリレーをサポートします：

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

PRECCは送信前にSSHトンネルを自動的に確立します。
