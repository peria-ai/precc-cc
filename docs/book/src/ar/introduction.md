# مقدمة

## ما هو PRECC؟

PRECC (التصحيح التنبؤي للأخطاء لـ Claude Code) هو أداة Rust تعترض أوامر bash في Claude Code عبر آلية خطاف PreToolUse الرسمية. يصلح الأخطاء *قبل حدوثها*، يوفر الرموز ويزيل حلقات إعادة المحاولة.

مجاني لمستخدمي المجتمع.

## المشكلة

يهدر Claude Code رموزاً كبيرة على أخطاء يمكن تجنبها:

- **أخطاء المجلد** -- تشغيل `cargo build` بدون `Cargo.toml`
- **حلقات إعادة المحاولة** -- أمر فاشل ينتج مخرجات مطولة
- **مخرجات مطولة** -- أوامر مثل `find` أو `ls -R` تنتج آلاف الأسطر

## الأركان الأربعة

### إصلاح السياق (cd-prepend)

يكتشف عندما تعمل أوامر مثل `cargo build` أو `npm test` في المجلد الخطأ ويضيف `cd /المسار/الصحيح &&` قبل التنفيذ.

### تصحيح أخطاء GDB

يكتشف فرص ربط GDB لتصحيح أعمق.

### تعدين الجلسات

يحلل سجلات جلسات Claude Code للعثور على أزواج الخطأ-الإصلاح.

### مهارات الأتمتة

مكتبة من المهارات التي تطابق أنماط الأوامر وتعيد كتابتها.

## كيف يعمل (نسخة 30 ثانية)

1. Claude Code على وشك تشغيل أمر bash.
2. يرسل خطاف PreToolUse الأمر كـ JSON.
3. يعالج `precc-hook` الأمر في أقل من 3 مللي ثانية.
4. يُعاد الأمر المصحح كـ JSON.
5. ينفذ Claude Code الأمر المصحح.

لا يرى Claude الخطأ أبداً.

### الضغط التكيفي

إذا فشل أمر بعد الضغط، يتخطى PRECC الضغط تلقائياً في المحاولة التالية حتى يحصل Claude على المخرجات الكاملة غير المضغوطة لتصحيح الأخطاء.

## إحصائيات الاستخدام المباشرة

الإصدار الحالي <span data-stat="current_version">--</span>:

| المقياس | القيمة |
|---|---|
| استدعاءات الخطاف | <span data-stat="total_invocations">--</span> |
| الرموز المحفوظة | <span data-stat="total_tokens_saved">--</span> |
| نسبة التوفير | <span data-stat="saving_pct">--</span>% |
| عمليات إعادة كتابة RTK | <span data-stat="rtk_rewrites">--</span> |
| تصحيحات CD | <span data-stat="cd_prepends">--</span> |
| زمن استجابة الخطاف | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| المستخدمون | <span data-stat="unique_users">--</span> |

### التوفير حسب الإصدار

<table id="version-breakdown" style="display:none">
<thead><tr><th>الإصدار</th><th>المستخدمون</th><th>استدعاءات الخطاف</th><th>الرموز المحفوظة</th><th>نسبة التوفير</th></tr></thead>
<tbody><tr><td colspan="5"><em>جاري التحميل...</em></td></tr></tbody>
</table>

<small>الأرقام تقديرات. كل فشل تم منعه يتجنب دورة إعادة محاولة كاملة: مخرجات الخطأ، تفكير النموذج، وأمر إعادة المحاولة. تُحدّث هذه الأرقام تلقائياً من القياسات المجهولة.</small>

## روابط

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- الموقع: [https://peria.ai](https://peria.ai)
- التوثيق: [https://precc.cc](https://precc.cc)
