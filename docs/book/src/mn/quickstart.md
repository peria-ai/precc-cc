# Хурдан эхлэх

PRECC-ийг 5 минутад ажиллуулна уу.

## Алхам 1: Суулгах

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## Алхам 2: Эхлүүлэх

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## Алхам 3: Hook идэвхтэй эсэхийг шалгах

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

## Алхам 4: Claude Code-ийг ердийнхөөрөө ашиглах

Claude Code-ийг нээж ердийнхөөрөө ажиллана уу. PRECC арын дэвсгэр дээр чимээгүй ажиллана. Claude амжилтгүй болох тушаал өгөхөд PRECC гүйцэтгэхээс өмнө засна.

### Жишээ: Буруу хавтаст Cargo Build хийх

Таны төсөл `~/projects/myapp/` дотор байгаа бөгөөд Claude дараахийг гүйцэтгэнэ гэж бодъё:

```
cargo build
```

`~/projects/` хавтаснаас (нэг түвшин өндөр, тэнд `Cargo.toml` байхгүй).

**PRECC-гүйгээр:** Claude `could not find Cargo.toml in /home/user/projects or any parent directory` алдааг авч, уншиж, боддог, дараа нь `cd myapp && cargo build`-ээр дахин оролдоно. Зардал: ~2,000 токен үрэгдэнэ.

**PRECC-тэй:** Hook дутуу `Cargo.toml`-ийг илрүүлж, `myapp/` дотор олж, тушаалыг дараах болгон дахин бичнэ:

```
cd /home/user/projects/myapp && cargo build
```

Claude алдаа хэзээ ч харахгүй. Тэг токен үрэгдэнэ.

## Алхам 5: Хэмнэлтээ шалгах

Сессийн дараа PRECC хэдэн токен хэмнэснийг харна уу:

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

## Дараагийн алхамууд

- [Чадварууд](skills.md) -- Бүх боломжтой чадваруудыг болон өөрийнхөө чадварыг хэрхэн үүсгэхийг харна уу.
- [Hook Pipeline](hook-pipeline.md) -- Дотор нь юу болж байгааг ойлго.
- [Хэмнэлт](savings.md) -- Токен хэмнэлтийн дэлгэрэнгүй шинжилгээ.
