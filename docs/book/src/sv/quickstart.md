# Snabbstart

Kom igång med PRECC på 5 minuter.

## Steg 1: Installera

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

## Steg 2: Initiera

```bash
$ precc init
[precc] Hook registered with Claude Code
[precc] Created ~/.local/share/precc/
[precc] Initialized heuristics.db with 8 built-in skills
[precc] Ready.
```

## Steg 3: Verifiera att hook är aktiv

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

## Steg 4: Använd Claude Code normalt

Öppna Claude Code och arbeta som vanligt. PRECC körs tyst i bakgrunden. När Claude kör ett kommando som skulle misslyckas korrigerar PRECC det före körning.

### Exempel: Cargo-bygge i fel katalog

Antag att ditt projekt finns i `~/projects/myapp/` och Claude kör:

```
cargo build
```

från `~/projects/` (en nivå för högt, ingen `Cargo.toml` där).

**Utan PRECC:** Claude får felet `could not find Cargo.toml in /home/user/projects or any parent directory`, läser det, resonerar och försöker igen med `cd myapp && cargo build`. Kostnad: ~2 000 tokens slösade.

**Med PRECC:** Hook upptäcker den saknade `Cargo.toml`, hittar den i `myapp/` och skriver om kommandot till:

```
cd /home/user/projects/myapp && cargo build
```

Claude ser aldrig ett fel. Noll tokens slösade.

## Steg 5: Kontrollera dina besparingar

Efter en session, se hur många tokens PRECC har sparat:

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

## Nästa steg

- [Färdigheter](skills.md) -- Se alla tillgängliga färdigheter och hur du skapar egna.
- [Hook Pipeline](hook-pipeline.md) -- Förstå vad som händer under huven.
- [Besparingar](savings.md) -- Detaljerad tokenbesparingsanalys.
