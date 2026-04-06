# المهارات

المهارات هي قواعد مطابقة الأنماط التي يستخدمها PRECC لاكتشاف الأوامر وتصحيحها. يمكن أن تكون مدمجة (موزعة كملفات TOML) أو مستخرجة من سجلات الجلسات.

## المهارات المدمجة

| المهارة | يُفعَّل عند | الإجراء |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` خارج مشروع Rust | إضافة `cd` إلى أقرب دليل `Cargo.toml` |
| `git-wrong-dir` | `git *` خارج مستودع git | إضافة `cd` إلى أقرب دليل `.git` |
| `go-wrong-dir` | `go build/test` خارج وحدة Go | إضافة `cd` إلى أقرب دليل `go.mod` |
| `make-wrong-dir` | `make` بدون Makefile في الدليل الحالي | إضافة `cd` إلى أقرب دليل Makefile |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` خارج مشروع Node | إضافة `cd` إلى أقرب دليل `package.json` |
| `python-wrong-dir` | `python/pytest/pip` خارج مشروع Python | إضافة `cd` إلى أقرب مشروع Python |
| `jj-translate` | `git *` في مستودع jj مشترك | إعادة الكتابة إلى أمر `jj` المكافئ |
| `asciinema-gif` | `asciinema rec` | إعادة الكتابة إلى `precc gif` |

## قائمة المهارات

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
  9 fix-pytest-path    mined     pytest with wrong test path
```

## عرض تفاصيل المهارة

```bash
$ precc skills show cargo-wrong-dir
Name:        cargo-wrong-dir
Type:        built-in
Source:      skills/builtin/cargo-wrong-dir.toml
Description: Detects cargo commands run outside a Rust project and prepends
             cd to the directory containing the nearest Cargo.toml.
Trigger:     ^cargo\s+(build|test|clippy|run|check|bench|doc)
Action:      prepend_cd
Marker:      Cargo.toml
Activations: 12
```

## تصدير مهارة إلى TOML

```bash
$ precc skills export cargo-wrong-dir
[skill]
name = "cargo-wrong-dir"
description = "Prepend cd for cargo commands outside a Rust project"
trigger = "^cargo\\s+(build|test|clippy|run|check|bench|doc)"
action = "prepend_cd"
marker = "Cargo.toml"
priority = 10
```

## تحرير مهارة

```bash
$ precc skills edit cargo-wrong-dir
```

يفتح هذا تعريف المهارة في `$EDITOR` الخاص بك. بعد الحفظ، يتم إعادة تحميل المهارة تلقائيًا.

## أمر Advise

يقوم `precc skills advise` بتحليل جلستك الأخيرة ويقترح مهارات جديدة بناءً على الأنماط المتكررة:

```bash
$ precc skills advise
Analyzed 47 commands from the last session.

Suggested skills:
  1. docker-wrong-dir: You ran `docker compose up` outside the project root 3 times.
     Suggested trigger: ^docker\s+compose
     Suggested marker: docker-compose.yml

  2. terraform-wrong-dir: You ran `terraform plan` outside the infra directory 2 times.
     Suggested trigger: ^terraform\s+(plan|apply|init)
     Suggested marker: main.tf

Accept suggestion [1/2/skip]?
```

## تجميع المهارات

```bash
$ precc skills cluster
```

يجمع المهارات المستخرجة المتشابهة معًا للمساعدة في تحديد الأنماط المتكررة أو المتداخلة.

## المهارات المستخرجة مقابل المدمجة

المهارات المدمجة تأتي مع PRECC ومحددة في `skills/builtin/*.toml`. تغطي أكثر أخطاء الدليل الخاطئ شيوعًا.

المهارات المستخرجة يتم إنشاؤها بواسطة `precc ingest` أو خدمة `precc-learner` من سجلات جلساتك. يتم تخزينها في `~/.local/share/precc/heuristics.db` وهي خاصة بسير عملك. انظر [التعدين](mining.md) للتفاصيل.
