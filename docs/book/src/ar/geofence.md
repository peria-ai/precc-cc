# السياج الجغرافي

يتضمن PRECC فحص الامتثال للسياج الجغرافي IP للبيئات المنظمة. هذه ميزة Pro.

## نظرة عامة

تتطلب بعض المنظمات أن تعمل أدوات التطوير فقط ضمن مناطق جغرافية معتمدة. تتحقق ميزة السياج الجغرافي في PRECC من أن عنوان IP للجهاز الحالي يقع ضمن قائمة المناطق المسموح بها.

## التحقق من الامتثال

```bash
$ precc geofence check
[precc] Current IP: 203.0.113.42
[precc] Region: US-East (Virginia)
[precc] Status: COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
```

إذا كان الجهاز خارج المناطق المسموح بها:

```bash
$ precc geofence check
[precc] Current IP: 198.51.100.7
[precc] Region: AP-Southeast (Singapore)
[precc] Status: NON-COMPLIANT
[precc] Policy: us-east-1, us-west-2, eu-west-1
[precc] Warning: Current region is not in the allowed list.
```

## تحديث بيانات السياج الجغرافي

```bash
$ precc geofence refresh
[precc] Fetching updated IP geolocation data...
[precc] Updated. Cache expires in 24h.
```

## عرض معلومات السياج الجغرافي

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

## مسح ذاكرة التخزين المؤقت

```bash
$ precc geofence clear
[precc] Geofence cache cleared.
```

## التكوين

يتم تعريف سياسة السياج الجغرافي في `~/.config/precc/geofence.toml`:

```toml
[geofence]
allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"]
check_on_init = true
block_on_violation = false
```

اضبط `block_on_violation = true` لمنع PRECC من العمل عند التواجد خارج المناطق المسموح بها.
