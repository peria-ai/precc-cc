# ضبط GIF

`precc gif` ضبط‌های GIF متحرک از جلسات ترمینال از اسکریپت‌های bash ایجاد می‌کند. این یک ویژگی Pro است.

## استفاده پایه

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

آرگومان اول یک اسکریپت bash حاوی دستورات اجرایی است. آرگومان دوم حداکثر طول ضبط است.

## فرمت اسکریپت

اسکریپت یک فایل bash استاندارد است:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## شبیه‌سازی ورودی

برای دستورات تعاملی، مقادیر ورودی را به عنوان آرگومان‌های اضافی ارائه دهید:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

هر آرگومان اضافی به عنوان یک خط stdin هنگامی که اسکریپت ورودی درخواست می‌کند ارائه می‌شود.

## گزینه‌های خروجی

فایل خروجی به طور پیش‌فرض بر اساس نام اسکریپت نام‌گذاری می‌شود (`script.gif`). GIF از تم تاریک ترمینال با ابعاد استاندارد 80x24 استفاده می‌کند.

## چرا GIF به جای asciinema؟

مهارت داخلی `asciinema-gif` به طور خودکار `asciinema rec` را به `precc gif` بازنویسی می‌کند. فایل‌های GIF قابل حمل‌تر هستند -- بدون نیاز به پخش‌کننده در GitHub README، Slack و ایمیل نمایش داده می‌شوند.
