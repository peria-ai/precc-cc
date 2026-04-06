# တူးဖော်ခြင်း

PRECC သည် Claude Code ဆက်ရှင်မှတ်တမ်းများကို ခွဲခြမ်းစိတ်ဖြာပြီး အမှား-ပြင်ဆင်ပုံစံများကို သင်ယူသည်။

## ဆက်ရှင်မှတ်တမ်းများ ထည့်သွင်းခြင်း

### ဖိုင်တစ်ခု ထည့်သွင်းခြင်း

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### မှတ်တမ်းအားလုံး ထည့်သွင်းခြင်း

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### အတင်း ပြန်ထည့်သွင်းခြင်း

ထည့်သွင်းပြီးသားဖိုင်များကို ပြန်လည်လုပ်ဆောင်ရန်:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## တူးဖော်ခြင်း အလုပ်လုပ်ပုံ

1. PRECC သည် ဆက်ရှင် JSONL မှတ်တမ်းဖိုင်ကို ဖတ်ပါသည်။
2. ပထမအမိန့် မအောင်မြင်ပြီး ဒုတိယအမိန့်က ပြင်ဆင်ထားသော ပြန်ကြိုးစားမှုဖြစ်သည့် အမိန့်အတွဲများကို ခွဲခြားသည်။
3. ပုံစံ (ဘာမှားသလဲ) နှင့် ပြင်ဆင်မှု (Claude ဘာကွာသလဲ) ကို ထုတ်ယူသည်။
4. ပုံစံများကို `~/.local/share/precc/history.db` တွင် သိမ်းဆည်းသည်။
5. ပုံစံတစ်ခု ယုံကြည်မှုအဆင့်သို့ ရောက်သောအခါ `heuristics.db` ရှိ ကျွမ်းကျင်မှုဖြစ်လာသည်။

### ပုံစံ ဥပမာ

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## precc-learner daemon

`precc-learner` daemon သည် နောက်ခံတွင် အလုပ်လုပ်ပြီး ဆက်ရှင်မှတ်တမ်းအသစ်များကို အလိုအလျောက် စောင့်ကြည့်သည်:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

Daemon သည် ဖိုင်စနစ်အကြောင်းကြားချက်များ (Linux တွင် inotify, macOS တွင် FSEvents) ကို အသုံးပြုသောကြောင့် ဆက်ရှင်ပြီးဆုံးသောအခါ ချက်ချင်းတုံ့ပြန်သည်။

## ပုံစံများမှ ကျွမ်းကျင်မှုများသို့

တူးဖော်ထားသော ပုံစံများသည် ဤစံနှုန်းများနှင့် ကိုက်ညီသောအခါ ကျွမ်းကျင်မှုများသို့ တိုးတက်သည်:

- ဆက်ရှင်များတွင် အနည်းဆုံး 3 ကြိမ် တွေ့မြင်ခဲ့
- တသမတ်တည်း ပြင်ဆင်မှုပုံစံ (တိုင်းတစ်ကြိမ် တူညီသော ပြင်ဆင်မှုအမျိုးအစား)
- အမှားရှာဖွေမှု မတွေ့ရှိ

ကျွမ်းကျင်မှု ကိုယ်စားလှယ်လောင်းများကို ပြန်လည်စစ်ဆေးနိုင်သည်:

```bash
$ precc skills advise
```

ကျွမ်းကျင်မှုများ စီမံခန့်ခွဲခြင်း အသေးစိတ်အတွက် [Skills](skills.md) ကို ကြည့်ပါ။

## ဒေတာ သိမ်းဆည်းခြင်း

- **အမှား-ပြင်ဆင်မှု အတွဲများ**: `~/.local/share/precc/history.db`
- **တိုးတက်ပြီးသော ကျွမ်းကျင်မှုများ**: `~/.local/share/precc/heuristics.db`

နှစ်ခုစလုံး လုံခြုံသော တပြိုင်နက်ဝင်ရောက်မှုအတွက် WAL မုဒ်ဖြင့် SQLite ဒေတာဘေ့စ်များဖြစ်သည်။
