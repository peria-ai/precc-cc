# အီးမေးလ်

PRECC သည် အီးမေးလ်ဖြင့် အစီရင်ခံစာများနှင့် ဖိုင်များ ပေးပို့နိုင်သည်။ SMTP တစ်ကြိမ်တည်း သတ်မှတ်ရန် လိုအပ်သည်။

## စနစ်ထည့်သွင်းခြင်း

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

## ပြင်ဆင်သတ်မှတ်ချက် ဖိုင်

ပြင်ဆင်သတ်မှတ်ချက်ကို `~/.config/precc/mail.toml` တွင် သိမ်းဆည်းထားသည်:

```toml
[smtp]
host = "smtp.gmail.com"
port = 587
username = "you@gmail.com"
password = "app-password-here"
from = "you@gmail.com"
tls = true
```

ဤဖိုင်ကို တိုက်ရိုက် ပြင်ဆင်နိုင်သည်:

```bash
$EDITOR ~/.config/precc/mail.toml
```

Gmail အတွက် သင့်အကောင့် စကားဝှက်အစား [App Password](https://support.google.com/accounts/answer/185833) ကို အသုံးပြုပါ။

## အစီရင်ခံစာများ ပေးပို့ခြင်း

```bash
$ precc mail report team@example.com
[precc] Generating report...
[precc] Sending to team@example.com...
[precc] Report sent.
```

## ဖိုင်များ ပေးပို့ခြင်း

```bash
$ precc mail send colleague@example.com output.log
[precc] Sending output.log to colleague@example.com...
[precc] Sent (14.2 KB).
```

## SSH relay ပံ့ပိုးမှု

သင့်စက်သည် SMTP ဆာဗာကို တိုက်ရိုက် ချိတ်ဆက်၍ မရပါက (ဥပမာ- ကုမ္ပဏီ firewall နောက်ကွယ်တွင်)၊ PRECC သည် SSH tunnel မှတစ်ဆင့် relay ပေးပို့ခြင်းကို ပံ့ပိုးသည်:

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

PRECC သည် ပေးပို့ခြင်းမပြုမီ SSH tunnel ကို အလိုအလျောက် ထူထောင်မည်ဖြစ်သည်။
