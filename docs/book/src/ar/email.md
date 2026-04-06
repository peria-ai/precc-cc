# البريد الإلكتروني

يمكن لـ PRECC إرسال التقارير والملفات عبر البريد الإلكتروني. يتطلب هذا إعداد SMTP لمرة واحدة.

## الإعداد

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

## ملف التكوين

يتم تخزين التكوين في `~/.config/precc/mail.toml`:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

يمكنك تحرير هذا الملف مباشرة:

```bash
$EDITOR ~/.config/precc/mail.toml
```

بالنسبة لـ Gmail، استخدم [كلمة مرور التطبيق](https://support.google.com/accounts/answer/185833) بدلاً من كلمة مرور حسابك.

## إرسال التقارير

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## إرسال الملفات

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## دعم ترحيل SSH

إذا لم يتمكن جهازك من الوصول إلى خادم SMTP مباشرة (مثلاً خلف جدار حماية مؤسسي)، يدعم PRECC الترحيل عبر نفق SSH:

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

سيقوم PRECC بإنشاء نفق SSH تلقائياً قبل الإرسال.
