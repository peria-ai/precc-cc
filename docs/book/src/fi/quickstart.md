# Pikaopas

Ota PRECC käyttöön 5 minuutissa.

## Vaihe 1: Asenna

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## Vaihe 2: Alusta

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## Vaihe 3: Vahvista hookin olevan aktiivinen

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

## Vaihe 4: Käytä Claude Codea normaalisti

Avaa Claude Code ja työskentele normaalisti. PRECC toimii hiljaisesti taustalla. Kun Claude suorittaa komennon joka epäonnistuisi, PRECC korjaa sen ennen suoritusta.

### Esimerkki: Cargo-koontiversion suoritus väärässä hakemistossa

Oletetaan, että projektisi on hakemistossa `~/projects/myapp/` ja Claude suorittaa:

```
cargo build
```

hakemistosta `~/projects/` (taso liian ylhäällä, ei `Cargo.toml`-tiedostoa siellä).

**Ilman PRECCiä:** Claude saa virheen `could not find Cargo.toml in /home/user/projects or any parent directory`, lukee sen, päättelee ja yrittää uudelleen komennolla `cd myapp && cargo build`. Hinta: ~2 000 hukkaan mennyttä tokenia.

**PRECCin kanssa:** Hook havaitsee puuttuvan `Cargo.toml`-tiedoston, löytää sen hakemistosta `myapp/` ja kirjoittaa komennon uudelleen:

```
cd /home/user/projects/myapp && cargo build
```

Claude ei koskaan näe virhettä. Nolla hukkaan mennyttä tokenia.

## Vaihe 5: Tarkista säästösi

Istunnon jälkeen katso kuinka monta tokenia PRECC säästi:

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

## Seuraavat vaiheet

- [Taidot](skills.md) -- Katso kaikki käytettävissä olevat taidot ja miten luot omia.
- [Hook Pipeline](hook-pipeline.md) -- Ymmärrä mitä tapahtuu konepellin alla.
- [Säästöt](savings.md) -- Yksityiskohtainen tokenisäästöanalyysi.
