# การวิเคราะห์ GitHub Actions

`precc gha` วิเคราะห์การรัน GitHub Actions ที่ล้มเหลวและแนะนำวิธีแก้ไข นี่เป็นฟีเจอร์ Pro

## การใช้งาน

ส่ง URL ของการรัน GitHub Actions ที่ล้มเหลว:

```bash
$ precc gha https://github.com/myorg/myrepo/actions/runs/12345678
[precc] Fetching run 12345678...
[precc] Run: CI / build (ubuntu-latest)
[precc] Status: failure
[precc] Failed step: Run cargo test

[precc] Log analysis:
  Error: test result: FAILED. 2 passed; 1 failed
  Failed test: tests::integration::test_database_connection
  Cause: thread 'tests::integration::test_database_connection' panicked at
         'called Result::unwrap() on an Err value: Connection refused'

[precc] Suggested fix:
  The test requires a database connection but the CI environment does not
  start a database service. Add a services block to your workflow:

    services:
      postgres:
        image: postgres:15
        ports:
          - 5432:5432
        env:
          POSTGRES_PASSWORD: test
```

## ทำอะไร

1. แยกวิเคราะห์ URL การรัน GitHub Actions เพื่อดึงเจ้าของ, repo และ ID การรัน
2. ดึงบันทึกการรันผ่าน GitHub API (ใช้ `GITHUB_TOKEN` หากตั้งค่าไว้ มิฉะนั้นเข้าถึงแบบสาธารณะ)
3. ระบุขั้นตอนที่ล้มเหลวและดึงบรรทัดข้อผิดพลาดที่เกี่ยวข้อง
4. วิเคราะห์ข้อผิดพลาดและแนะนำวิธีแก้ไขตามรูปแบบความล้มเหลว CI ทั่วไป

## รูปแบบความล้มเหลวที่รองรับ

- คอนเทนเนอร์บริการที่ขาดหายไป (ฐานข้อมูล, Redis ฯลฯ)
- ระบบปฏิบัติการหรือสถาปัตยกรรม runner ไม่ถูกต้อง
- ตัวแปรสภาพแวดล้อมหรือ secrets ที่ขาดหายไป
- การติดตั้ง dependency ล้มเหลว
- การทดสอบหมดเวลา
- ข้อผิดพลาดด้านสิทธิ์
- แคชพลาดทำให้ build ช้า
