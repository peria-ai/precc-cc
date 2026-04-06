# မိတ်ဆက်

## PRECC ဆိုတာ ဘာလဲ?

PRECC (Claude Code အတွက် ကြိုတင်ခန့်မှန်းအမှားပြင်ဆင်ခြင်း) သည် တရားဝင် PreToolUse hook ယန္တရားမှတဆင့် Claude Code bash ညွှန်ကြားချက်များကို ကြားဖြတ်ယူသည့် Rust tool ဖြစ်သည်။

အသိုင်းအဝိုင်းသုံးစွဲသူများအတွက် အခမဲ့။

## ပြဿနာ

Claude Code wastes significant tokens on preventable mistakes:

- **Wrong-directory errors** -- Running `cargo build` in a parent directory that has no `Cargo.toml`, then retrying after reading the error.
- **Retry loops** -- A failed command produces verbose output, Claude reads it, reasons about it, and retries. Each cycle burns hundreds of tokens.
- **Verbose output** -- Commands like `find` or `ls -R` dump thousands of lines that Claude must process.

## မဏ္ဍိုင်လေးခု

### Context ပြင်ဆင်ခြင်း (cd-prepend)

`cargo build` သို့မဟုတ် `npm test` ကဲ့သို့ command များ directory မှားတွင် run နေကြောင်း ရှာဖွေပြီး execution မတိုင်မီ `cd /မှန်ကန်သော/path &&` ကို ထည့်ပေးသည်။

### GDB အမှားရှာခြင်း

Detects opportunities to attach GDB for deeper debugging of segfaults and crashes, providing structured debug information instead of raw core dumps.

### Session ကို တူးဖော်ခြင်း

Mines Claude Code session logs for failure-fix pairs. When the same mistake recurs, PRECC already knows the fix and applies it automatically.

### အလိုအလျောက် ကျွမ်းကျင်မှုများ

A library of built-in and mined skills that match command patterns and rewrite them. Skills are defined as TOML files or SQLite rows, making them easy to inspect, edit, and share.

## How It Works (30-Second Version)

1. Claude Code is about to run a bash command.
2. The PreToolUse hook sends the command to `precc-hook` as JSON on stdin.
3. `precc-hook` runs the command through the pipeline (skills, directory correction, compression) in under 3 milliseconds.
4. The corrected command is returned as JSON on stdout.
5. Claude Code executes the corrected command instead.

Claude never sees the error. No tokens wasted.

### Adaptive Compression

command တစ်ခု compression ပြီးနောက် fail ဖြစ်ပါက PRECC သည် နောက်တစ်ကြိမ် retry တွင် compression ကို အလိုအလျောက် ကျော်ပစ်ပြီး Claude အတွက် debug ရန် output အပြည့်အစုံကို ပေးသည်။

## Live Usage Statistics

Current version <span data-stat="current_version">--</span>:

| Metric | Value |
|---|---|
| Hook invocations | <span data-stat="total_invocations">--</span> |
| Tokens saved | <span data-stat="total_tokens_saved">--</span> |
| Saving ratio | <span data-stat="saving_pct">--</span>% |
| RTK rewrites | <span data-stat="rtk_rewrites">--</span> |
| CD corrections | <span data-stat="cd_prepends">--</span> |
| Hook latency | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| Unique users | <span data-stat="unique_users">--</span> |

### Savings by Release

<table id="version-breakdown" style="display:none">
<thead><tr><th>Version</th><th>Unique users</th><th>Hook invocations</th><th>Tokens saved</th><th>Saving ratio</th></tr></thead>
<tbody><tr><td colspan="5"><em>Loading...</em></td></tr></tbody>
</table>

<small>ကိန်းဂဏန်းများသည် ခန့်မှန်းချက်များဖြစ်သည်။ ကာကွယ်ထားသော failure တိုင်းသည် retry cycle တစ်ခုလုံးကို ရှောင်ရှားပေးသည်။ These numbers update automatically from anonymized telemetry.</small>

## Links

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Website: [https://peria.ai](https://peria.ai)
- Documentation: [https://precc.cc](https://precc.cc)
