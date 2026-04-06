# الضغط

يقوم `precc compress` بتقليص CLAUDE.md وملفات السياق الأخرى لتقليل استخدام الرموز عندما يحملها Claude Code. هذه ميزة Pro.

## الاستخدام الأساسي

```bash
$ precc compress .
[precc] Scanning directory: .
[precc] Found 3 context files:
         CLAUDE.md (2,847 tokens -> 1,203 tokens, -57.7%)
         ARCHITECTURE.md (4,112 tokens -> 2,044 tokens, -50.3%)
         ALTERNATIVES.md (3,891 tokens -> 1,967 tokens, -49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
[precc] Files compressed. Use --revert to restore originals.
```

## تشغيل تجريبي

معاينة ما سيتغير دون تعديل الملفات:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## الاستعادة

يتم نسخ الأصول احتياطيًا تلقائيًا. لاستعادتها:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## ما الذي يتم ضغطه

يطبق الضاغط عدة تحويلات:

- يزيل المسافات الفارغة والأسطر الفارغة الزائدة
- يختصر الصياغة المطولة مع الحفاظ على المعنى
- يكثف الجداول والقوائم
- يزيل التعليقات والتنسيق الزخرفي
- يحافظ على جميع كتل الكود والمسارات والمعرفات التقنية

المخرجات المضغوطة لا تزال قابلة للقراءة -- ليست مصغرة أو مبهمة.

## استهداف ملفات محددة

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
