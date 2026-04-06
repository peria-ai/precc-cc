# ทักษะ

ทักษะคือกฎการจับคู่รูปแบบที่ PRECC ใช้เพื่อตรวจจับและแก้ไขคำสั่ง สามารถเป็นแบบในตัว (จัดส่งเป็นไฟล์ TOML) หรือขุดจากบันทึกเซสชัน

## ทักษะในตัว

| ทักษะ | ทริกเกอร์เมื่อ | การดำเนินการ |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` นอกโปรเจกต์ Rust | เพิ่ม `cd` ไปยังไดเรกทอรี `Cargo.toml` ที่ใกล้ที่สุด |
| `git-wrong-dir` | `git *` นอกรีโพ git | เพิ่ม `cd` ไปยังไดเรกทอรี `.git` ที่ใกล้ที่สุด |
| `go-wrong-dir` | `go build/test` นอกโมดูล Go | เพิ่ม `cd` ไปยังไดเรกทอรี `go.mod` ที่ใกล้ที่สุด |
| `make-wrong-dir` | `make` โดยไม่มี Makefile ในไดเรกทอรีปัจจุบัน | เพิ่ม `cd` ไปยังไดเรกทอรี Makefile ที่ใกล้ที่สุด |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` นอกโปรเจกต์ Node | เพิ่ม `cd` ไปยังไดเรกทอรี `package.json` ที่ใกล้ที่สุด |
| `python-wrong-dir` | `python/pytest/pip` นอกโปรเจกต์ Python | เพิ่ม `cd` ไปยังโปรเจกต์ Python ที่ใกล้ที่สุด |
| `jj-translate` | `git *` ในรีโพ jj ที่อยู่ร่วมกัน | เขียนใหม่เป็นคำสั่ง `jj` ที่เทียบเท่า |
| `asciinema-gif` | `asciinema rec` | เขียนใหม่เป็น `precc gif` |

## แสดงรายการทักษะ

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
  9 fix-pytest-path    mined     pytest with wrong test path
```

## แสดงรายละเอียดทักษะ

```bash
$ precc skills show cargo-wrong-dir
Name:        cargo-wrong-dir
Type:        built-in
Source:      skills/builtin/cargo-wrong-dir.toml
Description: Detects cargo commands run outside a Rust project and prepends
             cd to the directory containing the nearest Cargo.toml.
Trigger:     ^cargo\s+(build|test|clippy|run|check|bench|doc)
Action:      prepend_cd
Marker:      Cargo.toml
Activations: 12
```

## ส่งออกทักษะเป็น TOML

```bash
$ precc skills export cargo-wrong-dir
[skill]
name = "cargo-wrong-dir"
description = "Prepend cd for cargo commands outside a Rust project"
trigger = "^cargo\\s+(build|test|clippy|run|check|bench|doc)"
action = "prepend_cd"
marker = "Cargo.toml"
priority = 10
```

## แก้ไขทักษะ

```bash
$ precc skills edit cargo-wrong-dir
```

คำสั่งนี้เปิดคำจำกัดความทักษะใน `$EDITOR` ของคุณ หลังจากบันทึก ทักษะจะถูกโหลดใหม่โดยอัตโนมัติ

## คำสั่ง Advise

`precc skills advise` วิเคราะห์เซสชันล่าสุดของคุณและแนะนำทักษะใหม่ตามรูปแบบที่ซ้ำกัน:

```bash
$ precc skills advise
Analyzed 47 commands from the last session.

Suggested skills:
  1. docker-wrong-dir: You ran `docker compose up` outside the project root 3 times.
     Suggested trigger: ^docker\s+compose
     Suggested marker: docker-compose.yml

  2. terraform-wrong-dir: You ran `terraform plan` outside the infra directory 2 times.
     Suggested trigger: ^terraform\s+(plan|apply|init)
     Suggested marker: main.tf

Accept suggestion [1/2/skip]?
```

## การจัดกลุ่มทักษะ

```bash
$ precc skills cluster
```

จัดกลุ่มทักษะที่ขุดได้ที่คล้ายกันเพื่อช่วยระบุรูปแบบที่ซ้ำซ้อนหรือทับซ้อนกัน

## ทักษะที่ขุดได้เทียบกับทักษะในตัว

ทักษะในตัวมาพร้อมกับ PRECC และถูกกำหนดใน `skills/builtin/*.toml` ครอบคลุมข้อผิดพลาดไดเรกทอรีผิดที่พบบ่อยที่สุด

ทักษะที่ขุดได้ถูกสร้างโดย `precc ingest` หรือ daemon `precc-learner` จากบันทึกเซสชันของคุณ ถูกเก็บใน `~/.local/share/precc/heuristics.db` และเฉพาะเจาะจงกับเวิร์กโฟลว์ของคุณ ดู [การขุด](mining.md) สำหรับรายละเอียด
