# การวัดระยะไกล

PRECC รองรับการวัดระยะไกลแบบไม่ระบุตัวตนเพื่อช่วยปรับปรุงเครื่องมือ ไม่มีการเก็บข้อมูลเว้นแต่คุณจะยินยอมอย่างชัดเจน

## เลือกเข้าร่วม

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## เลือกไม่เข้าร่วม

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## ตรวจสอบสถานะ

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## ดูตัวอย่างข้อมูลที่จะถูกส่ง

ก่อนเลือกเข้าร่วม คุณสามารถดูได้ว่าข้อมูลใดจะถูกเก็บ:

```bash
$ precc telemetry preview
Telemetry payload (this session):
{
  "version": "0.3.0",
  "os": "linux",
  "arch": "x86_64",
  "skills_activated": 12,
  "commands_intercepted": 87,
  "pillars_used": [1, 4],
  "avg_hook_latency_ms": 2.3,
  "session_count": 1
}
```

## ข้อมูลที่เก็บรวบรวม

- เวอร์ชัน PRECC ระบบปฏิบัติการ และสถาปัตยกรรม
- จำนวนรวม: คำสั่งที่สกัด ทักษะที่เปิดใช้งาน เสาหลักที่ใช้
- ความหน่วงเฉลี่ยของ hook
- จำนวนเซสชัน

## ข้อมูลที่ไม่ได้เก็บรวบรวม

- ไม่มีข้อความคำสั่งหรืออาร์กิวเมนต์
- ไม่มีเส้นทางไฟล์หรือชื่อไดเรกทอรี
- ไม่มีชื่อโปรเจกต์หรือ URL ของที่เก็บ
- ไม่มีข้อมูลส่วนบุคคลที่ระบุตัวตนได้ (PII)
- ไม่มีที่อยู่ IP (เซิร์ฟเวอร์ไม่บันทึก)

## การแทนที่ด้วยตัวแปรสภาพแวดล้อม

เพื่อปิดการวัดระยะไกลโดยไม่ต้องรันคำสั่ง (มีประโยชน์ใน CI หรือสภาพแวดล้อมที่ใช้ร่วมกัน):

```bash
export PRECC_NO_TELEMETRY=1
```

สิ่งนี้มีความสำคัญเหนือการตั้งค่าความยินยอม

## ปลายทางข้อมูล

ข้อมูลการวัดระยะไกลจะถูกส่งไปยัง `https://telemetry.peria.ai/v1/precc` ผ่าน HTTPS ข้อมูลนี้ใช้เพื่อทำความเข้าใจรูปแบบการใช้งานและจัดลำดับความสำคัญของการพัฒนาเท่านั้น
