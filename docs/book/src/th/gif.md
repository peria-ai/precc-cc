# การบันทึก GIF

`precc gif` สร้างการบันทึก GIF แบบเคลื่อนไหวของเซสชันเทอร์มินัลจากสคริปต์ bash นี่เป็นฟีเจอร์ Pro

## การใช้งานพื้นฐาน

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

อาร์กิวเมนต์แรกเป็นสคริปต์ bash ที่มีคำสั่งที่จะรัน อาร์กิวเมนต์ที่สองคือความยาวการบันทึกสูงสุด

## รูปแบบสคริปต์

สคริปต์เป็นไฟล์ bash มาตรฐาน:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## การจำลองอินพุต

สำหรับคำสั่งแบบโต้ตอบ ให้ระบุค่าอินพุตเป็นอาร์กิวเมนต์เพิ่มเติม:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

อาร์กิวเมนต์เพิ่มเติมแต่ละตัวจะถูกป้อนเป็นบรรทัด stdin เมื่อสคริปต์ร้องขออินพุต

## ตัวเลือกเอาต์พุต

ไฟล์เอาต์พุตจะตั้งชื่อตามสคริปต์เป็นค่าเริ่มต้น (`script.gif`) GIF ใช้ธีมเทอร์มินัลสีเข้มขนาดมาตรฐาน 80x24

## ทำไมต้อง GIF แทน asciinema?

ทักษะในตัว `asciinema-gif` จะเขียนใหม่ `asciinema rec` เป็น `precc gif` โดยอัตโนมัติ ไฟล์ GIF พกพาได้ง่ายกว่า -- แสดงแบบอินไลน์ใน GitHub README, Slack และอีเมลโดยไม่ต้องใช้โปรแกรมเล่น
