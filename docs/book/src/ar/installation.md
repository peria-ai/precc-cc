# التثبيت

## التثبيت السريع (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

يقوم هذا بتنزيل أحدث إصدار ثنائي لمنصتك، والتحقق من المجموع الاختباري SHA256، ووضعه في `~/.local/bin/`.

بعد التثبيت، قم بتهيئة PRECC:

```bash
precc init
```

يقوم `precc init` بتسجيل خطاف PreToolUse مع Claude Code، وإنشاء أدلة البيانات، وتهيئة قاعدة بيانات المهارات.

## خيارات التثبيت

### التحقق من SHA256

بشكل افتراضي، يتحقق المثبت من المجموع الاختباري للملف الثنائي مقابل مجموع SHA256 المنشور. لتخطي التحقق (غير مستحسن):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### بادئة تثبيت مخصصة

التثبيت في موقع مخصص:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### أدوات مصاحبة (--extras)

يأتي PRECC مع أدوات مصاحبة اختيارية. قم بتثبيتها باستخدام `--extras`:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

هذا يثبت:

| أداة | الغرض |
|------|---------|
| **RTK** | مجموعة أدوات إعادة كتابة الأوامر |
| **lean-ctx** | ضغط السياق لملفات CLAUDE.md والتعليمات |
| **nushell** | صدفة منظمة لخطوط الأنابيب المتقدمة |
| **cocoindex-code** | فهرسة الكود لحل السياق بشكل أسرع |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

ثم قم بالتهيئة:

```powershell
precc init
```

## التثبيت اليدوي

1. قم بتنزيل الملف الثنائي لمنصتك من [GitHub Releases](https://github.com/peria-ai/precc-cc/releases).
2. تحقق من المجموع الاختباري SHA256 مقابل ملف `.sha256` في الإصدار.
3. ضع الملف الثنائي في دليل ضمن `PATH` الخاص بك (مثل `~/.local/bin/`).
4. قم بتشغيل `precc init`.

## التحديث

```bash
precc update
```

فرض التحديث إلى إصدار محدد:

```bash
precc update --force --version 0.3.0
```

تمكين التحديثات التلقائية:

```bash
precc update --auto
```

## التحقق من التثبيت

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

إذا لم يتم العثور على `precc`، تأكد من أن `~/.local/bin` موجود في `PATH` الخاص بك.
