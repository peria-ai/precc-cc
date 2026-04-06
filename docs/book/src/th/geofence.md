# รั้วภูมิศาสตร์

PRECC รวมการตรวจสอบการปฏิบัติตาม IP geofence สำหรับสภาพแวดล้อมที่มีการควบคุม นี่เป็นฟีเจอร์ Pro

## ภาพรวม

บางองค์กรกำหนดให้เครื่องมือพัฒนาทำงานเฉพาะในภูมิภาคที่ได้รับอนุมัติเท่านั้น ฟีเจอร์ geofence ของ PRECC จะตรวจสอบว่าที่อยู่ IP ของเครื่องปัจจุบันอยู่ในรายการภูมิภาคที่อนุญาต

## การตรวจสอบการปฏิบัติตาม

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

หากเครื่องอยู่นอกภูมิภาคที่อนุญาต:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## การรีเฟรชข้อมูล geofence

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## ดูข้อมูล geofence

```bash
$ precc geofence info
Geofence Configuration
======================
Policy file:    ~/.config/precc/geofence.toml
Allowed regions: us-east-1, us-west-2, eu-west-1
Cache age:      2h 14m
Last check:     2026-04-03 09:12:00 UTC
Status:         COMPLIANT
```

## การล้างแคช

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## การกำหนดค่า

นโยบาย geofence ถูกกำหนดใน `~/.config/precc/geofence.toml`:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

ตั้งค่า `block_on_violation = true` เพื่อป้องกันไม่ให้ PRECC ทำงานเมื่ออยู่นอกภูมิภาคที่อนุญาต
