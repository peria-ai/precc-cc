# Cài đặt

## Cài đặt nhanh (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

Lệnh này tải xuống bản phát hành mới nhất cho nền tảng của bạn, xác minh mã SHA256 và đặt vào `~/.local/bin/`.

Sau khi cài đặt, khởi tạo PRECC:

```bash
precc init
```

`precc init` đăng ký hook PreToolUse với Claude Code, tạo các thư mục dữ liệu và khởi tạo cơ sở dữ liệu kỹ năng.

## Tùy chọn cài đặt

### Xác minh SHA256

Mặc định, trình cài đặt xác minh mã checksum của tệp nhị phân với SHA256 đã công bố. Để bỏ qua xác minh (không khuyến nghị):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### Tiền tố cài đặt tùy chỉnh

Cài đặt vào vị trí tùy chỉnh:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### Công cụ đi kèm (--extras)

PRECC đi kèm với các công cụ tùy chọn. Cài đặt chúng với `--extras`:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

Lệnh này cài đặt:

| Công cụ | Mục đích |
|------|---------|
| **RTK** | Bộ công cụ viết lại lệnh |
| **lean-ctx** | Nén ngữ cảnh cho CLAUDE.md và các tệp prompt |
| **nushell** | Shell có cấu trúc cho pipeline nâng cao |
| **cocoindex-code** | Lập chỉ mục mã để giải quyết ngữ cảnh nhanh hơn |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

Sau đó khởi tạo:

```powershell
precc init
```

## Cài đặt thủ công

1. Tải xuống tệp nhị phân cho nền tảng của bạn từ [GitHub Releases](https://github.com/peria-ai/precc-cc/releases).
2. Xác minh mã SHA256 với tệp `.sha256` trong bản phát hành.
3. Đặt tệp nhị phân vào thư mục trong `PATH` của bạn (ví dụ: `~/.local/bin/`).
4. Chạy `precc init`.

## Cập nhật

```bash
precc update
```

Buộc cập nhật lên phiên bản cụ thể:

```bash
precc update --force --version 0.3.0
```

Bật cập nhật tự động:

```bash
precc update --auto
```

## Xác minh cài đặt

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

Nếu không tìm thấy `precc`, hãy đảm bảo `~/.local/bin` nằm trong `PATH` của bạn.
