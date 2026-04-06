# Hızlı Başlangıç

PRECC'i 5 dakikada çalıştırın.

## Adım 1: Kurulum

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## Adım 2: Başlatma

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## Adım 3: Hook'un aktif olduğunu doğrulayın

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

## Adım 4: Claude Code'u normal şekilde kullanın

Claude Code'u açın ve her zamanki gibi çalışın. PRECC arka planda sessizce çalışır. Claude başarısız olacak bir komut verdiğinde, PRECC bunu çalıştırmadan önce düzeltir.

### Örnek: Yanlış dizinde Cargo Build

Projenizin `~/projects/myapp/` konumunda olduğunu ve Claude'un şunu çalıştırdığını varsayın:

```
cargo build
```

`~/projects/` dizininden (bir seviye yukarıda, orada `Cargo.toml` yok).

**PRECC olmadan:** Claude `could not find Cargo.toml in /home/user/projects or any parent directory` hatasını alır, okur, düşünür ve `cd myapp && cargo build` ile tekrar dener. Maliyet: ~2.000 token israf.

**PRECC ile:** Hook eksik `Cargo.toml`'u algılar, `myapp/` içinde bulur ve komutu şu şekilde yeniden yazar:

```
cd /home/user/projects/myapp && cargo build
```

Claude hiçbir zaman hata görmez. Sıfır token israf.

## Adım 5: Tasarruflarınızı kontrol edin

Bir oturumdan sonra PRECC'in kaç token tasarruf ettiğini görün:

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

## Sonraki adımlar

- [Yetenekler](skills.md) -- Tüm mevcut yetenekleri ve kendi yeteneklerinizi nasıl oluşturacağınızı görün.
- [Hook Pipeline](hook-pipeline.md) -- Kaputun altında neler olduğunu anlayın.
- [Tasarruflar](savings.md) -- Ayrıntılı token tasarruf analizi.
