# البدء السريع

شغّل PRECC في 5 دقائق.

## الخطوة 1: التثبيت

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## الخطوة 2: التهيئة

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## الخطوة 3: تحقق من أن الخطاف نشط

```bash
$ precc skills list
  # Name               Type      Triggers
  1 cargo-wrong-dir    built-in  cargo build/test/clippy outside Rust project
  2 git-wrong-dir      built-in  git * outside a repo
  3 go-wrong-dir       built-in  go build/test outside Go module
  4 make-wrong-dir     built-in  make without Makefile in cwd
  5 npm-wrong-dir      built-in  npm/npx/pnpm/yarn outside Node project
  6 python-wrong-dir   built-in  python/pytest/pip outside Python project
  7 jj-translate       built-in  git * in jj-colocated repo
  8 asciinema-gif      built-in  asciinema rec
```

## الخطوة 4: استخدم Claude Code بشكل طبيعي

افتح Claude Code واعمل كالمعتاد. يعمل PRECC بصمت في الخلفية. عندما يصدر Claude أمراً سيفشل، يقوم PRECC بتصحيحه قبل التنفيذ.

### مثال: تشغيل Cargo Build في المجلد الخطأ

افترض أن مشروعك في `~/projects/myapp/` وأصدر Claude:

```
cargo build
```

من `~/projects/` (مستوى واحد أعلى من اللازم، لا يوجد `Cargo.toml` هناك).

**بدون PRECC:** يحصل Claude على الخطأ `could not find Cargo.toml in /home/user/projects or any parent directory`، يقرأه، يفكر فيه، ثم يعيد المحاولة بـ `cd myapp && cargo build`. التكلفة: ~2,000 رمز مهدر.

**مع PRECC:** يكتشف الخطاف `Cargo.toml` المفقود، يجده في `myapp/`، ويعيد كتابة الأمر إلى:

```
cd /home/user/projects/myapp && cargo build
```

لا يرى Claude أي خطأ أبداً. صفر رموز مهدرة.

## الخطوة 5: تحقق من مدخراتك

بعد الجلسة، تحقق من عدد الرموز التي وفرها PRECC:

```bash
$ precc savings
Session Token Savings
=====================
Total estimated savings: 4,312 tokens

Breakdown:
  Pillar 1 (cd prepends):       2,104 tokens  (3 corrections)
  Pillar 4 (skill activations):   980 tokens  (2 activations)
  RTK rewrites:                 1,228 tokens  (5 rewrites)
```

## الخطوات التالية

- [المهارات](skills.md) -- عرض جميع المهارات المتاحة وكيفية إنشاء مهاراتك الخاصة.
- [خط أنابيب الخطاف](hook-pipeline.md) -- افهم ما يحدث خلف الكواليس.
- [المدخرات](savings.md) -- تحليل مفصل لتوفير الرموز.
