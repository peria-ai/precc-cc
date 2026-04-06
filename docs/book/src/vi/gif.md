# Ghi GIF

`precc gif` tạo bản ghi GIF động của phiên terminal từ các script bash. Đây là tính năng Pro.

## Cách sử dụng cơ bản

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

Đối số đầu tiên là một script bash chứa các lệnh cần chạy. Đối số thứ hai là thời lượng ghi tối đa.

## Định dạng script

Script là một tệp bash tiêu chuẩn:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Mô phỏng đầu vào

Đối với các lệnh tương tác, cung cấp giá trị đầu vào làm đối số bổ sung:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Mỗi đối số bổ sung được cung cấp dưới dạng một dòng stdin khi script yêu cầu đầu vào.

## Tùy chọn đầu ra

Tệp đầu ra được đặt tên theo script mặc định (`script.gif`). GIF sử dụng giao diện terminal tối với kích thước chuẩn 80x24.

## Tại sao GIF thay vì asciinema?

Kỹ năng tích hợp `asciinema-gif` tự động viết lại `asciinema rec` thành `precc gif`. Tệp GIF dễ di chuyển hơn -- chúng hiển thị trực tiếp trong GitHub README, Slack và email mà không cần trình phát.
