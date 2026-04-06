# ထည့်သွင်းခြင်း

## အမြန်ထည့်သွင်းခြင်း (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

ဤအရာသည် သင့်ပလက်ဖောင်းအတွက် နောက်ဆုံးထွက်ဗားရှင်းကို ဒေါင်းလုဒ်လုပ်ပြီး SHA256 checksum ကို စစ်ဆေးကာ `~/.local/bin/` တွင် ထားပေးပါသည်။

ထည့်သွင်းပြီးနောက် PRECC ကို စတင်ပါ:

```bash
precc init
```

`precc init` သည် PreToolUse hook ကို Claude Code တွင် မှတ်ပုံတင်ပြီး ဒေတာလမ်းညွှန်များ ဖန်တီးကာ ကျွမ်းကျင်မှုဒေတာဘေ့စ်ကို စတင်ပါသည်။

## ထည့်သွင်းမှုရွေးချယ်စရာများ

### SHA256 အတည်ပြုခြင်း

ပုံမှန်အားဖြင့် installer သည် ထုတ်ဝေထားသော SHA256 နှင့် binary checksum ကို စစ်ဆေးပါသည်။ စစ်ဆေးခြင်းကို ကျော်လွှားရန် (အကြံပြုမထားပါ):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### စိတ်ကြိုက်ထည့်သွင်းမှုလမ်းကြောင်း

စိတ်ကြိုက်နေရာတွင် ထည့်သွင်းရန်:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### လက်တွဲကိရိယာများ (--extras)

PRECC တွင် ရွေးချယ်နိုင်သော လက်တွဲကိရိယာများ ပါဝင်ပါသည်။ `--extras` ဖြင့် ထည့်သွင်းပါ:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

ဤအရာသည် အောက်ပါတို့ကို ထည့်သွင်းပါသည်:

| ကိရိယာ | ရည်ရွယ်ချက် |
|------|---------|
| **RTK** | Command ပြန်လည်ရေးသားခြင်း toolkit |
| **lean-ctx** | CLAUDE.md နှင့် prompt ဖိုင်များအတွက် context ချုံ့ခြင်း |
| **nushell** | အဆင့်မြင့် pipeline များအတွက် ဖွဲ့စည်းထားသော shell |
| **cocoindex-code** | ပိုမြန်သော context ဖြေရှင်းခြင်းအတွက် code indexing |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

ထို့နောက် စတင်ပါ:

```powershell
precc init
```

## လက်ဖြင့်ထည့်သွင်းခြင်း

1. သင့်ပလက်ဖောင်းအတွက် release binary ကို [GitHub Releases](https://github.com/peria-ai/precc-cc/releases) မှ ဒေါင်းလုဒ်လုပ်ပါ။
2. release ရှိ `.sha256` ဖိုင်နှင့် SHA256 checksum ကို စစ်ဆေးပါ။
3. binary ကို သင့် `PATH` ရှိ directory တွင် ထားပါ (ဥပမာ `~/.local/bin/`)။
4. `precc init` ကို run ပါ။

## အပ်ဒိတ်လုပ်ခြင်း

```bash
precc update
```

အတိအကျ version တစ်ခုသို့ အတင်း update လုပ်ရန်:

```bash
precc update --force --version 0.3.0
```

အလိုအလျောက် update များကို ဖွင့်ရန်:

```bash
precc update --auto
```

## ထည့်သွင်းမှုကို အတည်ပြုခြင်း

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

`precc` ကို ရှာမတွေ့ပါက `~/.local/bin` သည် သင့် `PATH` တွင် ရှိမရှိ စစ်ဆေးပါ။
