# Hook Pipeline

`precc-hook` binary သည် PRECC ၏ အဓိကအစိတ်အပိုင်းဖြစ်သည်။ Claude Code နှင့် shell ကြားတွင်ရှိပြီး bash command တိုင်းကို 5 millisecond အတွင်း လုပ်ဆောင်သည်။

## Claude Code က Hook ကို ခေါ်ယူပုံ

Claude Code သည် PreToolUse hook များကို ပံ့ပိုးသည် -- လုပ်ဆောင်မှုမတိုင်မီ tool input များကို စစ်ဆေးပြီး ပြင်ဆင်နိုင်သော ပြင်ပပရိုဂရမ်များဖြစ်သည်။ Claude က bash command ကို run မည့်အခါ stdin မှတဆင့် `precc-hook` သို့ JSON ပေးပို့ပြီး stdout မှ တုံ့ပြန်မှုကို ဖတ်သည်။

## Pipeline အဆင့်များ

```
Claude Code
    |
    v
+---------------------------+
| 1. Parse JSON stdin       |  Read the command from Claude Code
+---------------------------+
    |
    v
+---------------------------+
| 2. Skill matching         |  Query heuristics.db for matching skills (Pillar 4)
+---------------------------+
    |
    v
+---------------------------+
| 3. Directory correction   |  Resolve correct working directory (Pillar 1)
+---------------------------+
    |
    v
+---------------------------+
| 4. GDB check              |  Detect debug opportunities (Pillar 2)
+---------------------------+
    |
    v
+---------------------------+
| 5. RTK rewriting          |  Apply command rewrites for token savings
+---------------------------+
    |
    v
+---------------------------+
| 6. Emit JSON stdout       |  Return modified command to Claude Code
+---------------------------+
    |
    v
  Shell executes corrected command
```

## ဥပမာ: JSON Input နှင့် Output

### Input (Claude Code မှ)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC သည် လက်ရှိ directory တွင် `Cargo.toml` မရှိသော်လည်း `./myapp/Cargo.toml` ရှိကြောင်း ရှာဖွေတွေ့ရှိသည်။

### Output (Claude Code သို့)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

ပြင်ဆင်ရန် မလိုအပ်ပါက `updatedInput.command` သည် ဗလာဖြစ်ပြီး Claude Code သည် မူရင်း command ကို အသုံးပြုသည်။

## အဆင့်အသေးစိတ်

### အဆင့် 1: JSON ခွဲခြမ်းစိတ်ဖြာ

stdin မှ JSON object အပြည့်အစုံကို ဖတ်သည်။ `tool_input.command` ကို ထုတ်ယူသည်။ parsing မအောင်မြင်ပါက hook ချက်ချင်းထွက်ပြီး Claude Code သည် မူရင်း command ကို အသုံးပြုသည် (fail-open ဒီဇိုင်း)။

### အဆင့် 2: Skill တိုက်ဆိုင်ခြင်း

trigger pattern နှင့် command ကိုက်ညီသော skill များအတွက် SQLite heuristics database ကို query လုပ်သည်။ Skill များကို ဦးစားပေးအစီအစဥ်အတိုင်း စစ်ဆေးသည်။

### အဆင့် 3: Directory ပြင်ဆင်ခြင်း

Build command များ (`cargo`, `go`, `make`, `npm`, `python` စသည်) အတွက် လက်ရှိ directory တွင် မျှော်လင့်ထားသော project file ရှိမရှိ စစ်ဆေးသည်။ မရှိပါက အနီးအနား directory များကို scan ပြီး `cd <dir> &&` ကို ရှေ့တွင် ထည့်သည်။

Directory scan သည် မြန်ဆန်မှုကို ထိန်းသိမ်းရန် 5 စက္ကန့် TTL ပါသော cached filesystem index ကို အသုံးပြုသည်။

### အဆင့် 4: GDB စစ်ဆေးခြင်း

Command သည် crash ဖြစ်နိုင်ခြေရှိပါက (ဥပမာ debug binary run ခြင်း)၊ PRECC သည် raw crash log များအစား structured debug output ကို ဖမ်းယူရန် GDB wrapper များကို အကြံပြုခြင်း သို့မဟုတ် ထည့်သွင်းနိုင်သည်။

### အဆင့် 5: RTK ပြန်လည်ရေးသားခြင်း

RTK (Rewrite Toolkit) စည်းမျဉ်းများကို အသုံးပြုပြီး verbose command များကို အတိုချုံ့ခြင်း၊ noisy output ကို ဖိနှိပ်ခြင်း သို့မဟုတ် token efficiency အတွက် command များကို ပြန်လည်ဖွဲ့စည်းခြင်း ပြုလုပ်သည်။

### အဆင့် 6: JSON ထုတ်လွှတ်ခြင်း

ပြင်ဆင်ထားသော command ကို JSON သို့ ပြန်လည် serialize ပြုလုပ်ပြီး stdout သို့ ရေးသည်။ ပြောင်းလဲမှုမရှိပါက output သည် Claude Code ကို မူရင်း command အသုံးပြုရန် အချက်ပြသည်။

## စွမ်းဆောင်ရည်

Pipeline တစ်ခုလုံးသည် 5 millisecond (p99) အတွင်း ပြီးမြောက်သည်။ အဓိက optimization များ:

- Lock-free concurrent reads အတွက် WAL mode ဖြင့် SQLite
- Skill matching အတွက် ကြိုတင် compile ပြုလုပ်ထားသော regex pattern များ
- Cached filesystem scans (5 စက္ကန့် TTL)
- Hot path တွင် network call များမရှိ
- Fail-open: error တစ်ခုခုဖြစ်ပါက မူရင်း command သို့ ပြန်သွားသည်

## Hook ကို လက်ဖြင့် စမ်းသပ်ခြင်း

Hook ကို တိုက်ရိုက် ခေါ်ယူနိုင်သည်:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
