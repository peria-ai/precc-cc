# شروع سریع

PRECC را در ۵ دقیقه راه‌اندازی کنید.

## مرحله ۱: نصب

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## مرحله ۲: مقداردهی اولیه

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## مرحله ۳: بررسی فعال بودن هوک

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

## مرحله ۴: از Claude Code به طور عادی استفاده کنید

Claude Code را باز کنید و طبق معمول کار کنید. PRECC بی‌صدا در پس‌زمینه اجرا می‌شود. وقتی Claude دستوری صادر می‌کند که شکست می‌خورد، PRECC آن را قبل از اجرا اصلاح می‌کند.

### مثال: اجرای Cargo Build در پوشه اشتباه

فرض کنید پروژه شما در `~/projects/myapp/` است و Claude دستور زیر را صادر می‌کند:

```
cargo build
```

از `~/projects/` (یک سطح بالاتر، بدون `Cargo.toml` در آنجا).

**بدون PRECC:** Claude خطای `could not find Cargo.toml in /home/user/projects or any parent directory` را دریافت می‌کند، می‌خواند، استدلال می‌کند و با `cd myapp && cargo build` دوباره تلاش می‌کند. هزینه: حدود ۲۰۰۰ توکن هدر رفته.

**با PRECC:** هوک `Cargo.toml` گمشده را تشخیص می‌دهد، آن را در `myapp/` پیدا می‌کند و دستور را بازنویسی می‌کند به:

```
cd /home/user/projects/myapp && cargo build
```

Claude هرگز خطایی نمی‌بیند. صفر توکن هدر رفته.

## مرحله ۵: صرفه‌جویی خود را بررسی کنید

پس از یک جلسه، ببینید PRECC چند توکن صرفه‌جویی کرده است:

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

## مراحل بعدی

- [مهارت‌ها](skills.md) -- مشاهده همه مهارت‌های موجود و نحوه ایجاد مهارت‌های خود.
- [خط لوله هوک](hook-pipeline.md) -- بفهمید در پشت صحنه چه اتفاقی می‌افتد.
- [صرفه‌جویی](savings.md) -- تحلیل دقیق صرفه‌جویی توکن.
