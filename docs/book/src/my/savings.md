# ချွေတာမှု

PRECC သည် ကြားဖြတ်မှုတိုင်းမှ ခန့်မှန်းတိုကင်ချွေတာမှုကို ခြေရာခံသည်။ PRECC မည်မျှ ဖြုန်းတီးမှုကို တားဆီးခဲ့သည်ကို ကြည့်ရန် `precc savings` ကို သုံးပါ။

## အကျဉ်းချုပ်

```bash
$ precc savings
Session Token Savings
=====================
Total estimated savings: <span data-stat="session_tokens_saved">8,741</span> tokens

Breakdown:
  Pillar 1 (cd prepends):         <span data-stat="session_p1_tokens">3,204</span> tokens  (<span data-stat="session_p1_count">6</span> corrections)
  Pillar 4 (skill activations):   <span data-stat="session_p4_tokens">1,560</span> tokens  (<span data-stat="session_p4_count">4</span> activations)
  RTK rewrites:                   <span data-stat="session_rtk_tokens">2,749</span> tokens  (<span data-stat="session_rtk_count">11</span> rewrites)
  Lean-ctx wraps:                 <span data-stat="session_lean_tokens">1,228</span> tokens  (<span data-stat="session_lean_count">2</span> wraps)
```

## အသေးစိတ်ခွဲခြမ်းစိတ်ဖြာမှု (Pro)

```bash
$ precc savings --all
Session Token Savings (Detailed)
================================
Total estimated savings: <span data-stat="session_tokens_saved">8,741</span> tokens

Command-by-command:
  #  Time   Command                          Saving   Source
  1  09:12  cargo build                      534 tk   cd prepend (cargo-wrong-dir)
  2  09:14  cargo test                       534 tk   cd prepend (cargo-wrong-dir)
  3  09:15  git status                       412 tk   cd prepend (git-wrong-dir)
  4  09:18  npm install                      824 tk   cd prepend (npm-wrong-dir)
  5  09:22  find . -name "*.rs"              387 tk   RTK rewrite (output truncation)
  6  09:25  cat src/main.rs                  249 tk   RTK rewrite (lean-ctx wrap)
  7  09:31  cargo clippy                     534 tk   cd prepend (cargo-wrong-dir)
  ...

Pillar Breakdown:
  Pillar 1 (context resolution):   <span data-stat="session_p1_tokens">3,204</span> tokens  <span data-stat="session_p1_pct">36.6</span>%
  Pillar 2 (GDB debugging):            0 tokens   0.0%
  Pillar 3 (mined preventions):        0 tokens   0.0%
  Pillar 4 (automation skills):    <span data-stat="session_p4_tokens">1,560</span> tokens  <span data-stat="session_p4_pct">17.8</span>%
  RTK rewrites:                    <span data-stat="session_rtk_tokens">2,749</span> tokens  <span data-stat="session_rtk_pct">31.5</span>%
  Lean-ctx wraps:                  <span data-stat="session_lean_tokens">1,228</span> tokens  <span data-stat="session_lean_pct">14.1</span>%
```

## ချွေတာမှုကို ခန့်မှန်းပုံ

ပြင်ဆင်မှု အမျိုးအစားတစ်ခုစီတွင် PRECC မရှိလျှင် ဖြစ်မည့်အရာအပေါ် အခြေခံ၍ ခန့်မှန်းတိုကင်ကုန်ကျစရိတ်ရှိသည်:

| ပြင်ဆင်မှုအမျိုးအစား | ခန့်မှန်းချွေတာမှု | အကြောင်းပြချက် |
|----------------|-----------------|-----------|
| cd prepend | ~500 tokens | အမှားရလဒ် + Claude ဆင်ခြင်မှု + ပြန်ကြိုးစားမှု |
| ကျွမ်းကျင်မှုအသက်သွင်းခြင်း | ~400 tokens | အမှားရလဒ် + Claude ဆင်ခြင်မှု + ပြန်ကြိုးစားမှု |
| RTK rewrite | ~250 tokens | Claude ဖတ်ရမည့် အကျယ်တစ်ပြန့်ရလဒ် |
| Lean-ctx wrap | ~600 tokens | ဖိုင်ကြီးအကြောင်းအရာများ ချုံ့ထားသည် |
| တူးဖော်ထားသော ကြိုတင်ကာကွယ်မှု | ~500 tokens | သိထားသော ကျရှုံးမှုပုံစံကို ရှောင်လွှဲသည် |

ဤအရာများသည် ခန့်မှန်းချက်များဖြစ်ပြီး အမှန်တကယ်ချွေတာမှုသည် ပိုများနိုင်သည်။

## စုဆောင်းချွေတာမှု

ချွေတာမှုများသည် PRECC ဒေတာဘေ့စ်တွင် session များအကြား ဆက်လက်ရှိနေသည်။ အချိန်ကြာလာသည်နှင့်အမျှ စုစုပေါင်းသက်ရောက်မှုကို ခြေရာခံနိုင်သည်:

```bash
$ precc savings
Session Token Savings
=====================
Total estimated savings: <span data-stat="session_tokens_saved">8,741</span> tokens

Lifetime savings: <span data-stat="total_tokens_saved">142,389</span> tokens across <span data-stat="total_sessions">47</span> sessions
```
