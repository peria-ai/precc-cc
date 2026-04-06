# Nén

`precc compress` thu nhỏ CLAUDE.md và các tệp ngữ cảnh khác để giảm sử dụng token khi Claude Code tải chúng. Đây là tính năng Pro.

## Sử dụng cơ bản

```bash
$ precc compress .
[precc] Scanning directory: .
[precc] Found 3 context files:
         CLAUDE.md (2,847 tokens -> 1,203 tokens, -57.7%)
         ARCHITECTURE.md (4,112 tokens -> 2,044 tokens, -50.3%)
         ALTERNATIVES.md (3,891 tokens -> 1,967 tokens, -49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
[precc] Files compressed. Use --revert to restore originals.
```

## Chạy thử

Xem trước những gì sẽ thay đổi mà không sửa đổi tệp:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## Hoàn nguyên

Các tệp gốc được sao lưu tự động. Để khôi phục chúng:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## Những gì được nén

Bộ nén áp dụng nhiều phép biến đổi:

- Xóa khoảng trắng và dòng trống thừa
- Rút ngắn cách diễn đạt dài dòng nhưng giữ nguyên ý nghĩa
- Cô đọng bảng và danh sách
- Loại bỏ chú thích và định dạng trang trí
- Giữ nguyên tất cả khối mã, đường dẫn và định danh kỹ thuật

Đầu ra nén vẫn đọc được -- không bị rút gọn hay làm rối.

## Nhắm mục tiêu tệp cụ thể

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
