# อีเมล

PRECC สามารถส่งรายงานและไฟล์ทางอีเมลได้ ต้องตั้งค่า SMTP ครั้งเดียว

## การตั้งค่า

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

## ไฟล์การกำหนดค่า

การกำหนดค่าถูกเก็บไว้ที่ `~/.config/precc/mail.toml`:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

คุณสามารถแก้ไขไฟล์นี้ได้โดยตรง:

```bash
$EDITOR ~/.config/precc/mail.toml
```

สำหรับ Gmail ให้ใช้ [รหัสผ่านแอป](https://support.google.com/accounts/answer/185833) แทนรหัสผ่านบัญชี

## การส่งรายงาน

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## การส่งไฟล์

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## การรองรับรีเลย์ SSH

หากเครื่องของคุณไม่สามารถเข้าถึงเซิร์ฟเวอร์ SMTP ได้โดยตรง (เช่น อยู่หลังไฟร์วอลล์ขององค์กร) PRECC รองรับการรีเลย์ผ่านอุโมงค์ SSH:

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

PRECC จะสร้างอุโมงค์ SSH โดยอัตโนมัติก่อนส่ง
