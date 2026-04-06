# Email

PRECC có thể gửi báo cáo và tệp qua email. Điều này yêu cầu cấu hình SMTP một lần.

## Cài đặt

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

## Tệp cấu hình

Cấu hình được lưu tại `~/.config/precc/mail.toml`:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

Bạn có thể chỉnh sửa tệp này trực tiếp:

```bash
$EDITOR ~/.config/precc/mail.toml
```

Đối với Gmail, hãy sử dụng [Mật khẩu ứng dụng](https://support.google.com/accounts/answer/185833) thay vì mật khẩu tài khoản.

## Gửi báo cáo

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## Gửi tệp

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## Hỗ trợ chuyển tiếp SSH

Nếu máy của bạn không thể truy cập trực tiếp máy chủ SMTP (ví dụ: sau tường lửa công ty), PRECC hỗ trợ chuyển tiếp qua đường hầm SSH:

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

PRECC sẽ tự động thiết lập đường hầm SSH trước khi gửi.
