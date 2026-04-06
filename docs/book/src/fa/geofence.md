# حصار جغرافیایی

PRECC شامل بررسی انطباق حصار جغرافیایی IP برای محیط‌های تحت نظارت است. این یک ویژگی Pro است.

## مرور کلی

برخی سازمان‌ها نیاز دارند که ابزارهای توسعه فقط در مناطق جغرافیایی تایید شده کار کنند. ویژگی حصار جغرافیایی PRECC تأیید می‌کند که آدرس IP دستگاه فعلی در لیست مناطق مجاز قرار دارد.

## بررسی انطباق

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

اگر دستگاه خارج از مناطق مجاز باشد:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## بروزرسانی داده‌های حصار جغرافیایی

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## مشاهده اطلاعات حصار جغرافیایی

```bash
$ precc geofence info
Geofence Configuration
======================
Policy file:    ~/.config/precc/geofence.toml
Allowed regions: us-east-1, us-west-2, eu-west-1
Cache age:      2h 14m
Last check:     2026-04-03 09:12:00 UTC
Status:         COMPLIANT
```

## پاک کردن حافظه پنهان

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## پیکربندی

سیاست حصار جغرافیایی در `~/.config/precc/geofence.toml` تعریف شده است:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

مقدار `block_on_violation = true` را تنظیم کنید تا از عملکرد PRECC در خارج از مناطق مجاز جلوگیری شود.
