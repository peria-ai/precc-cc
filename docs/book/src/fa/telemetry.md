# تله‌متری

PRECC از تله‌متری ناشناس اختیاری برای بهبود ابزار پشتیبانی می‌کند. هیچ داده‌ای بدون رضایت صریح شما جمع‌آوری نمی‌شود.

## فعال‌سازی

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## غیرفعال‌سازی

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## بررسی وضعیت

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## پیش‌نمایش داده‌هایی که ارسال خواهد شد

قبل از فعال‌سازی، می‌توانید دقیقاً ببینید چه داده‌هایی جمع‌آوری خواهد شد:

```bash
$ precc telemetry preview
Telemetry payload (this session):
{
  "version": "0.3.0",
  "os": "linux",
  "arch": "x86_64",
  "skills_activated": 12,
  "commands_intercepted": 87,
  "pillars_used": [1, 4],
  "avg_hook_latency_ms": 2.3,
  "session_count": 1
}
```

## آنچه جمع‌آوری می‌شود

- نسخه PRECC، سیستم‌عامل و معماری
- شمارش‌های تجمیعی: دستورات رهگیری‌شده، مهارت‌های فعال‌شده، ستون‌های استفاده‌شده
- میانگین تأخیر هوک
- تعداد جلسات

## آنچه جمع‌آوری نمی‌شود

- بدون متن دستور یا آرگومان
- بدون مسیر فایل یا نام پوشه
- بدون نام پروژه یا آدرس مخزن
- بدون اطلاعات شناسایی شخصی (PII)
- بدون آدرس IP (سرور آنها را ثبت نمی‌کند)

## بازنویسی متغیر محیطی

برای غیرفعال کردن تله‌متری بدون اجرای دستور (مفید در CI یا محیط‌های مشترک):

```bash
export PRECC_NO_TELEMETRY=1
```

این بر تنظیمات رضایت اولویت دارد.

## مقصد داده‌ها

داده‌های تله‌متری از طریق HTTPS به `https://telemetry.peria.ai/v1/precc` ارسال می‌شود. داده‌ها فقط برای درک الگوهای استفاده و اولویت‌بندی توسعه استفاده می‌شود.
