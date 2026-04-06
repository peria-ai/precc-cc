# การขุด

PRECC ขุดค้นล็อกเซสชัน Claude Code เพื่อเรียนรู้รูปแบบข้อผิดพลาด-แก้ไข เมื่อพบข้อผิดพลาดเดิมอีกครั้ง จะแก้ไขโดยอัตโนมัติ

## การนำเข้าล็อกเซสชัน

### นำเข้าไฟล์เดียว

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### นำเข้าล็อกทั้งหมด

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### บังคับนำเข้าใหม่

เพื่อประมวลผลไฟล์ที่นำเข้าแล้วอีกครั้ง:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## การขุดทำงานอย่างไร

1. PRECC อ่านไฟล์ล็อก JSONL ของเซสชัน
2. ระบุคู่คำสั่งที่คำสั่งแรกล้มเหลวและคำสั่งที่สองเป็นการลองใหม่ที่แก้ไขแล้ว
3. แยกรูปแบบ (อะไรผิดพลาด) และการแก้ไข (Claude ทำอะไรต่างไป)
4. รูปแบบถูกเก็บใน `~/.local/share/precc/history.db`
5. เมื่อรูปแบบถึงเกณฑ์ความเชื่อมั่น จะกลายเป็นทักษะที่ขุดได้ใน `heuristics.db`

### ตัวอย่างรูปแบบ

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## Daemon precc-learner

Daemon `precc-learner` ทำงานเบื้องหลังและเฝ้าดูล็อกเซสชันใหม่โดยอัตโนมัติ:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

Daemon ใช้การแจ้งเตือนระบบไฟล์ (inotify บน Linux, FSEvents บน macOS) จึงตอบสนองทันทีเมื่อเซสชันสิ้นสุด

## จากรูปแบบสู่ทักษะ

รูปแบบที่ขุดได้จะเลื่อนขั้นเป็นทักษะเมื่อตรงตามเกณฑ์เหล่านี้:

- พบอย่างน้อย 3 ครั้งข้ามเซสชัน
- รูปแบบการแก้ไขสม่ำเสมอ (การแก้ไขประเภทเดียวกันทุกครั้ง)
- ไม่พบผลบวกปลอม

คุณสามารถตรวจสอบผู้สมัครทักษะได้ด้วย:

```bash
$ precc skills advise
```

ดู [Skills](skills.md) สำหรับรายละเอียดการจัดการทักษะ

## การจัดเก็บข้อมูล

- **คู่ข้อผิดพลาด-แก้ไข**: `~/.local/share/precc/history.db`
- **ทักษะที่เลื่อนขั้น**: `~/.local/share/precc/heuristics.db`

ทั้งสองเป็นฐานข้อมูล SQLite ในโหมด WAL เพื่อการเข้าถึงพร้อมกันอย่างปลอดภัย
