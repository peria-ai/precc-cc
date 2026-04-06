# Câu hỏi thường gặp

## PRECC có an toàn không?

Có. PRECC sử dụng cơ chế hook PreToolUse chính thức của Claude Code -- cùng điểm mở rộng mà Anthropic thiết kế cho mục đích này. Hook:

- Chạy hoàn toàn ngoại tuyến (không có lệnh gọi mạng trong đường dẫn nóng)
- Hoàn thành trong dưới 5 mili giây
- Là fail-open: nếu có lỗi, lệnh gốc chạy không thay đổi
- Chỉ sửa đổi lệnh, không bao giờ tự thực thi
- Lưu trữ dữ liệu cục bộ trong cơ sở dữ liệu SQLite

## PRECC có hoạt động với các công cụ lập trình AI khác không?

PRECC được thiết kế riêng cho Claude Code. Nó phụ thuộc vào giao thức hook PreToolUse mà Claude Code cung cấp. Nó không hoạt động với Cursor, Copilot, Windsurf hoặc các công cụ lập trình AI khác.

## Đo lường từ xa gửi dữ liệu gì?

Đo lường từ xa chỉ hoạt động khi bạn đồng ý. Khi được bật, nó gửi:

- Phiên bản PRECC, hệ điều hành và kiến trúc
- Số liệu tổng hợp (lệnh bị chặn, kỹ năng được kích hoạt)
- Độ trễ hook trung bình

Nó **không** gửi văn bản lệnh, đường dẫn tệp, tên dự án hoặc bất kỳ thông tin nhận dạng cá nhân nào. Bạn có thể xem trước dữ liệu chính xác với `precc telemetry preview` trước khi đồng ý. Xem [Đo lường từ xa](telemetry.md) để biết chi tiết.

## Làm thế nào để gỡ cài đặt PRECC?

??faq_uninstall_a_intro??

1. Xóa đăng ký hook:
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. Xóa tệp nhị phân:
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. Xóa dữ liệu (tùy chọn):
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## Giấy phép của tôi đã hết hạn. Điều gì xảy ra?

PRECC trở về tầng Community. Tất cả chức năng cốt lõi tiếp tục hoạt động:

- Các kỹ năng tích hợp vẫn hoạt động
- Pipeline hook chạy bình thường
- `precc savings` hiển thị chế độ xem tóm tắt
- `precc ingest` và khai thác phiên hoạt động

Các tính năng Pro không khả dụng cho đến khi bạn gia hạn:

- `precc savings --all` (phân tích chi tiết)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- Báo cáo qua email

## Hook dường như không chạy. Làm thế nào để gỡ lỗi?

??faq_debug_a_intro??

1. Kiểm tra xem hook đã được đăng ký chưa:
   ```bash
   precc init
   ```

2. Kiểm tra hook thủ công:
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. Kiểm tra xem tệp nhị phân có trong PATH không:
   ```bash
   which precc-hook
   ```

4. Kiểm tra cấu hình hook của Claude Code trong `~/.claude/settings.json`.

## PRECC có làm chậm Claude Code không?

Không. Hook hoàn thành trong dưới 5 mili giây (p99). Điều này không thể cảm nhận được so với thời gian Claude dành cho suy luận và tạo phản hồi.

## Tôi có thể sử dụng PRECC trong CI/CD không?

PRECC được thiết kế cho các phiên Claude Code tương tác. Trong CI/CD, không có phiên bản Claude Code nào để hook vào. Tuy nhiên, `precc gha` có thể phân tích các lần chạy GitHub Actions thất bại từ bất kỳ môi trường nào.

## Các kỹ năng khai thác khác gì so với kỹ năng tích hợp?

Các kỹ năng tích hợp được cung cấp cùng PRECC và bao gồm các mẫu sai thư mục phổ biến. Các kỹ năng khai thác được học từ nhật ký phiên cụ thể của bạn -- chúng nắm bắt các mẫu riêng biệt cho quy trình làm việc của bạn. Cả hai đều được lưu trữ trong SQLite và được đánh giá giống nhau bởi pipeline hook.

## Tôi có thể chia sẻ kỹ năng với nhóm không?

Có. Xuất bất kỳ kỹ năng nào sang TOML với `precc skills export NAME` và chia sẻ tệp. Các thành viên nhóm có thể đặt nó trong thư mục `skills/` hoặc nhập vào cơ sở dữ liệu heuristics.
