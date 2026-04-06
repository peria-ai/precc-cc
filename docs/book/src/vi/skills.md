# Kỹ năng

Kỹ năng là các quy tắc khớp mẫu mà PRECC sử dụng để phát hiện và sửa lệnh. Chúng có thể là tích hợp sẵn (phân phối dưới dạng tệp TOML) hoặc khai thác từ nhật ký phiên.

## Kỹ năng tích hợp

| Kỹ năng | Kích hoạt khi | Hành động |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` ngoài dự án Rust | Thêm `cd` đến thư mục `Cargo.toml` gần nhất |
| `git-wrong-dir` | `git *` ngoài kho git | Thêm `cd` đến thư mục `.git` gần nhất |
| `go-wrong-dir` | `go build/test` ngoài module Go | Thêm `cd` đến thư mục `go.mod` gần nhất |
| `make-wrong-dir` | `make` khi không có Makefile trong thư mục hiện tại | Thêm `cd` đến thư mục Makefile gần nhất |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` ngoài dự án Node | Thêm `cd` đến thư mục `package.json` gần nhất |
| `python-wrong-dir` | `python/pytest/pip` ngoài dự án Python | Thêm `cd` đến dự án Python gần nhất |
| `jj-translate` | `git *` trong kho jj đồng vị trí | Viết lại thành lệnh `jj` tương đương |
| `asciinema-gif` | `asciinema rec` | Viết lại thành `precc gif` |

## Liệt kê kỹ năng

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

## Hiển thị chi tiết kỹ năng

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

## Xuất kỹ năng sang TOML

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

## Chỉnh sửa kỹ năng

```bash
$ precc skills edit cargo-wrong-dir
```

Thao tác này mở định nghĩa kỹ năng trong `$EDITOR` của bạn. Sau khi lưu, kỹ năng được tải lại tự động.

## Lệnh Advise

`precc skills advise` phân tích phiên gần đây của bạn và đề xuất kỹ năng mới dựa trên các mẫu lặp lại:

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

## Gom nhóm kỹ năng

```bash
$ precc skills cluster
```

Nhóm các kỹ năng khai thác tương tự lại với nhau để giúp xác định các mẫu trùng lặp hoặc chồng chéo.

## Kỹ năng khai thác và kỹ năng tích hợp

Kỹ năng tích hợp được phân phối cùng PRECC và được định nghĩa trong `skills/builtin/*.toml`. Chúng bao gồm các lỗi thư mục sai phổ biến nhất.

Kỹ năng khai thác được tạo bởi `precc ingest` hoặc daemon `precc-learner` từ nhật ký phiên của bạn. Chúng được lưu trong `~/.local/share/precc/heuristics.db` và đặc thù cho quy trình làm việc của bạn. Xem [Khai thác](mining.md) để biết chi tiết.
