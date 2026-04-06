# القياس عن بعد

يدعم PRECC القياس عن بعد المجهول الاختياري للمساعدة في تحسين الأداة. لا يتم جمع أي بيانات ما لم توافق صراحةً.

## الاشتراك

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## إلغاء الاشتراك

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## التحقق من الحالة

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## معاينة البيانات التي سيتم إرسالها

قبل الاشتراك، يمكنك رؤية البيانات التي سيتم جمعها بالضبط:

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

## ما يتم جمعه

- إصدار PRECC ونظام التشغيل والبنية
- أعداد مجمعة: الأوامر المعترضة، المهارات المفعلة، الركائز المستخدمة
- متوسط زمن الاستجابة
- عدد الجلسات

## ما لا يتم جمعه

- لا نصوص أوامر أو معاملات
- لا مسارات ملفات أو أسماء مجلدات
- لا أسماء مشاريع أو عناوين مستودعات
- لا معلومات تعريف شخصية (PII)
- لا عناوين IP (الخادم لا يسجلها)

## تجاوز متغير البيئة

لتعطيل القياس عن بعد بدون تنفيذ أمر (مفيد في CI أو البيئات المشتركة):

```bash
export PRECC_NO_TELEMETRY=1
```

هذا له الأولوية على إعداد الموافقة.

## وجهة البيانات

يتم إرسال بيانات القياس عن بعد إلى `https://telemetry.peria.ai/v1/precc` عبر HTTPS. تُستخدم البيانات فقط لفهم أنماط الاستخدام وتحديد أولويات التطوير.
