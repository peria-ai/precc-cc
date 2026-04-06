# تحلیل GitHub Actions

`precc gha` اجراهای ناموفق GitHub Actions را تحلیل کرده و اصلاحات پیشنهاد می‌دهد. این یک ویژگی Pro است.

## استفاده

URL اجرای ناموفق GitHub Actions را ارسال کنید:

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

## عملکرد

1. URL اجرای GitHub Actions را تحلیل کرده و مالک، مخزن و شناسه اجرا را استخراج می‌کند.
2. لاگ‌های اجرا را از طریق API GitHub دریافت می‌کند (در صورت تنظیم از `GITHUB_TOKEN` استفاده می‌کند، در غیر این صورت دسترسی عمومی).
3. مرحله ناموفق را شناسایی کرده و خطوط خطای مربوطه را استخراج می‌کند.
4. خطا را تحلیل کرده و بر اساس الگوهای رایج خرابی CI اصلاحی پیشنهاد می‌دهد.

## الگوهای خرابی پشتیبانی‌شده

- کانتینرهای سرویس گمشده (پایگاه‌های داده، Redis و غیره)
- سیستم‌عامل یا معماری runner نادرست
- متغیرهای محیطی یا secrets گمشده
- خرابی‌های نصب وابستگی
- وقفه‌های زمانی تست
- خطاهای مجوز
- عدم وجود کش که باعث ساخت کند می‌شود
