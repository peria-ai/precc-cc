# Hàng rào địa lý

PRECC bao gồm kiểm tra tuân thủ hàng rào địa lý IP cho các môi trường được quản lý. Đây là tính năng Pro.

## Tổng quan

Một số tổ chức yêu cầu các công cụ phát triển chỉ hoạt động trong các vùng địa lý được phê duyệt. Tính năng hàng rào địa lý của PRECC xác minh rằng địa chỉ IP của máy hiện tại nằm trong danh sách vùng được phép.

## Kiểm tra tuân thủ

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

Nếu máy nằm ngoài các vùng được phép:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## Làm mới dữ liệu hàng rào địa lý

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## Xem thông tin hàng rào địa lý

```bash
$ precc geofence info
Geofence Configuration
======================
Policy file:    ~/.config/precc/geofence.toml
Allowed regions: us-east-1, us-west-2, eu-west-1
Cache age:      2h 14m
Last check:     2026-04-03 09:12:00 UTC
Status:         COMPLIANT
```

## Xóa bộ nhớ đệm

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## Cấu hình

Chính sách hàng rào địa lý được định nghĩa trong `~/.config/precc/geofence.toml`:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

Đặt `block_on_violation = true` để ngăn PRECC hoạt động khi nằm ngoài các vùng được phép.
