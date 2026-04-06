# အမြန်စတင်ခြင်း

PRECC ကို မိနစ် ၅ အတွင်း စတင်ပါ။

## အဆင့် ၁: တပ်ဆင်ခြင်း

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## အဆင့် ၂: စတင်ခြင်း

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## အဆင့် ၃: Hook အလုပ်လုပ်နေသည်ကို စစ်ဆေးခြင်း

```bash
$ precc skills list
  # Name               Type      Triggers
  1 cargo-wrong-dir    built-in  cargo build/test/clippy outside Rust project
  2 git-wrong-dir      built-in  git * outside a repo
  3 go-wrong-dir       built-in  go build/test outside Go module
  4 make-wrong-dir     built-in  make without Makefile in cwd
  5 npm-wrong-dir      built-in  npm/npx/pnpm/yarn outside Node project
  6 python-wrong-dir   built-in  python/pytest/pip outside Python project
  7 jj-translate       built-in  git * in jj-colocated repo
  8 asciinema-gif      built-in  asciinema rec
```

## အဆင့် ၄: Claude Code ကို ပုံမှန်အတိုင်း အသုံးပြုခြင်း

Claude Code ကိုဖွင့်ပြီး ပုံမှန်အတိုင်း အလုပ်လုပ်ပါ။ PRECC သည် နောက်ကွယ်တွင် တိတ်ဆိတ်စွာ အလုပ်လုပ်နေသည်။ Claude က ပျက်ကွက်မည့် command တစ်ခုကို ထုတ်သောအခါ PRECC က လုပ်ဆောင်မှုမတိုင်မီ ပြင်ဆင်ပေးသည်။

### ဥပမာ: မှားယွင်းသော directory တွင် Cargo Build

သင့်ပရောဂျက်သည် `~/projects/myapp/` တွင်ရှိပြီး Claude က အောက်ပါ command ကို ထုတ်သည်ဟု ယူဆပါ:

```
cargo build
```

`~/projects/` မှ (အဆင့်တစ်ခု မြင့်လွန်းပြီး ထိုနေရာတွင် `Cargo.toml` မရှိပါ)။

**PRECC မရှိဘဲ:** Claude သည် `could not find Cargo.toml in /home/user/projects or any parent directory` error ကိုရရှိပြီး ဖတ်ပြီး ခွဲခြမ်းစိတ်ဖြာကာ `cd myapp && cargo build` ဖြင့် ပြန်ကြိုးစားသည်။ ကုန်ကျစရိတ်: token ~၂,၀၀၀ ဖြုန်းတီး။

**PRECC ဖြင့်:** Hook သည် ပျောက်ဆုံးနေသော `Cargo.toml` ကို ရှာဖွေတွေ့ရှိပြီး `myapp/` တွင် တွေ့ကာ command ကို ပြန်လည်ရေးသားသည်:

```
cd /home/user/projects/myapp && cargo build
```

Claude သည် error ကို ဘယ်တော့မှ မမြင်ရပါ။ token ဖြုန်းတီးမှု သုညဖြစ်သည်။

## အဆင့် ၅: သင့်ချွေတာမှုကို စစ်ဆေးခြင်း

session တစ်ခုပြီးနောက် PRECC က token ဘယ်လောက် ချွေတာခဲ့သည်ကို ကြည့်ပါ:

```bash
$ precc savings
Session Token Savings
=====================
Total estimated savings: 4,312 tokens

Breakdown:
  Pillar 1 (cd prepends):       2,104 tokens  (3 corrections)
  Pillar 4 (skill activations):   980 tokens  (2 activations)
  RTK rewrites:                 1,228 tokens  (5 rewrites)
```

## နောက်ထပ် အဆင့်များ

- [ကျွမ်းကျင်မှုများ](skills.md) -- ရရှိနိုင်သော ကျွမ်းကျင်မှုအားလုံးနှင့် သင့်ကိုယ်ပိုင် ဖန်တီးနည်းကို ကြည့်ပါ။
- [Hook Pipeline](hook-pipeline.md) -- အတွင်းပိုင်းတွင် ဘာဖြစ်နေသည်ကို နားလည်ပါ။
- [ချွေတာမှု](savings.md) -- Token ချွေတာမှု အသေးစိတ် ခွဲခြမ်းစိတ်ဖြာမှု။
