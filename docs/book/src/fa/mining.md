# استخراج

PRECC لاگ‌های جلسات Claude Code را استخراج می‌کند تا الگوهای خطا-اصلاح را بیاموزد. وقتی همان اشتباه را دوباره می‌بیند، اصلاح را به‌طور خودکار اعمال می‌کند.

## دریافت لاگ‌های جلسه

### دریافت یک فایل

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### دریافت همه لاگ‌ها

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### اجبار دریافت مجدد

برای پردازش مجدد فایل‌هایی که قبلاً دریافت شده‌اند:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## نحوه کار استخراج

1. PRECC فایل لاگ JSONL جلسه را می‌خواند.
2. جفت دستوراتی را شناسایی می‌کند که دستور اول شکست خورده و دومی تلاش مجدد اصلاح‌شده بود.
3. الگو (چه اشتباهی رخ داد) و اصلاح (چه کاری متفاوت انجام شد) را استخراج می‌کند.
4. الگوها در `~/.local/share/precc/history.db` ذخیره می‌شوند.
5. وقتی الگویی به آستانه اطمینان برسد، به مهارت استخراج‌شده در `heuristics.db` تبدیل می‌شود.

### نمونه الگو

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## دیمن precc-learner

دیمن `precc-learner` در پس‌زمینه اجرا می‌شود و به‌طور خودکار لاگ‌های جلسه جدید را زیر نظر می‌گیرد:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

دیمن از اعلان‌های سیستم فایل (inotify در لینوکس، FSEvents در macOS) استفاده می‌کند و بلافاصله پس از پایان جلسه واکنش نشان می‌دهد.

## از الگوها به مهارت‌ها

الگوهای استخراج‌شده زمانی به مهارت ارتقا می‌یابند که این معیارها را برآورده کنند:

- حداقل 3 بار در جلسات مختلف دیده شده
- الگوی اصلاح سازگار (همان نوع اصلاح هر بار)
- هیچ مثبت کاذبی شناسایی نشده

می‌توانید نامزدهای مهارت را بررسی کنید:

```bash
$ precc skills advise
```

برای جزئیات مدیریت مهارت‌ها به [Skills](skills.md) مراجعه کنید.

## ذخیره‌سازی داده

- **جفت‌های خطا-اصلاح**: `~/.local/share/precc/history.db`
- **مهارت‌های ارتقا یافته**: `~/.local/share/precc/heuristics.db`

هر دو پایگاه داده SQLite در حالت WAL برای دسترسی همزمان امن هستند.
