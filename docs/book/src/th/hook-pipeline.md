# ไปป์ไลน์ Hook

ไบนารี `precc-hook` เป็นแกนหลักของ PRECC อยู่ระหว่าง Claude Code กับ shell ประมวลผลทุกคำสั่ง bash ภายใน 5 มิลลิวินาที

## วิธีที่ Claude Code เรียกใช้ Hook

Claude Code รองรับ hook PreToolUse -- โปรแกรมภายนอกที่สามารถตรวจสอบและแก้ไขอินพุตของเครื่องมือก่อนการทำงาน เมื่อ Claude กำลังจะรันคำสั่ง bash จะส่ง JSON ไปยัง `precc-hook` ผ่าน stdin และอ่านผลลัพธ์จาก stdout

## ขั้นตอน Pipeline

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

## ตัวอย่าง: อินพุตและเอาต์พุต JSON

### อินพุต (จาก Claude Code)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC ตรวจพบว่าไดเรกทอรีปัจจุบันไม่มี `Cargo.toml` แต่ `./myapp/Cargo.toml` มีอยู่

### เอาต์พุต (ไปยัง Claude Code)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

หากไม่จำเป็นต้องแก้ไข `updatedInput.command` จะว่างเปล่าและ Claude Code จะใช้คำสั่งเดิม

## รายละเอียดขั้นตอน

### ขั้นตอนที่ 1: แยกวิเคราะห์ JSON

อ่านอ็อบเจกต์ JSON ทั้งหมดจาก stdin ดึงค่า `tool_input.command` หากการแยกวิเคราะห์ล้มเหลว hook จะออกทันทีและ Claude Code จะใช้คำสั่งเดิม (การออกแบบ fail-open)

### ขั้นตอนที่ 2: การจับคู่ Skill

สอบถามฐานข้อมูล heuristic ของ SQLite เพื่อหา skill ที่มีรูปแบบทริกเกอร์ตรงกับคำสั่ง ตรวจสอบ skill ตามลำดับความสำคัญ ทั้ง skill TOML ในตัวและ skill ที่ขุดได้จะถูกประเมิน

### ขั้นตอนที่ 3: การแก้ไขไดเรกทอรี

สำหรับคำสั่ง build (`cargo`, `go`, `make`, `npm`, `python` ฯลฯ) ตรวจสอบว่าไฟล์โปรเจกต์ที่คาดหวังมีอยู่ในไดเรกทอรีปัจจุบันหรือไม่ หากไม่มี จะสแกนไดเรกทอรีใกล้เคียงเพื่อหาผลลัพธ์ที่ตรงที่สุดและเติม `cd <dir> &&` ข้างหน้า

การสแกนไดเรกทอรีใช้ดัชนีระบบไฟล์ที่แคชไว้โดยมี TTL 5 วินาทีเพื่อรักษาความเร็ว

### ขั้นตอนที่ 4: ตรวจสอบ GDB

หากคำสั่งมีแนวโน้มที่จะทำให้เกิด crash (เช่น รัน debug binary) PRECC สามารถแนะนำหรือแทรก GDB wrapper เพื่อจับเอาต์พุต debug แบบมีโครงสร้างแทน crash log ดิบ

### ขั้นตอนที่ 5: การเขียนใหม่ RTK

ใช้กฎ RTK (Rewrite Toolkit) ที่ย่อคำสั่งที่ยาว ระงับเอาต์พุตที่มีเสียงรบกวน หรือปรับโครงสร้างคำสั่งเพื่อประสิทธิภาพของ token

### ขั้นตอนที่ 6: ส่งออก JSON

ซีเรียลไลซ์คำสั่งที่แก้ไขกลับเป็น JSON และเขียนลง stdout หากไม่มีการเปลี่ยนแปลง เอาต์พุตจะส่งสัญญาณให้ Claude Code ใช้คำสั่งเดิม

## ประสิทธิภาพ

ไปป์ไลน์ทั้งหมดเสร็จสิ้นภายใน 5 มิลลิวินาที (p99) การเพิ่มประสิทธิภาพหลัก:

- SQLite ในโหมด WAL สำหรับการอ่านพร้อมกันแบบไม่ล็อก
- รูปแบบ regex ที่คอมไพล์ล่วงหน้าสำหรับการจับคู่ skill
- การสแกนระบบไฟล์ที่แคชไว้ (TTL 5 วินาที)
- ไม่มีการเรียกเครือข่ายใน hot path
- Fail-open: ข้อผิดพลาดใดๆ จะกลับไปใช้คำสั่งเดิม

## ทดสอบ Hook ด้วยตนเอง

คุณสามารถเรียกใช้ hook โดยตรง:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
