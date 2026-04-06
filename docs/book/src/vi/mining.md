# Khai thác

PRECC khai thác nhật ký phiên Claude Code để học các mẫu lỗi-sửa. Khi gặp lại cùng một lỗi, nó tự động áp dụng cách sửa.

## Nhập nhật ký phiên

### Nhập một tệp đơn

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### Nhập tất cả nhật ký

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### Buộc nhập lại

Để xử lý lại các tệp đã nhập:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## Cách khai thác hoạt động

1. PRECC đọc tệp nhật ký JSONL của phiên.
2. Nó xác định các cặp lệnh trong đó lệnh đầu tiên thất bại và lệnh thứ hai là lần thử lại đã sửa.
3. Nó trích xuất mẫu (điều gì sai) và cách sửa (Claude đã làm gì khác).
4. Các mẫu được lưu trong `~/.local/share/precc/history.db`.
5. Khi một mẫu đạt ngưỡng tin cậy (gặp nhiều lần), nó trở thành kỹ năng khai thác trong `heuristics.db`.

### Ví dụ mẫu

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## Daemon precc-learner

Daemon `precc-learner` chạy nền và tự động theo dõi nhật ký phiên mới:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

Daemon sử dụng thông báo hệ thống tệp (inotify trên Linux, FSEvents trên macOS) nên phản ứng ngay khi phiên kết thúc.

## Từ mẫu đến kỹ năng

Các mẫu khai thác được nâng cấp thành kỹ năng khi đáp ứng các tiêu chí sau:

- Xuất hiện ít nhất 3 lần qua các phiên
- Mẫu sửa nhất quán (cùng loại sửa mỗi lần)
- Không phát hiện dương tính giả

Bạn có thể xem xét các ứng viên kỹ năng với:

```bash
$ precc skills advise
```

Xem [Skills](skills.md) để biết chi tiết về quản lý kỹ năng.

## Lưu trữ dữ liệu

- **Cặp lỗi-sửa**: `~/.local/share/precc/history.db`
- **Kỹ năng đã nâng cấp**: `~/.local/share/precc/heuristics.db`

Cả hai đều là cơ sở dữ liệu SQLite ở chế độ WAL để truy cập đồng thời an toàn.
