# ایمیل

PRECC می‌تواند گزارش‌ها و فایل‌ها را از طریق ایمیل ارسال کند. این نیاز به تنظیم یکباره SMTP دارد.

## راه‌اندازی

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

## فایل پیکربندی

پیکربندی در `~/.config/precc/mail.toml` ذخیره می‌شود:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

می‌توانید این فایل را مستقیماً ویرایش کنید:

```bash
$EDITOR ~/.config/precc/mail.toml
```

برای Gmail، از [رمز عبور برنامه](https://support.google.com/accounts/answer/185833) به جای رمز عبور حساب خود استفاده کنید.

## ارسال گزارش‌ها

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## ارسال فایل‌ها

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## پشتیبانی از رله SSH

اگر دستگاه شما نمی‌تواند مستقیماً به سرور SMTP دسترسی پیدا کند (مثلاً پشت فایروال شرکتی)، PRECC از رله از طریق تونل SSH پشتیبانی می‌کند:

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

PRECC قبل از ارسال به طور خودکار تونل SSH را برقرار می‌کند.
