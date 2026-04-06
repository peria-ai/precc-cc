# تحليل GitHub Actions

يقوم `precc gha` بتحليل عمليات تشغيل GitHub Actions الفاشلة ويقترح إصلاحات. هذه ميزة Pro.

## الاستخدام

مرر عنوان URL لتشغيل GitHub Actions الفاشل:

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

## ماذا يفعل

1. يحلل عنوان URL لتشغيل GitHub Actions لاستخراج المالك والمستودع ومعرف التشغيل.
2. يجلب سجلات التشغيل عبر واجهة GitHub API (يستخدم `GITHUB_TOKEN` إذا تم تعيينه، وإلا الوصول العام).
3. يحدد الخطوة الفاشلة ويستخرج أسطر الخطأ ذات الصلة.
4. يحلل الخطأ ويقترح إصلاحاً بناءً على أنماط فشل CI الشائعة.

## أنماط الفشل المدعومة

- حاويات خدمة مفقودة (قواعد بيانات، Redis، إلخ.)
- نظام تشغيل أو بنية المشغل غير صحيحة
- متغيرات بيئة أو أسرار مفقودة
- فشل تثبيت التبعيات
- انتهاء مهلة الاختبارات
- أخطاء الأذونات
- إخفاقات التخزين المؤقت التي تسبب بناء بطيء
