# Bắt đầu nhanh

Khởi chạy PRECC trong 5 phút.

## Bước 1: Cài đặt

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## Bước 2: Khởi tạo

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## Bước 3: Xác minh hook đang hoạt động

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

## Bước 4: Sử dụng Claude Code bình thường

Mở Claude Code và làm việc bình thường. PRECC chạy ngầm trong nền. Khi Claude phát ra lệnh sẽ thất bại, PRECC sửa nó trước khi thực thi.

### Ví dụ: Cargo Build sai thư mục

Giả sử dự án của bạn ở `~/projects/myapp/` và Claude ra lệnh:

```
cargo build
```

từ `~/projects/` (cao hơn một cấp, không có `Cargo.toml` ở đó).

**Không có PRECC:** Claude nhận lỗi `could not find Cargo.toml in /home/user/projects or any parent directory`, đọc, suy luận, rồi thử lại với `cd myapp && cargo build`. Chi phí: ~2.000 token lãng phí.

**Với PRECC:** Hook phát hiện thiếu `Cargo.toml`, tìm thấy trong `myapp/` và viết lại lệnh thành:

```
cd /home/user/projects/myapp && cargo build
```

Claude không bao giờ thấy lỗi. Không lãng phí token nào.

## Bước 5: Kiểm tra tiết kiệm của bạn

Sau một phiên làm việc, xem PRECC đã tiết kiệm bao nhiêu token:

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

## Bước tiếp theo

- [Kỹ năng](skills.md) -- Xem tất cả kỹ năng có sẵn và cách tạo kỹ năng riêng.
- [Pipeline Hook](hook-pipeline.md) -- Hiểu những gì xảy ra bên trong.
- [Tiết kiệm](savings.md) -- Phân tích chi tiết tiết kiệm token.
