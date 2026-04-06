# گزارش‌ها

`precc report` یک داشبورد تحلیلی تولید می‌کند که فعالیت PRECC و صرفه‌جویی توکن را خلاصه می‌کند.

## ایجاد گزارش

```bash
$ precc report
PRECC Report -- 2026-04-03
==========================

Sessions analyzed: 12
Commands intercepted: 87
Total token savings: 42,389

Top skills by activation:
  1. cargo-wrong-dir     34 activations   17,204 tokens saved
  2. npm-wrong-dir       18 activations    9,360 tokens saved
  3. git-wrong-dir       12 activations    4,944 tokens saved
  4. RTK rewrite         15 activations    3,750 tokens saved
  5. python-wrong-dir     8 activations    4,131 tokens saved

Savings by pillar:
  Pillar 1 (context resolution):  28,639 tokens  67.6%
  Pillar 4 (automation skills):    7,000 tokens  16.5%
  RTK rewrites:                    3,750 tokens   8.8%
  Lean-ctx wraps:                  3,000 tokens   7.1%

Recent corrections:
  2026-04-03 09:12  cargo build -> cd myapp && cargo build
  2026-04-03 09:18  npm test -> cd frontend && npm test
  2026-04-03 10:05  git status -> cd repo && git status
  ...
```

## ارسال گزارش با ایمیل

گزارش را به آدرس ایمیل ارسال کنید (نیاز به تنظیمات ایمیل دارد، به [Email](email.md) مراجعه کنید):

```bash
$ precc report --email
[precc] Report sent to you@example.com
```

آدرس گیرنده از `~/.config/precc/mail.toml` خوانده می‌شود. همچنین می‌توانید از `precc mail report EMAIL` برای ارسال به آدرس خاص استفاده کنید.

## داده‌های گزارش

گزارش‌ها از پایگاه داده محلی PRECC در `~/.local/share/precc/history.db` تولید می‌شوند. هیچ داده‌ای از دستگاه شما خارج نمی‌شود مگر اینکه صراحتاً گزارش را ایمیل کنید.
