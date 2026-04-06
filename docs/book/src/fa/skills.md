# مهارت‌ها

مهارت‌ها قوانین تطبیق الگو هستند که PRECC برای شناسایی و اصلاح دستورات استفاده می‌کند. آن‌ها می‌توانند داخلی (به صورت فایل‌های TOML) یا استخراج‌شده از لاگ‌های جلسه باشند.

## مهارت‌های داخلی

| مهارت | فعال‌سازی | عملکرد |
|-------|-------------|--------|
| `cargo-wrong-dir` | `cargo build/test/clippy` خارج از پروژه Rust | افزودن `cd` به نزدیک‌ترین پوشه `Cargo.toml` |
| `git-wrong-dir` | `git *` خارج از مخزن git | افزودن `cd` به نزدیک‌ترین پوشه `.git` |
| `go-wrong-dir` | `go build/test` خارج از ماژول Go | افزودن `cd` به نزدیک‌ترین پوشه `go.mod` |
| `make-wrong-dir` | `make` بدون Makefile در پوشه فعلی | افزودن `cd` به نزدیک‌ترین پوشه Makefile |
| `npm-wrong-dir` | `npm/npx/pnpm/yarn` خارج از پروژه Node | افزودن `cd` به نزدیک‌ترین پوشه `package.json` |
| `python-wrong-dir` | `python/pytest/pip` خارج از پروژه Python | افزودن `cd` به نزدیک‌ترین پروژه Python |
| `jj-translate` | `git *` در مخزن jj هم‌مکان | بازنویسی به دستور `jj` معادل |
| `asciinema-gif` | `asciinema rec` | بازنویسی به `precc gif` |

## فهرست مهارت‌ها

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

## نمایش جزئیات مهارت

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

## صادرکردن مهارت به TOML

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

## ویرایش مهارت

```bash
$ precc skills edit cargo-wrong-dir
```

این تعریف مهارت را در `$EDITOR` شما باز می‌کند. پس از ذخیره، مهارت به طور خودکار بارگذاری مجدد می‌شود.

## دستور Advise

`precc skills advise` جلسه اخیر شما را تحلیل می‌کند و مهارت‌های جدید را بر اساس الگوهای تکراری پیشنهاد می‌دهد:

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

## خوشه‌بندی مهارت‌ها

```bash
$ precc skills cluster
```

مهارت‌های استخراج‌شده مشابه را گروه‌بندی می‌کند تا الگوهای تکراری یا هم‌پوشان را شناسایی کند.

## مهارت‌های استخراج‌شده در مقابل داخلی

مهارت‌های داخلی همراه PRECC ارائه می‌شوند و در `skills/builtin/*.toml` تعریف شده‌اند. آن‌ها رایج‌ترین خطاهای پوشه نادرست را پوشش می‌دهند.

مهارت‌های استخراج‌شده توسط `precc ingest` یا دیمن `precc-learner` از لاگ‌های جلسه شما ایجاد می‌شوند. در `~/.local/share/precc/heuristics.db` ذخیره می‌شوند و مختص گردش کار شما هستند. برای جزئیات [استخراج](mining.md) را ببینید.
