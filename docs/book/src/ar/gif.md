# تسجيل GIF

يقوم `precc gif` بإنشاء تسجيلات GIF متحركة لجلسات الطرفية من نصوص bash. هذه ميزة Pro.

## الاستخدام الأساسي

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

الوسيط الأول هو نص bash يحتوي على الأوامر المراد تشغيلها. الوسيط الثاني هو الحد الأقصى لطول التسجيل.

## تنسيق النص

النص هو ملف bash قياسي:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## محاكاة الإدخال

للأوامر التفاعلية، قدم قيم الإدخال كوسائط إضافية:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

يتم تقديم كل وسيط إضافي كسطر stdin عندما يطلب النص إدخالاً.

## خيارات الإخراج

يتم تسمية ملف الإخراج باسم النص بشكل افتراضي (`script.gif`). يستخدم GIF سمة طرفية داكنة بأبعاد قياسية 80x24.

## لماذا GIF بدلاً من asciinema؟

تقوم مهارة `asciinema-gif` المدمجة بإعادة كتابة `asciinema rec` تلقائياً إلى `precc gif`. ملفات GIF أكثر قابلية للنقل -- تُعرض مباشرة في ملفات GitHub README وSlack والبريد الإلكتروني دون الحاجة إلى مشغل.
