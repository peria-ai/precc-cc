# Giới thiệu

## PRECC là gì?

PRECC (Sửa lỗi dự đoán cho Claude Code) là một công cụ Rust chặn các lệnh bash của Claude Code thông qua cơ chế hook PreToolUse chính thức. Nó sửa lỗi *trước khi chúng xảy ra*, tiết kiệm token và loại bỏ vòng lặp thử lại.

Miễn phí cho người dùng cộng đồng.

## Vấn đề

Claude Code lãng phí token đáng kể vào các lỗi có thể phòng tránh:

- **Lỗi thư mục** -- Chạy `cargo build` trong thư mục cha không có `Cargo.toml`.
- **Vòng lặp thử lại** -- Lệnh thất bại tạo ra đầu ra dài dòng.
- **Đầu ra dài dòng** -- Các lệnh như `find` hoặc `ls -R` xuất hàng nghìn dòng.

## Bốn trụ cột

### Sửa ngữ cảnh (cd-prepend)

Phát hiện khi các lệnh như `cargo build` hoặc `npm test` chạy trong thư mục sai và thêm `cd /đường/dẫn/đúng &&` trước khi thực thi.

### Gỡ lỗi GDB

Phát hiện cơ hội gắn GDB để gỡ lỗi sâu hơn.

### Khai thác phiên

Khai thác nhật ký phiên Claude Code để tìm các cặp lỗi-sửa.

### Kỹ năng tự động hóa

Thư viện các kỹ năng khớp với mẫu lệnh và viết lại chúng.

## Cách hoạt động (phiên bản 30 giây)

1. Claude Code chuẩn bị chạy một lệnh bash.
2. Hook PreToolUse gửi lệnh tới `precc-hook` dưới dạng JSON.
3. `precc-hook` xử lý lệnh trong dưới 3 mili giây.
4. Lệnh đã sửa được trả về dưới dạng JSON.
5. Claude Code thực thi lệnh đã sửa.

Claude không bao giờ thấy lỗi.

### Nén thích ứng

Nếu một lệnh thất bại sau khi nén, PRECC tự động bỏ qua nén ở lần thử tiếp theo để Claude nhận được đầu ra đầy đủ không nén để gỡ lỗi.

## Thống kê sử dụng trực tiếp

Phiên bản hiện tại <span data-stat="current_version">--</span>:

| Chỉ số | Giá trị |
|---|---|
| Số lần gọi hook | <span data-stat="total_invocations">--</span> |
| Token đã tiết kiệm | <span data-stat="total_tokens_saved">--</span> |
| Tỷ lệ tiết kiệm | <span data-stat="saving_pct">--</span>% |
| Viết lại RTK | <span data-stat="rtk_rewrites">--</span> |
| Sửa CD | <span data-stat="cd_prepends">--</span> |
| Độ trễ hook | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| Người dùng | <span data-stat="unique_users">--</span> |

### Tiết kiệm theo phiên bản

<table id="version-breakdown" style="display:none">
<thead><tr><th>Phiên bản</th><th>Người dùng</th><th>Số lần gọi hook</th><th>Token đã tiết kiệm</th><th>Tỷ lệ tiết kiệm</th></tr></thead>
<tbody><tr><td colspan="5"><em>Đang tải...</em></td></tr></tbody>
</table>

<small>Các con số là ước tính. Mỗi lỗi được ngăn chặn tránh được một chu trình thử lại hoàn chỉnh: đầu ra lỗi, suy luận của mô hình và lệnh thử lại. Các con số này tự động cập nhật từ dữ liệu đo lường ẩn danh.</small>

## Liên kết

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Trang web: [https://peria.ai](https://peria.ai)
- Tài liệu: [https://precc.cc](https://precc.cc)
