# เริ่มต้นอย่างรวดเร็ว

เริ่มใช้งาน PRECC ใน 5 นาที

## ขั้นตอนที่ 1: ติดตั้ง

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## ขั้นตอนที่ 2: เริ่มต้นระบบ

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## ขั้นตอนที่ 3: ตรวจสอบว่า hook ทำงานอยู่

```bash
$ precc skills list
  # Name               Type      Triggers
  1 cargo-wrong-dir    built-in  cargo build/test/clippy outside Rust project
  2 git-wrong-dir      built-in  git * outside a repo
  3 go-wrong-dir       built-in  go build/test outside Go module
  4 make-wrong-dir     built-in  make without Makefile in cwd
  5 npm-wrong-dir      built-in  npm/npx/pnpm/yarn outside Node project
  6 python-wrong-dir   built-in  python/pytest/pip outside Python project
  7 jj-translate       built-in  git * in jj-colocated repo
  8 asciinema-gif      built-in  asciinema rec
```

## ขั้นตอนที่ 4: ใช้ Claude Code ตามปกติ

เปิด Claude Code แล้วทำงานตามปกติ PRECC ทำงานอยู่เบื้องหลังอย่างเงียบๆ เมื่อ Claude ออกคำสั่งที่จะล้มเหลว PRECC จะแก้ไขก่อนการดำเนินการ

### ตัวอย่าง: Cargo Build ในไดเรกทอรีที่ผิด

สมมติว่าโปรเจกต์ของคุณอยู่ที่ `~/projects/myapp/` และ Claude ออกคำสั่ง:

```
cargo build
```

จาก `~/projects/` (สูงกว่าหนึ่งระดับ ไม่มี `Cargo.toml` ที่นั่น)

**ไม่มี PRECC:** Claude ได้รับข้อผิดพลาด `could not find Cargo.toml in /home/user/projects or any parent directory` อ่าน วิเคราะห์ แล้วลองใหม่ด้วย `cd myapp && cargo build` ค่าใช้จ่าย: ~2,000 โทเค็นสูญเปล่า

**ใช้ PRECC:** hook ตรวจพบว่าไม่มี `Cargo.toml` หาเจอใน `myapp/` และเขียนคำสั่งใหม่เป็น:

```
cd /home/user/projects/myapp && cargo build
```

Claude ไม่เห็นข้อผิดพลาดเลย ไม่มีโทเค็นสูญเปล่า

## ขั้นตอนที่ 5: ตรวจสอบการประหยัดของคุณ

หลังจากเซสชัน ดูว่า PRECC ประหยัดโทเค็นได้เท่าไหร่:

```bash
$ precc savings
Session Token Savings
=====================
Total estimated savings: 4,312 tokens

Breakdown:
  Pillar 1 (cd prepends):       2,104 tokens  (3 corrections)
  Pillar 4 (skill activations):   980 tokens  (2 activations)
  RTK rewrites:                 1,228 tokens  (5 rewrites)
```

## ขั้นตอนถัดไป

- [ทักษะ](skills.md) -- ดูทักษะทั้งหมดที่มีและวิธีสร้างทักษะของคุณเอง
- [Hook Pipeline](hook-pipeline.md) -- เข้าใจว่าเกิดอะไรขึ้นเบื้องหลัง
- [การประหยัด](savings.md) -- การวิเคราะห์การประหยัดโทเค็นอย่างละเอียด
