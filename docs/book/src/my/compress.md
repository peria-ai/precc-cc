# ချုံ့ခြင်း

`precc compress` သည် Claude Code တင်သောအခါ token အသုံးပြုမှုကို လျှော့ချရန် CLAUDE.md နှင့် အခြား context ဖိုင်များကို ချုံ့သည်။ ၎င်းသည် Pro အင်္ဂါရပ်ဖြစ်သည်။

## အခြေခံ အသုံးပြုမှု

```bash
$ precc compress .
[precc] Scanning directory: .
[precc] Found 3 context files:
         CLAUDE.md (2,847 tokens -> 1,203 tokens, -57.7%)
         ARCHITECTURE.md (4,112 tokens -> 2,044 tokens, -50.3%)
         ALTERNATIVES.md (3,891 tokens -> 1,967 tokens, -49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
[precc] Files compressed. Use --revert to restore originals.
```

## စမ်းသပ် လည်ပတ်ခြင်း

ဖိုင်များကို မပြောင်းလဲဘဲ ဘာတွေပြောင်းမလဲ ကြိုတင်ကြည့်ရှုခြင်း:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## ပြန်လည်ပြင်ဆင်ခြင်း

မူရင်းများကို အလိုအလျောက် အရန်ကူးထားသည်။ ပြန်ရယူရန်:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## ဘာတွေ ချုံ့မလဲ

ချုံ့စက်သည် အသွင်ပြောင်းမှုများ အများအပြား ကျင့်သုံးသည်:

- မလိုအပ်သော နေရာလွတ်နှင့် အလွတ်လိုင်းများကို ဖယ်ရှားသည်
- အဓိပ္ပါယ်ကို ထိန်းသိမ်းလျက် ရှည်လျားသော စကားရပ်များကို အတိုချုံ့သည်
- ဇယားများနှင့် စာရင်းများကို ချုံ့သည်
- မှတ်ချက်များနှင့် အလှဆင်ပုံစံချမှုများကို ဖယ်ရှားသည်
- ကုဒ်ဘလောက်၊ လမ်းကြောင်းနှင့် နည်းပညာ ခွဲခြားသတ်မှတ်မှုများအားလုံးကို ထိန်းသိမ်းသည်

ချုံ့ထားသော ရလဒ်သည် လူသားဖတ်နိုင်ဆဲဖြစ်သည် -- ချုံ့ခြင်း သို့မဟုတ် ရှုပ်ထွေးစေခြင်း မဟုတ်ပါ။

## တိကျသော ဖိုင်များကို ရွေးချယ်ခြင်း

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
