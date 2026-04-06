# ကျွမ်းကျင်မှုများ

ကျွမ်းကျင်မှုများသည် PRECC က command များကိုရှာဖွေပြင်ဆင်ရန်အသုံးပြုသော ပုံစံတိုက်ဆိုင်စစ်ဆေးခြင်းစည်းမျဉ်းများဖြစ်သည်။

## ပါဝင်သော ကျွမ်းကျင်မှုများ

| ကျွမ်းကျင်မှု | အစပျိုးခြင်း | လုပ်ဆောင်ချက် |
|-------|-------------|--------|
| `cargo-wrong-dir` | Rust ပရောဂျက်ပြင်ပတွင် `cargo build/test/clippy` | အနီးဆုံး `Cargo.toml` လမ်းကြောင်းသို့ `cd` ထည့်ခြင်း |
| `git-wrong-dir` | git repo ပြင်ပတွင် `git *` | အနီးဆုံး `.git` လမ်းကြောင်းသို့ `cd` ထည့်ခြင်း |
| `go-wrong-dir` | Go module ပြင်ပတွင် `go build/test` | အနီးဆုံး `go.mod` လမ်းကြောင်းသို့ `cd` ထည့်ခြင်း |
| `make-wrong-dir` | လက်ရှိလမ်းကြောင်းတွင် Makefile မရှိဘဲ `make` | အနီးဆုံး Makefile လမ်းကြောင်းသို့ `cd` ထည့်ခြင်း |
| `npm-wrong-dir` | Node ပရောဂျက်ပြင်ပတွင် `npm/npx/pnpm/yarn` | အနီးဆုံး `package.json` လမ်းကြောင်းသို့ `cd` ထည့်ခြင်း |
| `python-wrong-dir` | Python ပရောဂျက်ပြင်ပတွင် `python/pytest/pip` | အနီးဆုံး Python ပရောဂျက်သို့ `cd` ထည့်ခြင်း |
| `jj-translate` | jj-colocated repo တွင် `git *` | ညီမျှသော `jj` command သို့ ပြန်ရေးခြင်း |
| `asciinema-gif` | `asciinema rec` | `precc gif` သို့ ပြန်ရေးခြင်း |

## ကျွမ်းကျင်မှုများစာရင်း

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
  9 fix-pytest-path    mined     pytest with wrong test path
```

## ကျွမ်းကျင်မှုအသေးစိတ်ပြသခြင်း

```bash
$ precc skills show cargo-wrong-dir
Name:        cargo-wrong-dir
Type:        built-in
Source:      skills/builtin/cargo-wrong-dir.toml
Description: Detects cargo commands run outside a Rust project and prepends
             cd to the directory containing the nearest Cargo.toml.
Trigger:     ^cargo\s+(build|test|clippy|run|check|bench|doc)
Action:      prepend_cd
Marker:      Cargo.toml
Activations: 12
```

## ကျွမ်းကျင်မှုကို TOML သို့ တင်ပို့ခြင်း

```bash
$ precc skills export cargo-wrong-dir
[skill]
name = "cargo-wrong-dir"
description = "Prepend cd for cargo commands outside a Rust project"
trigger = "^cargo\\s+(build|test|clippy|run|check|bench|doc)"
action = "prepend_cd"
marker = "Cargo.toml"
priority = 10
```

## ကျွမ်းကျင်မှုကိုတည်းဖြတ်ခြင်း

```bash
$ precc skills edit cargo-wrong-dir
```

ဤအရာသည် သင်၏ `$EDITOR` တွင် ကျွမ်းကျင်မှုအဓိပ္ပာယ်ဖွင့်ဆိုချက်ကိုဖွင့်သည်။ သိမ်းဆည်းပြီးနောက် ကျွမ်းကျင်မှုကို အလိုအလျောက် ပြန်လည်ဖွင့်သည်။

## Advise command

`precc skills advise` သည် သင်၏လတ်တလောစက်ရှင်ကိုခွဲခြမ်းစိတ်ဖြာပြီး ထပ်ခါထပ်ခါပုံစံများအပေါ်အခြေခံ၍ ကျွမ်းကျင်မှုအသစ်များအကြံပြုသည်။

```bash
$ precc skills advise
Analyzed 47 commands from the last session.

Suggested skills:
  1. docker-wrong-dir: You ran `docker compose up` outside the project root 3 times.
     Suggested trigger: ^docker\s+compose
     Suggested marker: docker-compose.yml

  2. terraform-wrong-dir: You ran `terraform plan` outside the infra directory 2 times.
     Suggested trigger: ^terraform\s+(plan|apply|init)
     Suggested marker: main.tf

Accept suggestion [1/2/skip]?
```

## ကျွမ်းကျင်မှုအုပ်စုဖွဲ့ခြင်း

```bash
$ precc skills cluster
```

ထပ်နေသောပုံစံများကိုသိရှိရန် ဆင်တူသောကျွမ်းကျင်မှုများကိုအုပ်စုဖွဲ့သည်။

## တူးဖော်ရရှိသောနှင့်ပါဝင်သောကျွမ်းကျင်မှုများ

ပါဝင်သောကျွမ်းကျင်မှုများသည် PRECC နှင့်အတူပါဝင်ပြီး `skills/builtin/*.toml` တွင်သတ်မှတ်ထားသည်။

တူးဖော်ရရှိသောကျွမ်းကျင်မှုများကို `precc ingest` သို့မဟုတ် `precc-learner` daemon မှဖန်တီးသည်။ `~/.local/share/precc/heuristics.db` တွင်သိမ်းဆည်းထားသည်။ အသေးစိတ်အတွက် [Mining](mining.md) ကိုကြည့်ပါ။
