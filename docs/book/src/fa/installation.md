# نصب

## نصب سریع (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

این فایل باینری آخرین نسخه را برای پلتفرم شما دانلود می‌کند، جمع‌آزمای SHA256 را تأیید می‌کند و آن را در `~/.local/bin/` قرار می‌دهد.

پس از نصب، PRECC را مقداردهی اولیه کنید:

```bash
precc init
```

`precc init` هوک PreToolUse را در Claude Code ثبت می‌کند، پوشه‌های داده را ایجاد می‌کند و پایگاه داده مهارت‌ها را مقداردهی اولیه می‌کند.

## گزینه‌های نصب

### تأیید SHA256

به طور پیش‌فرض، نصب‌کننده جمع‌آزمای باینری را با SHA256 منتشر شده تأیید می‌کند. برای رد شدن از تأیید (توصیه نمی‌شود):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### پیشوند نصب سفارشی

نصب در مسیر دلخواه:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### ابزارهای همراه (--extras)

PRECC با ابزارهای همراه اختیاری ارائه می‌شود. آنها را با `--extras` نصب کنید:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

این موارد زیر را نصب می‌کند:

| ابزار | کاربرد |
|------|---------|
| **RTK** | مجموعه ابزار بازنویسی دستورات |
| **lean-ctx** | فشرده‌سازی زمینه برای CLAUDE.md و فایل‌های prompt |
| **nushell** | شل ساختاریافته برای پایپلاین‌های پیشرفته |
| **cocoindex-code** | نمایه‌سازی کد برای حل سریع‌تر زمینه |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

سپس مقداردهی اولیه کنید:

```powershell
precc init
```

## نصب دستی

1. فایل باینری نسخه مربوط به پلتفرم خود را از [GitHub Releases](https://github.com/peria-ai/precc-cc/releases) دانلود کنید.
2. جمع‌آزمای SHA256 را با فایل `.sha256` در نسخه تأیید کنید.
3. فایل باینری را در پوشه‌ای در `PATH` خود قرار دهید (مثلاً `~/.local/bin/`).
4. `precc init` را اجرا کنید.

## به‌روزرسانی

```bash
precc update
```

به‌روزرسانی اجباری به نسخه مشخص:

```bash
precc update --force --version 0.3.0
```

فعال‌سازی به‌روزرسانی خودکار:

```bash
precc update --auto
```

## تأیید نصب

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

اگر `precc` پیدا نشد، مطمئن شوید که `~/.local/bin` در `PATH` شما قرار دارد.
