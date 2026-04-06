# အစီရင်ခံစာများ

`precc report` သည် PRECC လုပ်ဆောင်ချက်နှင့် token ချွေတာမှုကို အကျဉ်းချုပ် ခွဲခြမ်းစိတ်ဖြာမှု ဒတ်ရှ်ဘုတ် ဖန်တီးသည်။

## အစီရင်ခံစာ ဖန်တီးခြင်း

```bash
$ precc report
PRECC Report -- 2026-04-03
==========================

Sessions analyzed: 12
Commands intercepted: 87
Total token savings: 42,389

Top skills by activation:
  1. cargo-wrong-dir     34 activations   17,204 tokens saved
  2. npm-wrong-dir       18 activations    9,360 tokens saved
  3. git-wrong-dir       12 activations    4,944 tokens saved
  4. RTK rewrite         15 activations    3,750 tokens saved
  5. python-wrong-dir     8 activations    4,131 tokens saved

Savings by pillar:
  Pillar 1 (context resolution):  28,639 tokens  67.6%
  Pillar 4 (automation skills):    7,000 tokens  16.5%
  RTK rewrites:                    3,750 tokens   8.8%
  Lean-ctx wraps:                  3,000 tokens   7.1%

Recent corrections:
  2026-04-03 09:12  cargo build -> cd myapp && cargo build
  2026-04-03 09:18  npm test -> cd frontend && npm test
  2026-04-03 10:05  git status -> cd repo && git status
  ...
```

## အစီရင်ခံစာကို အီးမေးလ် ပို့ခြင်း

အစီရင်ခံစာကို အီးမေးလ်လိပ်စာသို့ ပို့ပါ (မေးလ်ဆက်တင် လိုအပ်သည်၊ [Email](email.md) ကြည့်ပါ):

```bash
$ precc report --email
[precc] Report sent to you@example.com
```

လက်ခံသူလိပ်စာကို `~/.config/precc/mail.toml` မှ ဖတ်သည်။ သီးခြားလိပ်စာသို့ ပို့ရန် `precc mail report EMAIL` ကိုလည်း သုံးနိုင်သည်။

## အစီရင်ခံစာ ဒေတာ

အစီရင်ခံစာများကို `~/.local/share/precc/history.db` ရှိ ဒေသတွင်း PRECC ဒေတာဘေ့စ်မှ ဖန်တီးသည်။ အစီရင်ခံစာကို အီးမေးလ်ဖြင့် တိကျစွာ မပို့မချင်း ဒေတာသည် သင့်စက်မှ မထွက်ပါ။
