# Gyorsindítás

Indítsd el a PRECC-et 5 perc alatt.

## 1. lépés: Telepítés

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## 2. lépés: Inicializálás

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## 3. lépés: Ellenőrizd, hogy a hook aktív

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

## 4. lépés: Használd a Claude Code-ot a szokásos módon

Nyisd meg a Claude Code-ot és dolgozz a szokásos módon. A PRECC csendben fut a háttérben. Amikor Claude egy sikertelen parancsot adna ki, a PRECC kijavítja végrehajtás előtt.

### Példa: Cargo Build rossz könyvtárban

Tegyük fel, hogy a projekted a `~/projects/myapp/` helyen van, és Claude kiadja:

```
cargo build
```

a `~/projects/` könyvtárból (egy szinttel túl magas, nincs ott `Cargo.toml`).

**PRECC nélkül:** Claude megkapja a hibát `could not find Cargo.toml in /home/user/projects or any parent directory`, elolvassa, gondolkodik rajta, majd újrapróbálja: `cd myapp && cargo build`. Költség: ~2000 token elvesztegetva.

**PRECC-kel:** A hook észleli a hiányzó `Cargo.toml`-t, megtalálja a `myapp/` mappában, és átírja a parancsot:

```
cd /home/user/projects/myapp && cargo build
```

Claude soha nem lát hibát. Nulla token elvesztegetva.

## 5. lépés: Ellenőrizd a megtakarításokat

Egy munkamenet után nézd meg, hány tokent takarított meg a PRECC:

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

## Következő lépések

- [Képességek](skills.md) -- Az összes elérhető képesség és saját készítése.
- [Hook Pipeline](hook-pipeline.md) -- Értsd meg, mi történik a háttérben.
- [Megtakarítások](savings.md) -- Részletes token-megtakarítási elemzés.
