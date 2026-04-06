# فشرده‌سازی

`precc compress` فایل CLAUDE.md و سایر فایل‌های زمینه را کوچک می‌کند تا مصرف توکن هنگام بارگذاری توسط Claude Code کاهش یابد. این یک ویژگی Pro است.

## استفاده پایه

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

## اجرای آزمایشی

پیش‌نمایش تغییرات بدون اصلاح فایل‌ها:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## بازگردانی

فایل‌های اصلی به‌طور خودکار پشتیبان‌گیری می‌شوند. برای بازیابی آنها:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## چه چیزی فشرده می‌شود

فشرده‌ساز چندین تبدیل اعمال می‌کند:

- فضاهای خالی و خطوط خالی اضافی را حذف می‌کند
- عبارات طولانی را کوتاه می‌کند و معنی را حفظ می‌کند
- جداول و فهرست‌ها را فشرده می‌کند
- نظرات و قالب‌بندی تزئینی را حذف می‌کند
- تمام بلوک‌های کد، مسیرها و شناسه‌های فنی را حفظ می‌کند

خروجی فشرده همچنان قابل خواندن است -- فشرده‌سازی یا مبهم‌سازی نشده است.

## هدف‌گیری فایل‌های خاص

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
