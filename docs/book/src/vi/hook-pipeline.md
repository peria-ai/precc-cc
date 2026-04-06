# Đường ống Hook

Binary `precc-hook` là cốt lõi của PRECC. Nó nằm giữa Claude Code và shell, xử lý mọi lệnh bash trong vòng dưới 5 mili giây.

## Cách Claude Code gọi Hook

Claude Code hỗ trợ các hook PreToolUse -- chương trình bên ngoài có thể kiểm tra và sửa đổi đầu vào công cụ trước khi thực thi. Khi Claude sắp chạy lệnh bash, nó gửi JSON đến `precc-hook` qua stdin và đọc phản hồi từ stdout.

## Các giai đoạn Pipeline

```
Claude Code
    |
    v
+---------------------------+
| 1. Parse JSON stdin       |  Read the command from Claude Code
+---------------------------+
    |
    v
+---------------------------+
| 2. Skill matching         |  Query heuristics.db for matching skills (Pillar 4)
+---------------------------+
    |
    v
+---------------------------+
| 3. Directory correction   |  Resolve correct working directory (Pillar 1)
+---------------------------+
    |
    v
+---------------------------+
| 4. GDB check              |  Detect debug opportunities (Pillar 2)
+---------------------------+
    |
    v
+---------------------------+
| 5. RTK rewriting          |  Apply command rewrites for token savings
+---------------------------+
    |
    v
+---------------------------+
| 6. Emit JSON stdout       |  Return modified command to Claude Code
+---------------------------+
    |
    v
  Shell executes corrected command
```

## Ví dụ: Đầu vào và đầu ra JSON

### Đầu vào (từ Claude Code)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC phát hiện thư mục hiện tại không có `Cargo.toml`, nhưng `./myapp/Cargo.toml` tồn tại.

### Đầu ra (đến Claude Code)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

Nếu không cần sửa đổi, `updatedInput.command` sẽ trống và Claude Code sử dụng lệnh gốc.

## Chi tiết các giai đoạn

### Giai đoạn 1: Phân tích JSON

Đọc toàn bộ đối tượng JSON từ stdin. Trích xuất `tool_input.command`. Nếu phân tích thất bại, hook thoát ngay lập tức và Claude Code sử dụng lệnh gốc (thiết kế fail-open).

### Giai đoạn 2: Khớp kỹ năng

Truy vấn cơ sở dữ liệu heuristic SQLite để tìm các kỹ năng có mẫu kích hoạt khớp với lệnh. Các kỹ năng được kiểm tra theo thứ tự ưu tiên. Cả kỹ năng TOML tích hợp và kỹ năng đã khai thác đều được đánh giá.

### Giai đoạn 3: Sửa thư mục

Đối với các lệnh build (`cargo`, `go`, `make`, `npm`, `python`, v.v.), kiểm tra xem tệp dự án mong đợi có tồn tại trong thư mục hiện tại không. Nếu không, quét các thư mục lân cận để tìm kết quả khớp gần nhất và thêm `cd <dir> &&` vào đầu.

Quá trình quét thư mục sử dụng chỉ mục hệ thống tệp được lưu trong bộ nhớ đệm với TTL 5 giây để duy trì tốc độ.

### Giai đoạn 4: Kiểm tra GDB

Nếu lệnh có khả năng gây ra sự cố (ví dụ: chạy binary debug), PRECC có thể đề xuất hoặc chèn các wrapper GDB để thu thập đầu ra debug có cấu trúc thay vì log crash thô.

### Giai đoạn 5: Viết lại RTK

Áp dụng các quy tắc RTK (Rewrite Toolkit) để rút ngắn lệnh dài dòng, loại bỏ đầu ra nhiễu hoặc tái cấu trúc lệnh để tiết kiệm token.

### Giai đoạn 6: Phát JSON

Tuần tự hóa lệnh đã sửa đổi thành JSON và ghi ra stdout. Nếu không có thay đổi, đầu ra báo hiệu cho Claude Code sử dụng lệnh gốc.

## Hiệu suất

Toàn bộ pipeline hoàn thành trong dưới 5 mili giây (p99). Các tối ưu hóa chính:

- SQLite ở chế độ WAL cho đọc đồng thời không khóa
- Các mẫu regex biên dịch trước để khớp kỹ năng
- Quét hệ thống tệp được lưu trong bộ nhớ đệm (TTL 5 giây)
- Không có cuộc gọi mạng trong đường dẫn nóng
- Fail-open: mọi lỗi đều chuyển về lệnh gốc

## Kiểm tra Hook thủ công

Bạn có thể gọi hook trực tiếp:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
