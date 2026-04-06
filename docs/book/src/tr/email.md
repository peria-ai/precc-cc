# E-posta

PRECC e-posta ile raporlar ve dosyalar gönderebilir. Bu, tek seferlik bir SMTP kurulumu gerektirir.

## Kurulum

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

## Yapılandırma dosyası

Yapılandırma `~/.config/precc/mail.toml` dosyasında saklanır:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

Bu dosyayı doğrudan düzenleyebilirsiniz:

```bash
$EDITOR ~/.config/precc/mail.toml
```

Gmail için hesap şifreniz yerine bir [Uygulama Şifresi](https://support.google.com/accounts/answer/185833) kullanın.

## Rapor gönderme

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## Dosya gönderme

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## SSH aktarma desteği

Makineniz doğrudan bir SMTP sunucusuna erişemiyorsa (örneğin, kurumsal bir güvenlik duvarı arkasında), PRECC bir SSH tüneli üzerinden aktarmayı destekler:

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

PRECC göndermeden önce SSH tünelini otomatik olarak kuracaktır.
