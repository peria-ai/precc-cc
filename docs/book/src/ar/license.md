# الترخيص

يقدم PRECC مستويين: Community (مجاني) و Pro.

## مستوى Community (مجاني)

يتضمن مستوى Community:

- جميع المهارات المدمجة (تصحيح الدليل الخاطئ، ترجمة jj، إلخ)
- خط أنابيب الخطاف مع دعم كامل لـ Pillar 1 و Pillar 4
- ملخص أساسي لـ `precc savings`
- تنقيب الجلسات مع `precc ingest`
- استخدام محلي غير محدود

## مستوى Pro

يفتح Pro ميزات إضافية:

- **تفصيل مفصل للتوفير** -- `precc savings --all` مع تحليل لكل أمر
- **تسجيل GIF** -- `precc gif` لإنشاء صور GIF متحركة للطرفية
- **الامتثال للسياج الجغرافي IP** -- للبيئات المنظمة
- **تقارير البريد الإلكتروني** -- `precc mail report` لإرسال التحليلات
- **تحليل GitHub Actions** -- `precc gha` لتصحيح أخطاء سير العمل الفاشلة
- **ضغط السياق** -- `precc compress` لتحسين CLAUDE.md
- **دعم ذو أولوية**

## تفعيل الترخيص

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## التحقق من حالة الترخيص

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## تفعيل GitHub Sponsors

إذا كنت ترعى PRECC عبر GitHub Sponsors، يتم تفعيل ترخيصك تلقائيًا عبر بريد GitHub الخاص بك. لا حاجة لمفتاح -- فقط تأكد من تطابق بريد الراعي:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## بصمة الجهاز

كل ترخيص مرتبط ببصمة جهاز. اعرض بصمتك باستخدام:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

إذا كنت بحاجة إلى نقل ترخيصك إلى جهاز جديد، قم بإلغاء التفعيل أولاً:

```bash
precc license deactivate
```

ثم قم بالتفعيل على الجهاز الجديد.

## هل انتهت صلاحية الترخيص؟

عندما تنتهي صلاحية ترخيص Pro، يعود PRECC إلى مستوى Community. تستمر جميع المهارات المدمجة والوظائف الأساسية في العمل. فقط الميزات الخاصة بـ Pro تصبح غير متاحة. راجع [الأسئلة الشائعة](faq.md) لمزيد من التفاصيل.
