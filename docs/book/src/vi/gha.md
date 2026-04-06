# Phân tích GitHub Actions

`precc gha` phân tích các lần chạy GitHub Actions thất bại và đề xuất sửa chữa. Đây là tính năng Pro.

## Cách sử dụng

Truyền URL của lần chạy GitHub Actions thất bại:

```bash
$ precc gha https://github.com/myorg/myrepo/actions/runs/12345678
[precc] Fetching run 12345678...
[precc] Run: CI / build (ubuntu-latest)
[precc] Status: failure
[precc] Failed step: Run cargo test

[precc] Log analysis:
  Error: test result: FAILED. 2 passed; 1 failed
  Failed test: tests::integration::test_database_connection
  Cause: thread 'tests::integration::test_database_connection' panicked at
         'called Result::unwrap() on an Err value: Connection refused'

[precc] Suggested fix:
  The test requires a database connection but the CI environment does not
  start a database service. Add a services block to your workflow:

    services:
      postgres:
        image: postgres:15
        ports:
          - 5432:5432
        env:
          POSTGRES_PASSWORD: test
```

## Chức năng

1. Phân tích URL chạy GitHub Actions để trích xuất chủ sở hữu, kho lưu trữ và ID chạy.
2. Lấy nhật ký chạy qua API GitHub (sử dụng `GITHUB_TOKEN` nếu được đặt, nếu không thì truy cập công khai).
3. Xác định bước thất bại và trích xuất các dòng lỗi liên quan.
4. Phân tích lỗi và đề xuất sửa chữa dựa trên các mẫu lỗi CI phổ biến.

## Các mẫu lỗi được hỗ trợ

- Thiếu container dịch vụ (cơ sở dữ liệu, Redis, v.v.)
- Hệ điều hành hoặc kiến trúc runner không chính xác
- Thiếu biến môi trường hoặc secrets
- Lỗi cài đặt phụ thuộc
- Hết thời gian chờ kiểm thử
- Lỗi quyền truy cập
- Cache miss gây ra build chậm
