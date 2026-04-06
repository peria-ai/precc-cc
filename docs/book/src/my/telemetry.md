# အဝေးမှတိုင်းတာခြင်း

PRECC သည် ကိရိယာကို ပိုမိုကောင်းမွန်အောင် ကူညီရန် ရွေးချယ်နိုင်သော အမည်မသိ အဝေးမှတိုင်းတာခြင်းကို ပံ့ပိုးသည်။

## ပါဝင်ရန် ရွေးချယ်ခြင်း

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## ပါဝင်ခြင်းမှ ထွက်ရန်

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## အခြေအနေ စစ်ဆေးခြင်း

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## ပေးပို့မည့် ဒေတာ ကြိုကြည့်ခြင်း

ပါဝင်ရန် မရွေးချယ်မီ မည်သည့်ဒေတာများ စုဆောင်းမည်ကို တိကျစွာ ကြည့်ရှုနိုင်သည်:

```bash
$ precc telemetry preview
Telemetry payload (this session):
{
  "version": "0.3.0",
  "os": "linux",
  "arch": "x86_64",
  "skills_activated": 12,
  "commands_intercepted": 87,
  "pillars_used": [1, 4],
  "avg_hook_latency_ms": 2.3,
  "session_count": 1
}
```

## စုဆောင်းသော ဒေတာ

- PRECC ဗားရှင်း၊ OS နှင့် ဗိသုကာ
- စုစုပေါင်း အရေအတွက်: ကြားဖြတ်သည့် ကွန်မန်းများ၊ အသက်ဝင်သော ကျွမ်းကျင်မှုများ
- ပျမ်းမျှ hook ကြန့်ကြာချိန်
- ဆက်ရှင် အရေအတွက်

## မစုဆောင်းသော ဒေတာ

- ကွန်မန်း စာသား သို့မဟုတ် အငြင်းပွားချက်များ မပါ
- ဖိုင်လမ်းကြောင်း သို့မဟုတ် ဖိုင်တွဲအမည်များ မပါ
- ပရောဂျက်အမည် သို့မဟုတ် repository URL များ မပါ
- ကိုယ်ရေးကိုယ်တာ သတင်းအချက်အလက် (PII) မပါ
- IP လိပ်စာများ မပါ

## ပတ်ဝန်းကျင် ကိန်းရှင်ဖြင့် အစားထိုးခြင်း

ကွန်မန်း မလုပ်ဆောင်ဘဲ အဝေးမှတိုင်းတာခြင်းကို ပိတ်ရန် (CI သို့မဟုတ် မျှဝေ ပတ်ဝန်းကျင်များတွင် အသုံးဝင်):

```bash
export PRECC_NO_TELEMETRY=1
```

ဤအရာသည် သဘောတူညီချက် ဆက်တင်ထက် ဦးစားပေးသည်။

## ဒေတာ သွားရောက်ရာနေရာ

အဝေးမှတိုင်းတာခြင်း ဒေတာကို HTTPS မှတစ်ဆင့် `https://telemetry.peria.ai/v1/precc` သို့ ပေးပို့သည်။ ဒေတာကို အသုံးပြုပုံ နားလည်ရန်နှင့် ဖွံ့ဖြိုးရေး ဦးစားပေးရန်အတွက်သာ အသုံးပြုသည်။
