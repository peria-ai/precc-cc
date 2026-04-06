# Giấy phép

PRECC cung cấp hai cấp: Community (miễn phí) và Pro.

## Cấp Community (miễn phí)

Cấp Community bao gồm:

- Tất cả kỹ năng tích hợp (sửa thư mục sai, dịch jj, v.v.)
- Pipeline hook với hỗ trợ đầy đủ Pillar 1 và Pillar 4
- Tóm tắt cơ bản `precc savings`
- Khai thác phiên với `precc ingest`
- Sử dụng cục bộ không giới hạn

## Cấp Pro

Pro mở khóa các tính năng bổ sung:

- **Phân tích chi tiết tiết kiệm** -- `precc savings --all` với phân tích theo lệnh
- **Ghi GIF** -- `precc gif` để tạo GIF động terminal
- **Tuân thủ geofence IP** -- Cho môi trường được quản lý
- **Báo cáo email** -- `precc mail report` để gửi phân tích
- **Phân tích GitHub Actions** -- `precc gha` để gỡ lỗi workflow thất bại
- **Nén ngữ cảnh** -- `precc compress` để tối ưu hóa CLAUDE.md
- **Hỗ trợ ưu tiên**

## Kích hoạt giấy phép

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## Kiểm tra trạng thái giấy phép

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## Kích hoạt GitHub Sponsors

Nếu bạn tài trợ PRECC qua GitHub Sponsors, giấy phép được kích hoạt tự động qua email GitHub của bạn. Không cần khóa -- chỉ cần đảm bảo email tài trợ khớp:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## Dấu vân tay thiết bị

Mỗi giấy phép được gắn với dấu vân tay thiết bị. Xem của bạn với:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

Nếu bạn cần chuyển giấy phép sang máy mới, hãy hủy kích hoạt trước:

```bash
precc license deactivate
```

Sau đó kích hoạt trên máy mới.

## Giấy phép hết hạn?

Khi giấy phép Pro hết hạn, PRECC quay lại cấp Community. Tất cả kỹ năng tích hợp và chức năng cốt lõi tiếp tục hoạt động. Chỉ các tính năng dành riêng cho Pro trở nên không khả dụng. Xem [FAQ](faq.md) để biết thêm chi tiết.
