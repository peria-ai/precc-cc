# Đo lường từ xa

PRECC hỗ trợ đo lường từ xa ẩn danh tùy chọn để giúp cải thiện công cụ. Không có dữ liệu nào được thu thập trừ khi bạn đồng ý rõ ràng.

## Đồng ý tham gia

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## Từ chối tham gia

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## Kiểm tra trạng thái

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## Xem trước dữ liệu sẽ được gửi

Trước khi đồng ý, bạn có thể xem chính xác dữ liệu nào sẽ được thu thập:

```bash
$ precc telemetry preview
Telemetry payload (this session):
{
  "version": "0.3.0",
  "os": "linux",
  "arch": "x86_64",
  "skills_activated": 12,
  "commands_intercepted": 87,
  "pillars_used": [1, 4],
  "avg_hook_latency_ms": 2.3,
  "session_count": 1
}
```

## Dữ liệu được thu thập

- Phiên bản PRECC, hệ điều hành và kiến trúc
- Số liệu tổng hợp: lệnh bị chặn, kỹ năng được kích hoạt, trụ cột được sử dụng
- Độ trễ hook trung bình
- Số phiên

## Dữ liệu KHÔNG được thu thập

- Không có văn bản lệnh hoặc đối số
- Không có đường dẫn tệp hoặc tên thư mục
- Không có tên dự án hoặc URL kho lưu trữ
- Không có thông tin nhận dạng cá nhân (PII)
- Không có địa chỉ IP (máy chủ không ghi lại chúng)

## Ghi đè bằng biến môi trường

Để tắt đo lường từ xa mà không cần chạy lệnh (hữu ích trong CI hoặc môi trường dùng chung):

```bash
export PRECC_NO_TELEMETRY=1
```

Điều này được ưu tiên hơn cài đặt đồng ý.

## Đích đến dữ liệu

Dữ liệu đo lường từ xa được gửi tới `https://telemetry.peria.ai/v1/precc` qua HTTPS. Dữ liệu chỉ được sử dụng để hiểu các mẫu sử dụng và ưu tiên phát triển.
