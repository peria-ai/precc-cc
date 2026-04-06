# Szybki start

Uruchom PRECC w 5 minut.

## Krok 1: Instalacja

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## Krok 2: Inicjalizacja

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## Krok 3: Sprawdź, czy hook jest aktywny

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

## Krok 4: Używaj Claude Code normalnie

Otwórz Claude Code i pracuj jak zwykle. PRECC działa cicho w tle. Gdy Claude wyda polecenie, które by się nie powiodło, PRECC poprawia je przed wykonaniem.

### Przykład: Cargo Build w złym katalogu

Załóżmy, że twój projekt jest w `~/projects/myapp/`, a Claude wydaje:

```
cargo build
```

z `~/projects/` (jeden poziom za wysoko, brak tam `Cargo.toml`).

**Bez PRECC:** Claude otrzymuje błąd `could not find Cargo.toml in /home/user/projects or any parent directory`, czyta go, analizuje i ponawia próbę z `cd myapp && cargo build`. Koszt: ~2000 tokenów zmarnowanych.

**Z PRECC:** Hook wykrywa brakujący `Cargo.toml`, znajduje go w `myapp/` i przepisuje polecenie na:

```
cd /home/user/projects/myapp && cargo build
```

Claude nigdy nie widzi błędu. Zero zmarnowanych tokenów.

## Krok 5: Sprawdź oszczędności

Po sesji sprawdź, ile tokenów zaoszczędził PRECC:

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

## Kolejne kroki

- [Umiejętności](skills.md) -- Zobacz wszystkie dostępne umiejętności i jak tworzyć własne.
- [Pipeline Hooka](hook-pipeline.md) -- Zrozum, co dzieje się pod maską.
- [Oszczędności](savings.md) -- Szczegółowa analiza oszczędności tokenów.
