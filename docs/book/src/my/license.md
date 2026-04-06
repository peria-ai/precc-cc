# လိုင်စင်

PRECC သည် အဆင့်နှစ်ခုပေးသည်: Community (အခမဲ့) နှင့် Pro။

## Community အဆင့် (အခမဲ့)

Community အဆင့်တွင် ပါဝင်သည်:

- ပါဝင်သော ကျွမ်းကျင်မှုအားလုံး (directory ပြင်ဆင်ခြင်း၊ jj ဘာသာပြန်ခြင်း စသည်)
- Pillar 1 နှင့် Pillar 4 အပြည့်အဝ ထောက်ပံ့သည့် Hook pipeline
- အခြေခံ `precc savings` အကျဉ်းချုပ်
- `precc ingest` ဖြင့် session mining
- အကန့်အသတ်မရှိ local အသုံးပြုမှု

## Pro အဆင့်

Pro သည် အပိုအင်္ဂါရပ်များကို ဖွင့်ပေးသည်:

- **အသေးစိတ်ချွေတာမှုခွဲခြမ်းစိတ်ဖြာမှု** -- `precc savings --all` command တစ်ခုချင်းစီ ခွဲခြမ်းစိတ်ဖြာမှုနှင့်
- **GIF မှတ်တမ်းတင်ခြင်း** -- `precc gif` animated terminal GIF များ ဖန်တီးရန်
- **IP geofence လိုက်နာမှု** -- ထိန်းချုပ်ထားသော ပတ်ဝန်းကျင်များအတွက်
- **အီးမေးလ် အစီရင်ခံစာများ** -- `precc mail report` analytics ပို့ရန်
- **GitHub Actions ခွဲခြမ်းစိတ်ဖြာမှု** -- `precc gha` မအောင်မြင်သော workflow debugging အတွက်
- **Context ချုံ့ခြင်း** -- `precc compress` CLAUDE.md optimization အတွက်
- **ဦးစားပေး ပံ့ပိုးမှု**

## လိုင်စင် အသက်သွင်းခြင်း

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## လိုင်စင်အခြေအနေ စစ်ဆေးခြင်း

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## GitHub Sponsors အသက်သွင်းခြင်း

GitHub Sponsors မှတဆင့် PRECC ကို ပံ့ပိုးပါက သင့် GitHub email မှတဆင့် လိုင်စင်ကို အလိုအလျောက် အသက်သွင်းပေးသည်။ key မလိုအပ်ပါ -- sponsor email တူညီကြောင်း သေချာပါ:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## စက်ပစ္စည်း လက်ဗွေ

လိုင်စင်တစ်ခုစီသည် စက်ပစ္စည်းလက်ဗွေနှင့် ချိတ်ဆက်ထားသည်။ သင့်အရာကို ကြည့်ရန်:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

လိုင်စင်ကို စက်အသစ်သို့ လွှဲပြောင်းလိုပါက အရင်ပိတ်ပါ:

```bash
precc license deactivate
```

ပြီးနောက် စက်အသစ်တွင် အသက်သွင်းပါ။

## လိုင်စင်သက်တမ်းကုန်ပြီလား?

Pro လိုင်စင်သက်တမ်းကုန်သောအခါ PRECC သည် Community အဆင့်သို့ ပြန်သွားသည်။ ပါဝင်သော ကျွမ်းကျင်မှုများနှင့် အဓိကလုပ်ဆောင်ချက်များ ဆက်လက်အလုပ်လုပ်သည်။ Pro သီးသန့်အင်္ဂါရပ်များသာ မရနိုင်တော့ပါ။ အသေးစိတ်အတွက် [FAQ](faq.md) ကိုကြည့်ပါ။
