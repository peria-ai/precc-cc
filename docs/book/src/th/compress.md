# การบีบอัด

`precc compress` ย่อ CLAUDE.md และไฟล์บริบทอื่นๆ เพื่อลดการใช้โทเค็นเมื่อ Claude Code โหลดไฟล์เหล่านั้น นี่เป็นฟีเจอร์ Pro

## การใช้งานพื้นฐาน

```bash
$ precc compress .
[precc] Scanning directory: .
[precc] Found 3 context files:
         CLAUDE.md (2,847 tokens -> 1,203 tokens, -57.7%)
         ARCHITECTURE.md (4,112 tokens -> 2,044 tokens, -50.3%)
         ALTERNATIVES.md (3,891 tokens -> 1,967 tokens, -49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
[precc] Files compressed. Use --revert to restore originals.
```

## ทดลองรัน

ดูตัวอย่างสิ่งที่จะเปลี่ยนแปลงโดยไม่แก้ไขไฟล์:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## การย้อนกลับ

ไฟล์ต้นฉบับจะถูกสำรองโดยอัตโนมัติ เพื่อกู้คืน:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## อะไรถูกบีบอัด

ตัวบีบอัดใช้การแปลงหลายอย่าง:

- ลบช่องว่างและบรรทัดว่างที่ซ้ำซ้อน
- ย่อข้อความที่ยืดยาวโดยรักษาความหมาย
- ย่อตารางและรายการ
- ลบความคิดเห็นและการจัดรูปแบบตกแต่ง
- รักษาบล็อกโค้ด เส้นทาง และตัวระบุทางเทคนิคทั้งหมด

ผลลัพธ์ที่บีบอัดยังคงอ่านได้โดยมนุษย์ -- ไม่ได้ถูกย่อหรือทำให้สับสน

## กำหนดเป้าหมายไฟล์เฉพาะ

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
