# ကွန်မန်း ရည်ညွှန်းချက်

PRECC ကွန်မန်း အားလုံးအတွက် ရည်ညွှန်းချက် အပြည့်အစုံ။

---

## precc init

PRECC ကို စတင်ပြီး Claude Code နှင့် hook မှတ်ပုံတင်ပါ။

```
precc init

Options:
  (none)

Effects:
  - Registers PreToolUse:Bash hook with Claude Code
  - Creates ~/.local/share/precc/ data directory
  - Initializes heuristics.db with built-in skills
  - Prompts for telemetry consent
```

---

## precc ingest

Session log များမှ failure-fix ပုံစံများ ရှာဖွေပါ။

```
precc ingest [FILE] [--all] [--force]

Arguments:
  FILE            Path to a session log file (.jsonl)

Options:
  --all           Ingest all session logs from ~/.claude/logs/
  --force         Re-process files that were already ingested

Examples:
  precc ingest session.jsonl
  precc ingest --all
  precc ingest --all --force
```

---

## precc skills

Automation ကျွမ်းကျင်မှုများ စီမံပါ။

### precc skills list

```
precc skills list

List all active skills (built-in and mined).
```

### precc skills show

```
precc skills show NAME

Show detailed information about a specific skill.

Arguments:
  NAME            Skill name (e.g., cargo-wrong-dir)
```

### precc skills export

```
precc skills export NAME

Export a skill definition as TOML.

Arguments:
  NAME            Skill name
```

### precc skills edit

```
precc skills edit NAME

Open a skill definition in $EDITOR.

Arguments:
  NAME            Skill name
```

### precc skills advise

```
precc skills advise

Analyze recent sessions and suggest new skills based on repeated patterns.
```

### precc skills cluster

```
precc skills cluster

Group similar mined skills to identify redundant or overlapping patterns.
```

---

## precc report

ခွဲခြမ်းစိတ်ဖြာမှု အစီရင်ခံစာ ထုတ်ပါ။

```
precc report [--email]

Options:
  --email         Send the report via email (requires mail setup)
```

---

## precc savings

Token ချွေတာမှု ပြသပါ။

```
precc savings [--all]

Options:
  --all           Show detailed per-command breakdown (Pro)
```

---

## precc compress

Token အသုံးပြုမှု လျှော့ချရန် context ဖိုင်များ ချုံ့ပါ။

```
precc compress [DIR] [--dry-run] [--revert]

Arguments:
  DIR             Directory or file to compress (default: current directory)

Options:
  --dry-run       Preview changes without modifying files
  --revert        Restore files from backup
```

---

## precc license

သင့် PRECC လိုင်စင် စီမံပါ။

### precc license activate

```
precc license activate KEY --email EMAIL

Arguments:
  KEY             License key (XXXX-XXXX-XXXX-XXXX)

Options:
  --email EMAIL   Email address associated with the license
```

### precc license status

```
precc license status

Display current license status, plan, and expiration.
```

### precc license deactivate

```
precc license deactivate

Deactivate the license on this machine.
```

### precc license fingerprint

```
precc license fingerprint

Display the device fingerprint for this machine.
```

---

## precc mail

အီးမေးလ် လုပ်ဆောင်ချက်။

### precc mail setup

```
precc mail setup

Interactive SMTP configuration. Saves to ~/.config/precc/mail.toml.
```

### precc mail report

```
precc mail report EMAIL

Send a PRECC analytics report to the specified email address.

Arguments:
  EMAIL           Recipient email address
```

### precc mail send

```
precc mail send EMAIL FILE

Send a file as an email attachment.

Arguments:
  EMAIL           Recipient email address
  FILE            Path to the file to send
```

---

## precc update

PRECC ကို နောက်ဆုံးဗားရှင်းသို့ အပ်ဒိတ်ပါ။

```
precc update [--force] [--version VERSION] [--auto]

Options:
  --force             Force update even if already on latest
  --version VERSION   Update to a specific version
  --auto              Enable automatic updates
```

---

## precc telemetry

အမည်မသိ အဝေးမှတိုင်းတာခြင်း စီမံပါ။

### precc telemetry consent

```
precc telemetry consent

Opt in to anonymous telemetry.
```

### precc telemetry revoke

```
precc telemetry revoke

Opt out of telemetry. No further data will be sent.
```

### precc telemetry status

```
precc telemetry status

Show current telemetry consent status.
```

### precc telemetry preview

```
precc telemetry preview

Display the telemetry payload that would be sent (without sending it).
```

---

## precc geofence

IP geofence လိုက်နာမှု (Pro)။

### precc geofence check

```
precc geofence check

Check if the current machine is in an allowed region.
```

### precc geofence refresh

```
precc geofence refresh

Refresh the IP geolocation cache.
```

### precc geofence clear

```
precc geofence clear

Clear the geofence cache.
```

### precc geofence info

```
precc geofence info

Display geofence configuration and current status.
```

---

## precc gif

bash script များမှ animated GIF များ ရိုက်ကူးပါ (Pro)။

```
precc gif SCRIPT LENGTH [INPUTS...]

Arguments:
  SCRIPT          Path to a bash script
  LENGTH          Maximum recording duration (e.g., 30s, 2m)
  INPUTS...       Optional input lines for interactive prompts

Examples:
  precc gif demo.sh 30s
  precc gif interactive.sh 60s "yes" "my-project"
```

---

## precc gha

GitHub Actions ပျက်ကွက်မှုများ ခွဲခြမ်းစိတ်ဖြာပါ (Pro)။

```
precc gha URL

Arguments:
  URL             GitHub Actions run URL

Example:
  precc gha https://github.com/org/repo/actions/runs/12345678
```

---

## precc cache-hint

လက်ရှိ ပရောဂျက်အတွက် cache hint သတင်းအချက်အလက် ပြသပါ။

```
precc cache-hint
```

---

## precc trial

Pro အစမ်းသုံးမှု စတင်ပါ။

```
precc trial EMAIL

Arguments:
  EMAIL           Email address for the trial
```

---

## precc nushell

PRECC ပေါင်းစပ်ထားသော Nushell session စတင်ပါ။

```
precc nushell
```
