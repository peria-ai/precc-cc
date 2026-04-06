# 이메일

PRECC는 이메일로 보고서와 파일을 보낼 수 있습니다. 일회성 SMTP 설정이 필요합니다.

## 설정

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

## 구성 파일

구성은 `~/.config/precc/mail.toml`에 저장됩니다:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

이 파일을 직접 편집할 수 있습니다:

```bash
$EDITOR ~/.config/precc/mail.toml
```

Gmail의 경우 계정 비밀번호 대신 [앱 비밀번호](https://support.google.com/accounts/answer/185833)를 사용하세요.

## 보고서 보내기

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## 파일 보내기

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## SSH 릴레이 지원

기기가 SMTP 서버에 직접 연결할 수 없는 경우(예: 회사 방화벽 뒤), PRECC는 SSH 터널을 통한 릴레이를 지원합니다:

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

PRECC는 전송 전에 SSH 터널을 자동으로 설정합니다.
