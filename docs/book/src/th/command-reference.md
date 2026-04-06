# อ้างอิงคำสั่ง

เอกสารอ้างอิงฉบับสมบูรณ์สำหรับคำสั่ง PRECC ทั้งหมด

---

## precc init

เริ่มต้น PRECC และลงทะเบียน hook กับ Claude Code

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

ขุดบันทึกเซสชันเพื่อหารูปแบบข้อผิดพลาด-การแก้ไข

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

จัดการทักษะอัตโนมัติ

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

สร้างรายงานวิเคราะห์

```
precc report [--email]

Options:
  --email         Send the report via email (requires mail setup)
```

---

## precc savings

แสดงการประหยัด token

```
precc savings [--all]

Options:
  --all           Show detailed per-command breakdown (Pro)
```

---

## precc compress

บีบอัดไฟล์บริบทเพื่อลดการใช้ token

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

จัดการใบอนุญาต PRECC ของคุณ

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

ฟังก์ชันอีเมล

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

อัปเดต PRECC เป็นเวอร์ชันล่าสุด

```
precc update [--force] [--version VERSION] [--auto]

Options:
  --force             Force update even if already on latest
  --version VERSION   Update to a specific version
  --auto              Enable automatic updates
```

---

## precc telemetry

จัดการการวัดระยะไกลแบบไม่ระบุตัวตน

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

การปฏิบัติตาม geofence IP (Pro)

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

บันทึก GIF แบบเคลื่อนไหวจากสคริปต์ bash (Pro)

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

วิเคราะห์การรัน GitHub Actions ที่ล้มเหลว (Pro)

```
precc gha URL

Arguments:
  URL             GitHub Actions run URL

Example:
  precc gha https://github.com/org/repo/actions/runs/12345678
```

---

## precc cache-hint

แสดงข้อมูลคำแนะนำแคชสำหรับโปรเจกต์ปัจจุบัน

```
precc cache-hint
```

---

## precc trial

เริ่มทดลองใช้ Pro

```
precc trial EMAIL

Arguments:
  EMAIL           Email address for the trial
```

---

## precc nushell

เปิดเซสชัน Nushell พร้อมการรวม PRECC

```
precc nushell
```
