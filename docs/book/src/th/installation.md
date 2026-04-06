# การติดตั้ง

## ติดตั้งด่วน (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

คำสั่งนี้จะดาวน์โหลดไบนารีเวอร์ชันล่าสุดสำหรับแพลตฟอร์มของคุณ ตรวจสอบ SHA256 checksum และวางไว้ใน `~/.local/bin/`

หลังจากติดตั้งแล้ว ให้เริ่มต้น PRECC:

```bash
precc init
```

`precc init` ลงทะเบียน hook PreToolUse กับ Claude Code สร้างไดเรกทอรีข้อมูล และเริ่มต้นฐานข้อมูลทักษะ

## ตัวเลือกการติดตั้ง

### การตรวจสอบ SHA256

ตามค่าเริ่มต้น ตัวติดตั้งจะตรวจสอบ checksum ของไบนารีกับ SHA256 ที่เผยแพร่ หากต้องการข้ามการตรวจสอบ (ไม่แนะนำ):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### คำนำหน้าการติดตั้งแบบกำหนดเอง

ติดตั้งในตำแหน่งที่กำหนดเอง:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### เครื่องมือเสริม (--extras)

PRECC มาพร้อมกับเครื่องมือเสริมที่เลือกได้ ติดตั้งด้วย `--extras`:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

คำสั่งนี้จะติดตั้ง:

| เครื่องมือ | วัตถุประสงค์ |
|------|---------|
| **RTK** | ชุดเครื่องมือเขียนคำสั่งใหม่ |
| **lean-ctx** | การบีบอัดบริบทสำหรับ CLAUDE.md และไฟล์ prompt |
| **nushell** | เชลล์แบบมีโครงสร้างสำหรับ pipeline ขั้นสูง |
| **cocoindex-code** | การจัดทำดัชนีโค้ดเพื่อการแก้ไขบริบทที่เร็วขึ้น |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

จากนั้นเริ่มต้น:

```powershell
precc init
```

## การติดตั้งด้วยตนเอง

1. ดาวน์โหลดไบนารีสำหรับแพลตฟอร์มของคุณจาก [GitHub Releases](https://github.com/peria-ai/precc-cc/releases)
2. ตรวจสอบ SHA256 checksum กับไฟล์ `.sha256` ในรุ่นที่เผยแพร่
3. วางไบนารีในไดเรกทอรีบน `PATH` ของคุณ (เช่น `~/.local/bin/`)
4. รัน `precc init`

## การอัปเดต

```bash
precc update
```

บังคับอัปเดตเป็นเวอร์ชันเฉพาะ:

```bash
precc update --force --version 0.3.0
```

เปิดใช้งานการอัปเดตอัตโนมัติ:

```bash
precc update --auto
```

## การตรวจสอบการติดตั้ง

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

หากไม่พบ `precc` ให้ตรวจสอบว่า `~/.local/bin` อยู่ใน `PATH` ของคุณ
