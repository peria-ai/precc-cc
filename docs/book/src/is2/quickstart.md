# Hraðleiðbeiningar

Komdu PRECC í gang á 5 mínútum.

## Skref 1: Setja upp

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## Skref 2: Frumstilla

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## Skref 3: Staðfesta að hook sé virkt

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

## Skref 4: Notaðu Claude Code eðlilega

Opnaðu Claude Code og vinndu eins og venjulega. PRECC keyrir hljóðlega í bakgrunni. Þegar Claude gefur út skipun sem myndi mistakast lagar PRECC hana fyrir keyrslu.

### Dæmi: Cargo-smíði í rangri möppu

Segjum að verkefnið þitt sé í `~/projects/myapp/` og Claude gefi út:

```
cargo build
```

frá `~/projects/` (einu stigi of hátt, ekkert `Cargo.toml` þar).

**Án PRECC:** Claude fær villuna `could not find Cargo.toml in /home/user/projects or any parent directory`, les hana, rökhugar og reynir aftur með `cd myapp && cargo build`. Kostnaður: ~2.000 tákn sóuð.

**Með PRECC:** Hook greinir `Cargo.toml` sem vantar, finnur það í `myapp/` og endurskrifar skipunina í:

```
cd /home/user/projects/myapp && cargo build
```

Claude sér aldrei villu. Engin tákn sóuð.

## Skref 5: Athugaðu sparnað þinn

Eftir lotu, sjáðu hversu mörg tákn PRECC sparaði:

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

## Næstu skref

- [Þekking](skills.md) -- Sjá alla tiltæka þekkingu og hvernig á að búa til sína eigin.
- [Hook Pipeline](hook-pipeline.md) -- Skilja hvað gerist undir húddinu.
- [Sparnaður](savings.md) -- Ítarleg greining á táknasparnaði.
