# ใบอนุญาต

PRECC เสนอสองระดับ: Community (ฟรี) และ Pro

## ระดับ Community (ฟรี)

ระดับ Community ประกอบด้วย:

- ทักษะในตัวทั้งหมด (แก้ไขไดเรกทอรีผิด, แปล jj ฯลฯ)
- ท่อ hook ที่รองรับ Pillar 1 และ Pillar 4 อย่างเต็มที่
- สรุปพื้นฐานของ `precc savings`
- การขุดเซสชันด้วย `precc ingest`
- ใช้งานในเครื่องไม่จำกัด

## ระดับ Pro

Pro ปลดล็อกฟีเจอร์เพิ่มเติม:

- **รายละเอียดการประหยัด** -- `precc savings --all` พร้อมการวิเคราะห์ต่อคำสั่ง
- **บันทึก GIF** -- `precc gif` สำหรับสร้าง GIF แอนิเมชันของเทอร์มินัล
- **การปฏิบัติตาม IP geofence** -- สำหรับสภาพแวดล้อมที่ถูกควบคุม
- **รายงานอีเมล** -- `precc mail report` เพื่อส่งการวิเคราะห์
- **การวิเคราะห์ GitHub Actions** -- `precc gha` สำหรับการแก้จุดบกพร่องของเวิร์กโฟลว์ที่ล้มเหลว
- **การบีบอัดบริบท** -- `precc compress` สำหรับการเพิ่มประสิทธิภาพ CLAUDE.md
- **การสนับสนุนแบบเร่งด่วน**

## การเปิดใช้งานใบอนุญาต

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## ตรวจสอบสถานะใบอนุญาต

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## การเปิดใช้งาน GitHub Sponsors

หากคุณสนับสนุน PRECC ผ่าน GitHub Sponsors ใบอนุญาตจะถูกเปิดใช้งานโดยอัตโนมัติผ่านอีเมล GitHub ของคุณ ไม่ต้องใช้คีย์ -- แค่ให้แน่ใจว่าอีเมลผู้สนับสนุนตรงกัน:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## ลายนิ้วมืออุปกรณ์

ใบอนุญาตแต่ละใบผูกกับลายนิ้วมืออุปกรณ์ ดูของคุณด้วย:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

หากต้องการโอนใบอนุญาตไปยังเครื่องใหม่ ให้ยกเลิกการเปิดใช้งานก่อน:

```bash
precc license deactivate
```

จากนั้นเปิดใช้งานบนเครื่องใหม่

## ใบอนุญาตหมดอายุ?

เมื่อใบอนุญาต Pro หมดอายุ PRECC จะกลับไปที่ระดับ Community ทักษะในตัวและฟังก์ชันหลักทั้งหมดยังคงทำงานต่อไป เฉพาะฟีเจอร์เฉพาะ Pro เท่านั้นที่จะใช้ไม่ได้ ดู [FAQ](faq.md) สำหรับรายละเอียดเพิ่มเติม
